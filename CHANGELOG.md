# Changelog

## [2.1.1] 2026-09-01

### Fixed

* Version mismatch: Cargo.toml versions now match GitHub release tag (v2.1.0
  binaries were built from code with v2.0.0 in Cargo.toml; now all versions
  consistent)

## [2.1.0] 2026-09-01

### Added

* New column metric: weighted conservation, based on the product of coverage and raw
  conservation.
* Jumps to conserved regions (`)` and `(`)
* Ex commands (only `:set` for now), including command history
* Vertical match jumps: `[count]N` and `[count]P` jump to the next/previous
  sequence search match without changing horizontal scroll position. 

### Fixed

* Sequence search no longer crashes on empty-matching patterns like `/z*` or `//`
* Malformed regex patterns (e.g. `/[`) now display an error message without being
  overwritten by a false "no current match" status
* Consensus sequence is now deterministic when computing tied residues: uses IUPAC
  ambiguity codes for nucleotides (R, Y, W, K, etc.) and 'X' for proteins, ensuring
  consistent results across runs

## [2.0.0] 2026-06-10

### Added

* Reference can now be set to any alignment sequence
* Diff mode: highlights residues that differ from the reference
* Prefix command for searches (`f` prefix; `/` and `"` still work)
* `--citation` flag: prints citation information and exits
* Alignment filename is no longer required when using `-b`/`--show-bindings`

### Changed

* Regex search within sequences now ignores gaps. Old behavior (literal gaps)
  now available through alignment search.
* Old column metric (function of coverage and entropy) now separated into two
  metrics, namely entropy and coverage (suitably normalized, etc.)
* Harmonized modeline messages displayed by prefix commands
* Sequence search ignores gaps by default
* Removed obscure options from short help
* Panel toggles now with prefix command

### Fixed

* When run without arguments, `termal` now outputs the short help instead of
  crashing with an error message

## [1.4.0]

### Added

* Regex search within sequences (no special treatment of gaps (yet))
* Jump to current search match
* Jump to sequence by position in original file
* Set any alignment sequence as reference for the similarity metric
* User manual (Markdown and PDF versions)

### Changed

* Help page allows scrolling if content doesn't fit on screen
* Release archives include new example alignments, an example ordering file, custom
  colormap examples, and the Markdown manual

### Fixed

* Jumps to matches now robust with respect to reorderings
* Option -V/--version now shows the binary name as 'termal', not 'termal-msa'

## [1.3.0]

### Added

* Vim-style count prefixes for motion commands and pane resizing
* Absolute and relative jump commands for horizontal and vertical navigation
* Regex-based search in sequence headers, with forward/backward navigation
* User-defined sequence ordering (`-o`)
* User-defined colormap (`-c`)

### Changed

* Modeline is now anchored to the bottom-left corner and displays:
* Pending command arguments (counts, search patterns)
* Current search match index (when applicable)

### Fixed

* No longer possible to crash by widening the left pane (`>`) all the way to the
  right.
* No longer possible to obscure the sequence metric and numbers by narrowing the
  left pane (`<`).

## [1.2.0] 

### Added

* Capacity to read Stockholm format
* Capacity to read files with sequences of different lengths

### Fixed

* Out-of-bounds error when zoombox is a single character.

## [1.1.0] 2025-05-20

### Fixed

* Color maps and color schemes

## [1.0.0] 2024-05-04

Initial release
