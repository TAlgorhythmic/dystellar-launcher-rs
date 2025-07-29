use std::sync::LazyLock;

use gtk::glib::MainContext;

static CTX: LazyLock<MainContext> = LazyLock::new(|| MainContext::default());

pub fn exec_safe_gtk<F>(f: F)
where
    F: FnOnce() + Send + 'static
{
    CTX.invoke(f);
}
