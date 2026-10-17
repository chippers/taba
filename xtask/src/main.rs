use std::io::{self, Error};
use std::process::Command;

use clap::{Args, Parser, Subcommand};

mod uv;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Properly format everything, or check if it already is.
    Fmt(Fmt),

    /// Run all linters.
    Lint,

    /// Run all tests, in every useful combination.
    Test(Test),

    /// Pin taba-rolldown to a rolldown release, or see which fit an MSRV.
    #[command(subcommand)]
    Rolldown(Rolldown),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Fmt(fmt) => fmt.run(),
        Commands::Lint => lint(),
        Commands::Test(test) => test.run(),
        Commands::Rolldown(rolldown) => rolldown.run(),
    };

    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

/// Run command and turn any non-successful exit into an error.
fn run(mut command: Command) -> io::Result<()> {
    let status = command.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::other(format!("{command:?} failed with {status}")))
    }
}

#[derive(Args)]
struct Fmt {
    /// Check the formatting instead of writing.
    #[arg(long)]
    check: bool,
}

impl Fmt {
    fn run(self) -> io::Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("+nightly"); // we use nightly-only rustfmt settings
        cmd.args(["fmt", "--all"]); // otherwise only the current dir's crate
        if self.check {
            cmd.arg("--check");
        }

        run(cmd)?;

        let mut ruff = vec!["format", "--quiet"];
        if self.check {
            ruff.push("--check");
        }

        uv::opt("ruff", &ruff)
    }
}

fn lint() -> io::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"]);
    run(cmd)?;

    uv::opt("ruff", &["check", "--quiet"])?;
    uv::opt("ty", &["check"])
}

#[derive(Args)]
struct Test {
    /// Test with release optimizations enabled.
    #[arg(long)]
    release: bool,
}

impl Test {
    fn run(self) -> io::Result<()> {
        let mut test = Command::new("cargo");
        test.args(["test", "--workspace"]);

        if self.release {
            test.arg("--release");
        }

        run(test)
    }
}

#[derive(Subcommand)]
enum Rolldown {
    /// Pin all Rolldown crates to the same version, and related Oxc version.
    Pin {
        /// The rolldown version, e.g. `1.2.5`.
        version: String,
    },

    /// List rolldown releases with the oxc each requires and that oxc's MSRV.
    Msrv,
}

impl Rolldown {
    fn run(self) -> io::Result<()> {
        match self {
            Rolldown::Pin { version } => uv::run("rolldown.py", &["pin", &version]),
            Rolldown::Msrv => uv::run("rolldown.py", &["msrv"]),
        }
    }
}
