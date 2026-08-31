termal 2.1.0 – Stability fixes, weighted conservation, and navigation improvements

This release brings important stability fixes for sequence search, a new column
metric for conservation analysis, and better navigation when dealing with
scattered matches.

Consensus sequences are now deterministic: when residues tie for frequency, the
new computation uses IUPAC ambiguity codes (for nucleotides) or 'X' (for
proteins), ensuring consistent results across runs.

Sequence search is now more robust: empty-matching patterns no longer crash, and
malformed regex patterns display helpful error messages without being silently
overwritten.

The new weighted conservation metric factors in sequence simil reference when
computing column conservation, providing a more nuanced view of alignment
variation.

Highlights:

- Consensus computation is now deterministic (IUPAC codes for tied residues)
- Sequence search handles edge cases gracefully (empty matches, malformed
  patterns)
- Weighted conservation: new column metric (conservation weighted by coverage)
- Vertical match jumps (N/P): navigate matches without changing horizontal view
- Conserved region jumps documented and configurable (with `:set ...`)

No breaking changes.

See the changelog for full details.
