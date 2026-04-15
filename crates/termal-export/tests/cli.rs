use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn fasta_fixture() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, ">seq1").unwrap();
    writeln!(file, "ACG").unwrap();
    writeln!(file, ">seq2").unwrap();
    writeln!(file, "TTA").unwrap();
    file
}

#[test]
#[ignore = "order not implemented yet"]
fn cli_requires_order_argument_for_now() {
    let input = fasta_fixture();

    let mut cmd = Command::cargo_bin("termal-export").unwrap();
    cmd.arg(input.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--order <ORDER>"));
}

#[test]
fn cli_rejects_invalid_rows_range_syntax() {
    let input = fasta_fixture();

    let mut cmd = Command::cargo_bin("termal-export").unwrap();
    cmd.args(["--order", "TODO"])
        .arg(input.path())
        .args(["--rows", "abc"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid range 'abc'"));
}

#[test]
fn cli_rejects_reversed_rows_range() {
    let input = fasta_fixture();

    let mut cmd = Command::cargo_bin("termal-export").unwrap();
    cmd.args(["--order", "TODO"])
        .arg(input.path())
        .args(["--rows", "5:2"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("range START must be <= END in '5:2'"));
}

#[test]
fn cli_rejects_invalid_cols_range_syntax() {
    let input = fasta_fixture();

    let mut cmd = Command::cargo_bin("termal-export").unwrap();
    cmd.args(["--order", "TODO"])
        .arg(input.path())
        .args(["--cols", "bogus"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid range 'bogus'"));
}
