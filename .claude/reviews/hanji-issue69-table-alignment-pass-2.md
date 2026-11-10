# Hanji issue 69, table alignment, pass 2

**Reviewed**: The remediated uncommitted contribution in
`crates/rdocx-layout/src/table.rs`, 1 file with 120 insertions and 10 deletions.
Also reviewed the companion Hanji
`crates/hanji-preview/tests/documents.rs` regression, 1 file with 95
insertions. Review records are excluded from these counts.

**Contract**: Preserve direct justification after base-first style resolution.
Resolve alignment and signed indentation against the table's leading margin,
then provide the paginator with an offset from the physical left margin.
Keep table sizing and package serialization unchanged. This includes the
RTL correction identified in pass 1.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found. Pass 1 D1 is resolved. The direction-aware conversion at
`crates/rdocx-layout/src/table.rs:541` subtracts the leading offset from the
remaining width for an RTL table. The reproduced 120 point table with 360
points available, direct trailing justification, and no indentation now
receives physical offset 0. Leading alignment with positive and negative
indentation is mirrored consistently, while centered placement is unchanged.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness. Reviewed left-to-right and right-to-left leading, centered,
  and trailing positions, signed indentation, narrow and full-width grids,
  and the existing width clamp. The zero-origin case no longer changes to
  the opposite margin in RTL.
- Contract. Direct table properties still overlay the style chain last.
  The cloned layout properties preserve authored justification. The direct
  width handling and column sizing paths are unchanged.
- Panics. The production change adds only finite width arithmetic and no
  indexing, unwrapping, recursion, or new allocation.
- OOXML. No parser, serializer, namespace, child-order, or retained XML
  operation changes. The companion source-built XML keeps `bidiVisual`
  before the width and justification elements.
- Tests. The layout tests cover 13 left-to-right cases, 8 style inheritance
  and override cases across both directions, and 10 explicit RTL cases.
  The Hanji companion checks 22 rendered cell bounds, including 7 RTL
  cases. The old suppression fails direct centered and trailing cases, and
  the pass 1 implementation fails the RTL trailing case. The SVG check
  measures absolute painted bounds, verifies a single shaded cell, and
  checks that cell text still renders.
- Structure. The fix stays in the existing table layout owner and adds no
  new trait, generic, wrapper, feature flag, or production module.

## Verification boundary

This review performed no source edits, builds, test runs, commits, pushes, or
baseline updates. Runtime verification remains with the orchestrator. The
expected rendering delta covers direct table justification and the corrected
RTL leading-margin interpretation. Source package XML is outside the changed
path. Hash-harness acceptance is a separate gate and has not been inferred
from static inspection.

## References

- [Microsoft Open XML TableJustification, ISO/IEC 29500-1
  remarks](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tablejustification?view=openxml-3.0.1)
- [Microsoft Open XML TableIndentation, ISO/IEC 29500-1
  remarks](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tableindentation?view=openxml-3.0.1)
- `docs/hld/08-rendering-spec.md:971`, base-first table style resolution and
  direct overlays.
- `docs/hld/12-testing-strategy.md:1003`, deterministic table geometry and
  painted cell bounds as a regression boundary.
