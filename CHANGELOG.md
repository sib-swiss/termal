# Changelog

## [1.5.0]

### Added

* Reference can now be set to an alignment sequence 
* Diff mode
* New column metric (coverage), and ability to change column metrics

### Changed

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
