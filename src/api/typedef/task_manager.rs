use std::{cell::RefCell, cmp::min, error::Error, rc::Rc, sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicUsize, Ordering}}, thread, time::Duration};

use slint::{Model, ModelRc, Timer, TimerMode, VecModel};

use crate::{generated::{DialogSeverity, TaskData, TaskState, TasksGroup}, logic::safe, ui::dialogs::present_dialog_standalone};

static NEXT_TASK_ID: AtomicI32 = AtomicI32::new(0);

pub trait Task: Send + Sync + 'static {
    fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn get_shared_state(&self) -> Arc<SharedTaskState>;
}

pub struct SharedTaskState {
    pub total: AtomicUsize,
    pub progress: AtomicUsize,
    pub state: AtomicU8,
}

impl SharedTaskState {
    pub fn new() -> Self {
        Self { total: AtomicUsize::new(0), progress: AtomicUsize::new(0), state: AtomicU8::new(TaskState::Waiting.into()) }
    }

    pub fn get_progress(&self) -> f32 {
        self.progress.load(Ordering::Relaxed) as f32 / self.total.load(Ordering::Relaxed) as f32
    }

    fn claim(&self) {
        self.state.store(TaskState::Starting.into(), Ordering::Relaxed);
    }
}

pub struct TaskManager {
    groups_ui: Rc<VecModel<TasksGroup>>,
    tasks: Arc<Mutex<Vec<(i32, Arc<SharedTaskState>, Option<Box<dyn Task>>)>>>,
    timer: Rc<Timer>,
    running: AtomicBool,
    semaphore: Arc<Semaphore>,
    threads: i16,
    on_finish: Rc<RefCell<Option<Box<dyn FnMut() + Send + Sync + 'static>>>>
}

impl TaskManager {
    pub fn new(model: Rc<VecModel<TasksGroup>>) -> Self {
        let threads = thread::available_parallelism().unwrap().get() as i16;

        Self {
            groups_ui: model,
            tasks: Arc::new(Mutex::new(vec![])),
            timer: Rc::new(Timer::default()),
            running: AtomicBool::new(false),
            semaphore: Arc::new(Semaphore::new(0)),
            threads,
            on_finish: Rc::new(RefCell::new(None))
        }
    }

    fn start_threads(&self) {
        for _ in 0..self.threads - 1 {
            let semaphore = self.semaphore.clone();
            let map = self.tasks.clone();

            thread::spawn(move || {
                loop {
                    let mut guard = map.lock().unwrap();
                    if guard.is_empty() {
                        return;
                    }

                    let mut idx = None;
                    for i in 0..guard.len() {
                        let entry = &guard[i];
                        if entry.2.is_some() {
                            idx = Some(i);
                            break;
                        }
                    }

                    if idx.is_none() {
                        drop(guard);
                        semaphore.acquire();
                        semaphore.release();
                        continue;
                    }

                    let idx = idx.unwrap();
                    let (id, shared_state, task) = &mut guard[idx];
                    let id = *id;

                    shared_state.claim();
                    let mut task = task.take().unwrap();
                    drop(guard);
                    semaphore.acquire();
                    if let Err(err) = task.run() {
                        safe(move || present_dialog_standalone("Task Error", format!("Error performing task: {}", err.to_string()).as_str(), DialogSeverity::Error));
                    }

                    let mut guard = map.lock().unwrap();
                    for i in 0..guard.len() {
                        if guard[i].0 == id {
                            guard.remove(i);
                            break;
                        }
                    }
                    semaphore.release();
                }
            });
        }
    }

    fn start_running(&self) {
        self.running.store(true, Ordering::Relaxed);
        let groups_ui = self.groups_ui.clone();
        let tasks = self.tasks.clone();
        let timer = self.timer.clone();
        let on_finish = self.on_finish.clone();
        let semaphore = self.semaphore.clone();
        let threads = self.threads;

        self.timer.start(TimerMode::Repeated, Duration::from_millis(150), move || {
            if groups_ui.row_count() == 0 {
                if let Some(f) = &mut *on_finish.borrow_mut() { f(); }

                on_finish.replace(None);
                semaphore.release_all();
                timer.stop();
                return;
            }

            let tasks_map = tasks.lock().unwrap();

            for i in 0..min(threads as usize, groups_ui.row_count()) {
                let group = groups_ui.row_data(i).unwrap();
                let mut j = 0;
                while j < min(threads as usize, group.tasks.row_count()) {
                    let mut task = group.tasks.row_data(j).unwrap();
                    let handle = tasks_map.iter().find(|i| i.0 == task.id);

                    if handle.is_none() {
                        group.get_model().remove(j);
                        continue;
                    }
                    let (_, handle, _) = handle.unwrap();
                    task.state = handle.state.load(Ordering::Relaxed).into();
                    task.progress = handle.get_progress();

                    group.get_model().set_row_data(j, task);
                    j += 1;
                }
            }

            drop(tasks_map);
            let mut i: usize = 0;
            while i < min(threads as usize, groups_ui.row_count()) {
                let group = groups_ui.row_data(i).unwrap();

                if group.get_model().row_count() == 0 {
                    groups_ui.remove(i);
                    continue;
                }
                i += 1;
            }
        });

        self.start_threads();
    }

    pub fn submit_task(&mut self, group: &str, name: &str, details: &str, task: impl Task) {
        let mut tasks = self.tasks.lock().unwrap();
        let should_start = tasks.is_empty() && !self.running.load(Ordering::Relaxed);

        let group = self.groups_ui.iter().find(|g| g.name.as_str() == group).unwrap_or_else(|| {
            self.groups_ui.push(TasksGroup::new(group));
            self.groups_ui.row_data(self.groups_ui.row_count() - 1).unwrap()
        });
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);
        let task: (i32, Arc<SharedTaskState>, Option<Box<dyn Task>>) = (id, task.get_shared_state(), Some(Box::new(task)));

        group.get_model().push(TaskData::new(id, name, details));
        tasks.push(task);
        drop(tasks);

        if should_start {
            self.semaphore.reset();
            self.semaphore.release();
            self.start_running();
        } else {
            self.semaphore.release();
        }
    }

    pub fn on_finish(&mut self, mut f: impl FnMut() + Send + Sync + 'static) {
        if !self.running.load(Ordering::Relaxed) {
            f();
            return;
        }

        self.on_finish.replace(Some(Box::new(f)));
    }
}

impl TaskData {
    pub fn new(id: i32, name: &str, details: &str) -> Self {
        Self { id, details: details.into(), name: name.into(), progress: 0.0, state: TaskState::Waiting }
    }
}

impl TasksGroup {
    pub fn new(name: &str) -> Self {
        let model = ModelRc::from(Rc::new(VecModel::from(vec![])));

        Self { name: name.into(), tasks: model }
    }

    pub fn get_model(&self) -> &VecModel<TaskData> {
        self.tasks.as_any().downcast_ref::<VecModel<TaskData>>().unwrap()
    }
}

pub struct Semaphore {
    lock: Mutex<i16>,
    cvar: Condvar
}

impl Semaphore {
    pub fn new(initial: i16) -> Self {
        Self { lock: Mutex::new(initial), cvar: Condvar::new() }
    }

    pub fn reset(&self) {
        let mut lock = self.lock.lock().unwrap();
        *lock = 0;
    }

    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }

        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.lock.lock().unwrap();
        *count += 1;
        self.cvar.notify_one();
    }

    pub fn release_all(&self) {
        let mut count = self.lock.lock().unwrap();
        *count = i16::MAX;
        self.cvar.notify_all();
    }
}
