---
title: Termal
---


`pub` required for some module, but not its siblings
====================================================

I have the following source structure

```bash
src
├── alignment.rs
├── app.rs
├── main.rs
├── ui
│   ├── color_scheme.rs
│   ├── conservation.rs
│   └── render.rs
├── ui.rs
```

And, in ui.rs`:

```rust
mod conservation;
mod color_scheme;
pub mod render;
```

Notice the `pub mod render`: this module must be public, else compilation
fails. The other two, which are its siblings, do not need to be public. I'm not
sure why. Here is where the modules are used:

Function                             Used in     Relationship
------                               -------     ------------
`conservation::values_barchart()`    `render.rs` sibling
`color_scheme::color_scheme_lesk()`  `ui.rs`     parent
`render::render_ui()`                `main.rs`   "grandparent"?

All three functions are `pub`.

So:

* `render.rs` can `use crate::ui::conservation::...` without condition;
* `ui.rs` can `use crate::ui::color_scheme::...` without condition;
* but `main.rs` cannot `use crate::ui::render::...` (unless `render` is public,
  that is).

Here are [the
rules](https://doc.rust-lang.org/reference/visibility-and-privacy.html) for
visibility:

1. If an item is public, then it can be accessed externally from some module `m`
   if you can access all the item's ancestor modules from `m`. 
2. If an item is private, it may be accessed by the current module and its
   descendants.

Now this becomes a little clearer: `render_ui()` is called from `main.rs`, which
has to `use ui::render::render_ui()`. Being a direct descendant of the root,
`ui` is visible from `main.rs`, but its descendants, by default, are are not. If
`render` is not public, then rule 1 prevents access to `render` and its
contents. Furthermore, the root module (which
`main.rs` is in) is not a descendant of any module that has access to
`render_ui()` (in fact it isn't a descendant of any module at all), so the
conditions for rule 2 aren't met. Hence, `render` _must_ be public.

`color_scheme_lesk()`, by constrast, is used in `ui`, a direct parent of
`color_scheme`. The latter need therefore not be public for rule 1 to grant
access to `color_scheme_lesk()` (though the function itself must be).

As for `conservation::values_barchart()`, which is used in its _sibling_ --- not
ancestor --- `render`, then rule 2 says that it may be used because `render` is
a descendant of `ui`, which itself has access to the function through rule 1 (as
a parent, and because the function is public.


Immutable borrow invalid, but mutable borrow ok
===============================================

I find this error unclear:

```bash
error[E0596]: cannot borrow `*f` as mutable, as it is behind a `&` reference
   --> src/ui.rs:644:5
    |
644 |     f.render_widget(corner_para, corner_chunk);
    |     ^ `f` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    |
help: consider changing this to be a mutable reference
    |
