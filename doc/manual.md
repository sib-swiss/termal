---
geometry:
    - margin=1in 
lang: en
colorlinks: true 
linkcolor: blue 
urlcolor: blue 
mainfont: TeX Gyre Pagella
monofont: JetBrains Mono
title: "Termal"
subtitle: "User Manual"
author: "Thomas Junier"
date: \today
bibliography: "termal.bib"
toc: true
header-includes:
  - |
    \usepackage{etoolbox}
    \makeatletter
    \apptocmd{\maketitle}{%
      \par\bigskip
      \begin{center}
      \ttfamily
      TERM-----AL--------------\\
      TERMinAL-ALIGNM--T-VIEWER\\
      TERM--AL-AL-----N--VIEWER\\
      ------AL-----------VIEW--\\
      ---------ALIGNMENT-VIEWER
      \end{center}
      \bigskip
    }{}{}
    \makeatother
---

#  Overview

**Termal** is a terminal-based viewer for multiple sequence alignments (MSAs).
It is designed for fast, keyboard-driven navigation of large alignments,
particularly in remote or SSH-based environments where graphical tools are
impractical.

Termal is a **read-only** viewer. It does not modify alignments. This may change
in future versions.

Termal is presented in [@junier2025termal].


#  Basic usage

##  Starting Termal

Simply pass your alignment file as an argument to `termal`, e.g.:

```
termal my-alignment.msa
```

### Alignment File Formats

Termal supports Fasta (default) and Stockholm (pass `-f stockholm` (or just `-f
s`)) formats. Any alignment lines shorter than the longest one will be padded
with gap characters at the end.

## Help

For general help, pass option `-h`; for a list of key bindings, pass `-b` or
press `?` after launching `termal`.

## Info mode

With option `-i/--info`, `termal` will output basic metrics about the alignment
(such as number of sequences) and quit.

## Options

The main options are shown in the table below (and are discussed in the text in
the corresponding sections). Other options exist, but they are either
experimental or used for debugging or testing. For a full list, do `termal
-h`.

| **short** | **long**                    | **function**                                                                   |
| :----     | :-------------------------- | :---------------------------                                                   |
| `-b`      | `--show-bindings`           | Show key bindings and exit successfully                                        |
| `-i`      | `--info`                    | Show alignment metrics and quit                                                |
| `-f`      | `--format <FORMAT>`         | Sequence file format [`fasta`/`stockholm`] (or just `f`/`s`); default: `fasta` |
| `-o`      | `--user-order <USER_ORDER>` | User-supplied order (filename)                                                 |
| `-c`      | `--color-map <COLOR_MAP>`   | Gecos color map (filename)                                                     |
| `-n`      | `--dry-run`                 | Dry run - show parameters and quit                                             |
| `-h`      | `--help`                    | Print help                                                                     |
| `-V`      | `--version`                 | Show version                                                                   |

# Screen layout

The interface is divided into four areas:

- **Alignment pane** (center - right): shows the aligned sequences, 
  some properties of the alignments, as well as some current UI settings.
