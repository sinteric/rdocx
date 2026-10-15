# F-X194, Complete Python table text and indentation APIs

**Status**: completed
**Sprint**: S91
**Size**: M
**Depends on**: none

The shared generation field names were reconciled with F-X206 before either
story was claimed. F-X194 still owns cell traversal and cell-only generation
validation. F-X206 owns the later shape-text and layout-inventory partition.

## Problem

PowerPoint `PyCell` exposes text and cell formatting but no `text_frame`.
TextFrame, Paragraph, Run and Font resolution is restricted to shape text
bodies in `crates/rpptx-py/src/text.rs:159`, `:172`, `:454`, `:894` and
`:1148`. Adding only a getter would produce a handle whose operations fail
or address the wrong body. Native cell text frames already exist in
`crates/rpptx/src/lib.rs:8558` and `:8643`. This closes Issue295.

Word authored-indent inspection exists at
`crates/rdocx/src/table.rs:2168`, but the Python table has no indent property.
The checked native setter at `:798` calls the nonnegative checker at `:398`.
A signed checker already exists at `:411`. This closes Issue300, including
negative values and removal, without replacing Word's placement policy.

## Spec reference

- `docs/hld/10-bindings-spec.md`, "The chosen design", "The invalidation
  problem, handled loudly", and "Python API shape", specifically the table
  and PowerPoint TextFrame, Paragraph and Font contracts.
- `docs/hld/05-drawingml-model.md`, "Text body" and "Tables".
- `docs/hld/08-rendering-spec.md`, "Tables", specifically Word alignment,
  margins, compatibility behavior and floating-table placement.
- `docs/hld/01-glossary.md`, units.
- `docs/hld/14-development-backlog.md`, "F-X194, Complete Python table text
  and indentation APIs".

## Approach

Extend existing types and files. Add `Cell.text_frame -> TextFrame` to the
PowerPoint binding. Capture its existing cell path including Row and Cell. Set the new optional
`ContentPath.text_revision` directly to the current cell-text generation.
Retrieving the handle validates ownership and both applicable generations
without creating a text body, mutating source or advancing a generation.

Add two concrete resolution functions in existing `text.rs`:

```rust
fn text_frame_ref_at<'a>(
    presentation: &'a rpptx::Presentation,
    path: &ContentPath,
) -> Option<rpptx::TextFrameRef<'a>>;
fn text_frame_mut_at<'a>(
    presentation: &'a mut rpptx::Presentation,
    path: &ContentPath,
) -> Option<rpptx::TextFrame<'a>>;
```

They use existing cell resolvers for a complete Row/Cell address, otherwise
existing shape resolvers. Route every TextFrame, Paragraph, paragraph/run
collection, Run, Font and Hyperlink text-body resolution through them.
An explicit grid-cell path remains that cell's address, including a merge
continuation. Do not silently redirect it to a merge origin.

Make borrowed native handles lifetime-safe without retaining references to
temporary cell wrappers. Name the existing native cell implementation
lifetimes and expose these accessors alongside the existing mutable borrow:

```rust
impl<'a> TableCellRef<'a> {
    pub fn text_frame(&self) -> Option<TextFrameRef<'a>>;
}
impl<'a> TableCellMut<'a> {
    pub fn into_text_frame(self) -> TextFrame<'a>;
}
```

The consuming accessor follows `ShapeMut::into_text_frame` at
`crates/rpptx/src/lib.rs:8086`. Reuse all current public text-frame operations:
multiple paragraphs, runs, alignment, line spacing, spacing before/after,
font size/name/bold/colour, vertical_anchor and word_wrap. Anchor and wrapping
already exist at `crates/rpptx-py/src/text.rs:381` and `:415`. The frame
anchor is bodyPr. Actual cell tcPr anchoring is separately owned by F-X206. Resolve hyperlink reads through the cell
body. Cell hyperlink assignment remains unsupported by the native relationship
setter at `crates/rpptx/src/lib.rs:3967`. Reject this assignment before creating
or changing any slide relationship. Do not add a native hyperlink API.

Extend the actual owner `PyPresentation` in existing `presentation.rs:152` with
`text_revisions: RevisionCounter`. Extend shared `ContentPath` in
`crates/oxml-py-support/src/lib.rs:29` with `text_revision: Option<u64>`.
Ordinary shape constructors and captures set it to None in this story.
F-X206 reuses these exact fields for existing shape-text consumers when it
implements the ancestor-handle partition. Do not add a second shape counter
or rename a field during that later implementation. The new cell-backed frame
sets Some directly, and paragraph, run, font and collection descendants inherit
that value rather than dropping it on capture. These concrete current consumers
justify one additional generation field. No per-cell map, generic revision
framework or forwarding builder is needed.

