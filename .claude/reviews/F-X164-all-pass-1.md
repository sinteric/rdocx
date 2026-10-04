# F-X164, all aspects, pass 1

**Reviewed**: Working diff against the F-X164 claim base, 18 files, 550 added
lines and 82 removed lines.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: The style is added to newly authored preset shapes on the
  slide and group paths. The imported HTML and ODP paths stay unstyled.
- Contract: The style, inherited font colour and SmartArt fallback match the
  approved design and the pinned python-pptx oracle.
- Panics: The changed parsing and shape creation paths do not add unchecked
  indexing or untrusted input panics.
- OOXML: The new style follows `p:spPr` and precedes `p:txBody`. Typed
  parsing retains the surrounding raw children.
- Tests: The added shape assertion fails without the new style. The Python
  comparison checks the parsed style tree, and the renderer checks the visible
  accent fill, line, shadow and light text. Existing imported shape tests
  assert the lack of an added theme style.
- Structure: The unstyled construction helper names an existing second
  consumer in HTML and ODP import. No new trait, generic or crate was added.

PowerPoint acceptance in `.claude/plans/F-X164-design.md:37` is still an
external gate. The generated deck opened and rendered in the pinned
LibreOffice oracle, but local PowerPoint automation timed out.