- **Left pane** (left): shows sequence numbers and headers, as well as
  a barplot of the current [metric](#metrics)
- **Consensus pane** (bottom): shows horizontal position, the consensus sequence, and a conservation barplot
- **Corner pane** (bottom left): shows the current metric and ordering (see
  below).

In addition, the last line contains a message area ("modeline"), which displays:

- pending command arguments (counts, searches), if any;
- search match information (when applicable).


#  Interaction

Termal is entirely keyboard-driven. All commands are issued by typing a single
character (usually a letter).

##  Prefix arguments

Many commands (mostly motion) accept a _prefix argument_, which is an integer
number typed _before_ the command character. Typing any digit (except 0) starts
a prefix argument. To cancel, press `Esc`. The meaning of the argument depends
on the command. For example, scrolling commands such as `j` ("scroll one line
down") interpret a prefix argument as a repetition count: typing just `j` will
move one sequence down, but typing `12j` will move twelve sequences down.

If no prefix argument is given, commands default to **1**.

##  String Arguments

String arguments are entered after typing the command character, and are entered
by typing `Return`/`Enter`. Currently only the [header search](#hdr-search) command (`"`)
and [sequence search](#seq-search) command (`/`) take a string argument.

#  Navigation fundamentals

Navigation in Termal is inspired by [Vim](https://vim.org)  but simplified and adapted to
multiple sequence alignments. The effect of motion commands depends on the [zoom
mode](#zooming), as follows:

* In zoomed-in mode, they change the portion of the alignment that is being
  displayed, much as a pager pages through a text file.
* In the zoomed-out modes, they move the zoom box.

Motion commands all take a prefix argument.

## Scrolling

Scrolling commands move the visible part of the alignment by one or more steps.
The prefix argument is interpreted as a repeat count.

### Single-step scrolling

The following keys move the viewport by one step. 

command    motion
--------   -------
`[count]h` scroll _count_ columns to the left
`[count]l` scroll _count_ columns to the right
`[count]j` scroll _count_ sequences down
`[count]k` scroll _count_ sequences up

### Scrolling by screenfuls

To move by a screenful, use uppercase motion commands:

command    motion
--------   -------
`[count]H` scroll _count_ screenfuls to the left
`[count]L` scroll _count_ screenfuls to the right
`[count]J` scroll _count_ screenfuls down
`[count]K` scroll _count_ screenfuls up

### Aliases

The arrow keys are aliases for the motion keys.

arrow    alias for
------   ----------
←        `h`
→        `l`
↓        `j`
↑        `k`
Shift-←  `H`
Shift-→  `L`
Shift-↓  `J`
Shift-↑  `K`

The Space key is an alias for `J` (move one screen down). This mimics the
behaviour of `less` and other Unix pagers.

### Examples

- `h`: move 1 column to the left
- `10l`: move 10 columns to the right
- `5K`: move 5 screens up
- `2<Spc>`: move two screens down

## Jumps

Jump commands move directly to a position, specified by the prefix argument. The
viewport moves so that the target sequence (respectively column) appears at the
top (respectively left) of the alignment pane.

**NOTE**: in vertical jumps, the sequence's number is counted from the _first
screen line_. This will be the same as the sequence's number in the alignment
file, unless the sequences have been [reordered](#ordering).

### Absolute jumps

In absolute jumps, the prefix argument denotes a specific sequence or column:

command    motion
--------   --------------
`[count]-` jump to sequence _count_ (in the current order)
`[count]=` jump to sequence _count_ (in original file order)
`[count]|` jump to column _count_

###  Relative jumps

In relative jumps, the prefix argument denotes a percentage of the alignment's
height or width, rounded to the nearest sequence/column.

command    motion
--------   --------------
`[count]%` jump to _count_ % of the alignment's height
`[count]#` jump to _column_ % of the alignment's width

### Jumps to search matches {#match-jumps}

Termal supports jumping to sequences whose headers match an arbitrary pattern
(see [Searching](#searching) for how to start the search). If any matches are
found, Termal will automatically jump to the first match. To jump to the next or
previous match, use `n` or `p`:

command    motion
--------   --------------
`[count]n` jump _count_ header matches forward
`[count]p` jump _count_ header matches backward
`<Return>` jump to the current match (which may be offscreen)

Next and previous match jumps wrap around, i.e. pressing `n` while on the last
match will move back to the first one. If no matches were found, match jump
commands have no effect.

### Examples

* `123|`: Jump to column 123 (if it exists).
* `200-`: Jump to the 200th sequence from the top of the display, which may differ from the original file order if sequences have been reordered.
* `50%`: Jump to the vertical midpoint of the alignment.
* `25#`: Jump to one quarter of the alignment width.
* `n`: jump to the next header match
* `3p`: jump three header matches backward

# Zooming

In Termal, *zooming* refers to changing how the alignment is displayed, rather
than scaling characters graphically (although this can be done by changing the
terminal emulator's font size). Zooming is only relevant if the whole alignment
does not fit on screen. It is possible for the alignment to fit on the screen
only in one of the two dimensions: in this case, the zooming only applies to the
other dimension.

In _zoomed-in_ mode, Termal displays as many _adjacent_ sequences and columns as
will fit on the screen. This shows a detailed view of a portion of the
alignment, but obscures its large-scale features. Motion commands change which
portion of the alignment is displayed, causing new sequences/columns to appear
into view and as many sequences/columns to disappear. Termal starts in
zoomed-in mode.

In _zoomed-out_ mode, Termal shows the first and last sequence/column as well as
a uniform sampling of sequences/columns between them. Enough sequences/columns
are sampled to fill the screen; the other sequences/columns are not shown. This
shows a "big picture" view of the alignment, at the expense of granularity.
Motion commands affect the part of the alignment that will be shown when moving
back to zoomed-in mode. This area is shown on screen as a rectangle known as the
_zoom box_.

A variant of the _zoomed-out_ mode works as above but preserves the alignment's
aspect ratio. It is called _zoomed-out-AR_.

To cycle forward through the zoom modes, press `z`; to cycle backward, press
`Z`.

# Searching {#searching}

##  Searching sequence headers {#hdr-search}

Termal supports searching within sequence headers using regular expressions.


To start a search:

1. Press `"` (double quote)
2. Enter a [regular expression](https://docs.rs/regex/latest/regex/#syntax)
3. Press `<Enter>` to confirm

To cancel a search, press `<Esc>`.

During a search, the modeline displays:

- the index of the current match,
- the total number of matches.

### Example:

```
"^Eco<Enter>
```

This jumps to the first sequence whose header starts with `Eco` (if any). The
search order is according to the current [ordering](#ordering).


##  Searching sequences {#seq-search}

Termal supports searching within sequences using regular expressions.

To start a search:

1. Press `/` 
2. Enter a [regular expression](https://docs.rs/regex/latest/regex/#syntax)
3. Press `<Enter>` to confirm

To cancel a search, press `<Esc>`.

During a search, the modeline displays:

- the index of the current match,
- the total number of matches.

**NOTE** The current implementation of sequence search does not treat gap
characters specially. So if you want e.g. to specify a possible gap between A
and T, you need to do so explicitly: `A-?T`. A smarter handling of gaps is
planned for a future release.

### Example:

```
/GAATTC<Enter>
```

This jumps to the first instance of `GAATTC`. The search order is according to
the current [ordering](#ordering), then left to right.

---

# Display Controls

##  Resizing the Left Pane

The left pane can be widened (perhaps to show more of the sequence headers) with
`>` and shrunk with `<`. Both accept a prefix argument, which is by how many
characters the pane is to be resized:

command    motion
--------   --------------
`[count]>` widen the left pane by _count_ characters
`[count]<` shrink the left pane by _count_ characters



## Ordering the Sequences {#ordering}

Initially, the sequences appear in the alignment pane in the same order as they
appear in the alignment file. However, the sequences may be ordered according to
the current [metric](#metrics), either ascending or descending. Note that the
"first" sequence (on the screen) is the _top_ sequence. As a result, when
sorting in ascending order, metric values increase from top to bottom.

The user may also supply a custom ordering, by passing option `-o` and
supplying the name of an ordering file as an argument to the option, e.g.:

```
termal -o my-order alignment.msa
```

An ordering file is simply the sequence headers in the desired order. For
example, passing the following ordering file would cause `AHMKMHDK_00298` to
appear first, then `CEFNMEKK_03699`, etc., regardless of the order they appear
in in the alignment file.

Note that the headers in the ordering file are expected to match those in the
alignment file.

```bash
AHMKMHDK_00298
CEFNMEKK_03699
IAGGDKJC_03995
FHLODNDD_02091
HJACHOPP_04370
```

### Changing the Ordering

To cycle forward through orderings, press (`o`). To cycle backward, press `O`.

The current ordering is displayed is the corner pane, as a symbol just below the
bar chart in the left pane:

symbol   meaning
-------  --------
`-`      file order
`↑`      current metric, ascending
`↓`      current metric, descending
`u`      user-supplied order (`-o`)

## Metrics {#metrics}

The left pane displays a bar chart of the current _metric_. This is a numeric
property of the sequences. Currently, there are two possible metrics:

a. Sequence length (not counting gaps)
b. Similarity to the consensus

To cycle forward through the metrics, press `t` (me**t**ric); press `T` to cycle
backward. The current metric is displayed in the corner pane.

The sequences can be [ordered](#ordering) according to the current metric.

## Residue Colormaps

Termal supports four built-in residue color maps:

source     residue class    reference
---------- ---------------- ----------
ClustalX   amino acids      [@larkin2007clustal]
Lesk       amino acids      [@lesk2019bioinformatics]
JalView    nucleotides      [@waterhouse2009jalview]
monochrome both             [themes](#themes)

It is also possible to supply a custom color map:

```bash
termal -c colormap.json my-alignment.msa
```

where the colormap is a JSON file in the Gecos format
(@kunzmann2020gecos). This is a straightworward format that looks like this:

```json
{
    "name": "my-colormap",
    "alphabet": [
        "A",
        "C",
        "G",
        "T",
    ],
    "colors": {
        "A": "#71564e",
        "C": "#2a3d00",
        "G": "#004f7c",
        "T": "#b03f42",
        "Y": "#d5dfac"
    }
}
```
To cycle through colormaps, type `m` (forward) or `M` (backward).  The initial
colormap is ClustalX for amino acids or JalView for nucleotides. The current
colormap is displayed in the top border of the Termal screen.

## Themes {#themes}

Termal supports three themes: dark, light, and monochrome. To cycle through the
themes, type `s` (forward) or `S` (backward). The current
theme is displayed in the top border of the Termal screen.

**NOTE** Termal has been tested predominantly in a dark-themed
terminal.

## Inverse Video

By default, Termal displays the sequence residues in inverse video. To toggle to
direct video (and back), press `i`. The current
video mode is displayed in the top border of the Termal screen.

##  Modeline and feedback

The modeline (in the bottom border of the Termal screen) provides feedback about:
- pending numeric arguments,
- active search state,
- current search match index.

---

#  Limitations and scope

Termal currently does **not** support:

- editing alignments
- graphical export
- alignment formats other than Multi-fasta and Stockholm

# Future Work

Features that will be added in the next release:

* custom colormaps

Features that will be added later

* arbitrary sequence as reference
* showing only residues that differ from the consensus, or that are fully
  conserved (à la EMBOSS's `showalign -show`)
* a color map for vision-impaired users

Features under consideration

* Showing a phylogeny in the left pane
* Simple edits (removing empty columns)
* Saving parts of the alignments

---

#  Design philosophy

Termal prioritizes:

- predictability over feature breadth,
- keyboard-driven navigation over menus,
- robustness over visual effects.

It is intended for users comfortable with terminal-based tools who value speed
and clarity over graphical interaction.

Many commands, as well as the prefix argument syntax, were deliberately copied
from [Vim](https://vim.org).

# References
