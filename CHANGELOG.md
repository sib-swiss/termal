# Changelog

## [1.4.0]

### Added

* Regex search within sequences (no special treatment of gaps (yet))
* Jump to current search match
* Jump to sequence by position in original file
* Command for setting the reference to an arbitrary sequence
* Manual (Markdown and PDF versions)

### Changed

* Help page allows scrolling if content doesn't fit on screen
* Release archives include new example alignments, an example ordering file, custom
  colormap examples, and the Markdown manual

### Fixed

* Jumps to matches now robust with respect to reorderings

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
