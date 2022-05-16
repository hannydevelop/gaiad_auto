//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::GaiadAutoConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;
use std::{ffi::OsStr, panic, process, process::Command as Pcommand, str};

use docker_command::*;
use std::error::Error;
use std::path::Path;

const GAIA_DOCKER_IMAGE: &str = "gaia";
const CHAINID: &str = "mytest";

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct StartCmd {
    img_version: String,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        // Begin readme example
        let output = Launcher::auto()
            .ok_or("container comand not found")
            .unwrap()
            .run(RunOpt {
                image: "docker pull ghcr.io/cosmos/gaia:sha-fca0a63".into(),
                command: Some(Path::new("sh").into()),
                args: vec!["/usr/test.sh".into()],
                ..Default::default()
            })
            .enable_capture()
            .run()
            .unwrap();
        println!("hello ugo");
        assert_eq!(output.stdout_string_lossy(), "hello world\n");
        // End readme example
    }
}

/// Invoke `docker run` with the given arguments, calling the provided function
/// after the container has booted and terminating the container after the
/// provided function completes, catching panics and propagating them to ensure
/// that the container reliably shuts down.
///
/// Prints log output from the container in the event an error occurred.
pub fn docker_run<A, S, F, R>(args: A, f: F) -> R
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    F: FnOnce() -> R + panic::UnwindSafe,
{
    let container_id = exec_docker_command("run", args);
    let result = panic::catch_unwind(f);

    if result.is_err() {
        let logs = exec_docker_command("logs", &[&container_id]);

        println!("\n---- docker stdout ----");
        println!("{}", logs);
    }

    exec_docker_command("kill", &[&container_id]);

    match result {
        Ok(res) => res,
        Err(err) => panic::resume_unwind(err),
    }
}

/// Execute a given `docker` command, returning what was written to stdout
/// if the command completed successfully.
///
/// Panics if the `docker` process exits with an error code.
fn exec_docker_command<A, S>(name: &str, args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = process::Command::new("docker")
        .arg(name)
        .args(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap_or_else(|err| panic!("error invoking `docker {}`: {}", name, err));

    if !output.status.success() {
        panic!("`docker {}` exited with error status: {:?}", name, output);
    }

    str::from_utf8(&output.stdout)
        .expect("UTF-8 error decoding docker output")
        .trim_end()
        .to_owned()
}

impl config::Override<GaiadAutoConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.

    fn override_config(&self, config: GaiadAutoConfig) -> Result<GaiadAutoConfig, FrameworkError> {
        Ok(config)
    }
}
