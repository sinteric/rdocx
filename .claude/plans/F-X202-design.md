# F-X202, Complete Word tables pictures and internal links

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X194, F-X198, F-X206

The design is approved. F-X194, F-X198 and F-X206 are completed. The
F-X203 scoped integration has released the shared Word source boundary.
Implementation claims the current canonical delivery checkpoint. Historical
line citations below identify the original analysis and must be reconciled
against the actual claim Base. The accepted operation contracts are unchanged.

## Problem

Issue305 requests complete everyday tables, pictures and internal links.
Issue301 follow-through adds synchronized scoped text-box edits. Python exposes checked merge primitives
and clone_row but lacks the requested empty-row, nested-table and table
formatting surface. Native `Document::clone_table_row` deliberately copies
content at `crates/rdocx/src/document.rs:19793`. Native table columns and whole
merge-region split have no corresponding public operations.

`Document::set_picture_size` at `crates/rdocx/src/document.rs:20672` deliberately
resizes every main-document picture sharing the relationship. The Python test
`crates/rdocx-py/tests/test_core.py:877` explicitly pins this behavior. The issue
demands one physical picture. `Paragraph::add_hyperlink_with_tooltip` at
`crates/rdocx/src/paragraph.rs:1104` sets anchor=None. F-X198's approved scoped
refusal safely prevents divergence but does not fulfill Issue301's required
successful edit of both Choice and Fallback copies.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-X202 and complete Issue305 criteria.
- `docs/hld/10-bindings-spec.md`, Python API shape, The invalidation problem,
  handled loudly, and Native Word facade stability, specifically table
  topology, picture ownership, hyperlinks and checked scoped replacement.
- `docs/hld/08-rendering-spec.md`, Tables, Autofit, and The renderer's input,
  specifically Word table geometry and drawing placement.
- `docs/hld/04-opc-and-packaging.md`, The package, deterministic saves and
  invalid text refusal.
- `docs/hld/12-testing-strategy.md`, Test taxonomy, Binding tests and The Word
  render fidelity gate.
- `docs/hld/01-glossary.md`, Units, with existing truncating conversions.

## Approach

Use existing native and binding files. Inventory every advertised operation
against the actual claimed Base before tests or implementation. Reuse X194
signed removable indentation and X198 namespace-qualified physical-owner
discovery. Do not duplicate either prerequisite's implementation or intake.

### Tables

Proposed Python surface, final spelling subject to the open contract choices:

```python
Table.indent: int | None  # EMU, negative supported
Table.autofit: bool
Table.style: str | None
Table.first_row: bool
Table.last_row: bool
Table.first_column: bool
Table.last_column: bool
Table.banded_rows: bool
Table.banded_columns: bool
Table.add_row() -> Row
Table.insert_column(index: int, *, before: bool = True) -> None
Table.delete_column(index: int) -> None
Cell.add_table(rows: int, cols: int) -> Table
Cell.split() -> None
```

Native checked methods in existing document.rs take existing checked table or
cell physical location and logical grid index, rather than an unvalidated
caller-owned mutable cell detached from its table. New column operations and
split return Result and stage the document, validate topology, reopen and
publish once. Proposed shape:

```rust
pub fn insert_table_column(&mut self, table_index: usize,
    index: usize, before: bool) -> Result<()>;
pub fn delete_table_column(&mut self, table_index: usize,
    index: usize) -> Result<()>;
pub fn split_table_cell(&mut self, table_index: usize,
    row: usize, logical_column: usize) -> Result<bool>;
pub fn add_empty_table_row(&mut self, table_index: usize) -> Result<usize>;
```

Confirm nested-table indexing against existing document-wide table inventory.
Python resolves ContentPath and checks revision before translating location.
Every topology edit bumps revision and returns fresh Row/Table handles where
promised. Held rows/cells become stale after structural mutation.

Native table formatting already exists at `crates/rdocx/src/table.rs:966`,
`:971`, `:998`, `:1021` and `:1028`. Bind these operations, including inversions
for noHBand/noVBand, without creating a forwarding native abstraction. Style
assignment must resolve a real table style display name or explicit style ID,
check type and refuse ambiguous names. Current Python table.rs:384 simply
writes the supplied string as an ID. Do not turn an unknown name into a guessed
slug or use a paragraph-style name resolver without type validation.

Empty add_row preserves last-row row and cell properties, widths and logical
grid occupancy, but creates required empty paragraphs instead of copying
text, relationships, nested tables or document-wide identities. Decide the
vertical-merge continuation rule from a pinned Word/python-docx control.
`clone_table_row` remains content cloning. Native `remove_table_row` at
document.rs:19846 already restarts continuing merges and preserves boundaries.

