# termal

`termal` is a terminal-based viewer for multiple sequence alignments (MSAs).  It
provides a smooth interface to explore alignments directly from the
command line — especially useful when working over SSH or in headless
environments.

---

##  Installation

```bash
cargo install termal
```

---

##  Quick Usage

Once installed, run:

```bash
termal <my-alignment>
```

Press `?` while running to see key bindings.

---

## Features

- Zoomed-in and zoomed-out views of the alignment
- Consensus sequence display
- Sequence metrics such as ungapped length and similarity to consensus
- Ordering by metrics
- Residue coloring and conservation indicators
- Full keyboard control, no mouse required

Best results in a dark-themed terminal.

---

## More Info

- Source, releases, and test data:  
  [https://github.com/sib-swiss/termal](https://github.com/sib-swiss/termal)

- Platform-specific binaries (Linux, macOS, Windows) available on the [Releases](https://github.com/sib-swiss/termal/releases) page.

---

## 📝 License

MIT
