---
title: "Termal: Roadmap"
---

Important Points
================

* The speed of the terminal is paramount. I made (admittedly not very accurate)
  profiling attempts with Flamegraph, and it shows that in a "slow" terminal,
  most of the time is spent in terminal (backend) code, so optimizing Termal
  would have little effect. In a fast terminal (Alacritty, Ghostty, WezTerm,
  etc.), it's fast enough anyway ;) .

* I tried computing the Spans that constitute the alignment only once, to avoid
  incessant lookups in the colormap. Also, on a busy, distant machine, there is
  a perceptible difference between monochrome, Lesk, and Clustal maps, even
  though all of them have the same number of keys (but not values: clustal has
  the most, monochrome the least (namely, 1)); interestingly, clustal is the
  slowest and monochrome the fastest. Not sure why, but anyway these HashMap
  lookups are mostly redundant. => This is done one branch `prec-spans` (only
  for zoomed-in mode, and with a fixed colormap), but the improvement is
    negligible (although the code is simpler and I still think this is th eway
    to go). The problem, I as beginning to understand (or so I think), is the
    number of changes to the screen: In the zoomed-out mode, very little changes
    (only the zoombox frame moves), while in zoomed-in mode, any motion causes
    the whole aln area to be redrawn. The result is that zoomed-in mode is much
    more likely to be laggy, and the effect increases with the size of the aln
    area (just change font size to see this). I'm not sure if double-buffering
    helps here (Ratatui does it anyway). Rather, I'm beginning to think that I
    should avoid redrawing the screen so many times. The way to do this would
    seem to be to convert a string of identical single-step motion commands
    (hjkl) into one motion of equivalent length, e.g. l l l l would be converted
    to a single l (but four steps instead of one).

    All in all, after trying it out on the cluster with large and small
    alignments, in debug and release targets, the difference is minimal and
    appears swamped by the connection speed and/or the machine's load average.
    So I'm keeping branch `squev` in case we eventually decide to implement the
    mechanism, but for now I don't merge this into `master`.

Miscellaneous Ideas
===================

* Despite the nonnegligible effort I put into them, it's not clear that the
  guides are very useful, as the "Adjacent" mode for the bottom panel seems very
  natural. Their main advantage is to make it very clear that the coordinates
  apply to the zoom box (so this leaves the vertical guides in zoomed-in mode
  without a clear purpose).

* Reinstate the "blinky" consensus, at least optionally

TODO
====

Urgent
------

B0011 Accomodate light backgrounds as well as dark ones. Note that RataTUI lets
Crossterm decide what colors to use _unless specified_. Residues work reasonably
well (though white gap chars are a little hard to read), but anything explicitly
white will not work on a light background. Also, some colors (such as sequence
numbers and possibly the metric barplots) are a bit light for light backgrounds.
Let the user switch between dark and light themes; if possible detect the
terminal's teme and set accordingly. In the future, a rethink of the color
scheme would be in order.

Normal
------

1. [ ] B0013 'M' is supposed to cycle backwards through colormaps, but doesn't.

1. [ ] B0012 cycling through color maps can only be done in one sense (with
   `t`): all `T` to cycle in the opposite sense.

1. [x] B0010: "No sequence found" with `data/test[13].fas`. What these have in
   common is that they have only 1 entry. => Fixed - was using nth(1) instead of
   nth(0) - sometimes I forget that Rust, contrary to Julia, is 0-based.

1. [x] Added a proper nucleotide colormap (RGB values from JalView).

1. [x] Help dialog now sourced from an external file at _compile time_ using
   `include_str!()`. The same file is also shown when `termal` is passed option
   `-b`.

1. Scrolling responds to arrows (hjkl still work, of course).

1. [x] The top line of the corner pane now shows the current metric, as well as
   the ordering. It is colored like the metric barchart, and right-aligned (this
   means I had to add a Layout to that pane).

1. [x] Added sequence length (ungapped) as a metric. Pressing 't' cycles through
   the metrics.

1. [x] B0009: metric change does not trigger recomputation of ordering. To
   reproduce: `termal data/test-metrics.msa`, 'o' to order by increasing metric,
   then 't': the metric does (correctly) switch to sequence length, but the
   order is still according to %id. Note that we _could_ order by one metric and
   display another, but it's more complicated and not what we're trying to
   achieve now. => Fixed (af6bac5).

1. [x] Sequence numbers start at 1, but positions start at 0. Make up your mind,
   dude... => Sequence positions in the bottom panel now start at 1.

1. [x] 'B' now toggles the zoom box (Maybe not very useful; I used it  mainly to
   check the ordering of sequences in the zoomed modes).

1. [x] Ctrl-C quits.

1. [.] 'o' cycles through orderings, where the available orderings are "original"
   (i.e., same as in source file), "metric" (that is, by increasing order of the
   current metric), and by decreasing order of the metric. TODO: i) only
   implemented for zoomed-in mode, and ii) there currently is only one metric,
   namely %ID WRT consensus.