Columns are logical tblGrid columns, not row cell-vector indices. Build one
bounded grid projection from current rows, gridSpan and vMerge. At insertion,
copy the selected neighbour's width and properties. Expand spans crossed by
the boundary or create a new independent cell according to the reviewed
native rule. At deletion, shrink intersecting spans, remove only a span that
loses its final column, and keep tblGrid plus cell widths consistent. Respect
gridBefore/gridAfter or explicitly refuse unsupported malformed topology before
publication. Refuse deletion of the final logical column.

Split operates on the complete selected horizontal and vertical merge region.
It removes gridSpan/vMerge and materializes the original logical grid cells.
Keep content once at the existing merge origin and preserve cell properties
using the reviewed rule. Do not duplicate comments, bookmarks, images, control
bindings or opaque XML. An unmerged cell is a defined no-op. Namespace aliases,
producer children, comment ownership and unrelated table boundaries survive.
Existing checked span setters at table.rs:1267 and Cell span setter:1876 are
not substitutes for this whole-region operation.

Nested Cell.add_table binds existing checked native Cell::add_table_checked at
table.rs:1975, with positive counts, bounded extents and mandatory terminal
paragraph policy. Do not call the infallible add_table at1930 with unchecked
Python input.

### Pictures

Expose the existing PictureOptions vocabulary at document.rs:504 and checked
authoring at17624, including description, title, decorative, crop, inline or
square/tight/top-and-bottom/behind/in-front placement, anchor relativity and
size. Reuse the native validators. Reject nonfinite positions and crop values,
invalid XML strings, conflicting inline/anchor inputs and wrong Python types
before mutable access. Through wrapping is not silently promised by this issue.

```python
Document.add_picture(..., options: PictureOptions | None = None) -> Picture
Run.add_picture(image: str | bytes, *, width: int, height: int,
    options: PictureOptions | None = None) -> Picture
Cell.add_picture(image: str | bytes, *, width: int, height: int,
    options: PictureOptions | None = None) -> Picture
Document.pictures() -> tuple[Picture, ...]
Document.set_picture_size(picture: Picture, width: int, height: int) -> None
```

Picture is a frozen snapshot of physical owner, size, description/title,
decorative state, content type and owned bytes, carrying checked document
identity, revision and qualified source identity for mutation. Inventory
specifies source order and story owner, distinguishes two drawings of one
relationship and admits supported picture owners without conflating charts,
SVG extension relationships, groups or alternate representations. Frozen
snapshots never become live by reinterpreting their stored relationship IDs.

Native Run::add_picture at run.rs:500 and Cell::add_picture at table.rs:1686
accept only existing relationship IDs. Package-owned import must allocate the
image relationship on the actual story part, stage qualified owner insertion
and publish atomically. Reuse checked add_picture_to_story/insert_picture_to_story
at document.rs:17472/17496. A stale run/cell refuses without adding media.

The proposed set_picture_size signature is an explicit pre-1.0 source and
behavior change. Rename the old relationship-wide native operation to an
explicit relationship-sized operation only if compatibility retention is
approved. Rewrite the existing multi-drawing test to pin both intentional
contracts, with reviewed release/API migration notes. Never silently map an
ambiguous relationship ID to the first drawing. Preserve relationship-aware
replace_image, which changes the shared media target intentionally.

### Links

```python
Paragraph.add_hyperlink(text: str, url: str, *, tooltip: str | None = None) -> Run
Paragraph.add_internal_hyperlink(text: str, bookmark: str,
    *, tooltip: str | None = None) -> Run
Document.add_heading_hyperlink(paragraph: Paragraph, heading: Paragraph,
    text: str, *, tooltip: str | None = None) -> Run
```

For add_hyperlink(..., '#name'), dispatch to the same internal-anchor operation
instead of writing an external relationship. Actual URL fragments remain
external URLs. Native authoring writes w:anchor and no external relationship.
Heading links create a unique valid bookmark start/end around the checked
heading only when none exists, reusing a suitable existing bookmark. Stage
source link, destination bookmark, IDs and relationships together. Refuse
missing targets, invalid names, stale handles and unsupported cross-owner
boundaries atomically. Preserve tooltip and formatting with native HyperlinkSpan
at paragraph.rs:1104, without text-only replacement or marker stripping.

### Synchronized scoped alternate text boxes

Extend X198 physical containment and qualified MC ownership into a complete
source-splice transaction. Identify every supported Choice/Fallback counterpart
of the selected semantic paragraph or descendant cell, not merely every box
sharing text. Validate branch topology, mapped owner and selection boundaries
before mutation. Logical found/expected count belongs to the selected view,
while each physical counterpart must independently match the mapped edit.
Count mismatch and zero-match semantics remain as in X198.

Stage edits against original qualified ranges and apply ranges in descending
offset order. Preserve each representation's run formatting, comments, fields,
namespace closure and opaque children. Publish only after every copy and
companion closure passes validation/reopen. Refuse unsupported asymmetric
topology or ambiguous owner mapping without changing any bytes. Do not remove
X198's guard and splice only Choice. Genuine public paired-copy success is
required by this story, alongside existing refusal and count controls.