635 | fn render_corner_pane(f: &mut Frame, corner_chunk: Rect) {
```

It I must not borrow `*f` as mutable, then why change it to `&mut`?

Unless... well, ok, this seems to make sense if it means that `render_widget()`
_needs_ to borrow `*f` as mutable (reasonable, since it's going to write to the
frame), but _can't_ because we're declared it as immutable.

"Hidden" Borrow Problem
=======================

The following code doesn't compile:

```rust
/* f: &Frame */
584     let seq = compute_sequence_pane_text(f, ui);
585     //debug!("showing {} sequences", sequences.len());
586     let aln_block = Block::default().title(title).borders(Borders::ALL);
587     let seq_para = Paragraph::new(seq)
588         .white()
589         .block(aln_block);
590     //f.render_widget(seq_para, layout_panes.sequence);
```

```
error[E0502]: cannot borrow `*f` as mutable because it is also borrowed as immutable
   --> src/ui.rs:590:5
    |
584 |     let seq = compute_sequence_pane_text(f, ui);
    |                                          - immutable borrow occurs here
...
590 |     f.render_widget(seq_para, layout_panes.sequence);
    |     ^^-------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |     | |
    |     | immutable borrow later used by call
    |     mutable borrow occurs here
```

So `f` is a `&Frame`, and `*f` is the `Frame` `f` references. Rustc complains
that `*f` cannot be borrowed as mutable because it is also borrowed as mutable,
no surprise here... except that it's far from obvious (to me, at least)  where
the other borrows of `*f` actually are. Ok, one is easy enough, and is helpfully
indicated by the compiler:

```rust
f.render_widget(seq_para, layout_panes.sequence);
^
|
mutable borrow occurs here
```

Looking at the [docs](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.render_widget), we see that the signature of `render_widget()` is

```rust
pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect)
```

Clearly, `f` is a mutable reference to the frame itself. This is not a problem
per se --- except, that is, if there is still _another_ reference to `*f`. And
this is exactly what Rustc complains about. But where this borrow may be is far
from clear to me. The message seems to imply that it is within one (or both) of
the parameters to `render_widget()`: let's try passing a dummy one that
absolutely doesn't reference `*f`:

```rust
f.render_widget(Paragraph::default(), layout_panes.sequence);
```

Now obviously this would display the sequence in a completely wring location,
but the point is to check if this _compiles_. Well:

```bash
$ cargo check
...
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
```

It does! So the problem is in `seq_para`. What could that be? 

```rust
    let seq_para = Paragraph::new(seq)
        .white()
        .block(aln_block);
```

Probably the dependence on `seq`. Indeed, changing to `Paragrapg::default()`
solves th eproblem (again, at the cost of a completely wrong display). So
somehow `seq` must contain a reference to `*f`. Here is the code:

```rust
fn compute_sequence_pane_text<'a>(f: &'a Frame<'a>, ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let mut sequences: Vec<Line>;

    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            sequences = zoom_in_seq_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            sequences = zoom_out_seq_text(f.size(), ui);
            if ui.show_zoombox { mark_zoombox(&mut sequences, f.size(), ui); }
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }

    sequences
}
```

First thing: I don't really need a ref to `f`, since I only ever use its size,
which is Copy. Even so, I do not see any reference to `*f` in the computation
of `sequences`. What happens if I suppress anny reference to `*f` when computing
`sequences`?

```rust
        ZoomLevel::ZoomedOut => {
            sequences = zoom_in_seq_text(ui);
            /*
            sequences = zoom_out_seq_text(f.size(), ui);
            if ui.show_zoombox { mark_zoombox(&mut sequences, f.size(), ui); }
            */
        }
```

Same error. In fact, what happens if I completely ignore `f` in the function;

```rust
fn compute_sequence_pane_text<'a>(f: &'a Frame<'a>, ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let mut sequences: Vec<Line>;

    /*
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            sequences = zoom_in_seq_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            sequences = zoom_in_seq_text(ui);
            /*
            sequences = zoom_out_seq_text(f.size(), ui);
            if ui.show_zoombox { mark_zoombox(&mut sequences, f.size(), ui); }
            */
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }
*/
```

Same error! So Rustc derives this reference solely from the function's
signature: as it were, since `compute_sequence_pane_text` is _passed_ a ref to
`*f`, its result (`sequences`) is "suspected" of itself referencing `*f`.

Soves this by not passing a ref to `*f`, which wasn't needed anyway. If it _had_
been needed, then there probably _would_ have been another ref to `*f`
somewhere.


Closure problem
---------------

At some point the code fails with the following message:

```rustc
error[E0502]: cannot borrow `app` as mutable because it is also borrowed as immutable
  --> src/main.rs:68:47
   |
39 |     let str_aln: Vec<&str> = app.alignment.sequences
   |                              ----------------------- immutable borrow occurs here
...
49 |             let line_aln: Vec<Line> = str_aln
   |                                       ------- immutable borrow later captured here by closure
...
68 |                         KeyCode::Char('k') => app.scroll_one_line_up(),
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

