#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jihaz_app::app;

pub fn main() {
    app::main();
}
