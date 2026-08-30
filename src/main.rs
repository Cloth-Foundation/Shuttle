#![forbid(unsafe_code)]

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let command = shuttle::cli::Cli::parse();
    let current_directory = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("shuttle: error: could not read the current directory: {error}");
            return ExitCode::from(2);
        }
    };

    match shuttle::cli::execute(command, &current_directory) {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            for diagnostic in failure.diagnostics {
                eprintln!("{diagnostic}");
            }
            ExitCode::from(failure.exit_code)
        }
    }
}