## Rejected alternatives

- Empty add_row implemented as clone_row, which copies document content.
- Column indices interpreted as vector cell indices, which breaks gridSpan.
- split clearing one cell's flags without restoring the vertical merge region.
- Relationship ID treated as picture identity, which violates Issue305.
- '#' URLs written as external file relationships.
- Removing the scoped refusal before a complete paired transaction exists.
- A new module or generic owner wrapper, when existing document and ContentPath
  routes already identify the supported physical owners.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | test_issue305_table_surface_on_installed_base | Every missing requested Python method fails through fresh installed Base before implementation |
| regression | empty_row_and_nested_table_preserve_properties | Empty content, retained last-cell properties, nested table reopen and valid terminal paragraph |
| regression | table_look_style_and_layout_roundtrip | Six look flags, named style resolution, unknown/ambiguous/type mismatch, autofit/fixed and signed removable indent |
| regression | logical_column_edits_preserve_merged_grid | Before/after edges, horizontal and vertical spans, unequal widths, nested tables, final-column refusal, malformed topology rollback |
| regression | split_restores_entire_merge_region_once | Horizontal/vertical/rectangular merge, content once, owner IDs, unmerged no-op, opaque bytes |
| regression | picture_options_and_owned_run_cell_insertions | Every requested option/anchor, story-owned relationship, stale owner and invalid input atomicity |
| regression | resize_one_of_two_shared_media_pictures | Exactly one physical drawing changes, other extent/media unchanged, explicit legacy transition and stale snapshot refusal |
| regression | picture_inventory_preserves_physical_identity | Bytes/content type/size/alt, same relationship multiple owners, headers/cells/text boxes, deterministic order |
| regression | bookmark_and_heading_links_are_internal | w:anchor, no external relation, tooltips, unique heading bookmark, target collision and stale/invalid rollback |
| regression | scoped_paired_text_box_edits_publish_all_copies | Qualified Choice/Fallback success, multiple branches, descendant cell, unequal runs, no unselected changes |
| regression | paired_mapping_refuses_ambiguity_atomically | Asymmetric owners, opaque/foreign MC, aliases, count/no-op/stale rules, complete bytes and comments |
| round-trip | issue305_saved_documents_reopen_without_repair | Every item saved/reopened, native Word opening and render for geometry/links/pictures/merge topology |
| differential | issue305_word_and_python_docx_controls | Pinned native merge/split and picture rules, supported python-docx row/style/nested APIs |
| binding | issue305_typed_and_stale_handle_contract | Current wheel, strict mypy, stubtest, correct exceptions, fresh returned handles and held stale controls |

Reuse existing regression_test.rs, integration_test.rs only if the eventual
writer claim permits it, and current Python test_core/test_formatting_tables/
typing_smoke entrypoints. No new test binary. Existing references include
Python test_core.py:877, formatting-table tests:523/549/577, native integration
merge controls:8699 and row restart:11232, plus the integrated X194/X198 tests.
Pin Word and python-docx versions. Deterministic fonts are required for render
comparisons. Google Docs imports are best effort with documented direct-shading
and tight-wrap limitations, not inferred native acceptance.

## HLD impact

- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

Update Tables/Autofit and renderer input where accepted topology or drawing
behavior changes, precise native/Python API and invalidation contracts, the
complete verification matrix, and actual Issue305 scope. HLD07 is PowerPoint
inheritance and is not a Word table model document.

## Risk routing

- Parser/serializer, HLD04 The package and HLD06 Preservation strategy:
  namespace alias/foreign/rebound controls, schema-order assertions and exact
  raw producer subtree preservation through save/reopen.
- Units, HLD01 Units and CLAUDE deliberate truncation: negative indentation,
  lengths, image crop/anchor input bounds and unchanged conversion semantics.
- Layout, HLD08 Tables/Autofit: deterministic PDF/PNG and native geometry
  controls, intentional output changes declared before baseline updates.
- Public API, HLD10 and structural rules: explicit pre-1.0 resize migration,
  no new source file/module/trait, verified patched workspace publish dry-run
  with --allow-dirty while scoped and all archives below10MiB.
- PyO3, HLD10: fresh installed runtime and strict typing/stubtest, both WASM
  target checks, exclude binding Rust test binaries at workspace gate.
- External oracle: differential-testing skill, pinned versions and immutable
  source/native-saved/reopened receipts. Native conclusions require actual
  captures, not code-generated expected layouts.
- Archive measurement carriers: if required by exact source inventory gate,
  rerecord only actual affected tuples/README rows under the Cargo lease,
  preserving all inventory, provenance and tolerances. HLD15 Packaging applies.

