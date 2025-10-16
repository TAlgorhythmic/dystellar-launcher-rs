use std::{error::Error, sync::{atomic::AtomicUsize}};

use ureq::{http::{header::{CONTENT_DISPOSITION, CONTENT_LENGTH}, Response}, Body, ResponseExt};

use crate::generated::TaskData;

pub struct HttpDownload<F> where F: Fn(TaskData) + Send + 'static {
    progress: AtomicUsize,
    total: usize,
    name: Box<str>,
    task: TaskData,
    http_stream: Response<Body>,
    on_change: F
}

impl<F> HttpDownload<F> where F: Fn(TaskData) + Send + 'static {
    pub fn new_from_http_body(res: Response<Body>, task_data: TaskData, on_change: F) -> Result<Self, Box<dyn Error>>  {
        let name = match res.headers().get(CONTENT_DISPOSITION) {
            Some(header) => String::from(header.to_str()?),
            None => {
                let mut url = res.get_uri().to_string();
                url.push_str(".cache");

                url
            }
        };

        let total_bytes: usize = res.headers().get(CONTENT_LENGTH).map(|h| h.to_str()).unwrap_or(Ok("0"))?.parse()?;

        Ok(HttpDownload {
            progress: AtomicUsize::new(0),
            total: total_bytes,
            name: name.into(),
            task: task_data,
            http_stream: res,
            on_change
        })
    }
}
