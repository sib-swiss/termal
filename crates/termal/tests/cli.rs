// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

#[test]
fn dry_run_reports_custom_colormap_from_file() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");
    let aln = repo_root.join("data/test1.fas");
    let cmap = repo_root.join("data/colormaps/test.json");

    let mut cmd = Command::cargo_bin("termal").expect("termal binary");
    cmd.args([
        "--dry-run",
        "--color-map",
        cmap.to_str().expect("utf-8 colormap path"),
        aln.to_str().expect("utf-8 alignment path"),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "User color map file: {}",
            cmap.display()
        )))
        .stdout(predicate::str::contains("User color map: {A: #7fffd4, "))
        .stdout(predicate::str::contains("C: #00ffff, "))
        .stdout(predicate::str::contains("Y: #d1fee1, "));
}
