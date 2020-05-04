use assert_cmd::Command;
use predicates::prelude::*; // Used for writing assertions

use std::io::Write;
use tempfile::NamedTempFile;

const BIN: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[test]
fn show_version() {
  let mut cmd = Command::new("cargo");
  let name_version = format!("{} {}", NAME, VERSION);
  let precidate_fn = predicate::str::contains(name_version);

  cmd.arg("run").arg("--").arg("--version");
  cmd.assert().success().stdout(precidate_fn);
}

#[test]
fn show_version_and_description_on_help() {
  let mut cmd = Command::new("cargo");
  let output = format!("{} {}\n{}", NAME, VERSION, DESCRIPTION);
  let precidate_fn = predicate::str::contains(output);

  cmd.arg("run").arg("--").arg("--help");
  cmd.assert().success().stdout(precidate_fn);
}

#[test]
fn input_from_stdin() {
  let mut cmd = Command::cargo_bin(BIN).unwrap();
  let precidate_fn = predicate::str::contains("hello stdin");

  cmd
    .write_stdin("hello {{ \"stdin\" }}")
    .assert()
    .success()
    .stdout(precidate_fn);
}

#[test]
fn input_from_args_process() {
  let mut cmd = Command::new("cargo");
  let precidate_fn = predicate::str::contains("hello world");

  cmd
    .arg("run")
    .arg("--")
    .arg("-i")
    .arg("hello {{ \"world\" }}");
  cmd.assert().success().stdout(precidate_fn);
}

#[test]
fn input_from_file() -> Result<(), Box<dyn std::error::Error>> {
  let mut file = NamedTempFile::new()?;
  writeln!(file, "hello from {{{{ \"file\" }}}}")?;
  let precidate_fn = predicate::str::contains("hello from file");

  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("--file").arg(file.path());
  cmd.assert().success().stdout(precidate_fn);

  Ok(())
}