```

As I understand, this is because the closure passed to `terminal.draw(|frame|)`
borrows `app`, and that the `app.scroll_one_line_up()` also borrows it, and
mutably at that. In this case the closure directly draws the UI.

This was solved by adopting the same architecture as in the examples (except the
simplest ones), namely to have the closure passed to termal call a function
(`ui()`) that itself takes a ref to App:

```rust
terminal.draw(|f| ui(f, &app))?;
```

The ref does not live past the call to `ui()`, so there is no extra ref to `App`
by the time a mutable borrow occurs in `scroll_one_line_up()`.

Mutable ref Problem
-------------------

```
error[E0502]: cannot borrow `app` as mutable because it is also borrowed as immutable
  --> src/main.rs:44:23
   |
38 |     let srefs: Vec<&str> = app.alignment.sequences.iter().map(String::as_ref).c...
   |                            ----------------------- immutable borrow occurs here
...
44 |         terminal.draw(|f| ui(f, &mut app, &seq_lines))?;
   |                       ^^^            ---   --------- immutable borrow later captured here by closure
   |                       |              |
   |                       |              second borrow occurs due to use of `app` in closure
   |                       mutable borrow occurs here
```

This is because I try to borrow from `app` (actually, its sequences, which are
`String`s) both immutably and mutably, which is of course _verboten_. Now
there's many ways to address this:

* Only borrow immutably. This would mean that updates to the app would have to
  be done after the lifetime of the immutable borrow(s).
* Have the UI own a copy of the strings, so it does not need to borrow them.
  That's an additional `clone()`, but it would only have to be done once. Maybe
  some of the clonings (such as when constructiong the `Alignment` struct) could
  be bypassed.

Missing Lifetime Specifier, `<'static>` hint
--------------------------------------------

```
error[E0106]: missing lifetime specifier
  --> src/ui.rs:21:27
   |
21 | fn one_line(l: String) -> Line {
   |                           ^^^^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`, or if you will only have owned values
   |
21 | fn one_line(l: String) -> Line<'static> {
```

The `one_line()` function is very simple:

```bash
fn one_line(l: String) -> Line { 
    l.into()
}
```

The interesting thing is that the compiler doesn't care what's going on inside
the function: even if I comment out the only line, I get the same error.  This
means it can spot the error simply by looking at the types of the argument and
return value.

So let's look more closely at what a `Line` looks like (from [the Ratatui
doc](https://docs.rs/ratatui/latest/ratatui/text/struct.Line.html)):

```rust
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
    pub style: Style,
    pub alignment: Option<Alignment>,
}
```

All right: it contains (among other things) a vector of Spans which must have
the same lifetime as itself. What does `Span` look like?

```rust
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}
```

Ok, a `Cow` is a type of smart pointer that implements _clone-on-write_ (hence
the name, I imagine). 

Right. I confess I don't quite understand why the compiler suggests a _static_
lifetime. And I'm pretty sure a static lifetime _won't_ do, since I don't want
the alignment baked in the binary (this would limit the program to a single,
fixed alignment, which seems rather pointless). So what would happen if I
supplied a non-static lifetime?

```bash
fn one_line<'a>(l: String) -> Line<'a> { 
    l.into()
}
```

This seems to work - for now.

Avoiding `clone()`
------------------

I tried several ways of avoiding calls to `clone()`, because of the associated
resource consumption.  

### Pre-computing the `Paragraph`

I first tried to pre-compute a `Paragraph` object, to be re-used in every call
to `draw()`. But, as far as I understand, the graphical primitives (`Span`,
`Line`) etc. are _consumed_ in the draw. There are also stateful versions, and
reference versions, but I wasn't able to make them work.

### Putting t hewhole alignment in a `Buffer`

I also tried pre-computing a large `Buffer` with the whole alignment, from which
I intended to quicly copy the visible parts to the screen. This might well have
worked, except that the buffer's area is a `Rect`, whose dimensions are `u16`
and thus not large enough to hold the whole alignment when there are hundreds of
sequences and more than a thousand columns (_to be checked_).

So for now, `to_string()` is called on every visible character in the alignment.
In fact, as I understand, the terminal library, at lower level, will consume the
strings we pass it anyway, so it might well be that there is no avoiding these
clones.

Anyway, it is still quite fast enough, even in debug mode, and what's more some
of the sluggishness (if any) may be due to the _terminal emulator_ rather than
Termal itself - try e.g. a fast emulator like Alacritty versus a more standard
one like Xfce4-terminal.

Inlining Functions ?
--------------------

At some point I look up the colour of a residue using a HashMap (`ui.rs`, l. 95
(among others)). The previous incarnation involved lookups into large vectors,
which involved enough computations for the fan to start, except in release mode.

Unfortunately, it may be that I need to do more for a given residue than just
look up its colour: I may need to highlight it (selection) or to replace it with
a frame (viewport in zoomed-out mode). Ideally, I'd just call a function that
returns a suitably-styled Span for the residue, but calling a function (as
compared to a HashMap lookup) for each residue again looks expensive. Unless,
that is (or so I hope), if I can convince the compiler to inline the function.
If not, I'll have to inline it myself, which will lead to duplication and longer
loops, which I'd rather avoid. Also noteworthy is that the compmiler may
actually figure out on its own that inlining is a good idea in this case - that
may explain why the overhead seemingly only occurs in debug mode.

Screen Measures and Invariants
------------------------------

### Zoomed-in

```
+--------------------------- w_a ----------------------------+         ^
|                                                            |         |
|   +-------- w_s ----------+                                |         |
|   | +-------- w_p -------+|                                |< t       
|     |                    ||                                |         t
  h_s                      ||                                |          
