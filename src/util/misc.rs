/// Takes time in seconds and converts it into a string in the form hour HH:mm:ss
/// # Arguments
/// * `time` - time in seconds
pub fn time_to_string(time: i32) -> String {
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