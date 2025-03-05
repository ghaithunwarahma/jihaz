use std::path::Path;
use jihaz::{copy_dir_all, error::print_error, string::capitalize_first_letter};
use jihaz_deferred::{icons::{produce_icns_file, ICONS_DIMENTIONS}, plist::produce_plist_file};

pub fn main() {
    let executable_path = "/Users/abdallah/Documents/rust-targets/release-lto/tarteel";
    let original_icon_path = "/Users/abdallah/Documents/Local Coding/tarteel/assets/tarteel.svg";
    let app_name_lower_case = "tarteel".to_string();
    let target_directory_path = "/Users/abdallah/Documents/Local Coding/tarteel/packages/";
    generate_mac_packages(
        &Path::new(&executable_path), 
        &[],
        &Path::new(original_icon_path), 
        app_name_lower_case, 
        &Path::new(target_directory_path),
    );
    if let Err(e) = copy_dir_all(
        "/Users/abdallah/Documents/Local Coding/tarteel/packages/Tarteel.app", 
        "/Applications/Tarteel.app"
    ) {
        eprintln!("Error copying Tarteel.app, {:?}", e);
    } else {
        println!("Generates app package for Takarum and copied to applications folder");
    }
}

pub fn generate_mac_packages(
    executable_path: &Path,
    other_executables: &[&Path],
    original_icon_path: &Path,
    app_name_lower_case: String,
    target_directory_path: &Path
) {
    let app_name = capitalize_first_letter(&app_name_lower_case);

    // Creates the .app directory for your app
    let mut app_dir = target_directory_path.to_path_buf();
    app_dir.push(format!("{app_name}.app/Contents/"));
    if let Err(e) = std::fs::create_dir_all(&app_dir) {
        eprintln!("Failed to create directory: {e}");
    }

    let main_executable_name = target_directory_path
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
        format!("com.takarum.apps.{app_name_lower_case}"), 
        "0.1.0".to_string(), 
        format!("{app_name_lower_case}.icns"), 
        plist_file_path
    );

    // Copy the main executable to the created .app directory
    let mut new_executable_path = app_dir.clone();
    new_executable_path.push("MacOS/");
    if let Err(e) = std::fs::create_dir_all(&new_executable_path) {
        eprintln!("Failed to create directory: {e}");
    }
    new_executable_path.push(executable_path.file_name().unwrap());
    let _ = std::fs::copy(executable_path, &new_executable_path);
    println!("Written main executable file {:?}", &new_executable_path);

    // Copy the other executables to the created .app directory
    for other_executable_path in other_executables.iter() {
        let mut new_executable_path = app_dir.clone();
        new_executable_path.push("MacOS/");
        if let Err(e) = std::fs::create_dir_all(&new_executable_path) {
            eprintln!("Failed to create directory: {e}");
        }
        new_executable_path.push(other_executable_path.file_name().unwrap());
        let _ = std::fs::copy(other_executable_path, &new_executable_path);
        println!("Written other executable file {:?}", &new_executable_path);
    }

    // Generate icns file and write it to the created .app directory
    let mut icon_path = app_dir;
    icon_path.push("Resources/");
    if let Err(e) = std::fs::create_dir_all(&icon_path) {
        eprintln!("Failed to create directory: {e}");
    }
    icon_path.push(format!("{app_name_lower_case}.icns"));
    print_error(produce_icns_file(
        original_icon_path, 
        icon_path, 
        &ICONS_DIMENTIONS,
        None
    ));
}