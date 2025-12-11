use std::{rc::Rc, sync::atomic::AtomicU8};

use slint::{Model, ModelRc, Timer, VecModel};

use crate::generated::{TaskData, TasksGroup};



pub trait Task: Send {
    fn get_progress(&self) -> f32;
}

pub struct TaskHandle {
    task: Box<dyn Task>,
    progress: AtomicU8
}

pub struct TaskManager {
    groups_ui: Rc<VecModel<TasksGroup>>,
    tasks: Vec<TaskHandle>,
    timer: Timer
}

impl TaskManager {
    pub fn new() -> Self {
        Self { groups_ui: Rc::new(VecModel::from(vec![])), tasks: vec![], timer: Timer::default() }
    }

    pub fn submit_task(&mut self, group: &str, name: &str, details: &str, task: impl Task) {
        let group = self.groups_ui.iter().find(|g| g.name.as_str() == group).unwrap_or_else(|| {
            self.groups_ui.push(TasksGroup::new(group));
            self.groups_ui.row_data(self.groups_ui.row_count() - 1).unwrap()
        });

        group.get_model().push(TaskData { details: details.into(), name: name.into(), progress: 0.0, state: crate::generated::TaskState::Waiting });
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
