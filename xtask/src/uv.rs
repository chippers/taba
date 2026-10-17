//! [`uv`](https://docs.astral.sh/uv/) handles all Python tooling.

use std::io::{self, Error, ErrorKind};
use std::process::Command;

const NEEDS_UV: &str = "needs uv (https://docs.astral.sh/uv/)";

fn uv(command: &str, args: &[&str]) -> Command {
    let mut uv = Command::new("uv");
    uv.args(["run", "--locked"]).arg(command).args(args).current_dir(env!("CARGO_MANIFEST_DIR"));
    uv
}

fn uv_missing(error: Error) -> Error {
    match error.kind() {
        ErrorKind::NotFound => Error::other(NEEDS_UV),
        _ => error,
    }
}

/// `uv run` from any cwd, failing on a stale `uv.lock`.
pub(crate) fn run(command: &str, args: &[&str]) -> io::Result<()> {
    crate::run(uv(command, args)).map_err(uv_missing)
}

/// Ensure `uv` is available.
pub(crate) fn require() -> io::Result<()> {
    let mut version = Command::new("uv");
    version.arg("--version");
    crate::run(version).map_err(uv_missing)
}

/// [`run()`], only if `uv` is available.
pub(crate) fn opt(command: &str, args: &[&str]) -> io::Result<()> {
    crate::run(uv(command, args)).or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            eprintln!("xtask warning: skipping `{command}`: {NEEDS_UV}");
            Ok(())
        }
        _ => Err(error),
    })
}
