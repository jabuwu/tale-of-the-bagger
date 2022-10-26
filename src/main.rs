// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tale_of_the_bagger::game();
}
