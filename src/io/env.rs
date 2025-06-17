use std::env;

/// Returns a vector of command-line arguments, including the program name.
pub fn get_full_args() -> Vec<String> {
    env::args().collect()
}

/// Returns a vector of command-line arguments, excluding the program name.
pub fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    if !args.is_empty() {
        args.remove(0);
    }
    args
}

/// Returns all command-line arguments as a single string, joined by spaces.
pub fn get_full_str_args() -> String {
    env::args().collect::<Vec<_>>().join(" ")
}

/// Returns command-line arguments (excluding program name) as a single string, joined by spaces.
pub fn get_str_args() -> String {
    let mut args = env::args();
    if args.next().is_some() {
        args.collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    }
}
