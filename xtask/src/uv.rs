//! [`uv`](https://docs.astral.sh/uv/) handles all Python tooling.

use std::io::{self, Error, ErrorKind};
use std::process::Command;

const NEEDS_UV: &str = "needs uv (https://docs.astral.sh/uv/)";

fn uv(command: &str, args: &[&str]) -> Command {
    let mut uv = Command::new("uv");
    uv.args(["run", "--locked"]).arg(command).args(args).current_dir(env!("CARGO_MANIFEST_DIR"));
    uv
}

/// `uv run` from any cwd, failing on a stale `uv.lock`.
pub fn run(command: &str, args: &[&str]) -> io::Result<()> {
    crate::run(uv(command, args)).map_err(|error| match error.kind() {
        ErrorKind::NotFound => Error::other(NEEDS_UV),
        _ => error,
    })
}

/// [`run()`], only if `uv` is available.
pub fn opt(command: &str, args: &[&str]) -> io::Result<()> {
    crate::run(uv(command, args)).or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            eprintln!("warning: skipping `{command}`: {NEEDS_UV}");
            Ok(())
        }
        _ => Err(error),
    })
}