1. [x] Make "inverse video" the default mode.

1. [.] B0008 Alignment chokes on '.' in sequence. Fix that, maybe adding an option for
   the default gap character. => Fixed, but consensus keeps '-' even when
   alignment has '.'; the character for blanks should be determined from the
   alignment itself.

1. [x] Add a column to the left panel, for showing sequence metrics (such as
   length, or similarity to consensus).

1. [x] Add lowercase capability to Gecos maps.

1. [x] Can now read and use [Gecos](https://gecos.biotite-python.org/intro.html)
   colour maps (uppercase only!), supplied by option `-c/--color-map`.

1. [ ] Provide at least one more color map, for nucleotides. Ideally, the
   type of macromolecule should be detected automatically. The right way to do
   this, I think, would be to compute residue frequencies according to a protein
   vs. a nucleotide model (incuding IUPAC ambiguity codes, which can also be
   mistaken for amino acids). Then determine which model is most probable using
   Bayes factors. Such distributions are not as easy to find as I hoped (but see
   e.g. https://dergipark.org.tr/en/download/article-file/2904099 for amino
   acids), but they can be computed.

1. [x] Provide a key binding for cycling through the maps (and show the current
   map in the message); if eventually we end up with a large number of maps,
   then allow selection via a dialog. => 'm' cycles through color maps.

1. [x] Allow inverse video with black on a colored background.

1. [x] Experimental "Press '?' for help" in the last line of the bottom panel.
   This is now switched off when the help display has been shown at least once.

1. [ ] Add a keybinding to switch to monochrome and back (is this really
   useful?) This should be a separate color scheme, with monochrome as a color
   map.

1. [x] It should be possible to interactively hide and unhide the bottom and
   left panes. => Done this for the left panel (labels), which can now
   be toggled on/off by pressing 'a' ('l' and 'L' being already in use for
   motion), as well as for the bottom panel ('c', since 'b' is used to toggle
   its position). Also added 'f' ("fullscreen"), which toggles _both_ panels.

1. [x] Added a help dialog, accessible through '?' (since 'h' is already used).

1. [x] B0007: the consensus is shorter than the alignment in
   `CD00377_ICL-PEPM_wDesc.msa`. This is due to sevral cases of non-letter
   characters being pushed onto the consensus; the first one is at position 271,
   and has ASCII code 129 (note that lowercase 'z' is 122; in fact no letter
   should be above 127. IDK if this is in the original data or an artifact of my
   computation of the residue frequencies.  => Fixed. This was due to a kludgy
   conversion to lowercase: `br.residue as u8 + 97 - 65`. This works if the
   residue is _uppercase_, but if it is already lowercase, it doesn't return the
   same character - worse, it it will yield a value > 127, which will be
   interpreted as a non-ASCII character. At the time of writing, we still only
   considered uppercase residues in the alignments.

1. [x] B0006: crash. To reproduce: `cr data/CD00377_ICL-PEPM_wDesc.msa`; then G L =>
   Fixed - was caused by an 'X' in the alignment, which was not a key in any
   color map. Color maps now accept 'X' (and 'x'), and color them white. Also
   changed unwrap() to expect(), so we get a more informative error message.

1. [x] BUG B0005: The zoom box does not reach the bottom line, and is squashed by
   one line when reaching the top. 94fb5b (master). <- Was caused by an
   erroneous call to `floor()` instead of `round()`, introduced in `6a04f93cb4`.
   Fixed in `e4c948e`.

1. [x] Group all colours under a color scheme struct, and have `UI` contain one
   such member for all colors. This should include the residue->color map, but
   also the zoombox, label number, and conservation colours.

1. [x] Add colour to label numbers.

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
   want element 94, but at least that won't exceed the array's bounds). But if
   my array has 10 elements, then 0.95 * 10 will be rounded to 10 - out of
   bounds! So one should use `floor()` instead. OTOH, using `floor()` everywhere
   is wrong too: the bottom position of the zoom box needs `round()`.

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
     scroll better or worse (e.g. alacritty is one of the fastest).

1. [-] Try storing the whole alignment's characters (with the corresponding
   Colors) in a Buffer => Won't work:: the number of Cells in a Buffer is a u16,
   and therefore limited to 65,535.  Thus, storing all the alignment in a Buffer
   will not work for larger alignments.

1. [x] Try constructing Paragraph only from the parts of the sequences that have
   to be displayed --- this should avoid `clone()`s.

1. [-] Try putting the whole alignment into a Paragraph upfront, then scrolling
   it into position => Not sure if this is possible. The constructors for Span,
   Line, and Paragraph all seem to consume their arguments; I tried WidgetRefs
   and StatefulWidgetRefs, to no avail. UPDATE (2025-01-17): Span::raw() takes a
   Cow, so it _should_ be possible to pass it a &str.

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

