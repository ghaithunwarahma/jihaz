//! Logic and ideas mostly taken from the Lapce crate

pub fn home_dir() -> Option<std::path::PathBuf> {
    directories::BaseDirs::new().map(|d| std::path::PathBuf::from(d.home_dir()))
}

/// A macro that generates different functions that provide and create directories for your app.
#[macro_export]
macro_rules! generate_directory_function {
    // // Passing on tokens for each action, seperated by commas
    // (
    //     $($($action_tokens:tt)*,)+
    // ) => {
    //     $(
    //         generate_functions($(action_tokens:tt)*)!;
    //     )+
    // };

    // For generating a function that creates the directories::ProjectDir to be used for your app.
    (
        $(#[$($attr:meta)*])*
        project
        $qualifier:literal
        $organisation:literal
        $app_name:literal
        $data_local_folder_name:literal
    ) => {
        $(#[$($attr)*])*
        $crate::project_directory!(
            $qualifier
            $organisation
            $app_name
            $data_local_folder_name
        );
    };

    // For generating a function that creates the directories::ProjectDir to be used for your app.
    // This provides the portable version of the project directory function.
    (
        $(#[$($attr:meta)*])*
        project_portable
        $qualifier:literal
        $organisation:literal
        $app_name:literal
        $data_local_folder_name:literal
    ) => {
        $(#[$($attr)*])*
        $crate::project_directory!(
            portable
            $qualifier
            $organisation
            $app_name
            $data_local_folder_name
        );
    };

    // For generating a function that provides the path to the local data directory for your app 
    // (and creates one if not present).
    (
        $(#[$($attr:meta)*])*
        data_local
    ) => {
        $(#[$($attr)*])*
        $crate::data_local_directory!();
    };

    // For generating a function that provides the path to the configuration directory for your app 
    (
        $(#[$($attr:meta)*])*
        config
    ) => {
        $(#[$($attr)*])*
        $crate::config_directory!();
    };

    // For generating a function that provides the path to the given directory in the local data directory for your app
    // (and creates one if not present).
    (
        $(#[$($attr:meta)*])*
        data_local
        $directory_name:ident
    ) => {
        $crate::in_data_local_directory!($(#[$($attr)*])* - $directory_name);

    };

    // For generating a function that provides the path to the given directory in the configuration directory for your app
    // (and creates one if not present).
    (
        $(#[$($attr:meta)*])*
        config
        $directory_name:ident
    ) => {
        $crate::in_config_directory!($(#[$($attr)*])* - $directory_name);
    };
}

/// Generates a function that creates the project directory for your app.
/// 
/// Creates a directories::ProjectDirs for the given project.
/// 
/// When the portable flag is added, the generated project_dirs function will create a directories::ProjectDirs,
/// in a path adjacent to app executable, and returns the path
#[macro_export]
macro_rules! project_directory {
    (
        $qualifier:literal
        $organisation:literal
        $app_name:literal
        $data_local_folder_name:literal
    ) => {
        
        /// Creates a directories::ProjectDirs for the given project.
        fn project_dirs() -> Option<$crate::directories::ProjectDirs> {
            $crate::directories::ProjectDirs::from($qualifier, $organisation, $app_name)
        }
    };
    (
        portable
        $qualifier:literal
        $organisation:literal
        $app_name:literal
        $data_local_folder_name:literal
    ) => {
        /// Creates a directories::ProjectDirs for the given project, but in a path adjacent to app executable, and returns the path.
        fn project_dirs() -> Option<$crate::directories::ProjectDirs> {
            if let Ok(current_exe) = std::env::current_exe() {
                if let Some(parent) = current_exe.parent() {
                    return $crate::directories::ProjectDirs::from_path(parent.join($data_local_folder_name));
                }
                unreachable!("Couldn't obtain current process parent path");
            }
            unreachable!("Couldn't obtain current process path");
        }
    }
}

/// Generates a function that creates the project directory for your app.
/// 
/// The generated project_dirs function helps create a directories::ProjectDirs from the given path.
#[macro_export]
macro_rules! project_directory_portable {
    (
        $qualifier:literal
        $organisation:literal
        $app_name:literal
        $data_local_folder_name:literal
    ) => {
        
        /// Creates a directories::ProjectDirs for the given project.
        fn project_dirs() -> Option<$crate::directories::ProjectDirs> {
            $crate::directories::ProjectDirs::from($qualifier, $organisation, $app_name)
        }
    }
}

/// Generates a function that provides the path to the local data directory for your app 
/// (and creates one if not present).
/// 
/// Need to use this macro along with the project_directory macro, as this macro calls
/// the project_dirs function.
/// Or you can call generate_directory_function macro with the project label 
/// to generate the project_dirs function.
#[macro_export]
macro_rules! data_local_directory {
    () => {
        // Get path of local data directory
        // Local data directory differs from data directory
        // on some platforms and is not transferred across
        // machines
        pub fn data_local_directory() -> Option<std::path::PathBuf> {
            match project_dirs() {
                Some(dir) => {
                    let dir = dir.data_local_dir();
                    if !dir.exists() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    Some(dir.to_path_buf())
                }
                None => None,
            }
        }
    }
}

/// Generates a function that provides the path to the configuration directory for your app 
/// (and creates one if not present).
/// 
/// Need to use this macro along with the project_directory macro, as this macro calls
/// the project_dirs function.
/// Or you can call generate_directory_function macro with the project label 
/// to generate the project_dirs function.
#[macro_export]
macro_rules! config_directory {
    () => {
        // Config directory contain only configuration files
        pub fn config_directory() -> Option<std::path::PathBuf> {
            match project_dirs() {
                Some(dir) => {
                    let dir = dir.config_dir();
                    if !dir.exists() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    Some(dir.to_path_buf())
                }
                None => None,
            }
        }
    }
}

/// Generates a function that provides the path to a given directory in the local data directory for your app
/// (and creates one if not present).
/// 
/// Need to use this macro along with the data_local_directory macro, as this macro calls
/// the data_local_directory function.
#[macro_export]
macro_rules! in_data_local_directory {
    ($directory_name:ident) => {
        in_data_local_directory(
            /// Gets the path to the given directory that resides in the local data directory
            - $directory_name
        );
    };
    (
        $(#[$($attr:meta)*])*
        -
        $directory_name:ident
    ) => {
        $crate::paste::paste! {
            $(#[$($attr)*])*
            pub fn [<$directory_name _directory>]() -> Option<std::path::PathBuf> {
                if let Some(dir) = data_local_directory() {
                    let dir = dir.join(stringify!($directory_name));
                    if !dir.exists() {
                        let _ = std::fs::create_dir(&dir);
                    }
                    Some(dir)
                } else {
                    None
                }
            }
        }
    };
}

/// Generates a function that provides the path to a given directory in the local data directory for your app
/// (and creates one if not present).
/// 
/// Need to use this macro along with the data_local_directory macro, as this macro calls
/// the data_local_directory function.
#[macro_export]
macro_rules! in_config_directory {
    ($directory_name:ident) => {
        in_config_directory(
            /// Gets the path to the given directory that resides in the configuration directory
            - $directory_name
        );
    };
    (
        $(#[$($attr:meta)*])*
        -
        $directory_name:ident
    ) => {
        $crate::paste::paste! {
            $(#[$($attr)*])*
            pub fn [<$directory_name _directory>]() -> Option<std::path::PathBuf> {
                if let Some(dir) = config_directory() {
                    let dir = dir.join(stringify!($directory_name));
                    if !dir.exists() {
                        let _ = std::fs::create_dir(&dir);
                    }
                    Some(dir)
                } else {
                    None
                }
            }
        }
    };
}