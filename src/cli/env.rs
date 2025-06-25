use std::env;

use terminal_size::{Height, Width, terminal_size};

/// Get terminal width and height (x, y)
#[inline]
pub fn txy() -> Option<(u16, u16)> {
    terminal_size().map(|(Width(w), Height(h))| (w, h))
}

/// Get terminal width (x)
#[inline]
pub fn tx() -> Option<u16> {
    terminal_size().map(|(Width(w), _)| w)
}

/// Get terminal height (y)
#[inline]
pub fn ty() -> Option<u16> {
    terminal_size().map(|(_, Height(h))| h)
}

/// Returns a vector of command-line arguments, including the program name.
#[inline]
pub fn full_args() -> Vec<String> {
    env::args().collect()
}

/// Returns a vector of command-line arguments, excluding the program name.
pub fn args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();

    if !args.is_empty() {
        args.remove(0);
    }

    args
}
