use crate::api::config::Config;
use crate::api::control::dir_provider::get_data_dir;
use crate::api::typedef::task_manager::TaskManager;
use crate::generated::{AppState, Callbacks, DialogSeverity, Main, Mod, ModsUI, ModInfo};
use crate::logic::{open_discord, open_youtube};
use crate::ui::dialogs::{create_welcome_ui, present_dialog_standalone};
use crate::ui::launch::{get_manifest, launch};
use crate::{api::control::database::store_session, logic::open_x};
use crate::api::control::http::login_existing;
use slint::{ComponentHandle, Image, ModelRc, VecModel, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}};
use std::cell::RefCell;
use std::fs;
use std::os::unix::process::CommandExt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::error::Error;

fn setup_callbacks(ui: Weak<Main>, config: Arc<Config>, session: Arc<Mutex<Option<MicrosoftSession>>>, task_manager: Rc<RefCell<TaskManager>>) {
    let ui_strong = ui.upgrade().unwrap();
    let callbacks = ui_strong.global::<Callbacks>();

    callbacks.on_click_x(move || open_x());
    callbacks.on_click_discord(move || open_discord());
    callbacks.on_click_youtube(move || open_youtube());
    callbacks.on_show_mods(|| {
        let mods_ui = ModsUI::new().unwrap();
        let model: VecModel<Mod> = VecModel::default();

        // Remove later
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

    callbacks.on_launch(move || {
        let session = session.clone();
        let task_manager = task_manager.clone();
        let config = config.clone();
        let weak = ui.clone();
        let strong = weak.upgrade().unwrap();

        strong.set_app_state(AppState::Loading);
        let res = get_manifest();

        if let Err(err) = &res {
            present_dialog_standalone("Manifest Error", format!("Error getting manifest from mojang servers: {}", err.to_string()).as_str(), DialogSeverity::Error);
            return;
        }

        let (manifest, version) = res.unwrap();
        let cmd = launch(manifest, &version, session.clone(), task_manager.clone(), config.clone());
        if let Err(err) = &cmd {
            present_dialog_standalone("Error", format!("Failed to launch the game: {}", err.to_string()).as_ref(), DialogSeverity::Error);
            return;
        }

        let mut cmd = cmd.unwrap();

        task_manager.borrow_mut().on_finish(move || {
            let strong = weak.upgrade().unwrap();
            let _ = cmd.status();
            strong.set_app_state(AppState::Ready);
        });
    });
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let tokens = retrieve_session()?;
    let s_mutex: Arc<Mutex<Option<MicrosoftSession>>> = Arc::new(Mutex::new(None));

    if tokens.is_none() {
        create_welcome_ui(s_mutex.clone())?.run()?; // Blocking call until the user is logged in.

        // If the user closes the program without logging in.
        let session = s_mutex.lock().unwrap();
        if session.is_none() {
            return Ok(());
        }
    }

    let session = s_mutex.lock().unwrap();
    let ui = Main::new()?;
    let config = Arc::new(Config::load(get_data_dir().join("config.json").to_str().unwrap())?);
    fs::create_dir_all(&*config.jdk_dir)?;
    fs::create_dir_all(&*config.cache_dir)?;
    fs::create_dir_all(&*config.game_dir)?;

    let groups = Rc::new(VecModel::from(vec![]));
    let task_manager = Rc::new(RefCell::new(TaskManager::new(groups.clone())));

    setup_callbacks(ui.as_weak(), config.clone(), s_mutex.clone(), task_manager.clone());
    ui.set_groups(ModelRc::from(groups));

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
