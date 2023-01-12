/// Print function for warning output
/// # Arguments
/// * `msg` - The message to output
#[macro_export]
macro_rules! warn {
    ($msg:literal) => {{
        colour::yellow_ln!("WARN:  {}", $msg)
    }};

    ($msg:literal, $($args:expr),*) => {{
        colour::yellow_ln!("WARN:  {}", format!($msg, $($args),*))
    }};
}

/// Print function for error output
/// # Arguments
/// * `msg` - The message to output
#[macro_export]
macro_rules! error {

    ($msg:literal) => {{
        colour::red_ln!("ERROR: {}", $msg)
    }};

    ($msg:literal, $($args:expr),*) => {{
        colour::red_ln!("ERROR: {}", format!($msg, $($args),*))
    }};
}

/// Print function for just basic log output
/// # Arguments
/// * `msg` - The message to output
#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        colour::blue_ln!("LOG:   {}", $msg)
    };

    ($msg:literal, $($args:expr),*) => {{
        colour::blue_ln!("LOG:   {}", format!($msg, $($args),*))
    }};
}