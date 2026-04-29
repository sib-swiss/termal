![Build](https://github.com/sib-swiss/termal/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/termal-msa.svg)](https://crates.io/crates/termal-msa)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19897259.svg)](https://doi.org/10.5281/zenodo.19897259)

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

Unpack the archive and run the binary directly. The archive also contains a
manual (`termal-manual.md` and `termal-manual.pdf`), as well as examples of
custom colormaps and ordering files and assorted sample alignments.

```bash
termal data/example-1.msa
termal -f stockholm data/PF00244.26.sto
termal -o data/OX_OFA-pg2.order data/OX_OFA-pg2.msa
termal -c data/colormaps/gecos_default.json data/example-1.msa
```

> macOS users may need to remove the quarantine flag:
> ```sh
> xattr -d com.apple.quarantine ./termal
> ```

### From source

```sh
cargo install termal-msa
```
