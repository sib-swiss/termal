---
geometry:
    - margin = 1in 
lang: en
colorlinks: true 
linkcolor: blue 
urlcolor: blue 
mainfont: TeX Gyre Pagella
monofont: JetBrains Mono
title: "termal — User Manual"
author: "Thomas Junier"
date: "January 2026"
toc: true
---

#  Overview

**termal** is a terminal-based viewer for multiple sequence alignments (MSAs).
It is designed for fast, keyboard-driven navigation of large alignments,
particularly in remote or SSH-based environments where graphical tools are
impractical.

Termal is a **read-only** viewer. It does not modify alignments. This may change
in future versions.

---

#  Basic usage

##  Starting termal

Simply pass your alignment file as an argument to `termal`, e.g.:

```
termal my-alignment.msa
```

Termal supports Fasta (default) and Stockholm (pass `-f`) formats.

##  Help

For general help, pass option `-h`; for a list of key bindings, pass `-b` or
press `?` after launching `termal`.

---

#  Screen layout

The interface is divided into four areas:

- **Alignment pane** (center - right): shows the aligned sequences, 
  some properties of the alignments, as well as some current UI settings.
- **Sequence labels pane** (left): shows sequence numbers and labels, as well as
  a barplot of the current metric (see below)
- **Consensus pane** (bottom): shows horizontal position, the consensus sequence, and a conservation barplot
- **Corner pane** (bottom left): shows the current metric and ordering (see
  below).

In addition, the last line contains a message area ("modeline"), which displays:

- pending command arguments (counts, searches), if any;
- search match information (when applicable).


---

#  Interaction

Termal is entirely keyboard-driven. All commands are issued by typing a single
character (usualy a letter).

##  Prefix arguments

Many commands (mostly motion) accept a _prefix argument_, which is an integer
number typed _before_ the command character. Typing any digit starts a prefix
argument. To cancel, press `Esc`. The meaning of the argument depends
on the command. For example, scrolling commands such as `j` ("scroll one line
down") interpret a prefix argument as a repetition count: typing just `j` will
move one sequence down, but typing `12j` will move twelve sequences down.

If no prefix argument is given, commands default to **1**.

##  String Arguments

String arguments are entered after typing the command character, and are entered
by typing `Return`/`Enter`. Currently only the Label Search command (`"`)
takes a string argument.

---

#  Navigation fundamentals

Navigation in termal is inspired by **Vim**, but simplified and adapted to
multiple sequence alignments. The effect of motion commands depends on the zoom
mode (see "Zooming" below), as follows:

* In zoomed-in mode, they change the portion of the alignment that is being
  displayed, much as a pager pages trough a text file.
* In the zoomed-out modes, they move the zoom box.

Motion commands all take a prefix argument.

## Scrolling

Scrolling commands move the visible part of the alignment by one or more steps.
The prefix argument is intepreted as a repeat count.

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

---

## Jumps

Jump commands move directly to a position, specified by the prefix argument. The
viewport moves so that the target sequence (respectively column) appears at the
top (respectively left) of the alignment pane.

**NOTE**: in vertical jumps, the sequence's number is counted from the _first
screen line_. This will be the same as the sequence's number in the alignment
file, unless the sequences have been reordered (see "Ordering", below).

### Absolute jumps

In absolute jumps, the prefix argument denotes a specific sequence or column:

command    motion
--------   --------------
`[count]-` jump to sequence _count_
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
(see [Searching](#Searching) for how to start the search). If any matches are
found, Termal will automatically jump to the first match. To jump to the next or
previous match, use `n` or `p`:

command    motion
--------   --------------
`[count]n` jump _count_ header matches forwards
`[count]p` jump _count_ header matches backwards

Match jumps wrap around, i.e. pressing `n` while on the last match will move
back to the first one. If no matches were found, match jump commands have no
effect.

### Examples

* `123|`: Jump to column 123 (if it exists).
* `200-`: Jump to the 200th sequence from the top of the display, which may
  differ from the original file order if sequences have been reordered.
* `50%`: Jump to the vertical midpoint of the alignment.
* `25#`: Jump to one quarter of the alignment width.
* `n`: jump to the next header match
* `3p`: jump three header matches backwards

---

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

To cycle forward through the zoom modes, press `z`; to cycle backwards, press
`Z`.

---

# Searching {#Searching}

##  Searching sequence headers

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

This jumps to the first sequence whose header starts with `Eco` (if any). 

---

#  Resizing the Left Pane

The left pane can be widened (perhaps to show more of the sequence headers) with
`>` and shrunk with `<`. Both accept a prefix argument, which is by how many
characters the pane is to be resized:

command    motion
--------   --------------
`[count]>` widen the left pane by _count_ characters
`[count]<` shrink the left pane by _count_ characters



---

#  Ordering the Sequences

Initially, the sequences appear in the alignment pane in the same order as they
appear in the alignment file. However, the sequences may be ordered according to
the current [metric](#metrics), either ascending or descending.

To cycle forward through orderings, press (`o`). To cycle backwards, press `O`.

---

# Residue Colormaps

Termal supports four built-in residue color maps:

source     residue class
---------- ----------------
ClustalX   amino acids
Lesk       amino acids
JalView    nucleotides
monochrome both

---

# Themes

---

#  Modeline and feedback

The modeline provides continuous feedback about:
- cursor position (row and column),
- pending numeric arguments,
- active search state,
- current search match index.

When a command is incomplete (for example, after typing a numeric prefix),
the modeline reflects this pending state explicitly.

---

#  Limitations and scope

termal intentionally does **not** support:
- editing alignments,
- modifying sequences,
- graphical export.

Its scope is limited to:
- inspection,
- navigation,
- orientation within large MSAs.

# Future Work


---

#  Design philosophy

termal prioritizes:
- predictability over feature breadth,
- keyboard-driven navigation over menus,
- robustness over visual effects.

It is intended for users comfortable with terminal-based tools who value speed
and clarity over graphical interaction.

Many commands, as well as the prefix argument syntax, were deliberately copied
from Vi/Vim.