`validate_path` checks the existing global document revision first, then the
optional cell-text generation. Global structural changes retain the established
all-handle invalidation behavior. Cell text replacement and successful merge or
split advance only the cell-text generation, preserving existing Table, Row,
Column and Cell grid handles. Native merge rejects a same-cell rectangle and
split rejects an unmerged cell at `crates/rpptx/src/lib.rs:8860` and `:8911`.
Those failures advance neither generation and leave source unchanged.

For cell-backed TextFrame or Paragraph text replacement and paragraph/run
addition, advance the cell-text generation only after success. Return new
handles with the new generation where the established API does so. Shape-backed
operations retain their current global behavior. `try_replace_text` with no
match advances neither generation. Run text replacement preserves run topology
as it does today at `crates/rpptx-py/src/text.rs:904`. Font, paragraph formatting,
anchor and wrapping edits retain both generations. The presentation-wide
cell-text counter conservatively invalidates nested handles in other cells,
while their Cell and Table handles stay live. A cell-generation mismatch raises
StaleElementError with a cell-text topology explanation and a fresh-cell-frame
recovery hint. Existing recovery paths already have the right shape at
`crates/rpptx-py/src/lib.rs:71`.

Use the existing signed twip checker for Word table indentation. Add:

```rust
pub fn set_indent_value(&mut self, length: Option<Length>) -> Result<()>;
```

`set_indent_checked` delegates to this checked optional setter. Validate
signed-i32 twip bounds before creating properties. Conversion truncates
toward zero. None removes direct tblInd only, and clearing an absent value
inserts no tblPr. Positive, zero and negative values remain explicit.

Add Python getter/setter in existing `PyTable`:

```rust
fn indent(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>>;
fn set_indent(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()>;
```

The public setter accepts Length, its established integer EMU equivalent,
or None. Read through TableRef::indent and write through PyTable.edit.
Invalid input fails before source mutation. Existing left/center/right,
margin, compatibility and floating-table placement behavior remains the
only rendering policy.

## Rejected alternatives

- Separate cell paragraph/font wrappers duplicate existing public objects.
- A shape-backed frame path does not identify cell text ownership.
- Clamping negative indent violates Issue300's explicit contract.
- New table positioning rules duplicate existing layout behavior.

## Test plan

**Test gate**: regression, copied from the backlog.

| Category | Test | Asserts |
|---|---|---|
| regression | `test_table_cells_expose_the_same_editable_text_frame_surface_as_shapes` | Two paragraphs and runs, font size/name/bold/colour, alignment, spacing, anchor and wrapping survive save/reopen while the table remains editable. |
| regression | `test_cell_text_frame_edits_preserve_merge_origin_and_continuation_ownership` | Origin visible content, continuation addressing and unrelated cells preserve explicit grid-cell ownership. |
| regression | `test_cell_text_frame_structural_edits_stale_every_earlier_nested_handle` | Frame, collections, paragraph, run and font stale after successful cell-text topology changes, including other-cell nested handles. Table and Cell handles remain live. |
| regression | `test_cell_text_topology_failures_and_no_match_preserve_source_and_handles` | Same-cell merge, invalid merge, unmerged split and unmatched replacement advance neither generation. Failed mutations preserve package source. |
| regression | `test_cell_text_value_edits_retain_handles_and_global_structure_stales_them` | Run text, formatting, anchor and wrapping retain handles. Existing global structural changes still stale both ordinary and cell-backed text handles. |
| regression | `test_cell_hyperlink_assignment_rejects_before_relationship_mutation` | Unsupported assignment leaves text, slide relationships and package bytes unchanged, with no orphan relationship. |
| regression | `test_cell_text_frame_edits_reach_pdf_and_raster_output` | Actual deterministic PDF and PNG show multi-paragraph cell edits and formatting. |
| regression | `test_table_indent_accepts_negative_zero_positive_and_none` | Length inspection, signed values, truncation and direct indent removal survive save/reopen. |
| regression | `test_invalid_table_indent_leaves_source_and_handles_unchanged` | Wrong types and signed twip overflow fail atomically without advancing revision. |
| integration | `checked_table_indentation_accepts_signed_twips_and_removes_direct_indent` | Native checked signed authoring, no-op clearing and source round trip. |
| regression | `signed_table_indentation_keeps_existing_alignment_margin_and_float_placement` | Actual layout/PDF placement controls for existing alignment, margins, compatibility modes and floating-table precedence. |

