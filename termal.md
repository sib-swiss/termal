% Termal(1) Version 0.1 | TUI Multiple Sequence Alignment Viewer
% Thomas Junier

NAME
====

Termal - Multiple sequence alignment viewer with a text interface

SYNOPSIS
========

`termal [options] <MSA file>`

where `<MSA file>` is an alignment in multiple FastA format.

OPTIONS
=======

`-h, --help`

: Show the help message and exit successfully


`-i, --info`
:    Info mode (no TUI)

`-w, --width <WIDTH>`
:    Fixed terminal width (mostly used for testing/debugging)

`-t, --height <HEIGHT>`
:    Fixed terminal height ("tall" -- -h is already used)

`-L, --hide-labels-pane`
:    Start with labels pane hidden

`-B, --hide-bottom-pane`
:    Start with bottom pane hidden

`-D, --debug`
:    (Currently no effect)

`-C, --no-color`
:    Disable color

`--no-scrollbars`
:    Disable scrollbars (mostly for testing)

`--poll-wait-time <POLL_WAIT_TIME>`
:    Poll wait time [ms] [default: 100]

`--panic`
:    Panic (for testing)

`--no-zoom-box`
:    Do not show zoom box (zooming itself is not disabled)

`--no-zb-guides`
:    Do not show zoom box guides (only useful if zoom box not shown)

`-h, --help`
:    Print help

`-V, --version`
:    Print version

INTERFACE
=========

Termal has a purely textual interface, and is entirely keyboard-driven.

Display
-------

Termal uses the entire screen and divides into three main areas, as follows:

* Top left: sequence numbers and headers 
* Top right: alignment (this is the main area)
* Bottom right: position and consensus.

The alignment area is always visible; the other two can be hidden to make room
for it (see KEY BINDINGS).

Zooming
-------

By default, Termal shows as much of the alignment as fits on the screen. Smaller
alignments can fit entirely on screen, but it's quite common for alignments to
be too large, at least in one dimension, sometimes both. To see more of the
alignment, there are two options:

* Scrolling: this simply shifts the displayed portion of the alignment left,
  right, up, or down. One can move by a single line (sequence) or column
  (position), by screenfuls, or directly to the top, bottom, leftmost, or
  rightmost positions (see KEY BINDINGS).

* Zooming Out: this shows the first and last sequences, as well as evenly-spaced
  sequences in between so as to show as many sequences as possible. It does the
  same for columns. For example, if the alignment has 100 sequences but only 50
  will fit on the screen, then Termal will show every second sequence.


KEY BINDINGS
============

Motion
------

* h,j,k,l: move view port / zoom box left, down, up, right
* H,J,K,L: like h,j,k,l, but large motions
* ^,G,g,$: full left, bottom, top, full right

Zooming
-------

* z,Z    : cycle through zoom modes
* r      : highlight zoom box residues in consensus
* v      : show view guides

Pane Size
---------

* <,>    : widen/narrow label pane
* a      : hide/show label pane

Other
-----

* Q,q    : quit

BUGS AND LIMITATIONS
====================

* Currently, Termal can only read Fasta alignments (i.e., no Phylip or other
formats).
* There is only one color scheme, and it is not suited for nucleotides.
