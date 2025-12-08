use std::{error::Error, process::{Child, Command}, sync::Arc, thread};

use crate::{api::config::Config, generated::Main, logic::safe};

pub fn launch<F>(ui: Main, callback: F, config: Arc<Config>) where F: Fn(Result<Child, Box<dyn Error + Send + Sync>>) {
    thread::spawn(move || {
        
        safe(f);
    });
}
