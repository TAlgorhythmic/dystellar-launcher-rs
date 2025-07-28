use libadwaita::Window;

use crate::api::typedef::ms_session::MicrosoftSession;

pub fn login_callback(session: MicrosoftSession) {

}

pub fn welcome_login_screen<F>(on_login: F)-> Window
where
    F: FnOnce() -> ()
{
    
}
