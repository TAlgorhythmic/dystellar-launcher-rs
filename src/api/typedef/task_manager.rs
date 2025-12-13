use std::{collections::HashMap, rc::Rc, sync::{Arc, Mutex, atomic::{AtomicI32, AtomicU8, Ordering}}};

use slint::{Model, ModelRc, Timer, VecModel};

use crate::generated::{TaskData, TaskState, TasksGroup};

static NEXT_TASK_ID: AtomicI32 = AtomicI32::new(0);

pub trait Task: Send + Sync {
    fn run(&self);
}

pub struct TaskSharedState {
    state: AtomicU8,
    progress: AtomicU8
}

pub struct TaskHandle {
    task: Box<dyn Task>,
    state: Arc<TaskSharedState>
}

pub struct TaskManager {
    groups_ui: Rc<VecModel<TasksGroup>>,
    tasks: Arc<Mutex<HashMap<i32, Arc<TaskHandle>>>>,
    timer: Timer
}

impl TaskManager {
    pub fn new() -> Self {
        Self { groups_ui: Rc::new(VecModel::from(vec![])), tasks: Arc::new(Mutex::new(HashMap::new())), timer: Timer::default() }
    }

    pub fn submit_task(&mut self, group: &str, name: &str, details: &str, task: impl Task) {
        let tasks = self.tasks.lock().unwrap();
        let should_start = tasks.is_empty();
        drop(tasks);

        let group = self.groups_ui.iter().find(|g| g.name.as_str() == group).unwrap_or_else(|| {
            self.groups_ui.push(TasksGroup::new(group));
            self.groups_ui.row_data(self.groups_ui.row_count() - 1).unwrap()
        });
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);
        
        group.get_model().push(TaskData::new(id, name, details));
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
