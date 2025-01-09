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
