# F-X164, all aspects, pass 2

**Reviewed**: Final working diff against the F-X164 claim base, 19 files, 555
added lines and 87 removed lines.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: Authored preset shapes receive the theme style in both slide
  and group paths. Imported HTML and ODP shapes keep their source formatting.
- Contract: The typed style and inherited text colour match python-pptx 1.0.2.
  Empty SmartArt text colour lists use the quick style font colour.
- Panics: The changed paths add no unchecked indexing or untrusted input panic.
- OOXML: `p:style` follows `p:spPr` and precedes `p:txBody`. Untouched
  imported shape XML remains preserved.
- Tests: The added shape Rust gate would fail if the new style were reverted.
  The Python differential compares the parsed style tree. The pinned corpus,
  hash harness, archive measurements and repository policy checks pass.
- Structure: The unstyled helper serves both HTML and ODP import without a new
  trait, generic, wrapper type or crate.

## External viewer evidence

The source-built deck at `/private/tmp/fx164-shape.pptx` has SHA-256
`5eee41a924ea6bda393e5eb89cd5921403dc82f518c3946c0b174237cdb12e5d`.
PowerPoint was inspected directly through CUA on 2026-10-02. It opened as one
slide without a repair dialog and showed white `White on accent` text on a
blue gradient rectangle with an outline and shadow. The installed PowerPoint
bundle reports version 16.113.3, build 16.113.26092714. The pinned
LibreOffice 26.2.5.2 Docker oracle opened the same deck, produced a PDF, and
its raster showed an accent blue interior pixel at RGB (62, 127, 204). The
rpptx deterministic renderer asserted the gradient fill, 0.75 point line,
shadow and white text. No second-machine animation manifest was observed.
