use std::process::{Command, ExitStatus};

use clap::{Args, Parser, Subcommand};
#[derive(Parser)]
#[command()]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Properly format everything, or check if it already is
    Fmt(Fmt),

    /// Run all linters
    Lint,

    /// Run all tests, in every useful combination
    Test(Test),
}

#[derive(Args)]
pub struct Fmt {
    /// Check the formatting instead of writing.
    #[arg(long)]
    check: bool,
}

impl Fmt {
    /// Format everything, or check it.
    fn run(self) -> std::io::Result<ExitStatus> {
        let mut cmd = Command::new("cargo");
        cmd.arg("+nightly"); // we use nightly-only rustfmt settings
        cmd.arg("fmt");

        if self.check {
            cmd.arg("--check");
        }

        cmd.status()
    }
}

/// Lint everything, failing on any warning.
fn lint() -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new("cargo");
    cmd.args(["clippy", "--", "-D", "warnings"]);
    cmd.status()
}

#[derive(Args)]
struct Test {
    /// Test with release optimizations enabled.
    #[arg(long)]
    release: bool,
}

impl Test {
    /// Quickly run nextest, then the leftover doctests.
    fn run(self) -> std::io::Result<ExitStatus> {
        let mut nextest = Command::new("cargo");
        nextest.args(["nextest", "run", "--workspace"]);

        if self.release {
            nextest.arg("--release");
        }

        let nextest = nextest.status()?;
        if !nextest.success() {
            return Ok(nextest);
        }

        let mut cmd = Command::new("cargo");
        cmd.args(["test", "--doc", "--workspace"]);

        if self.release {
            cmd.arg("--release");
        }

        cmd.status()
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let exit = match cli.command {
        Commands::Fmt(fmt) => fmt.run(),
        Commands::Lint => lint(),
        Commands::Test(test) => test.run(),
    }?;

    if !exit.success() {
        std::process::exit(exit.code().unwrap_or(1));
    }

    Ok(())
}
