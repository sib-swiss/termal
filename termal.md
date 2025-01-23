% Termal(1) Version 0.1 | TUI Multiple Sequence Alignment Viewer
% Thomas Junier

NAME
====

Termal - Multiple sequence alignment viewer with a text interface

SYNOPSIS
========

`termal [options] <MSA file>`

where `<MSA file>` is an alignment in multiple FastA format.

OPTIONS (SHORT)
===============

There are many options, but most are for debugging --- see the full OPTIONS section at
the end of this man page. You're most likely to use the ones listed below, but
most have equivalent key bindings (see KEY BINDINGS).

`-h, --help`

: Show the help message and exit successfully

`-i, --info`
:    Info mode (no TUI) - prints out statistics about the alignment.

`-C, --no-color`
:    Disable color

`--poll-wait-time <POLL_WAIT_TIME>`
:    Poll wait time [ms] [default: 100] Used for tweaking reactivity.

`-V, --version`
:    Print version


INTERFACE
=========

Termal has a purely textual interface, and is entirely keyboard-driven.

Display
-------

Termal uses the entire screen and divides it into three main areas, as follows (see
Figure 1 below):

* Top left: sequence numbers and headers, as well as a sequence metric (A).
* Top right: alignment (B - this is the main area)
* Bottom right: position and consensus (C).

```
┌───┌──────────┌──┌ data/aln5.pep - 18/226s (0.08) x 40/105┐
│  1│JPNFFBMG_0│█▊│------------MSTT------------------------█
│  2│FMNIGCAI_0│█▊│------------MSTT-----------------------T║
│  3│JHNJIINN_0│█▊│------------MENT-----------------------T║
│  4│BPCAMGGF_0│█▊│------------MSTT-----------------------A║
│  5│EINLENDL_0│█▊│------------META----------------------DN║
│  6│LEACOLDL_0│█▊│------------MSDT-----------------------N║
│  7│JIFGGMGC_0│█▊│------------MATT-----------------------D║
│  8│PGMANCIO_0│█▋│------------MTTSQ----------------------N║
│                 │------------                -----------N║
│       A         │------------        B       -----------V║
│                 │------------                ------------║
│ 12│NDKPGHOA_0│█▌│------------MVDDSL----------------------║
│ 13│FGDGKIFP_0│█▌│------------MNLKCKMKAFLGFLKEGFFVVD------║
│ 14│MODHFIIH_0│█▌│------------MTDET----------------------T║
│ 15│FIAOOHFG_0│█▋│------------MSTDQ-----------------------║
│ 16│LCKICBJP_0│█▋│------------MTTRS-----------------------║
│ 17│DCDHNMCP_0│█▎│----------------------------------------║
│ 18│KCHCLCAP_0│█▋│------------MANES-----------------------║
└───└──────────└──└🬹═══════════════════════════════════════┘
│                 │|    :    |    :    |    :    |    :    │
│Position         │0        10                  30        4│
│Consensus        │------------Mstt-     C     ------------│
│Conservation     │            █▁▁▁                       ▁│
└─────────────────└ Press '?' for help ────────────────────┘
```
**Figure 1**: Termal's display areas.


The alignment area is always visible; the other two can be hidden to make room
for it (see KEY BINDINGS).

Zooming
-------

By default, Termal shows as much of the alignment as fits on the screen. Smaller
alignments can fit entirely on screen, but it's quite common for alignments to
be too large, at least in one dimension, sometimes both. To see more of the
alignment, there are two options:

* Scrolling: this simply shifts the displayed portion ("view port") of the
  alignment left, right, up, or down. One can move by a single line (sequence)
  or column (position), by screenfuls, or directly to the top, bottom, leftmost,
  or rightmost positions (see KEY BINDINGS).

* Zooming Out: this shows the first and last sequences, as well as evenly-spaced
  sequences in between so as to show as many sequences as possible. The same
  sampling is applied to columns. A box shows the location of the view port,
  that is, what part of the alignment would fill the alignment area when zooming
  back in. The zoom box can be moved using the same commands as for scrolling
  (see above).


KEY BINDINGS
============

Scrolling
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

BUGS AND LIMITATIONS
====================

* Currently, Termal can only read Fasta alignments (i.e., no Phylip or other formats).

* A fast terminal is recommended (e.g., Alacritty, Ghostty, or WezTerm).
