use crate::api::config::Config;
use crate::api::control::dir_provider::get_data_dir;
use crate::generated::{AppState, Callbacks, DialogSeverity, Main, Mod, ModsUI, WelcomeUI, ModInfo};
use crate::logic::{open_discord, open_youtube};
use crate::ui::dialogs::present_dialog_standalone;
use crate::ui::launch::launch;
use crate::{api::control::database::store_session, logic::open_x};
use crate::api::control::http::{login, login_existing};
use slint::{ComponentHandle, Image, ModelRc, VecModel, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::error::Error;

fn setup_callbacks(ui: Weak<Main>, config: Arc<Config>, session: Arc<Mutex<Option<MicrosoftSession>>>) {
    let ui_strong = ui.upgrade().unwrap();
    let callbacks = ui_strong.global::<Callbacks>();

    callbacks.on_click_x(move || open_x());
    callbacks.on_click_discord(move || open_discord());
    callbacks.on_click_youtube(move || open_youtube());
    callbacks.on_show_mods(|| {
        let mods_ui = ModsUI::new().unwrap();
        let model: VecModel<Mod> = VecModel::default();

        model.push(Mod { author: "Algorhythmics".into(), default: false, description: "Test".into(), enabled: false, image: Image::default(), loading: false, name: "Test".into(), url: "Test".into(), version: "1.0".into() });
        let the_model = ModelRc::from(Rc::new(model).clone());
        mods_ui.set_mods(the_model);

        let cl = mods_ui.clone_strong();

        mods_ui.on_close(move || cl.hide().unwrap());
        mods_ui.on_mod_click(|r_mod| {
            let modinf_ui = ModInfo::new().unwrap();

            modinf_ui.set_mod(r_mod);
            modinf_ui.show().unwrap();
        });
        
        mods_ui.show().unwrap();
    });
    callbacks.on_launch({
        let ui = ui_strong.clone_strong();
        move || launch(ui.clone_strong(), |_| {})
    });
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let tokens = retrieve_session()?;
    let s_mutex: Arc<Mutex<Option<MicrosoftSession>>> = Arc::new(Mutex::new(None));

    if tokens.is_none() {
        let win = WelcomeUI::new()?;
        let mutex_cl = s_mutex.clone();
        let win_weak = win.as_weak();

        win.on_login(move || {
            let win = win_weak.upgrade().unwrap();
            let win_weak = win.as_weak();
            let mutex_cl = mutex_cl.clone();

            win.set_waiting(true);
            login(move |result| {
                let win = win_weak.upgrade().unwrap();

                win.set_waiting(false);
                if let Err(err) = &result {
                    present_dialog_standalone(err.title, &err.description, DialogSeverity::Error);
                    return;
                }

                let session = result.unwrap();
                if let Err(err) = store_session(&session.access_token, &session.refresh_token) {
                    present_dialog_standalone("Failed to store session", format!("Failed to store session in storage: {} You'll have to login again next time.", err.to_string()).as_str(), DialogSeverity::Error);
                }

                let mut guard = mutex_cl.lock().unwrap();

                *guard = Some(session);
                let _ = win.hide();
            });
        });

        win.run()?; // Blocking call until the user is logged in.

        // If the user closes the program without logging in.
        let session = s_mutex.lock().unwrap();
        if session.is_none() {
            return Ok(());
        }
        drop(session);
    }
    let session = s_mutex.lock().unwrap();
    let ui = Main::new()?;
    let config = Arc::new(Config::load(get_data_dir().join("config.json").to_str().unwrap())?);

    setup_callbacks(ui.as_weak(), config.clone(), s_mutex.clone());
    ui.set_groups(ModelRc::from(Rc::new(VecModel::from(vec![]))));

    if session.is_none() {
        let (access_token, refresh_token) = tokens.unwrap();
        let ui_weak = ui.as_weak();
        let mutex_cl = s_mutex.clone();

        login_existing(access_token, refresh_token, move |result| {
            let ui = ui_weak.upgrade().unwrap();

            if let Err(err) = &result {
                present_dialog_standalone("Login Error".into(), &err.to_string(), DialogSeverity::Error);
                ui.set_app_state(AppState::SessionError);
                return;
            }

            let session = result.unwrap();

            if let Err(err) = store_session(&session.access_token, &session.refresh_token) {
                present_dialog_standalone("Failed to store session", format!("Failed to store session in storage: {} You'll have to login again next time.", err.to_string()).as_str(), DialogSeverity::Error);
            }

            let mut guard = mutex_cl.lock().unwrap();

            *guard = Some(session);
            ui.set_app_state(AppState::Ready);
        });
    }
    drop(session);
    ui.run()?;
    Ok(())
}
