---
title: "Termal: Roadmap"
---

Miscellaneous Ideas
===================

* Despite the nonnegligible effort I put into them, it's not clear that the
  guides are very useful, as the "Adjacent" mode for the bottom panel seems very
  natural. Their main advantage is to make it very clear that the coordinates
  apply to the zoom box (so this leaves the vertical guides in zoomed-in mode
  without a clear purpose).

* Make the labels panel part of a more general left pane, which could contain
  other "by-sequence" panes such as length and conservation (WRT consensus)

* Reinstate the "blinky" consensus, at least optionally

* For "wide" alignments, let the empty lines below the sequences and above the
  bottom panel prolong the axis ticks, aiding the reading of the positions.

TODO
====

Urgent
------

Normal
------

1. [ ] Other styles (such as the scrollbars' and zoombox positions' and tick
   marks') should also be de-hard-coded, like the zoom box's (see below).

1. [x] The cyan zoombox is not very visible. Its style should be experimented
   with, and possibly settable by the user. Which means that it should not be
   hard-coded, but rather be an element of the UI struct. => UI now has a
   zoombox_style member. Setting it to bold has no effect, OTOH setting it to
   reverse video makes it stand out (perhaps too much, in fact).

1. [x] In many cases, using `round()` may cause an out-of-bounds error. Say I
   need the element at 0.95 times the length of an array. If my array has length
   100, that would be element 95 - no problem (well, except that I might really
   want element 94, but at least that won't exceed the array's bounds). But but
   if my array has 10 elements, then 0.95 * 10 will be rounded to 10 - out of
     bounds! So one should use `floor()` instead.

1. [x] The expression `let ratio = ui.h_ratio().min(ui.v_ratio());` (or its
   equivalent with `self` instead of `ui`) appears 6 times. This should be
   replaced with a function in `crate::ui.rs`, and this function should return
   the minimum _unless the whole alignment would still fit when using the
   maximum_ (see pt. below).

1. [x] BUG B0004. In AR mode, the (common) ratio used is currently the _minimum_
   of the horizontal and vertical ratios. This does ensure that the alignment
   fits on screen; however in some cases (such as with `wide.msa`, which is very
   much wider than it is tall), using the minimum results in unoccupied space on
   the right of the alignment. In this case, the max would work better. So the
   rul eshould be to use the max if the alignment still fits.

1. [x] (`ar_adjbp`) `draw_zoombox_guides()` should be called _outside_ of
   `compute_aln_pane_text()`, so that the latter function returns only the
   alignment --- the guides being a different object. In the current situation,
   we can no longer use the length of the vector returned by
   `compute_aln_pane_text()` to count the number of displayed sequences, which
   is unfortunate.

1. [x] "Adjacent" bottom panel now also works (well, almost) in AR mode
   (previously only in zoomed-in and (non-AR) zoomed-out). This required
   computing the number of lines that _would_ be displayed in "ScreenBottom"
   mode, and then using this to compute the layout for "Adjacent". IOW, two
   layouts need to be computed, the second of which is the one actually used on
   the disaply, and the first supplying the crucial parameter to the second.

1. [x] `zb_top` and friends get computed at several places, perhaps there should
   be a single function which could also handle checking for the zoom mode. This
   might bake it possible to dispense wit hdifferent `*_ar` versions of some
   functions. => done for `zb_top` and `zb_bottom` (ui::zoombox_top(),
   ui::zoombox_bottom()); still TODO for left and right.

1. The code in render.rs seems to hesitate between u16 and usize, try to sort
   this out.

1. [x] Try adding visual guides to the zoom box, provoding a, well, zoom effect.

1. [x] Experiment with (i) sticking the bottom directly below the alignment pane
   when there is unused space, and (ii) keeping the unused space, but providing
   visual guides. => Both work, and (i) is better IMHO, BUT it doesn't really
   work in AR mode, because the number of sequences shown will depend on the
   sequence panel's height, so making that height depend on the number of
   sequencs shown would introduce a circular dependency. In the end, all three
   modes should be available - let the user decide.

1. [x] Experiment with putting tick marks and position _first_ (on top), and
   consensus and conservation below. => Better, keeping this.

1. [x] When reading the consensus, I got fooled into thinking that there was an
   error because I mistook a '.' (low conservation) for a '-'. Changed the '.'
   to an asterisk (`be5cd667`).

1. [x] Refactor key-handling code in `main.rs` into a separate module. =>
   `src/ui/key_handling.rs`.

1. [x] Introduce an "info" mode (`-i`, `--info`) that doesn't launch the TUI but
   prints out stats about the alignment to stdout. => For now: name, #seq,
   #cols.

1. [x] BUG B0003 - panic: `cr -- -t 15 -w 80 --poll-wait-time 500
   ../data/aln5.pep`: press `zz` => fixed in the same way as B0001
   (`f2deb98b0dde`).

1. [x] BUG B0002 - panic: `cr -- -t 15 -w 80 --poll-wait-time 500
   ../data/aln5.pep`: press `zG` => fixed by keeping `zb_top` below
   `seq_para.len()` - see `7571603`.

1. [x] BUG B0001 - panic: `../target/debug/termal -t 15 -w 50 ../data/aln5.pep`;
   press `z`. Note: doesn't happen when the zoom box is disabled. => Occurres
   when the zoom box had zero height (`zb_top` == `zb_bottom`) or width (or
   both). Fixed in `d2e333af`.

1. [x] It should be able to switch the highlighting of retained columns on or
   off. Maybe 'H' could be used for this.

1. [x] Highlighting of retained columns (those in the zoombox) still doesn't
   work for "tall" alignments in AR mode - try with `tall.msa`: in zoomed-out
   mode, the box is as wide as the complete sequence (namely, 41 residues), and
   accordingly all 41 positions in the consensus are (correctly) highlighted; in
   AR mode, the box is about 1/4 the width of the alignment (11 residues) , but
   all consensus residues are still highlighted (there should only be 11 (give
   or take 1, depending on rounding). NOTE: this depends on screen size, of
   course. => Fixed, among others by refactoring the code that computes the
   retained indexes into a separate function (that does it right) and calling
   that function everywhere instead of re-computing said indxes every time (and
   getting it wrong at least once).

1. [x] In the zoomed-out modes, highlight which residues in the consensus are
   shown in the zoom box. => Tried several styles, the best IMHO is reverse
   video. This was also very useful in confirming that the sampling was wrong in
   AR mode - something I had suspected but couldn't quite convince myself of.

1. [x] Color the scrollbars and conservation metric

1. [x] Apply color scheme to consensus sequence

1. [ ] When the horizontal scrollbar is visible, pushing the labels pane all the
   way to the right (admittedly not a very frequent situation) causes a panic.

1. [x] Disable right (resp. bottom) scrollbar for alignments that are short
   (resp. narrow) enough to fit. Test on `./data/{tall,wide}.msa`

1. [x] Use `Option` for _all_ and _only_ UI fields that cannot be initialized on
   construction, e.g. like `frame_size` (which depends on the terminal size) but
   not `label_pane_width` (which can be given a default (currently 15)). This
   applies e.g. to  

1. [x] Refactor long functions, especially `ui()` -> New module `render`.

1. [x] Add an option to disable scrollbars, so that pre-scrollbar tests don't
   fail.

1. [x] Try adding scrollbars. These should reflect the position of the zoom box WRT
   the whole alignment. They should NOT be shown when in zoomed-out mode, as the
   zoom box already carries the same information.
   * [x] vertical scrollbar +- ok.
   * [x] horizontal scrollbar +- ok.

1. [x] Add labels for the bottom panel (Consensus, etc.) in the corner panel.

1. [x] One's complement of normalized entropy is better, but gap-rich columns
   tend to show as highly conserved (gaps do not count against entropy, but the
   low number of residues sometimes lower the entropy, yielding an illusion of
   high conservation). In this sense, they _are_ well conserved; OTOH they do
   not represent a strong conservation signal. Introduce a new metric: one's
   complement of normalized entropy, _weighted by column density_ (which is the
   ratio of number of residues to number of sequences, IOW a column wihout gaps
   has density 1.0).

1. [x] Entropy, as such, measures not conservation but rather divergence. Show
   the complement to 1 of entropy, so that highly-conserved columns are
   represented as tall bars.

1. [x] Convert the entropy (float) to an ASCII-graphics representation, using
   block characters like "▅", etc. This allows conservation to be plotted as a
   barchart. -> My first attempt involved a Ratatui Sparkline, but this is
   overkill: such block chars can be made into a String, itself into a Line, and
   everything in the bottom pane's Paragraph. The entropy is relative to the
   maximum entropy across the alignment's columns, so it should be called
   _normalized_ entropy.

1. [x] Add a function for computing  relative entropy. This should go in
   Alignment. -> well, for now it's just entropy (i.e., per-column, Shannon).
   This is because it's not obvious how to get base-rate frequencies (although
   estimating them from t healignment is an obvious candidate).

1. [x] Zooming causes a panic in "tall" and "wide" alignments (those in which
   the alignment is short enough to be displayed, but not all sequences fit on
   screen, or (respectively) those whose sequences can all be shown, though not
   in their entirety. -> Fixed by modifying `every_nth()` so that the number of
   indices returned never exceeds `l` (IOW, it can zoom out, but not in, as it
   were).

1. [x] Zooming causes a panic when the whole alignment fits on screen (which
   makes zooming kind of pointless anyway...). Zooming should not happen (no-op)
   in these situations.
   * add a function that determines if the alignment fits (vertically and
     horizontally)
   * call that function to decide whether or not to zoom.

1. [x] Add a consensus sequence to the bottom pane, just above the tick marks.
   For now, only zoomed-in, no colouring, no fancy speed optimization. -> Works,
   but the consensus is computed at every screen write, which is inefficient.
   The funny consequence is that positions at which several residues are tied
   for most frequent acually "blink" as the one that gets selected changes every
     time (see src/alignment::best_residue()). This is actually rather cool (if
     totally unexpected); a "blink" consensus can be done without recomputing
     the consensus all the time and it might be nice to have it -- although as
     an option, because blinking things may become annoying. Also, just as for
     coordinates and tick marks, no need to change the zoom box: the consensus
     acually summarises the contents of the box, which is again rather cool (and
     again, wasn't at all intended).

1. [x] Add alignment coordinates to the bottom pane, just below the tick marks.
   Only zoomed-in for now -> In fact, this works pretty well in zoomed-out mode:
   the tick marks and coordinates simply refer to the contents of the zoom box.

1. [x] Add tick marks to the bottom panel (every 10th residue); zoomed-in only
   for now. 

1. [x] Show the bottom panel (for now, fixed-height).

1. [x] Allow `<` and `>`to adjust the size of the label pane

1. [x] Make label pane work in zoomed-out mode.

1. [x] Make it possible to hide the labels pane, because some existing tests
   fail when it's shown.

1. [x] Add the left panel, for sequence labels. For now it can be fixed-width
   and always shown, and in Zoomed-in mode.

1. [x] Resizes to a larger screen causes a panic (-> fixed; see
   c5996bc7498eeac...; concomitant changes including h,v ratios accessible only
   trough functions -> recomputed every time -> always up to date; this WASN'T
   the original problem though, so restoring simple variables for h_ratio and
   v_ratio is not off the table).

1. [x] Represent the zoom box in zoomed-out mode.

1. [x] UI-related variables (top line, zoom ratio, etc.) should go in ...UI (not
   App). In fact, the "App" structure does not really seem to be useful, at
   least not as long as the alignment is read-only. We'll keep it because later
   on we may add functions that _change_ the alignment, and that should not be
   coupled to the UI. "App" is perhaps best thought of as "Model" (in the MVC
   sense).

1. [x] Add app-level tests. This means:
   * [x] Make it possible to fix the app's size (-> Viewport::Fixed)
   * [x] Find a way to automate TUI interaction (-> good old Expect)
   * [x] A test dir and a script to run Expect scripts in it (->
     `app_tests/run-tests.sh` - runs tests in parallel).

1. [x] Transform the Message panel into a Debug panel, which should be optional.
   Messages could be shown in the bottom pane (the one that will eventally show
   consensus, etc.) or (later) as (temporary) notifications in the center of the
   terminal, masking the rest.

1. [x] Provide a monochrome mode to simplify tests with Expect (otherwise I'll have
   to deal with ANSI colour sequences for _every_ residue)

1. [x] CLI args: now uses Clap; may specify fixed height and width (useful for
   testing with Expect)..

1. [x] Enable toggling between zoomed-in and zoomed-out, using key 'z'.

1. [x] See about computing a "summary" screen. This should toggle between
   summary and residue views. => Eventually, this was named the ZoomedOut mode
   (or "level")..
   * [x] implement a "every-nth" function that selects _n_ indices out of _l_ so as
     to spread them as evenly as possible. This will be used to select sequences
     and columns for the zoom-out.

1. [x] To avoid computing the Color of every visible residue at every keystroke,
   _store_ those colours in a Vec<Vec<Color>> beforehand.
   * [x] ... come to think of it, if we're going to look up the colour of
     all residues on screen, why not just use a residue -> Color map? That
     would be far easier than storing every single position's colour (for
     the whole alignment, no less!), and probably still be faster than
     calling a function every time.
     => Not visibly much faster, but at least the fan doesn't start whirring as
     soon as I start scrolling (in debug mode; release mode never causes fan
     whirring), as it did up to now (whether when getting the colour through a
     function or by precomputing them all). Different terminals also seem to
     scroll better or worse (e.g.  alacritty is one of the fastest).

1. [-] Try storing the whole alignment's characters (with the corresponding
   Colors) in a Buffer => Won't work:: the number of Cells in a Buffer is a u16,
   and therefore limited to 65,535.  Thus, storing all the alignment in a Buffer
   will not work for larger alignments.

1. [x] Try constructing Paragraph only from the parts of the sequences that have
   to be displayed --- this should avoid `clone()`s.

1. [-] Try putting the whole alignment into a Paragraph upfront, then scrolling
   it into position => Not sure if this is possible. The constructors for Span,
   Line, and Paragraph all seem to consume their arguments; I tried WidgetRefs
   and StatefulWidgetRefs, to no avail.

1. [x] Move ui code to a separate module.

1. [x] Provide shortcuts to begin, end, top, and bottom.

1. [x] Prevent scrolling down (right) if the bottom line (rightmost column) is
   visible.

1. [x] Prevent scrolling past the top or left margins (coordinates become
   negative, which causes a panic as they are usize).

1. [x] See if using a separate `ui()` function might solve the closure problem.
   (It does).

1. [x] Move the alignment (for now: only moves down...)

1. [x] Put the App in its own module.

1. [x] Try alignments that do not fit on the screen, and see how Ratatui handles
   them. Result: pretty well, in fact. I tried an alignment with sequences too
   long for a screen, and they are displayed by Paragraph without a glitch, and
   automatically handle screen resizing, which is pretty cool.

1. [x] Pass the name of the alignment file as positional argument.

3. [x] Read a Fasta File (see ~/projects/rasta) and display it in the Alignment
   box.

1. [x] Define a struct for alignments. It should have a list of headers and one
   of sequences, and there should be a constructor that takes a file path. This
   should be in a separate file.

2. [x] Display a rectangular array of chars, but using Ratatui widgets 

1. [x] Explore TUI libraries (tried a few, including Cursive and Ratatui - will
   try that last one for now)

1. [x] Display a rectangular array of characters at the top left corner of the screen.

