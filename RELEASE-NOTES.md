termal 2.0.0 – Diff mode, gap-aware search, and split column metrics

This major release introduces diff mode, which highlights residues that differ
from the reference sequence, making it easy to spot variation at a glance.
The reference can now be set to any sequence in the alignment (not just the
consensus), which combines naturally with diff mode.

Sequence search now ignores gaps by default: regular expressions match against
the ungapped sequence and match positions are mapped back to the alignment
coordinates. The previous literal-gap behavior is still available via the
alignment search command.

The single column metric has been replaced by two separate, independently
normalized metrics: entropy and coverage (density). This is a breaking change
for any scripts or workflows that relied on the old combined metric.

Highlights:

- Diff mode: shows residues that differ from the reference
- Any alignment sequence can serve as reference
- Gap-aware regex search within sequences (new default)
- Column metrics split into entropy and coverage
- Prefix command `f` for searches (`/` and `"` still work)
- `--citation` flag: prints citation information and exits
- Running without arguments now shows help instead of crashing

Breaking changes: column metric API changed; search-within-sequences behavior
changed (gaps now ignored by default).

Termal remains a viewer, not an editor (for now).

See the changelog for full details.
