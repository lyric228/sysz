use std::{
    io::{self, Write},
    process::{Command, Output, Stdio},
};

use crate::{Error, Result};

/// Executes a command silently and returns its Output.
pub fn slrun(command_line: &str) -> Result<Output> {
    let trimmed = command_line.trim();
    if trimmed.is_empty() {
        return Err(Error::IoError("Empty command line".into()));
    }

    let mut args =
        shell_words::split(trimmed).map_err(|e| Error::IoError(format!("Parse error: {e}")))?;

    if args.is_empty() {
        return Err(Error::IoError("No command specified".into()));
    }

    let program = args.remove(0);

    Command::new(&program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| Error::IoError(format!("Execution error: {e}")))
}

/// Executes a command, prints stdout, and returns its Output.
pub fn run(command: &str) -> Result<Output> {
    let output = slrun(command)?;

    if !output.stdout.is_empty() {
        io::stdout()
            .write_all(&output.stdout)
            .map_err(|e| Error::IoError(format!("Failed to write stdout: {e}")))?;
    }

    if !output.stderr.is_empty() {
        io::stderr()
            .write_all(&output.stderr)
            .map_err(|e| Error::IoError(format!("Failed to write stderr: {e}")))?;
    }

    Ok(output)
}

/// Macro to call `slrun` with a formatted command string.
#[macro_export]
macro_rules! slrunf {
    ($($arg:tt)*) => {
        slrun(&format!($($arg)*))
            .map_err(Error::from)
    }
}
pub use slrunf;

/// Macro to call `run` with a formatted command string.
#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*))
            .map_err(|e| Error::IoError(format!("Execution error: {e}")))
    }
}
pub use runf;

/// Reads a line from stdin into the provided buffer.
pub fn input_buf(buffer: &mut String) -> Result<()> {
    buffer.clear();
    io::stdin()
        .read_line(buffer)
        .map_err(|e| Error::IoError(format!("Failed to read line: {e}")))?;

    let len = buffer.trim_end_matches(['\r', '\n']).len();
    buffer.truncate(len);

    Ok(())
}

/// Reads a line from stdin and returns a new String.
pub fn input() -> Result<String> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .map_err(|e| Error::IoError(format!("Failed to read line: {e}")))?;

    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    Ok(s)
}