## Hash harness

Additive methods should leave existing49 cases unchanged. A resize contract
transition, paired-copy success or topology/render behavior may affect cases
that explicitly exercise those operations. Enumerate actual affected entries
and independently review expected deltas before any baseline movement. Never
guess that existing snapshots or relationships are new per-picture identities.
Only the designated exclusive sprint story may write the harness baseline.

## Exclusive file claims and integration

Proposed native scope: existing document.rs, table.rs, paragraph.rs, run.rs and
only necessary existing oxml table/drawing serializers. Python scope: existing
document.rs/table.rs/paragraph.rs/run.rs, lib registration, _rdocx.pyi and
current test entrypoints. Confirm exact source scope after topology/oracle work.
Serialize document.rs and binding/stub overlaps with X200/X201/X203 and HLD
semantic integration. X204 consumes these APIs and owns general CLI commands.
Workers never write sprint totals. Keep X194/X198 integrated evidence as the
dependency prefix, and update this draft's citations at the actual claim Base.

## Implementation checklist

- [x] Integrate reviewed X194/X198 and settle explicit API transition decisions.
- [x] Pin native topology/row/picture controls and define unsupported admissions.
- [x] Freeze full API/stub/source inventory and prove genuine public Base failures.
- [x] Implement checked tables, picture identity/options, links and paired edits.
- [x] Prove all criteria through reopen, render and full atomic/opaque controls.
- [x] Run declared scoped, binding, WASM, package and hash riders.
- [x] Complete independent zero-defect, zero-smell review and declared HLD work.
- [x] Prepare validated worker handoff without sprint-ledger writes.

## Open questions

Resolved by the explicit occurrence-local Picture transition, logical-grid,
link and qualified paired-owner contracts below, with genuine native positive
pairing and empty-row/vMerge evidence now captured and authenticated.
F-X198's completed safety prefix supplies no-op/count/refusal order, and
F-X206 supplies bound origin/causal tokens once. Shared source/test files
are serialized against those providers. Installed positive synchronization,
source-built ambiguity and all namespace/late-failure controls remain required
before completion. This control does not imply arbitrary producer pairing.

## PR317 synchronization reuse assessment

Immutable document.rs head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0 offers
useful namespace-closed fragment edits, each physical replacement count
equal to the logical count at27195/27210, descending splice offsets at
27212/27215 and prepare/reopen before publication at27216/27229. Reuse
those mechanics with the existing F-X198 qualified owner router.

Do not adopt local txbxContent routing at8275/8292 or unqualified
AlternateContent/branch admission at8323/8335 and8419/8433. Opaque earlier
Choice content must not suppress a later qualified modeled owner. Paired
cell, nested topology and whole-box equality checks at27101/27130 and
8363/8370 currently precede expected-count/noop checks at27180/27191.
Preserve the existing proven count/noop/stale ordering before synchronized
preflight. Genuine success and unsupported atomic refusal both require
qualified multiple-choice/grouped/identical/foreign/opaque controls,
asymmetric box counts, nested cells and unselected-paragraph divergence.
Contributor claimed regression tests have not been independently verified
by this static source assessment. Do not claim local success from them.

## Resolved table and physical owner contracts


Freeze the existing clone operation as content cloning. Native document.rs:19793 copies properties, nested content, relationships and producer XML, freshens identities and omits comment anchors. Python clone_row currently bumps structural revision and returns a newly captured Row at rdocx-py/table.rs:597. It remains available with that contract.

Choose `Table.add_row() -> Row` as empty append, backed by checked native add_empty_table_row(table_index)->Result<usize>. Copy last direct row's row/cell formatting and horizontal logical occupancy, then replace each cell content with its required empty paragraph. Do not copy nested content, bookmarks/comments, bindings, drawing relationships or paragraph identities. Clear vMerge on all new cells so append does not extend an existing vertical merge or start a fresh multirow region inadvertently. The native bounded Insert Below control below establishes the ordinary vertical-merge reset rule. Omitted/wrapped row cases retain their separate tests. A last row with gridBefore/gridAfter keeps those omissions and gridSpan occupancy. A table without a last direct row refuses until a separate initial-row contract exists. Wrapped row/cell control topology cannot be cloned as formatting: refuse before publication if it cannot be projected without copying control ownership. Do not turn add_row into clone_row plus ad hoc text clearing.

Keep proposed Python insert_column(index:int,*,before:bool=True)->None and delete_column(index:int)->None. Index identifies an existing logical tblGrid column, in0..column_count. Insertion boundary is index or index+1. No negative indexing for mutation, no guessed clamping, and invalid inputs fail before native mutable access. Native document methods use document-wide recursive table ordinal, as documented for clone_table_row at document.rs:19788 and implemented by table/table_mut at19769/19778. The current Python ContentPath top segment is not sufficient for arbitrary nested table translation: table.rs:23 finds the first Body segment. Resolve the full existing ContentPath to the checked native physical table before choosing its recursive ordinal. Do not claim nested parity by feeding a body index to a tables-only ordinal.

