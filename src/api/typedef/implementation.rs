use std::{error::Error, fs::File, io::{Read, Write}, path::{Path, PathBuf}, sync::atomic::{AtomicU8, AtomicUsize, Ordering}};

use sha1::Sha1;
use sha2::Digest;
use ureq::{BodyReader, get, http::header::CONTENT_LENGTH};
use zip::ZipArchive;

use crate::{api::typedef::task_manager::Task, generated::TaskState};

const BUFFER_SIZE: usize = 16 * 1024;

#[cfg(target_os = "macos")]
const NATIVE_EXT: &'static str = ".dylib";
#[cfg(target_os = "windows")]
const NATIVE_EXT: &'static str = ".dll";
#[cfg(target_os = "linux")]
const NATIVE_EXT: &'static str = ".so";

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

pub struct HttpDownloadTask {
    pub url: Box<str>,
    pub total: AtomicUsize,
    pub progress: AtomicUsize,
    pub state: AtomicU8,
    pub output: Box<str>,
    pub post_scripts: Vec<Box<dyn Fn(&mut HttpDownloadTask) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>,
}

impl HttpDownloadTask {
    pub fn new(url: &str, output: &str, post_scripts: Vec<Box<dyn Fn(&mut HttpDownloadTask) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>) -> Result<Self, Box<dyn Error + Send + Sync>>
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

    pub fn post_verify_sha1(&mut self, path: PathBuf, sha1: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.state.store(TaskState::Verifying.into(), Ordering::Relaxed);
        let mut file = File::open(path)?;
        let mut buf = [0u8; 8192];
        let mut hasher = Sha1::new();

        self.progress.store(0, Ordering::Relaxed);
        self.total.store(file.metadata()?.len() as usize, Ordering::Relaxed);

        while let rd = file.read(&mut buf)? && rd > 0 {
            hasher.update(&buf[..rd]);
            self.progress.fetch_add(rd, Ordering::Relaxed);
        }

        let result = hasher.finalize();
        let hex = format!("{:x}", result);

        if hex.as_str() != sha1 {
            return Err("Integrity test failed, sha1sum mismatch!".into());
        }

        Ok(())
    }

    pub fn post_unpack_natives(&mut self, path: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.state.store(TaskState::Unpacking.into(), Ordering::Relaxed);

        let mut zip = ZipArchive::new(File::open(path)?)?;

        for i in 0..zip.len() {
            let entry = zip.by_index_raw(i)?;

            if entry.name().ends_with(NATIVE_EXT) {
                let name = entry.enclosed_name().ok_or("Failed to get file path")?;
                let filename = name.file_name().ok_or("Failed to get filename")?;
                let mut out = File::create(output.join(filename))?;
                drop(entry);
                let mut input = zip.by_index(i)?;
                let mut buf = [0u8; BUFFER_SIZE];

                while let rd = input.read(&mut buf)? && rd > 0 {
                    out.write_all(&mut buf[..rd])?;
                }
            }
        }

        Ok(())
    }
}

impl Task for HttpDownloadTask {
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
