use std::{cell::{RefCell, UnsafeCell}, collections::HashMap, error::Error, rc::Rc, sync::{Arc, Condvar, Mutex, atomic::{AtomicBool, AtomicI32, Ordering}}, thread, time::Duration};

use slint::{Model, ModelRc, Timer, TimerMode, VecModel};

use crate::{generated::{DialogSeverity, TaskData, TaskState, TasksGroup}, logic::safe, ui::dialogs::present_dialog_standalone};

static NEXT_TASK_ID: AtomicI32 = AtomicI32::new(0);

pub trait Task: Send + Sync + 'static {
    fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn get_progress(&self) -> f32;
    fn get_state(&self) -> TaskState;
    fn claim(&mut self);
}

struct TaskWrapper {
    task: UnsafeCell<Box<dyn Task>>
}

impl TaskWrapper {
    pub fn new(task: impl Task) -> Self {
        Self { task: UnsafeCell::new(Box::new(task)) }
    }
}

unsafe impl Send for TaskManager {}
unsafe impl Sync for TaskManager {}
unsafe impl Send for TaskWrapper {}
unsafe impl Sync for TaskWrapper {}

pub struct TaskManager {
    groups_ui: Rc<VecModel<TasksGroup>>,
    tasks: Arc<Mutex<HashMap<i32, Arc<TaskWrapper>>>>,
    timer: Rc<Timer>,
    running: AtomicBool,
    semaphore: Arc<Semaphore>,
    threads: i16,
    on_finish: Rc<RefCell<Option<Box<dyn Fn() + Send + Sync + 'static>>>>
}

impl TaskManager {
    pub fn new(model: Rc<VecModel<TasksGroup>>) -> Self {
        let threads = thread::available_parallelism().unwrap().get() as i16;

        Self {
            groups_ui: model,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            timer: Rc::new(Timer::default()),
            running: AtomicBool::new(false),
            semaphore: Arc::new(Semaphore::new(0)),
            threads,
            on_finish: Rc::new(RefCell::new(None))
        }
    }

    fn start_threads(&self) {
        for _ in 0..self.threads {
            let semaphore = self.semaphore.clone();
            let map = self.tasks.clone();

            thread::spawn(move || {
                loop {
                    let guard = map.lock().unwrap();
                    if guard.is_empty() {
                        return;
                    }

                    let entry = guard.iter().find(|e| unsafe {(&*e.1.task.get()).get_state()} == TaskState::Waiting);
                    if entry.is_none() {
                        semaphore.acquire();
                        semaphore.release();
                        continue;
                    }
                    
                    let (id, task) = entry.unwrap();
                    let id = *id;
                    let task = unsafe {&mut *task.clone().task.get()};
                    task.claim();

                    drop(guard);
                    semaphore.acquire();
                    if let Err(err) = task.run() {
                        safe(move || present_dialog_standalone("Task Error", format!("Error performing task: {}", err.to_string()).as_str(), DialogSeverity::Error));
                    }

                    let mut guard = map.lock().unwrap();
                    guard.remove(&id);
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

        self.timer.start(TimerMode::Repeated, Duration::from_millis(80), move || {
            if groups_ui.row_count() == 0 {
                if let Some(f) = &*on_finish.borrow() {
                    f();
                }

                on_finish.replace(None);
                semaphore.release_all();
                timer.stop();
                return;
            }

            let tasks_map = tasks.lock().unwrap();
            let mut removals: Vec<i32> = vec![];

            for i in 0..groups_ui.row_count() {
                let group = groups_ui.row_data(i).unwrap();
                for j in 0..group.tasks.row_count() {
                    let mut task = group.tasks.row_data(j).unwrap();
                    let handle = tasks_map.get(&task.id);

                    if handle.is_none() {
                        removals.push(task.id);
                        break;
                    }
                    let handle = unsafe { &*handle.unwrap().task.get() };
                    task.state = handle.get_state();
                    task.progress = handle.get_progress();

                    group.get_model().set_row_data(j, task);
                }
            }
            if !removals.is_empty() {
                let mut i = 0;
                while i < groups_ui.row_count() {
                    let group = groups_ui.row_data(i).unwrap();
                    let mut j = 0;
                    while j < group.tasks.row_count() {
                        let task = group.tasks.row_data(j).unwrap();

                        if removals.iter().find(|id| **id == task.id).is_some() {
                            group.get_model().remove(j);
                            j -= 1;
                        }
                    }

                    if group.get_model().row_count() == 0 {
                        groups_ui.remove(i);
                        i -= 1;
                    }
                }
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
        let task = Arc::new(TaskWrapper::new(task));

        group.get_model().push(TaskData::new(id, name, details));
        tasks.insert(id, task);
        drop(tasks);

        if should_start {
            self.semaphore.reset();
            self.semaphore.release();
            self.start_running();
        }
    }

    pub fn on_finish(&mut self, f: impl Fn() + Send + Sync + 'static) {
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