Use existing row_cell_ranges and validate_table_topology, table.rs:584/636. They already validate positive widths, exact active-grid coverage, omissions and matching vertical continuation ranges. Build the edit's one local logical range projection from them, not an independently tolerant grid interpretation.

Insertion rules:

- Copy the referenced logical neighbour's grid width. Within a spanning cell's strict interior, increase its gridSpan and dxa cell-width hint by that grid width, retaining its content and vMerge. Apply the same logical boundary to every row, so a vertically merged range expands consistently.
- At a cell boundary, add an independent empty cell with properties of the referenced neighbour covering index in that row, required empty paragraph, gridSpan/vMerge/hMerge removed, and dxa width equal to the inserted grid width. When index is omitted in that row, extend the corresponding gridBefore/gridAfter omission rather than inventing a cell.
- Shift omitted-range boundaries consistently. Preserve percentage/auto/nil table or cell width policy rather than treating it as dxa. Update only existing dxa aggregate width hints using checked arithmetic, leaving absent hints absent. Refuse an inconsistent explicit-width case if it cannot retain the selected width policy without guessing.

Deletion rules:

- Shrink every range crossing the deleted logical column. A multi-column cell loses one gridSpan unit and the deleted dxa width while retaining all content once. A one-column cell intersecting deletion is removed with its complete content.
- Decrement gridBefore/gridAfter if the deleted column is omitted. Preserve remaining cell/property/raw order and adjust stored opaque boundary positions instead of deleting unrelated raw siblings.
- A vertical merge shrinks by the same logical column in each member. Removing its final logical column removes every member of that region. Run existing comment/marker removal reconciliation on the complete candidate, document.rs:19888 and comments.rs:3234, so deleted anchors do not leave an invalid package. Refuse ownership that cannot be reconciled rather than leaving dangling IDs.
- Refuse final-column deletion, zero-width/overflow, orphan continuation and unsupported legacy hMerge/control-wrapped topology before publication. Support gridBefore/gridAfter when the existing positive-width exact-coverage projection succeeds, rather than refusing all omission metadata.

Choose `Cell.split() -> None`, native split_table_cell(table_index,row,logical_column)->Result<bool>, where bool means a structural change. Resolve a selected continuation to its restart, then discover the complete contiguous rectangle defined by exact logical range and matching vMerge chain. Materialize one physical cell for every original tblGrid column in every region row. Original merge-origin content stays once at that origin. Clear gridSpan/vMerge on every materialized cell. Keep each existing row member's content at its leftmost resulting cell if it owns distinct retained content, rather than silently deleting a continuation's producer content. If distinct continuation content or raw ownership makes complete region reconstruction ambiguous, refuse atomically. Newly materialized cells copy only cell formatting and take the corresponding grid width, with an empty paragraph. Preserve existing member tcPr producer XML in place and copy formatting into new cells only when it is free of unmodelled document-wide identity/relationship ownership. Do not clone raw content into each new cell.

An unmerged cell is a no-op: false native result, Python None, unchanged bytes/revision/handles. A horizontal-only, vertical-only or rectangular region is one transaction. Validate pre/post topology, reconcile removed markers if any, prepare/reopen and publish once. Valid topology is not proof that omitted producer content was retained, so assert complete unrelated part equality as well.

Structural table edits bump Python document revision once only after native success. This invalidates all prior rows/cells/tables, including the receiver. add_row and Cell.add_table return fresh handles captured after the bump. Column methods and split return None, so callers reacquire document.tables to continue. Failures and split no-op do not bump. Native Cell.add_table_checked at table.rs:1975 remains the bounded route with a mandatory trailing paragraph, rather than the unchecked add_table at1930.

Native Word/python-docx controls are still mandatory verification for the chosen empty-row/merge/width rules. These decisions define a proposed concrete algorithm and admission boundary, not a substituted oracle pass. Any disagreement earns plan correction before implementation, not an unexplained output delta.

## Picture occurrence identity and compatibility

Preserve existing native Document::set_picture_size(rel_id,width,height)->Result<usize> unchanged as the relationship-wide main-document operation, document.rs:20672. Existing Python test_core.py:877 pins multiple matching drawings. Do not reinterpret that native relationship parameter or silently take its first occurrence.