Use existing `crates/rpptx-py/tests/test_documented_examples.py` for the
PowerPoint tests, `crates/rdocx-py/tests/test_formatting_tables.py` for Python
Word tests, and existing `crates/rdocx/tests/integration_test.rs` and
`regression_test.rs` for native evidence. No new integration binary. Include
grouped-shape table ownership and missing optional cell-body read controls.
Update both existing typing smoke files and stubs. Reproduce both missing
APIs against the claimed base before implementation.

## HLD impact

- `docs/hld/10-bindings-spec.md`: Python cell-backed TextFrame ownership,
  complete reused surface, optional cell-text generation and unchanged grid
  handle validity, unsupported cell hyperlink assignment, native consuming
  accessor and signed table indent authoring/removal. No HLD08 placement change is intended.

## Risk routing

- Unit conversion: read HLD01 units and CLAUDE's truncation contract. Check
  signed bounds and truncation toward zero. Do not round lengths.
- Public native API: the lifetime-safe consuming cell accessor and optional
  indent setter are additive pre-1.0 API changes. Run applicable
  `cargo publish --dry-run` and `.crate` size assertions for changed published
  facade packages. The change to checked negative-indent rejection is
  intentional and stated explicitly.
- PyO3: read HLD10. Run installed Python runtime, typing and stub checks.
  Run `cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm`.
  Workspace test runs exclude rdocx-py and rpptx-py.
- Rendering controls: read HLD08 and use deterministic fonts for every
  PDF/raster baseline. No baseline recording against system fonts.
- No new file, module, crate, trait, generic parameter or feature flag.

## Hash harness

Expected unchanged. New authoring entrypoints do not change existing fixture
inputs or layout rules. An unexplained delta blocks completion.

## Implementation checklist

- [x] Reproduce both missing APIs on the claimed base.
- [x] Add native lifetime-safe accessors and cell-aware text resolvers.
- [x] Route every nested text and font operation through those resolvers.
- [x] Add Python cell text_frame and preserve revisions/ownership.
- [x] Add signed optional checked Word table indent and Python property.
- [x] Update existing stubs, examples and typing smoke.
- [x] Run installed Python/native regressions and risk riders.
- [x] Complete scoped verify and a zero-finding microscope.

Exclusive implementation paths include shared `oxml-py-support/src/lib.rs`,
`rpptx-py/src/presentation.rs` and path validation in `rpptx-py/src/lib.rs`,
plus the existing native `rpptx/src/lib.rs`,
`rdocx/src/table.rs`, both families' binding table/text files, existing Python
stubs/tests and native Word integration/regression entrypoints. F-X197 must
follow this story for overlapping PowerPoint text/font paths.

## Open questions

None. Preserve existing explicit-grid-cell merge addressing and grid-handle
validity. Global revision invalidation remains strict. The additional cell-text
generation invalidates the new nested cell-text handles only. Unsupported cell
hyperlink assignment fails before relationship mutation.

## PR317 reuse assessment

Assess immutable head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0 before claim.
Reuse cell-aware resolver slices, borrowed native lifetime and consuming
accessor, signed checked indent and installed test scaffolding. Adapt the
contributor clear_indent into the approved checked optional setter, keeping
absent-property clearing a no-op. Do not adopt first-read txBody creation at
contributor rpptx-py/table.rs:646 or global text bumps at text.rs:284/589/651.
Use approved text_revision/text_revisions propagation and preserve grid
handles. Successful Cell.set_text/merge/split must stale nested handles.

The consuming native accessor eagerly creates missing txBody. For no-match,
count mismatch or error, preflight or stage the operation before publication.
Add missing-body byte and generation controls, including zero replacements,
not merely counts. The cell hyperlink guard precedes native relationship
creation and is reusable with the required no-orphan/unchanged-byte test.

Actual cell vertical placement uses a:tcPr anchor, distinct from TextFrame
bodyPr. F-X206 owns the contributor model/parser/writer/property/layout slice
and its rendering control. Do not claim bodyPr vertical_anchor alone meets
Issue309 cell anchor. Current F-X194 still exposes the reused frame surface.
