![Build](https://github.com/sib-swiss/termal/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/termal-msa.svg)](https://crates.io/crates/termal-msa)
[![DOI](https://zenodo.org/badge/976490057.svg)](https://doi.org/10.5281/zenodo.15352914)

Summary
=======

Termal is a program for examining multiple sequence **al**ignments in a **term**inal.

* No installer (just download and uncompress)
* No dependencies.
* Best results in a dark-themed terminal.

Quick Start 
============

Download the latest archive, uncompress it, and run the binary on the example
alignment (see below). Press "`?`" for help.


### Linux (x86_64)

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.0.0/termal-v1.0.0-linux-x86_64.tar.gz)

```bash
tar -xzf termal-v1.0.0-linux-x86_64.tar.gz
./termal data/example-1.msa
```

---

### Windows

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.0.0/termal-v1.0.0-windows-x86_64.zip)

1. Unzip the archive
2. Open a terminal and run:

```powershell
termal.exe example-1.msa
```

---

### macOS

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.0.0/termal-v1.0.0-macos-x86_64.tar.gz)

```bash
tar -xzf termal-v1.0.0-macos-x86_64.tar.gz
./termal data/example-1.msa
```

Tested on:
- Linux (EndeavourOS Build-ID 2024-01-15)
- macOS 10.13.6 (High Sierra)
- Windows 11

Synopsis
========

Just pass your alignment as argument to the `termal` binary:

```bash
$ termal [options] <alignment>
```

Interface
=========

`termal` runs in a terminal and is entirely keyboard-driven. 

Key Bindings
------------

To see the key bindings, press "`?`" while running `termal`, or run `termal -h`.

Features
========

Termal has the basic features you'd expect of an alignment viewer, such as:

* moving and jumping around
* zoomed in or zoomed-out (whole alignment) views
* consensus sequence
* residue colouring
* representation of conservation

Motivation
==========

Primary
-------

Multiple sequence alignments often reside on distant machines like HPC clusters,
for two reasons:

1. They may require nontrivial resources to compute (this is especially true of
   large alignments);
1. They may serve as inputs to heavy-duty analyses like computing phylogenetic
   trees.

Like any other input data, it's a good idea to have a quick look at an alignment
before it is fed to an analysis pipeline. There are many fine tools for doing
this, but most of them have a graphical user interface, so they (usually) can't
be used over SSH. A multiple sequence alignment is basically just text, so it is
well suited for a TUI.

Example
=======

```bash
$ termal data/aln4.pep
```
```
┌──┌───────────┌ data/aln4.pep - 16/16s (1.00) x 70/561c (0.12) - ────────────────────┐
│ 1│Abro_00865 │ELTDGFHLIIDALKLNGLNTIYGVPGIPITDFGRMAQAEGIRVLSFRHEQNAGYAASIAGFLTK-KPGVC│
│ 2│Aoxa_03215 │NLTDGFHALIDAFKKNDINNIYAVAGIPITDVLRLAVEREMKVIAFRHESNAGHAAAIAGYLTQ-KPGIC│
│ 3│Bden_01758 │ELTDGFHLVIDAMKLNGIDTIYNVPGIPITDLGRMAQAEGIRVLSFRHEQNAGYAAAIAGFLTK-KPGVC│
│ 4│Bphy_06740 │ETTDGFHLVIDALKLNDIKTIFGLVGIPITDLARLAQAEGMRFIGFRHEQHAGNAAAVSGYMTK-KPGIC│
│ 5│Bsp1_02714 │ELTDGFHLVIDALKLNGIDTVYGVPGIPITDLGRMAQAAGIRVLSFRHEQNAGYAASIAGFLTK-RPGVC│
│ 6│Cnec_05522 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 7│Cox1_00651 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 8│Cox2_00498 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 9│Mmas_01638 │ALTDGFHLVIDALKLNGINTIYDVPGIPISDLLRMAQAEGMRVISFRHEQNAGNAAAIAGFLTK-KPGVC│
│10│Msp1_01810 │ELTDGFHLVIDALKLNGIDTIYGVPGIPITDLGRMCQEEGMRVISFRHEQNAGNAAAIAGFLTK-KPGIC│
│11│Ofo1_01671 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARMWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
│12│Ofo2_00503 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARMWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
│13│Ofo3_01693 │ELTDGFHVLKDTLKLNGIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIQGDKPGVC│
│14│Osp1_01723 │ELTDGFHVLKDVLKVNGIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIHGDKPGVC│
│15│Osp2_01577 │ELTDGFHVLMDTLKMNDIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIQGDKPGVC│
│16│Osp3_01912 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARLWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
└──└───────────└🬹🬹🬹🬹🬹🬹🬹🬹🬹═══════════════════════════════════════════════════════┘
│              │|    :    |    :    |    :    |    :    |    :    |    :    |    :    │
│Position      │0        10        20        30        40        50        60        7│
│Consensus     │elTDGFHlvIDALKlNgIdtiYGvpGIPITdLaRlaqadGmrviSFRHEQnAGyAAaIAGyltk-KPGVC│
│Conservation  │▅▅█████▄▄▆█▆▆█▄█▅▇▃▆▄▇▅▆▄███▆▇▅▆▄█▅▄▅▃▂▇▄▆▅▄▆████▇▅██▄██▅▇▇█▅▄▄▃▁▇██▆█│
└──────────────└──────────────────────────────────────────────────────────────────────┘
```

**Notes**

1. The above example appears in monochrome due to Markdown rendring, but by default Termal uses colours, e.g. to reflect amino acid chemistry.
2. The above example had to be slightly tweaked because the separation line between the main and bottom panel is rendered too wide in some Markdown engines.


LICENSE
=======

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