Add native `set_picture_size_at(&mut self, location:&ContentLocation, occurrence:usize, width:Length, height:Length)->Result<()>`. Location is a checked paragraph or supported direct content owner, and occurrence is the source ordinal among qualified direct ungrouped DrawingML picture carriers within that selected owner, not relationship ID, docPr ID, raw byte offset or global picture ordinal. Keep the existing native relationship-wide method available and explicitly document the difference. Python `Document.set_picture_size(picture:Picture,width:int,height:int)->None` uses the occurrence route, and `set_picture_sizes_for_relationship(relationship_id:str,width:int,height:int)->int` preserves the legacy Python behavior under an explicit name. The change to the old Python target/signature requires the approved pre-1.0 migration note and both tests, not deletion of the previous regression.

Picture is an immutable Python snapshot retaining its originating PyDocument object, revision, checked ContentLocation, within-owner qualified occurrence, occurrence source fingerprint and metadata/owned media bytes. Reject foreign originating document, stale revision or failed native physical correspondence before any write. No caller can fabricate a token from an arbitrary relationship string. Native callers explicitly supply the checked location and occurrence, following existing StoryId fingerprint semantics at document.rs:930 rather than introducing a new global document UUID system.

Inventory emits supported story owners in native stories order and each qualifying occurrence in physical source order. Scan physical parts once, then assign the most specific containing supported owner so body, TableCell and TextBox views cannot duplicate one occurrence. Distinguish two carriers with the same embed relationship. Relationship lookup is on the actual owning part. The snapshot carries content type, bytes, extent, description/title/decorative and anchor metadata. If a required owner or media relationship is malformed, return a checked error rather than silently returning a partial inventory. No charts, group-child drawings, foreign lookalikes or SVG extension blip are reinterpreted as ordinary picture identities.

For resize, qualify the selected carrier using the existing resize_story_pictures paths at document.rs:10073, update only its wp:extent and pic:spPr/a:xfrm/a:ext, and scale effectExtent with the existing checked arithmetic. Preserve anchor placement, crop, unrelated drawing children, media, relationships and every other occurrence. Re-resolve the actual source before applying source-range edits. For qualified alternate picture carriers with a VML or unproven additional Choice representation, retain atomic refusal until an independently reviewed complete representation-size rule exists. Keep inventory honest about supported editable carriers. This picture limitation is distinct from the text-box synchronized text transaction.

Owned run/cell insertion uses a checked original physical run boundary or cell paragraph owner, allocates media on that story part and appends into that owner, not a new unrelated body paragraph. Existing insert_picture_to_story at17496 creates an inline paragraph, so it is a reusable staging/media ingredient, not direct proof of Run.add_picture support. Native Run/Cell relationship-only helpers cannot provide checked package import by themselves. A successful insertion returns a fresh Picture after one structural revision bump. Failure adds no relationship/media and preserves old handles. A successful resize likewise bumps the Python revision once to make old extent snapshots stale, with no-op same-size handling explicit and no bump when unchanged.

## Bookmarks and internal links

Choose checked native owner operations in existing document/comments/paragraph files, operating on ContentLocation/StoryRunRange and staging one candidate. Python Paragraph.add_internal_hyperlink(text,bookmark,*,tooltip=None)->Run appends one hyperlink run at the paragraph's final supported accepted-run boundary. The link has anchor=bookmark, rel_id=None and tooltip preserved. Validate XML text and native bookmark grammar first, then require exactly one existing complete bookmark with that name in the same Document. Existing comments.rs:1889 defines authored name policy and comments.rs:657 inventories checked paired ranges. A bookmark target may be in a different supported owner of the same document because it is a name target, but the inserted source run and any newly authored target range must each be one exact supported owner.

Only a URL whose first character is # dispatches to internal authoring, stripping exactly that first character and validating the remainder. An ordinary URL containing a fragment remains external. Empty # is invalid. External hyperlink behavior otherwise remains native relationship-backed. Current paragraph.rs:1104 always authors rel_id and anchor=None, so add a direct anchor authoring branch, not an external relationship followed by mutation/cleanup.

`Document.add_heading_hyperlink(paragraph:Paragraph,heading:Paragraph,text:str,*,tooltip=None)->Run` validates both owner documents and revisions, requires the target to be a supported native heading paragraph, and appends at the source paragraph's final supported boundary. A target bookmark is suitable only when its paired checked range exactly encloses the heading's accepted run extent in that one owner. Reuse the first suitable name in native range source order. Do not reuse a mere same-name or partial bookmark elsewhere. When absent, allocate deterministic valid name Heading_<fresh_id> without a reserved leading underscore, bounded by the native40-byte rule, and anchor exactly the heading range using existing checked story-range machinery, comments.rs:415 and document.rs:16201. Never derive the only identity from mutable heading text.

An empty heading uses the native supported zero-width paragraph boundary only if the existing exact range authoring admits it. Otherwise refuse before publication, rather than insert dummy text or invent a range. Boundary crossing an inline control/revision edge follows existing exact native refusal, comments.rs:1036. Stage new bookmark ID and source hyperlink together and publish once, so a failed source insertion cannot leave a new orphan bookmark. A successful append is structural and returns a fresh source Run after one revision bump. A failure changes no bytes, IDs, relationships or revision. No new caller-owned marker edit API or detached XML handle is needed.

