![Build](https://github.com/sib-swiss/termal/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/termal-msa.svg)](https://crates.io/crates/termal-msa)
[![DOI](https://zenodo.org/badge/976490057.svg)](https://doi.org/10.5281/zenodo.15472432)

# termal

**termal** is a terminal-based viewer for multiple sequence alignments, designed
for fast, keyboard-driven navigation in local and remote (SSH/HPC) environments.

It requires no installer, has no runtime dependencies, and runs directly in a
terminal.

---

## Installation

### Prebuilt binaries

Download a prebuilt binary from the GitHub Releases page:
https://github.com/sib-swiss/termal/releases

For the current release (v1.3.0):
https://github.com/sib-swiss/termal/releases/tag/v1.3.0

Unpack the archive and run the binary directly.

> macOS users may need to remove the quarantine flag:
> ```sh
> xattr -d com.apple.quarantine ./termal
> ```

### From source

```sh
cargo install termal-msa
```
