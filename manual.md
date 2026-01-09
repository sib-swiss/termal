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
subtitle: "Terminal-based MSA viewer"
author: "Thomas Junier"
date: "January 2026"
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

The simplest invocation is:

```
termal alignment.msa
```

Termal supports Fasta (default) and Stockholm (pass `-f`) formats.

##  Help

For general help, type `termal -h` in the shell. For a list of key bindings, type `termal -b` (shell)
or press `?` after launching `termal`.

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
multiple sequence alignments.

## Single-step scrolling

The following keys move the viewport by one unit:

command        motion
--------       -------
`[count]h` scroll one column to the left
`[count]l` scroll one column to the right
`[count]j` scroll one sequence down
`[count]k` scroll one sequence up

`[count]h` / ←   scroll one column to the left
`[count]l` / →   scroll one column to the right
`[count]j` / ↓   scroll one sequence down
`[count]k` / ↑   scroll one sequence up
## Scrolling by screenfuls

To move by a screenful, use the Shift key together with the above motion keys,
e.g. `K` moves one screen up.

All the scrolling commands accept a prefix argument, which is interpreted as a
repeat count.

Examples:

- `10l` : move 10 columns to the right
- `5J` : move 5 screens down

---

## Absolute jumps

Termal supports jumping directly to specific absolute positions. These commands
make most sense with a prefix argument

command    motion
--------   --------------
`[count]-` jump to sequence `count`, counting from the top
`[count]|` jump to column `count`

```
123|
```

This jumps to column 123 if it exists.

### Absolute sequence index

Use `-` to jump to a sequence by rank in the current ordering:

```
200-
```

This jumps to the 200th sequence from the top of the display, which may differ
from the original file order if sequences have been reordered.


##  Relative jumps

You can jump to positions relative to the total alignment size.

### Vertical position (percentage)

Use `%` to jump to a percentage of the alignment's height (number of sequences):

```
50%
```

This jumps to the vertical midpoint of the alignment.

### Horizontal position (percentage)

Use `#` to jump to a percentage of the alignment's width (number of columns):

```
25#
```

This jumps to one quarter of the alignment width.

---

# Zooming

In Termal, *zooming* refers to changing how the alignment is displayed, rather
than scaling characters graphically (although this can be done by changing the
terminal emulator's font size). This is only relevant if the whole alignment
does not fit on screen.

In _zoomed-in_ mode, Termal displays as many adjacent sequences and columns as
will fit on the screen. Lines beyond the boundaries are not displayed at all.
This shows a detailed view of a local portion of the alignment, but obscures its
large-scale features. Motion commands move the boundaries, causing new
sequences/columns to be displayed and as many sequences/columns to be no longer
shown.

In _zoomed-out_ mode, Termal shows the first and last sequence/column as well as
a uniform sampling of sequences/columns between them. Enough sequences/columns
are sampled to fill the screen; the other sequences/columns are not shown. This
shows a "big picture" view of the alignment, at the expense of granularity.
Motion commands affect the part of the alignment that will be shown when moving
back to zoomed-in mode. This area is shown on screen as a rectangle known as the
_zoom box_.

A variant of the _zoomed-out_ mode works as above but preserves the alignment's
aspect ratio.

To cycle forward through the zoom modes, press `z`; to cycle backwards, press
`Z`.

---

#  Searching

##  Searching sequence headers

Termal supports searching within **sequence labels** using regular expressions.

https://hypertext/test

To start a search:

1. Press `"` (double quote)
2. Enter a [regular expression](https://docs.rs/regex/latest/regex/#syntax)
3. Press `<Enter>` to confirm

To cancel a search, press `<Esc>`.

Example:

```
"^Eco<Enter>
```

This jumps to the first sequence whose header starts with `Eco`.

---

##  Navigating search results

After a successful search:
- `n` jumps to the next match
- `p` jumps to the previous match

The modeline displays:
- the index of the current match,
- the total number of matches.

If no matches exist, navigation commands have no effect.

---

#  Labels and ordering

##  Label pane

The label pane displays sequence identifiers and can be:
- shown or hidden,
- resized horizontally,

Resizing commands accept count prefixes (see 'Prefix arguments' above).

---

##  User-defined ordering

termal supports **user-defined sequence orderings** via the `-o` option.

This allows:
- grouping sequences logically,
- restoring a preferred order,
- overriding file order without modifying the alignment itself.

The exact ordering format depends on the option used; see `termal --help` for
details.

---

#  Colors and colormaps

##  Default colors

By default, termal uses a conservative color palette designed to:
- remain readable in typical terminal environments,
- work well over SSH,
- avoid dependence on full truecolor support.

---

##  Custom colormaps

Custom colormaps can be provided using the `-c` option.

This enables:
- custom residue coloring,
- emphasis of specific symbols or regions,
- compatibility with established coloring conventions.

Colormap files are plain-text and can be version-controlled.

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

For editing or annotation tasks, use a dedicated alignment editor.

---

#  Design philosophy

termal prioritizes:
- predictability over feature breadth,
- keyboard-driven navigation over menus,
- robustness over visual effects.

It is intended for users comfortable with terminal-based tools who value speed
and clarity over graphical interaction.
