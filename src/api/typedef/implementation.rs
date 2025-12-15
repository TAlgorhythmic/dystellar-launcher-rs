use std::{error::Error, fs::File, io::{Read, Write}, sync::atomic::{AtomicU8, AtomicUsize, Ordering}};

use ureq::{BodyReader, get, http::header::CONTENT_LENGTH};

use crate::{api::typedef::task_manager::Task, generated::TaskState};

const BUFFER_SIZE: usize = 16 * 1024;

impl From<u8> for TaskState {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::InProgress,
            2 => Self::Starting,
            3 => Self::Unpacking,
            4 => Self::Verifying,
            5 => Self::Finished,
            6 => Self::Failed,
            _ => Self::Waiting
        }
    }
}

impl From<TaskState> for u8 {
    fn from(value: TaskState) -> Self {
        match value {
            TaskState::Waiting => 0,
            TaskState::InProgress => 1,
            TaskState::Starting => 2,
            TaskState::Unpacking => 3,
            TaskState::Verifying => 4,
            TaskState::Finished => 5,
            TaskState::Failed => 6
        }
    }
}

pub struct HttpDownloadTask<F>
    where F: Fn(&mut HttpDownloadTask<F>) -> Result<(), Box<dyn Error + Send + Sync>>
    + Send
    + Sync
    + 'static
{
    pub url: Box<str>,
    pub total: AtomicUsize,
    pub progress: AtomicUsize,
    pub state: AtomicU8,
    pub output: Box<str>,
    pub post_scripts: Vec<F>,
}

impl<F> HttpDownloadTask<F>
    where F: Fn(&mut HttpDownloadTask<F>) -> Result<(), Box<dyn Error + Send + Sync>>
    + Send
    + Sync
    + 'static
{
    pub fn new(url: &str, output: &str, post_scripts: Vec<F>) -> Result<Self, Box<dyn Error + Send + Sync>>
        where F: Fn(&mut HttpDownloadTask<F>) -> Result<(), Box<dyn Error + Send + Sync>>
        + Send
        + Sync
        + 'static
    {
        Ok(HttpDownloadTask {
            url: url.into(),
            total: AtomicUsize::new(0),
            progress: AtomicUsize::new(0),
            state: AtomicU8::new(0),
            output: output.into(),
            post_scripts: post_scripts
        })
    }
}

impl<F> Task for HttpDownloadTask<F>
    where F: Fn(&mut HttpDownloadTask<F>) -> Result<(), Box<dyn Error + Send + Sync>>
    + Send
    + Sync
    + 'static
{
    fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut res = get(&*self.url).call()?;
        let total_size = res.headers().get(CONTENT_LENGTH).map(|e| e.to_str().unwrap().parse::<usize>().unwrap()).unwrap_or(0);
        let mut reader = res.body_mut().as_reader();
        let mut file = File::create(&*self.output)?;

        self.total.store(total_size, Ordering::Relaxed);
        self.state.store(TaskState::InProgress.into(), Ordering::Relaxed);

        let mut buf = [0u8; BUFFER_SIZE];

        while let n = BodyReader::read(&mut reader, &mut buf)? && n > 0 {
            file.write_all(&buf[..n])?;
            self.progress.fetch_add(n, Ordering::Relaxed);
        }

        let scripts = std::mem::take(&mut self.post_scripts);
        for f in scripts {
            f(self)?;
        }

        self.state.store(TaskState::Finished.into(), Ordering::Relaxed);

        Ok(())
    }

    fn get_progress(&self) -> f32 {
        let total = self.total.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let progress = self.progress.load(Ordering::Relaxed);

        progress as f32 / total as f32
    }

    fn get_state(&self) -> TaskState {
        self.state.load(Ordering::Relaxed).into()
    }

    fn claim(&mut self) {
        self.state.store(TaskState::Starting.into(), Ordering::Relaxed);
    }
}