## Paired text-box preparation and ambiguity

Preserve X198 stale-target/physical correspondence validation, expected count mismatch and zero-match return before any synchronization topology refusal. For positive selected count, qualify the nearest containing modeled txbxContent and its MC AlternateContent using X198's structural namespace router, frozen worker document.rs:8309. Foreign namespace/local-name matches and descendants of opaque producer wrappers remain outside the mapping. PR317 local-name first-text-box admission is not reused, and global story-owner inventory does not change to facilitate scoped mutation.

Supported correspondence is within that one qualified AlternateContent, across its qualified Choice/Fallback children that contain modeled textbox representations. Branches without qualified modeled boxes are preserved but are not fabricated into copies. Prepare source-order vectors of top-level modeled boxes in each representation. Nested AlternateContent/textbox ownership or grouped opaque routes without a provable qualified counterpart refuse. Require equal box cardinality and a consistent one-to-one vector of normalized semantic box signatures at corresponding ordinals. A signature includes qualified direct block-kind topology, accepted paragraph text vectors, nested table logical topology and supported control owner boundaries, but excludes DrawingML/VML carrier spelling, run splitting and formatting. Do not compare complete raw XML, because genuine representations differ.

For a single box per branch, this proves the bounded owner slot. For multiple boxes, every corresponding signature must agree and the selected signature must be unique among sibling boxes in each branch. Identical sibling signatures are ambiguous and refuse, even when ordinal order looks plausible. Different signatures reordered across branches refuse rather than guessing a text-based permutation. Qualification, enclosing AlternateContent, ordinal topology and uniqueness are all required, so this is not an unconstrained search for another box sharing text. Do not use unrelated docPr/VML shape IDs as a shared key without actual source evidence that they are paired identities.

Within the proved box, map the selected direct paragraph or descendant TableCell using its qualified structural path, direct kind sequence and logical table-grid coordinates. Validate all boundary/count correspondence before splicing. Unequal runs/formatting are supported because each physical paragraph runs the existing block-local replacement independently. Unselected whole-box text divergence is an explicit asymmetric-refusal case under the first bounded contract, not silent unselected edits or a claimed success. A later relaxation needs stronger nontext correspondence evidence, not broader matching by placeholder.

Each physical mapped replacement must independently match exactly the selected logical count. Stage every source edit against original ranges, namespace-close only the required fragment context, retain each branch's formatting/opaque bytes and all comments/fields, then splice in descending offset order. Reconcile companions, prepare/reopen and publish once. Return the logical selected count, not copy count. Repeated identical paragraphs within one box remain distinguishable by checked structural slot, but repeated indistinguishable sibling boxes trigger the explicit ambiguity refusal. Preserve X198 guard for every unsupported route rather than deleting it globally.

Complete regression matrix includes ordinary single-box DrawingML/VML positive success, three qualified representations, two distinct unambiguous boxes positive success, identical sibling ambiguity, reversed distinct order refusal, unequal cardinality, unselected-text divergence, unequal runs/formatting, direct and descendant-cell selection, nested typed controls, count mismatch/no-op/stale target before topology checks, aliases and rebound foreign namespaces, opaque producer lookalikes, complete unchanged package on each refusal, global replacement unchanged, and logical count once. Word save/reopen oracle remains mandatory for genuine positive representation updates.

## Source and evidence receipt

No proposed algorithm has been executed. This appendix is incorporated into the draft contract. Approval and actual prerequisites remain required before implementation. Source citations use canonical working source, with X198 helper explicitly cited from its frozen worker. Immutable PR317 head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0 and source hash remain in .claude/scratch/S91-PR317-alternate-assessment.md. The conservative ambiguity admission is a concrete proposed safety boundary, not a claim of complete arbitrary producer pairing.

Canonical read HEAD: `95278005c482420ad927c8c1b18fe2482c14461b`.

- `crates/rdocx/src/document.rs` SHA256 `3b8eec9fc3980a9dc743be2faef39d1126c819be102a61b58a4f6ae798a59620`.
- `crates/rdocx/src/table.rs` SHA256 `8e1d2d6f456643ccee1883b2ffb4b93433735226984cbe1787829d4cf4e1a690`.
- `crates/rdocx/src/comments.rs` SHA256 `eda9a14535a0771475830cf9faa84a4df3f85b60877a4d9bdfdf5a9024e060c4`.
- `crates/rdocx/src/paragraph.rs` SHA256 `83198917fce911d18c1cdcf732c9f6e992feae279cf3092f1c43a48084a6da0c`.
- `crates/rdocx-py/src/table.rs` SHA256 `64d2c310a011732d26c472b6693c8a06d287756ea7548c845b929034048e9301`.

