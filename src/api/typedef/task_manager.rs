use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex, atomic::{AtomicBool, AtomicI32, Ordering}, mpsc::{self, Receiver, Sender}}, thread, time::Duration};

use slint::{Model, ModelRc, Timer, TimerMode, VecModel};

use crate::generated::{TaskData, TaskState, TasksGroup};

static NEXT_TASK_ID: AtomicI32 = AtomicI32::new(0);

pub trait Task: Send + Sync + 'static {
    fn run(&self);
    fn get_progress(&self) -> f32;
    fn get_state(&self) -> TaskState;
    fn get_id(&self) -> i32;
}

pub struct TaskManager {
    groups_ui: Rc<VecModel<TasksGroup>>,
    tasks: Arc<Mutex<HashMap<i32, Arc<dyn Task>>>>,
    timer: Timer,
    running: AtomicBool,
    queue: Option<(Sender<Arc<dyn Task>>, Receiver<Arc<dyn Task>>)>
}

impl TaskManager {
    pub fn new(model: Rc<VecModel<TasksGroup>>) -> Self {
        Self {
            groups_ui: model,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            timer: Timer::default(),
            running: AtomicBool::new(false),
            queue: None
        }
    }

    fn destroy_queue(&mut self) {
        drop(self.queue.take());
    }

    fn start_threads(&self) {
        if let Some((s, r)) = self.queue {
            
            for _ in 0..thread::available_parallelism().unwrap().get() {
                thread::spawn(|| {
                    
                })
            }
        }
    }

    fn start_running(&self) {
        self.running.store(true, Ordering::Relaxed);
        let groups_ui = self.groups_ui.clone();
        let tasks = self.tasks.clone();

        self.timer.start(TimerMode::Repeated, Duration::from_millis(80), move || {
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
                    let handle = handle.unwrap();
                    task.state = handle.get_state();
                    task.progress = handle.get_progress();

                    group.get_model().set_row_data(j, task);
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
        let task = Arc::new(task);

        group.get_model().push(TaskData::new(id, name, details));
        tasks.insert(id, task.clone());
        drop(tasks);

        if should_start {
            self.queue = Some(mpsc::channel());
            self.start_running();
        }
        if let Some((s, _)) = &self.queue {
            s.send(task);
        }
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
