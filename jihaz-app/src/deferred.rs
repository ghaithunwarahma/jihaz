use std::path::PathBuf;
use anyhow::Result;
use jihaz::{
    error::print_error, 
    pathbuf_to_string::PathBufToString, 
    string::capitalize_first_letter
};
use jihaz_deferred::{icons::{produce_icns_file, ICONS_DIMENTIONS, SLEEP}, message::DeferredTaskMessage, plist::produce_plist_file, sleep_for};
use xilem_core::MessageProxy;

use crate::progress::ProgressMessage;

/// The payload used by the asynchronous generate packages task.
#[derive(Default, Clone, Debug)]
pub struct GeneratePackagesTaskPayload {
    pub executable_path: PathBuf,
    pub other_executables: Vec<PathBuf>,
    pub original_icon_path: PathBuf,
    pub app_name_lower_case: String,
    pub target_directory_path: PathBuf
}

pub fn generate_packages(
    payload: GeneratePackagesTaskPayload,
    progress_message_proxy: &MessageProxy<ProgressMessage>,
    task_message_proxy: &MessageProxy<DeferredTaskMessage>,
) -> Result<()> {    
    print_error(progress_message_proxy.message(ProgressMessage::began_production(
        payload.app_name_lower_case.clone(), 
        payload.target_directory_path.to_string().unwrap()
    )));

    let app_name = capitalize_first_letter(&payload.app_name_lower_case);

    // Creates the .app directory for your app
    let mut app_dir = payload.target_directory_path.to_path_buf();
    app_dir.push(format!("{app_name}.app/Contents/"));
    std::fs::create_dir_all(&app_dir)?;

    let main_executable_name = payload.executable_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Generate Info.plist file for your app and write it to the created .app directory
    let mut plist_file_path = app_dir.clone();
    plist_file_path.push("Info.plist");
    produce_plist_file(
        &main_executable_name, 
        &app_name, 
        format!("com.takarum.apps.{}", &payload.app_name_lower_case), 
        "0.1.0".to_string(), 
        format!("{}.icns", &payload.app_name_lower_case), 
        plist_file_path
    );

    // Copy the main executable to the created .app directory
    let mut new_executable_path = app_dir.clone();
    new_executable_path.push("MacOS/");
    std::fs::create_dir_all(&new_executable_path)?;
    new_executable_path.push(payload.executable_path.file_name().unwrap());
    let _ = std::fs::copy(&payload.executable_path, &new_executable_path);
    sleep_for(SLEEP);
    print_error(progress_message_proxy.message(ProgressMessage::written_executable(
        new_executable_path.to_string().unwrap(), true
    )));

    // Copy the other executables to the created .app directory
    for other_executable_path in payload.other_executables.iter() {
        let mut new_executable_path = app_dir.clone();
        new_executable_path.push("MacOS/");
        std::fs::create_dir_all(&new_executable_path)?;
        new_executable_path.push(other_executable_path.file_name().unwrap());
        let _ = std::fs::copy(other_executable_path, &new_executable_path);
        sleep_for(SLEEP);
        print_error(progress_message_proxy.message(ProgressMessage::written_executable(
            new_executable_path.to_string().unwrap(), false
        )));
    }

    // Generate icns file and write it to the created .app directory
    let mut icon_path = app_dir;
    icon_path.push("Resources/");
    std::fs::create_dir_all(&icon_path)?;
    icon_path.push(format!("{}.icns", &payload.app_name_lower_case));
    produce_icns_file(
        &payload.original_icon_path, 
        icon_path, 
        &ICONS_DIMENTIONS,
        Some(task_message_proxy),
    )?;
    sleep_for(SLEEP);
    print_error(progress_message_proxy.message(ProgressMessage::finished(
        payload.app_name_lower_case.clone(), 
        payload.target_directory_path.to_string().unwrap()
    )));
    Ok(())
}