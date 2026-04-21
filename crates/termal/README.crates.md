# termal

`termal` is a terminal-based viewer for multiple sequence alignments (MSAs).  It
provides a smooth interface to explore alignments directly from the
command line — especially useful when working over SSH or in headless
environments.

---

##  Installation

```bash
cargo install termal-msa
```

---

##  Quick Usage

Once installed, run:

```bash
termal <my-alignment>
```
where `my-alignment` is a multiple alignment in Fasta or Stockholm format.

For help, run

```bash
termal -h
```

Or press `?` while running to see key bindings.


---

## Features

- Zoomed-in and zoomed-out views of the alignment
- Consensus sequence display
- Sequence metrics such as ungapped length and similarity to consensus
- Ordering by metrics 
- Conservation indicators
- Color maps for nucleotides and amino acids
- Color themes
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
