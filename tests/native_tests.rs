// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Opt-in execution tests; require a compiler with its native toolchain.
#[allow(dead_code)]
mod support;

use std::fs;
use support::{Fixture, compiler, expect_status, run};

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn builds_and_runs_only_the_selected_root_entry() {
    let fixture = Fixture::new();
    let built = run(&mut fixture.shuttle("build", &compiler()));
    expect_status(&built, 0);
    assert!(built.stdout.is_empty() && built.stderr.is_empty());
    let target = fixture.root.join("app/target/x86_64");
    assert!(
        target
            .join(format!("app{}", std::env::consts::EXE_SUFFIX))
            .is_file()
    );
    let artifacts = fs::read_dir(target.join("packages"))
        .expect("package artifacts")
        .collect::<Result<Vec<_>, _>>()
        .expect("package artifact entries");
    assert_eq!(artifacts.len(), 4);
    assert!(artifacts.iter().all(|entry| {
        entry.path().is_file() && entry.path().extension().is_some_and(|value| value == "cpa")
    }));
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 0);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("UTF-8 output")
            .replace("\r\n", "\n"),
        "41\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn native_build_accepts_unicode_project_paths() {
    let fixture = Fixture::named("Cloth \u{03bb} native &");
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 0);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("UTF-8 output")
            .trim(),
        "41"
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn propagates_the_program_exit_status_and_does_not_run_stale_outputs() {
    let fixture = Fixture::new();
    fixture.write(
        "app/src/Main.co",
        "static func Main(): int32 { println(\"ran\"); return 7; }\n",
    );
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 7);
    assert_eq!(
        String::from_utf8(output.stdout).expect("output").trim(),
        "ran"
    );
    let executable = fixture
        .root
        .join("app/target/x86_64")
        .join(format!("app{}", std::env::consts::EXE_SUFFIX));
    let previous = fs::read(&executable).expect("completed executable");
    fixture.write("app/src/Main.co", "static func Main() { missing(); }\n");
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 1);
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read(executable).expect("previous executable"), previous);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn rejects_an_ineligible_entry_instead_of_using_another_main() {
    let fixture = Fixture::new();
    fixture.write("app/src/Main.co", "static func NotMain() {}\n");
    let output = run(&mut fixture.shuttle("build", &compiler()));
    expect_status(&output, 1);
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .expect("error")
            .contains("Main")
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn relocated_native_builds_are_byte_identical() {
    let first = Fixture::new();
    let second = Fixture::new();
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    expect_status(&run(&mut first.shuttle("build", &compiler())), 0);
    // Cross a PE timestamp boundary to detect wall-clock-dependent link output.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    expect_status(&run(&mut second.shuttle("build", &compiler())), 0);
    assert!(
        fs::read(first.root.join(&executable)).expect("first binary")
            == fs::read(second.root.join(&executable)).expect("second binary"),
        "relocated native builds differ"
    );
}
