use std::any::{Any, type_name};

use std::sync::OnceLock;
use regex::Regex;

/// Regex to remove namespace qualifiers.
static QUALIFIER_RE: OnceLock<Regex> = OnceLock::new();

/// Regex getter
#[inline(always)]
fn qualifier_re() -> &'static Regex {
    QUALIFIER_RE.get_or_init(|| {
        Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*::)+")
            .expect("Failed to compile qualifier regex")
    })
}

/// Removes namespace qualifiers from a non-generic type string.
pub fn simplify_nonlist_type<'a>(type_str: &'a str) -> String {
    type_str.rsplit("::").next().unwrap_or(type_str).to_owned()
}

/// Gets the type name of a value using `std::any::type_name`.
pub fn get_type<T: Any>(_: &T) -> &'static str {
    type_name::<T>()
}

/// Checks if a type string appears to be a generic (like `Vec<T>`, `Mutex<T>` or `[T]`).
pub fn is_list_like(type_str: &str) -> bool {
    let mut first_non_ws = None;
    let mut last_non_ws = None;

    for c in type_str.chars() {
        if c == '<' || c == '>' {
            return true;
        }

        if !c.is_whitespace() {
            if first_non_ws.is_none() {
                first_non_ws = Some(c);
            }
            last_non_ws = Some(c);
        }
    }

    first_non_ws == Some('[') && last_non_ws == Some(']')
}

/// Removes namespace qualifiers from a type string, preserving generics structure.
pub fn simplify_type(type_str: &str) -> String {
    if !is_list_like(type_str) {
        return simplify_nonlist_type(type_str);
    }

    let mut result = String::with_capacity(type_str.len());
    let mut token = String::new();
    let mut bracket_depth: i32 = 0;
    let mut requires_processing = false;
    let mut last_char = ' ';

    for c in type_str.chars() {
        match c {
            '<' => {
                bracket_depth += 1;
                token.push(c);
            }
            '>' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                token.push(c);
            }
            ',' if bracket_depth == 0 => {
                process_token(&mut result, &token, requires_processing);
                token.clear();
                requires_processing = false;
            }
            _ => {
                if last_char == ':' && c == ':' {
                    requires_processing = true;
                }
                token.push(c);
                last_char = c;
            }
        }
    }

    process_token(&mut result, &token, requires_processing);
    result
}

/// Processes a single type token, removing namespace qualifiers if needed and appending to result.
fn process_token(result: &mut String, token: &str, requires_processing: bool) {
    if !result.is_empty() {
        result.push_str(", ");
    }

    if requires_processing {
        result.push_str(&qualifier_re().replace_all(token, ""));
    } else {
        result.push_str(token);
    }
}
