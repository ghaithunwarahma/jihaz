use jihaz_primal::pathbuf_to_string::PathBufToString;
use jihaz::sys_task::handle::SystemTaskHandle;
use std::path::PathBuf;
use xilem::{EventLoop, WidgetView, Xilem};
use crate::{
    progress::ProgressMessage, 
    view::{tablet::{tablet, Tablet}, window::Window}
};

#[derive(Clone)]
pub struct AppState {
    pub executable_path: Option<PathBuf>,
    pub original_icon_path: Option<PathBuf>,
    pub target_packages_directory_path: Option<PathBuf>,
    pub app_name_lowercase: String,
    pub progress_message: ProgressMessage,
}

impl AppState {
    pub fn new(
        executable_path: impl Into<Option<PathBuf>>,
        original_icon_path: impl Into<Option<PathBuf>>,
        target_packages_directory_path: impl Into<Option<PathBuf>>,
        app_name_lowercase: String,
    ) -> Self {
        Self {
            executable_path: executable_path.into(),
            original_icon_path: original_icon_path.into(),
            target_packages_directory_path: target_packages_directory_path.into(),
            app_name_lowercase,
            progress_message: ProgressMessage::Nop,
        }
    }
}

fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    Window::new(tablet(
        state.executable_path.to_string().unwrap_or_default(),
        state.original_icon_path.to_string().unwrap_or_default(),
        state.target_packages_directory_path.to_string().unwrap_or_default(),
        state.app_name_lowercase.to_string(),
        &mut state.progress_message,
    ))
}

pub fn main() {
    SystemTaskHandle::activate().unwrap();
    
    let executable_path = "/Users/abdallah/Documents/rust-targets/release/hikma";
    let original_icon_path = "/Users/abdallah/Documents/مشاريع/hikma.svg";
    let target_packages_directory_path = "/Applications";
    let app_name_lowercase: &str = "hikma";

    let app_state = AppState::new(
        PathBuf::from(executable_path),
        PathBuf::from(original_icon_path),
        PathBuf::from(target_packages_directory_path),
        app_name_lowercase.to_string()
    );
    
    let app = Xilem::new(app_state, app_logic);
    let _ = app.run_windowed(EventLoop::with_user_event(), "جهاز".to_string());
    SystemTaskHandle::global().finished();
}