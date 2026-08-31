//! A real child process used only by the process-contract tests.
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    let query = arguments
        .first()
        .is_some_and(|value| value == "--shuttle-protocol-version");
    let phase = if query {
        "query"
    } else if arguments.is_empty() {
        "run"
    } else {
        "compile"
    };
    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open(env::var_os("SHUTTLE_STUB_LOG").expect("stub log"))
        .expect("open log");
    writeln!(
        log,
        "{phase}:{}",
        env::current_exe().expect("stub path").display()
    )
    .expect("write log");
    let mode = env::var("SHUTTLE_STUB_MODE").unwrap_or_default();
    if query {
        match mode.as_str() {
            "query-version" => println!("2"),
            "query-no-newline" => print!("1"),
            "query-stderr" => {
                println!("1");
                eprintln!("unexpected diagnostic");
            }
            "query-exit" => std::process::exit(1),
            _ => println!("1"),
        }
        return;
    }
    if phase == "run" {
        println!("program");
        eprintln!("program stderr");
        std::process::exit(7);
    }
    if let Some(code) = mode.strip_prefix("compile-") {
        eprintln!("fixture.co:3:5: error: stub rejection");
        std::process::exit(code.parse().expect("stub exit code"));
    }
    if let Some(index) = arguments.iter().position(|argument| argument == "--output") {
        fs::copy(
            env::current_exe().expect("stub executable"),
            PathBuf::from(&arguments[index + 1]),
        )
        .expect("create fake executable");
    }
}
