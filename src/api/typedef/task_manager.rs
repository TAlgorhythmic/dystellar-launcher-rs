use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex, atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering}}};

use slint::{Model, ModelRc, Timer, VecModel};

use crate::generated::{TaskData, TaskState, TasksGroup};

static NEXT_TASK_ID: AtomicI32 = AtomicI32::new(0);

pub trait Task: Send + Sync + 'static {
    /**
    * Heavy work, like IO blocking, will be called from random threads
    */
    fn run(&self);

    /**
    * Thread safe
    */
    fn get_progress(&self) -> f32;
}

pub struct TaskManager<'a> {
    groups_ui: &'a VecModel<TasksGroup>,
    tasks: Arc<Mutex<HashMap<i32, Arc<dyn Task>>>>,
    timer: Timer,
    running: AtomicBool
}

impl<'a> TaskManager<'a> {
    pub fn new(model: &'a VecModel<TasksGroup>) -> Self {
        Self { groups_ui: model, tasks: Arc::new(Mutex::new(HashMap::new())), timer: Timer::default(), running: AtomicBool::new(false) }
    }

    fn start_threads(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    pub fn submit_task(&mut self, group: &str, name: &str, details: &str, task: impl Task) {
        let mut tasks = self.tasks.lock().unwrap();
        let should_start = tasks.is_empty() && !self.running.load(Ordering::Relaxed);

        let group = self.groups_ui.iter().find(|g| g.name.as_str() == group).unwrap_or_else(|| {
            self.groups_ui.push(TasksGroup::new(group));
            self.groups_ui.row_data(self.groups_ui.row_count() - 1).unwrap()
        });
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);

        group.get_model().push(TaskData::new(id, name, details));
        tasks.insert(id, Arc::new(task));
        drop(tasks);

        if should_start {
            self.start_threads();
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
