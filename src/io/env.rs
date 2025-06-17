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
