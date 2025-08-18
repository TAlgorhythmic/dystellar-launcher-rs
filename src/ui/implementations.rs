use std::{error::Error, sync::{atomic::AtomicUsize, mpsc::{self, channel, Receiver, Sender}, LazyLock}, thread};

use gtk::{glib::object::{IsA, ObjectExt}, prelude::*, ProgressBar, Spinner, Widget};
use libadwaita::{prelude::BinExt, Bin};
use ureq::{http::{header::{CONTENT_DISPOSITION, CONTENT_LENGTH}, Response}, Body, ResponseExt};

use crate::api::control::callbacks::exec_safe_gtk;

pub struct HttpDownload {
    progress: AtomicUsize,
    total: usize,
    name: Box<str>,
    progress_bar: ProgressBar,
    bar_content: gtk::Box,
    http_stream: Response<Body>
}

impl HttpDownload {
    pub fn new_from_http_body(res: Response<Body>, progress_bar: ProgressBar, bar_content: gtk::Box) -> Result<Self, Box<dyn Error>> {
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
            progress_bar, bar_content,
            http_stream: res
        })
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;
static WORKER: LazyLock<Sender<Task>> = LazyLock::new(|| {
    let (sender, receiver) = channel::<Task>();
    
    thread::spawn(|| {
        for task in receiver {
            task();
        }
    });

    sender
});

pub struct LoadableContainer {
    container: Bin,
    spinner: Spinner
}

unsafe impl Send for LoadableContainer {}
unsafe impl Sync for LoadableContainer {}

impl LoadableContainer {
    pub fn new(bin: Bin) -> Self {
        let spinner = Spinner::builder().halign(gtk::Align::Fill).valign(gtk::Align::Fill).build();

        bin.set_child(Some(&spinner));
        spinner.start();
        Self { container: bin, spinner }
    }

    pub fn load<W, F>(self, exec: F)
    where
        W: IsA<Widget> + 'static,
        F: FnOnce(&LoadableContainer) -> W + Send + 'static
    {
        let boxed = Box::new(move || {
            exec_safe_gtk(move || {
                let widget = exec(&self).upcast::<Widget>();

                self.spinner.stop();
                self.get_widget().set_child(Some(&widget));
            });
        });

        if let Err(err) = WORKER.send(boxed) {
            eprintln!("Failed to send task: {}", err.to_string());
        }
    }

    pub fn get_widget(&self) -> &Bin {
        &self.container
    }
}
