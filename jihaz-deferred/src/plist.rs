use plist::Value;
use std::path::Path;

/// Produce the XML encoded plist for the MacOS app
pub fn produce_plist_file(
    executable_name: &str,
    name: &str,
    bundle_identifier: String,
    version_string: String,
    icns_file_path: String,
    target_file_path: impl AsRef<Path>,
) {
    let mut dictionary = plist::Dictionary::new();
    dictionary.insert("CFBundleDevelopmentRegion".to_string(), "en".into());
    dictionary.insert("CFBundleExecutable".to_string(), executable_name.into());
    dictionary.insert("CFBundleIdentifier".to_string(), bundle_identifier.into());
    dictionary.insert("CFBundleInfoDictionaryVersion".to_string(), "0.6.0".into());
    dictionary.insert("CFBundleName".to_string(), name.into());
    dictionary.insert("CFBundlePackageType".to_string(), "APPL".into());
    dictionary.insert("CFBundleShortVersionString".to_string(), version_string.into());
    dictionary.insert("CFBundleDevelopmentRegion".to_string(), "en".into());
    dictionary.insert(
        "CFBundleSupportedPlatforms".to_string(),
        Value::Array(vec![Value::String("MacOSX".to_string())])
    );
    dictionary.insert("CFBundleVersion".to_string(), "1".into());
    dictionary.insert("CFBundleIconFile".to_string(), icns_file_path.into());
    dictionary.insert("NSHighResolutionCapable".to_string(), true.into());
    dictionary.insert("NSMainNibFile".to_string(), "".into());
    dictionary.insert("NSSupportsAutomaticGraphicsSwitching".to_string(), true.into());
    dictionary.insert("CFBundleDisplayName".to_string(), name.into());
    dictionary.insert("NSRequiresAquaSystemAppearance".to_string(), "NO".into());
    dictionary.insert(
        "NSAppleEventsUsageDescription".to_string(),
        format!("An application in {name} would like to access AppleScript.").into()
    );
    dictionary.insert(
        "NSCalendarsUsageDescription".to_string(),
        format!("An application in {name} would like to access calendar data.").into()
    );
    dictionary.insert(
        "NSCameraUsageDescription".to_string(),
        format!("An application in {name} would like to access the camera.").into()
    );
    dictionary.insert(
        "NSContactsUsageDescription".to_string(),
        format!("An application in {name} wants to access your contacts.").into()
    );
    dictionary.insert(
        "NSLocationAlwaysUsageDescription".to_string(),
        format!("An application in {name} would like to access your location information, even in the background.").into()
    );
    dictionary.insert(
        "NSLocationUsageDescription".to_string(),
        format!("An application in {name} would like to access your location information.").into()
    );
    dictionary.insert(
        "NSLocationWhenInUseUsageDescription".to_string(),
        format!("An application in {name} would like to access your location information while active.").into()
    );
    dictionary.insert(
        "NSMicrophoneUsageDescription".to_string(),
        format!("An application in {name} would like to access your microphone.").into()
    );
    dictionary.insert(
        "NSRemindersUsageDescription".to_string(),
        format!("An application in {name} would like to access your reminders.").into()
    );
    dictionary.insert(
        "NSSystemAdministrationUsageDescription".to_string(),
        format!("An application in {name} requires elevated permissions.").into()
    );

    let _ = plist::to_file_xml(&target_file_path, &dictionary);

    // println!("Written plist file {:?}", target_file_path.as_ref());
    // if let Err(e) = proxy.message(IconProgressMessage::written_png(
    //     frame_side_length as usize, 
    //     png_save_path.to_string().unwrap()
    // ).into()) {
    //     println!("{:?}", e);
    // }
}