## PR322 contribution intake

Immutable head6b5e2e352047357299d1f366f807f4661037430e closes Issue305.
Complete22-file Git blob capture and272919-byte patch are retained under
.claude/scratch/S91-PR322-6b5e2e-* with per-source OID, bytes and SHA checks.
Reuse only accepted native grid, picture and link slices after full source
assessment. Nested NotImplemented paths, body-only picture enumeration,
heading bookmark publication before link validation and table-generation
exception history require reconciliation with this full contract and X211.
This PR does not implement the separately owned indentation or successful
paired-box edits. No contributed gate, archive tuple or baseline is adopted.

## Immutable contribution assessment decisions

Completed immutable PR322 assessment is retained in .claude/scratch/S91-PR322-6b5e2e-assessment.md. Adapt logical-grid validation and qualified picture scans under the existing full contract. PyPicture at contributor binding document.rs:741 carries index/revision without its document owner, while resize at:2531 checks only revision. Reject cross-document same-revision pictures before any write. Python heading bookmark publication at paragraph.rs:216 precedes source validation at:231, while native comments.rs:1077 reuses a bookmark merely starting at the heading. Use the approved atomic whole-heading and link transaction, with full heading scope and unique identity. Native table.rs:719/:1656 clear opaque row/cell properties, and some mutations bypass complete document staging. Preserve unrelated opaque formatting and stage all fallible topology operations. The unbounded TableEdit exception history is not an approved replacement for X211 owner identity and causal invalidation. Nested NotImplemented paths and body-only pictures remain full-scope gaps. No local gate is claimed.

## PR322 advanced head and authentic style setter control

PR322 now points to347091e6c9f2747c73de4b1b171c84203a9041ae. Its incremental
change allows undefined table-style strings with a warning. Retain this
story's strict missing-definition refusal instead. Actual python-docx1.2.0
public setters refuse unknown display names and IDs with KeyError and exact
package preservation. Defined display names succeed silently, defined IDs
warn about deprecated ID lookup, and warning-as-error refuses before edits.
The quickstart style exists in its bundled template. Private XML assignment
of an unknown ID is not public setter success. Duplicate-name first-match
oracle behavior remains a documented divergence from selected ambiguity
refusal. Keep current explicit-ID compatibility independently of the oracle's
warning. Complete controls/receipt are at
/private/tmp/S91-F-X202-style-policy-control. Incremental contribution receipt
SHA0350451b0e26b764d0adb4a6ec756478881bf62ed22e436023378dceced27b40.
No source, baseline, completed acceptance or new F-ID follows from intake.

## Authenticated native positive pairing and empty-row control

Actual Word16.113.2 build16.113.26092012 authored the uniqueX202_NATIVE_PAIR
textbox through native Draw Text Box, producing one qualified Choice/WPS
and one Fallback/VML owner with matching paragraph text. No synthetic
alternate was inserted. A native2x2 table with first-column vMerge then
Insert Below fromB2 retains A1/A2 and B1/B2, grid4428/4428, and creates
a third row of independent empty cells without vMerge.

Normal reopen/save preserves document XML and all relationships exactly.
One-page native Letter PDF text-bearing streams, extracted text and90dpi
pixels agree across reopen. Root verified23 immutable study artifacts plus
the unchanged original/template copy, independently parsed qualified native
branches/new-row facts and visually inspected the native page. Complete
study receiptSHA4b24d87e89936f97a5d9fb5ed06c17326479b6d566f75bb2ccca1592c1af73ef
resides in /private/tmp/S91-F-X202-native-pair-table.

The failed UI font selection is explicitly qualified in
font-input-qualification.md: saved table has no authenticated Arial12 run
properties, so no font-parity claim is made. Actual Poppler26.09.0 was used
for this study, not the older pinned26.01 corpus oracle. The textbox carrier
extends beyond the page edge. This provides genuine topology and native-row
evidence, not geometry fidelity or passing repository replacement/rendering.
Ordinary pairing success, both edited copies, logical count1, actual native
reopen and complete no-op/refusal/source-preservation gates still must pass
on the implementation. No tracked binary fixture or new source file is added.

## PR343 contribution reuse

PR343 at c6a78f683f327be8d7674af509874f74945080f3 contributes native story image helpers, owned by F-X201. Compose their checked media allocation with this story's actual Run picture insertion on one candidate. Do not call a separately committing embedding helper followed by an insertion that may fail on a stale, foreign or unsupported run. Preserve no-orphan refusal, physical picture occurrence identity, body-only legacy collection semantics, source aliases and saved/reopened drawing relationships. Header, footer, note and admitted cell/text-box controls must exercise the installed binding and actual drawing or render output. The donor contains no Python implementation, and relationship-only tests provide no visible-picture acceptance. Reuse the assessed patch once and retain the full existing contract.
