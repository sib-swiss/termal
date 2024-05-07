Termal
======

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

