# Hanji, vertical page stories, correctness, pass 3

**Reviewed**: ten-file working implementation/spec/package-record diff after note registration
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D3, row refitting must stay on the vertical continuation path

`crates/rdocx-layout/src/paginator.rs:873`

Both page-transition sites compare the old and new content width, but the
refit branch does not also check vertical body rotation. Horizontal cell splits
do not trim retained logical input, intentionally preserving their existing
path. If a horizontal transition changes width, rebreaking that untrimmed
input could replay already painted cell content. Require vertical body rotation
before replacing or refitting pending rows at either transition site.

## Smells

None.

## Nitpicks

None.

## Not found

D1 and D2 are resolved, with focused keep-next and three-measure note tests.
Source spans in split cells are now checked for contiguous starts, not only
text length. Small cacheable tables prove three width-aware cache entries are
reused, while large cell stories remain subject to the unchanged 2 MiB table
cache budget. Changed-story warm output equals fresh output. Package metadata
refreshes are derived from actual local archives, with member counts unchanged.
The Hanji regression fails against the original immutable renderer pin and
passes with the corrected local renderer.