h_a   h_p                  ||                                |         |
    |                      ||            +...... w_p ........|< t_max  v
|   | |                    ||            .                   |
|   | +------- seq panel --+|            .                   |
|   |                       |            h                   |
|   +--------------- scrn --+            .                   |
|                                        .                   |
+--------------------------------------------------- aln ----+
      ^                                  ^
      l                                  l_max
<-------------------- l ---------------->                                         
```

where

symbol   meaning
------   --------
_w~a~_   width (= number of columns) of the alignment
_h~a~_   height (= number of sequences (= lines)) of the alignment
_w~s~_   width of the screen
_h~s~_   height of the screen
_w~p~_   width of the sequence panel (not counting borders)
_h~p~_   height of the sequence panel (not counting borders)
_l_      leftmost alignment column visible in the sequence panel
_t_      topmost sequence visible in the sequence panel 
_l~max~_ maximum possible value for _l_
_t~max~_ maximum possible value for _t_

**NOTES**

* The sequence panel is the _main_, but (usually) not the _only_ visible part of the
  UI. It also has a border on each side. Therefore, the number of alignment
  columns shown on screen, _w~p~_, is always smaller than the screen width (by
  at least 2). This value is determined by the layout solver and can change,
  e.g. due to resizes of the label panel or the termnal itself. A similar
  situation obtains for height, naturally. 
* (_l_, _t_) is the top-left corner of the sequence panel, if we imagine it as
  positioned within the alignment (whose top left corner is at (0,0)).
* The value of _l_ cannot exceed the width of the alignment _w~a~_ minus the
  number of columns shown in the sequence panel (_w~p~_). Likewise for _t_

In symbols:

* $l_{\mathrm{max}} = w_a - w_p$ 
* $t_{\mathrm{max}} = h_a - h_p$ 
* $0 \leq{} l \leq{} l_{\mathrm{max}}$
* $0 \leq{} t \leq{} t_{\mathrm{max}}$

### Zoomed-Out

In zoomed-out mode, the sequence panel shows a uniform sample of the whole
alignment, that is, the first column in the panel is the first in the alignment,
and so is the last column. Every other column shown is sampled from the
alignment such as every _n_^th^ column is displayed. 

The ratio of columns displayed to total alignment columns is called the
_horizontal ratio_ _r~h~_. The _vertical ratio_ _r~v~_ is definied analogously.

A rectangle called the _zoom box_ is also displayed. This shows the portion of
the alignment that would be displayed in the sequence panel when zooming in
again. Note that the box's border _covers_ the top, bottom, left, and right
boundaries of the box. This means that the zoom box never fuses with the
sequence panel's border. 

The upper corner of the box (_l~b~_, _t~b~_) corresponds to _l_ and _t_ (see
above) multiplied by the appropriate ratio (see equations below); its width
and height derived from the width and height of the sequence panel, also
multiplied by the corresponding ratio. 

**NOTE**: in the following drawing, the alignment boundaries are not shown.

```
+----------------------------- w_s -------------------------------+
|   +--------------------------- w_p ----------------------------+|
|   |                                                            ||
|   |                                                            ||
|   |                                                            ||
|   |               +------ w_b ------+............................<- t_b
|                   |                 |                          ||
    h_p                               |                          ||
