//! Jihaz app is a utility app that generates release packages for your app.
//! You provide is the executable file, raw icon png file, and other information and
//! Jihaz app generates creates the standard icon files and generates the app packages.

mod simple_task;
pub mod app_helpers;
pub mod app;
pub mod deferred;
pub mod app_message;
pub mod progress;
pub mod view;
pub mod widget;

pub use app::*;
pub use simple_task::*;