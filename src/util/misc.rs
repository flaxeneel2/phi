use std::env;
use std::process::exit;
use crate::{config, error};

/// Takes time in seconds and converts it into a string in the form hour HH:mm:ss
/// # Arguments
/// * `time` - time in seconds
pub fn time_to_string(time: u32) -> String {
    let mut string = "".to_string();
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;
    if hours!=0 {
        string.push_str(format!("{} hours", hours).as_str())
    }
    if minutes != 0 {
        if hours != 0 {
            string.push(' ');
        }
        string.push_str(format!("{} minutes", minutes).as_str())
    }
    if seconds != 0 {
        if hours != 0 || minutes != 0 {
            string.push(' ')
        }
        string.push_str(format!("{} seconds", seconds).as_str())
    }
    string
}

pub fn run_startup_checks() {
    let jar_loc = config().jar_location.clone();
    if ! jar_loc.exists() {
        error!("Jar location not set! Please check help page for usage");
        exit(1)
    }
    env::set_current_dir(jar_loc.parent().unwrap()).unwrap();
}