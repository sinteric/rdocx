# Hanji issue 69, table alignment, pass 1

**Reviewed**: The uncommitted `crates/rdocx-layout/src/table.rs` contribution at
the beginning of this pass, 1 file with 74 insertions and 7 deletions. Also
reviewed the companion Hanji `crates/hanji-preview/tests/documents.rs`
regression, 1 file with 87 insertions.

**Contract**: Preserve direct table justification through style resolution.
Use the final laid-out width for center and trailing alignment. Preserve
leading alignment and signed table indentation. Do not change source XML,
table sizing, or export behavior. This is an external issue contribution and
has no assigned sprint F-ID or design plan.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, direct trailing justification moves an RTL table to the wrong side

`crates/rdocx-layout/src/table.rs:487`

Preserving direct justification now exposes the direction-independent branch
at `crates/rdocx-layout/src/table.rs:532`. An input with `bidi_visual = true`,
direct `jc = Right`, a 120 point grid, 360 points of available width, and no
indentation produces `table_indent = 240`. The flowed placement adds that
offset to the physical left margin at
`crates/rdocx-layout/src/paginator.rs:1039`.

ISO/IEC 29500 table justification reverses its interpretation for a table
using `bidiVisual`. This input therefore belongs at physical offset 0. Before
this contribution, dropping direct justification happened to produce that
correct position. The contribution moves it 240 points to the wrong side.
The existing bidi column reversal does not fix this because it only changes
cell placement inside the already positioned table.

Resolve justification and signed indentation from the table's leading edge,
then translate that result to physical page coordinates. Add a direct RTL
regression, including an inherited alignment case and a leading indent case.

Specification evidence: [Microsoft Open XML TableJustification, ISO/IEC
29500-1 remarks](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tablejustification?view=openxml-3.0.1),
the table alignment section. The direction-dependent interpretation and
leading-margin default are explicit. [TableIndentation
remarks](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tableindentation?view=openxml-3.0.1)
define indentation from the leading margin and suppress it for other
justifications.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness in the inspected left-to-right cases. Center and trailing
  alignment use the final column width sum. Leading alignment preserves
  positive and negative indentation.
- Contract drift in source XML or export. The mutation is confined to the
  cloned layout properties, and no parser or serializer changes are present.
- Style precedence errors. The base-first chain overlays direct properties
  last, and the new tests include both inheritance and direct overrides.
- New panic paths, unsafe indexing, schema ordering changes, namespace
  changes, or unmodeled XML loss in the production diff.
- Structural smells. The production fix introduces no new abstraction.
- An ineffective left-to-right regression. The layout assertions check exact
  width and offset. The Hanji companion checks the actual shaded cell bounds
  after SVG rendering, includes multiple page widths, and retains a rendered
  text assertion. Reverting direct alignment preservation fails its direct
  center and trailing cases.

## Verification boundary

This was a read-only review. No source or test changes, builds, test runs,
commits, pushes, or baseline updates were performed in this pass. The
orchestrator is running the toolchain and verification separately. Expected
output changes must include the final reviewed alignment semantics. Package
XML should remain unchanged. Hash-harness acceptance remains a separate gate
and was not inferred from static review.
