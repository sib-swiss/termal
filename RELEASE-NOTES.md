termal 1.4.0 – Sequence searches

This release adds regular expression search within sequences, and adds commands
for returning to the current match and jumping to a sequence by its original
position in the alignment file. It also adds a command for setting the reference
to any alignment sequence (instead of the consensus).

It also enables the user to select an alignment sequence as the reference (for t
hesimilarity metric). Up to now only the consensus could serve as reference.

The release archives now also include the manual, sample alignments, an
example ordering file, and curated custom colormap examples.

Highlights:

- Regular expression search within sequences
- Jump to the current search match
- Jump to a sequence by its original file position
- Scrollable help page for smaller terminals
- Release archives with manual and example data

No breaking changes.

Termal remains a viewer, not an editor (for now).

See the changelog for full details.
