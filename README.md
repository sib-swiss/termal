![Build](https://github.com/sib-swiss/termal/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/termal-msa.svg)](https://crates.io/crates/termal-msa)
[![DOI](https://zenodo.org/badge/976490057.svg)](https://doi.org/10.5281/zenodo.15352914)

Summary
=======

Termal is a program for examining multiple sequence **al**ignments in a **term**inal.

* No installer (just download and uncompress)
* No dependencies
* Best results in a terminal that supports 24-bit ("true color") color output

Quick Start 
============

Download the latest archive, uncompress it, and run the binary on the example
alignment (see below). Press "`?`" for help.

### Linux (x86_64)

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.1.0/termal-v1.0.0-linux-x86_64.tar.gz)

```bash
tar -xzf termal-v1.1.0-linux-x86_64.tar.gz
./termal data/example-1.msa
```

---

### Windows

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.1.0/termal-v1.0.0-windows-x86_64.zip)

1. Unzip the archive
2. Open a terminal and run:

```powershell
termal.exe example-1.msa
```

---

### macOS

* [Download](https://github.com/sib-swiss/termal/releases/download/v1.1.0/termal-v1.0.0-macos-x86_64.tar.gz)

```bash
tar -xzf termal-v1.1.0-macos-x86_64.tar.gz
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

To see the key bindings while running `termal`, press "`?`". To display them in
the console, run `termal -b`. The main bindings are as follows:

### Motion

arrows: scroll 1 column/line; shift-arrows : scroll 1 screenful
        h,j,k,l are aliases for left, down, up, and right arrow
^,G,g,$: full left, bottom, top, full right

### Zooming

z,Z: cycle through zoom modes

### Adjusting the Panes

<,>: widen/narrow label pane
a  : hide/show label pane
c  : hode/show consensus pane
f  : toggle fullscreen alignment pane

### Video

s: next color scheme
m: next color map
i: toggle inverse/direct video

Try dark/inverse for best results (this is the default).

### Metrics and Orderings

o: next ordering
t: next metric


Features
========

Termal has the basic features you'd expect of an alignment viewer, such as:

* moving and jumping around
* zoomed in or zoomed-out (whole alignment) views (see _Zooming_ below)
* consensus sequence
* residue coloring
* representation of conservation

Zooming
-------

By default, Termal shows as much of the alignment as fits on the screen. Smaller
alignments can fit entirely on screen, but it's quite common for alignments to
be too large, at least in one dimension, sometimes both. To see more of the
alignment, there are two options:

* Scrolling: this simply shifts the displayed portion ("view port") of the
  alignment left, right, up, or down. One can move by a single line (sequence)
  or column (position), by screenfuls, or directly to the top, bottom, leftmost,
  or rightmost positions. This is done with the motion keys, including arrows

* Zooming Out: this shows the first and last sequences, as well as evenly-spaced
  sequences in between so as to show as many sequences as possible. The same
  sampling is applied to columns. A box shows the location of the view port,
  that is, what part of the alignment would fill the alignment area when zooming
  back in. The zoom box can be moved using the same commands as for scrolling
  (see above). A variant of zooming out will sample rows and columns will
  preserve the aspect ratio of the alignment, at the expense of potentially
  fewer sequences or columns shown.

Colors and Themes
-----------------

The program was developed and tested in a dark-themed terminal, but it works
reasonably well in light themes. It also has a monochrome "theme", which adapts
to the theme (white on black in a dark theme, and the other way around). Press
`s` to move from dark to light to monochrome (and back).

`Termal` can map residues to colors (except in monochrome mode) using one of the
built-in color maps. The default color map is Clustal's for amino acids, and
JalView's for nucleotides. Press `m` to change color maps.

By default, residues are colored in inverse video, as is done by most alignment
viewers, but direct video is also possible. The `i` key toggles between the
video modes.

Example
=======

```bash
$ termal data/aln4.pep
```
```
┌──┌───────────┌ data/aln4.pep | 16/16s x 70/561c | Dark Mono White | Zoomed in ──────┐
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

1. The above example shows `termal` in monochrome mode due to limitations in
   Markdown rendring, but by default Termal uses colours, e.g. to reflect amino
   acid chemistry.
2. The above example had to be slightly tweaked because the separation line
   between the main and bottom panel is rendered too wide in some Markdown
   engines.

Motivations
===========

Primary
-------

Multiple sequence alignments often reside on distant machines like HPC clusters,
for two reasons:

1. They may require nontrivial resources to compute (this is especially true of
   large alignments);
1. They may serve as inputs to heavy-duty analyses like computing phylogenetic
   trees.

Like for any other input data, it's a good idea to have a quick look at an
alignment before it is fed to an analysis pipeline. There are many fine tools
for doing this, but most of them have a graphical user interface, so they
  (usually) can't be used over SSH. A multiple sequence alignment is basically
  just text, so it is well suited for a text interface.

Secondary
---------

Even on a local machine, there are use cases for viewing alignments in a
terminal, such as:

* Short load times: Rust programs can be _very_ fast, and it is convenient to be
  able to have a quick look at an alignment without waiting several seconds for
  the program and alignment to load.
* Not needing to leave one's work environment - use `termal` on an alignment
  like you would use `less`, `bat`, etc. on any text file.

BUGS AND LIMITATIONS
====================

* Currently, Termal can only read Fasta alignments (i.e., no Phylip or other formats).

LICENSE
=======

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
