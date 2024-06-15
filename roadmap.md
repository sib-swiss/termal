Termal
======

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


TODO
====

1. [x] Represent the view port in zommed-out mode.
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