h_s                 h_b               |                          ||
    |                                 |                          ||
|   |               |                 |                          ||
|   |               +-----------------+                          ||
|   |               .                                            ||
|   |               .                                            ||
|   +---------------.------------------------------ seq panel ---+|
|                   .                                             |
+-------------------.------------------------------- screen ------+
                    ^
                    l_b
```

* $r_h = w_p / w_a = w_b / w_p$
* $r_v = h_p / h_a = h_b / h_p$
* $l_b = \mathrm{round}(r_h l)$
* $t_b = \mathrm{round}(r_v t)$

Measures of column-wise Conservation
------------------------------------

### SPS

### Relative entropy

For a given column: 

Miscellaneous Ideas
===================

* Make the labels panel part of a more general left pane, which could contain
  other "by-sequence" panes such as length and conservation (WRT consensus)
* Reinstate the "blinky" consensus, at least optionally

TODO
====

Urgent
------

1. When the horizontal scrollbar is visible, pushing the labels pane all the way
   to the right causes a panic.

Normal
------

1. [x] Disable right (resp. bottom) scrollbar for alignments that are short
   (resp. narrow) enough to fit. Test on `./data/{tall,wide}.msa`

1. [ ] Use `Option` for _all_ UI fields that cannot be initialized on
   construction, e.g. like `label_pane_width`. This applies e.g. to  

1. [.] Refactor long functions, especially `ui()` -> New module `render`.

1. [x] Add an option to disable scrollbars, so that pre-scrollbar tests don't
   fail.

1. [x] Try adding scrollbars. These should reflect the position of the zoom box WRT
   the whole alignment. They should NOT be shown when in zoomed-out mode, as the
   zoom box already carries the same information.
   * [x] vertical scrollbar +- ok.

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

1. [x] Show the bottom panel (for now, fixed-width).

1. [x] Allow `<` and `>`to set the size of the label pane

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

1. [x] Represent the view port in zoomed-out mode.

1. [ ] UI-related variables (top line, zoom ratio, etc.) should go in ...UI (not
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

1. [x] CLI args: now uses Clap; may specify fixed height and width.

1. [x] Enabble toggling between zoomed-in and zoomed-out, using key 'z'.

1. [x] See about computing a "summary" screen. This should toggle between
   summary and residue views.
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

1. [ ] Try putting the whole alignment into a Paragraph upfront, then scrolling
   it into position => Not sure if this is possible. The constructors for Span,
   Line, and Paragraph all seem to consume their arguments; I tried WidgetRefs
   and StatefulWidgetRefs, to no avail.

1. [x] Move ui code to a separate module.

1. [x] Provide shortcuts to begin, end, top, and bottom.

1. [x] Prevent scrolling down (right) if the bottom line (rightmost column) is
   visible.

1. [x] Prevent scrolling past the top or left margins (corrdinates become
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

