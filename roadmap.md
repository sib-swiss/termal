Termal
======

Closure problem
---------------

At some point the code fails with the following message:

```
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
mutably at that.

TODO
====

1. [ ] See if using a separate `ui()` function might solve the closure problem.
1. [ ] Move the alignment (for now: only moves down...)
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
   try that last one for now)
1. [x] Explore TUI libraries (tried a few, including Cursive and Ratatui - will
1. [x] Display a rectangular array of characters at the top left corner of the screen.

