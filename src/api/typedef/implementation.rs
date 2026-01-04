use std::{error::Error, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}, sync::atomic::{AtomicU8, AtomicUsize, Ordering}};

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
    pub output: PathBuf,
    pub post_scripts: Vec<Box<dyn Fn(&mut HttpDownloadTask) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>,
}

impl HttpDownloadTask {
    pub fn new(url: &str, output: PathBuf, post_scripts: Vec<Box<dyn Fn(&mut HttpDownloadTask) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>) -> Result<Self, Box<dyn Error + Send + Sync>>
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
        let mut file = File::open(&path)?;
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
            fs::remove_file(path)?;
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

    #[cfg(target_os = "windows")]
    pub fn post_unpack_package(&mut self, path: PathBuf, output: PathBuf, skip_top: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.state.store(TaskState::Unpacking.into(), Ordering::Relaxed);

        let mut zip = ZipArchive::new(File::open(&path)?)?;

        if skip_top {
            let top_folder = zip.file_names()
                .filter(|f| f.ends_with('/') && f.matches('/').count() == 1)
                .next();

            if top_folder.is_none() {
                return Err("No top folder found".into());
            }

            let top_folder: Box<str> = top_folder.unwrap().into();
            zip.extract(&output)?;
            let extracted = output.join(top_folder.as_ref());

            for entry in fs::read_dir(&extracted)? {
                let entry = entry?;
                fs::rename(entry.path(), output.join(entry.file_name()))?;
            }

            fs::remove_dir(extracted)?;
        } else {
            zip.extract(output)?;
        }
        fs::remove_file(path)?;

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn post_unpack_package(&mut self, path: PathBuf, output: PathBuf, skip_top: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        self.state.store(TaskState::Unpacking.into(), Ordering::Relaxed);

        let file = File::open(&path)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        if skip_top {
            let mut top: Option<PathBuf> = None;

            for entry in archive.entries()? {
                let entry = entry?;
                let path = entry.path()?;

                if let Some(first) = path.components().next() {
                    let first = PathBuf::from(first.as_os_str());
                    match &top {
                        None => top = Some(first),
                        Some(existing) if *existing != first => {
                            return Err("Archive has multiple top-level entries".into());
                        }
                        _ => {}
                    }
                }
            }

            let top = top.ok_or("No top folder found")?;

            let file = File::open(&path)?;
            let decoder = GzDecoder::new(file);
            let mut archive = Archive::new(decoder);

            archive.unpack(&output)?;
            let extracted = output.join(top);

            for entry in fs::read_dir(&extracted)? {
                let entry = entry?;
                fs::rename(entry.path(), output.join(entry.file_name()))?;
            }

            fs::remove_dir(extracted)?;
        } else {
            archive.unpack(output)?;
        }

        fs::remove_file(path)?;

        Ok(())
    }
}

impl Task for HttpDownloadTask {
    fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut res = get(&*self.url).call()?;
        if let Some(total_size) = res.headers().get(CONTENT_LENGTH).map(|e| e.to_str().unwrap().parse::<usize>().unwrap()) {
            self.total.store(total_size, Ordering::Relaxed);
        }
        let mut reader = res.body_mut().as_reader();
        let mut file = File::create(&*self.output)?;

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
