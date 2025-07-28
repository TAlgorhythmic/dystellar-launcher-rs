use std::sync::LazyLock;

use gtk::glib::MainContext;

use crate::api::typedef::ms_session::MicrosoftSession;

static CTX: LazyLock<MainContext> = LazyLock::new(|| MainContext::default());

pub fn exec_safe_gtk<F>(f: F)
where
    F: FnOnce() + Send + 'static
{
    CTX.invoke(f);
}

pub fn login_callback(session: MicrosoftSession) {

}
