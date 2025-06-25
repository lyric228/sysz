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
