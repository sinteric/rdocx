# F-X201, Complete Python section stories and page authoring

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X206

## Problem

Issue304 requires complete rich section and note authoring. Native insertion
at `crates/rdocx/src/document.rs:23892` creates empty section properties.
Python `update_section` at `crates/rdocx-py/src/document.rs:2531` supplies
page dimensions only when a dimension keyword is given. Native
`Section::set_orientation` at document.rs:3230 already swaps existing
width/height correctly, but cannot swap the absent dimensions of a new section.
Python `create_section_story` at document.rs:3059 delegates without enabling
even-page settings. Current story and section accessors return snapshots
rather than live rich authoring handles, per HLD10's section contract.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Native Word facade stability, section
  geometry, story lifecycle, footnote/endnote authoring and Python snapshots.
- `docs/hld/10-bindings-spec.md`, The chosen design and invalidation problem.
- `docs/hld/04-opc-and-packaging.md`, The package and preserved relationships.
- `docs/hld/08-rendering-spec.md`, section pagination and note layout.
- `docs/hld/14-development-backlog.md`, F-X201 and Issue304 acceptance.

## Approach

Preserve the existing checked orientation normalization. New sections inherit
resolved previous-section page size, margins, gutter and header/footer
geometry, filling actual layout defaults where source geometry is absent.
Do not accidentally clone explicit header/footer relationships or other
section ownership. Define index-zero inheritance from the current first
section, and an empty document from the existing Letter defaults. Invalid
lengths, indices and partial changes remain atomic. Add incomplete pgSz
findings to the shared validation contract consumed by F-X203.

Even story creation and settings must publish in one staged transaction.
Expose `settings.odd_and_even_pages_header_footer` through the existing
native settings API, with success-only revision changes. A refused story
creation must not turn the setting on. Retain inherited and first-page
ownership and opaque source content.

Implement live rich section/header/footer access in existing binding modules,
with existing checked story identity and paragraph/run resolution. Existing
snapshot fields and Document.update_section callers remain usable. Story
paragraphs, add_paragraph, add_table, run.add_picture and fields address the
actual owning part, never build in the body then silently transfer content.
Expose different_first_page_header_footer and is_linked_to_previous through
native checked lifecycle operations.

Expose note creation/deletion plus numbering/restart from the native staged
note APIs. Accept a checked paragraph or run from the same document, rejecting
foreign or stale handles before publishing the note or reference. Add run
page/column breaks through native run authoring. Page colour writes both
w:background and displayBackgroundShape while retaining unknown children.
Expose existing text/image watermarks and page borders. Prefer DrawingML
behind-text header watermarks for the requested Google import behavior.

Section lengths remain EMU integers for compatibility and accept existing
Length subclasses, including Twips, Pt and Inches, by their EMU value. Preserve
pinned truncation and diagnose invalid unit/type values explicitly.

## Rejected alternatives

- Rewrite orientation normalization, which already works with dimensions.
- Copy complete sectPr relationships when only geometry is inherited.
- Author section content in the body and expose detached snapshots as live.
- Enable even settings before a later failing story creation.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | inserted_section_inherits_geometry_and_normalizes_orientation | New section has complete geometry, landscape swaps dimensions, inherited margins and distances survive reopen |
| regression | even_footer_creation_and_setting_are_atomic | Even footer is effective, failed creation preserves settings and bytes |
| regression | section_lengths_accept_existing_length_units | EMU integers and Twips/Pt/Inches agree, no silent twip confusion |
| round-trip | rich_footer_page_x_of_y_uses_live_story_handles | Entire footer built in Python, styled fields, picture and table use footer relationships and render correct numbers |
| regression | footnote_endnote_authoring_rejects_foreign_and_stale_locations | Creation/deletion and numbering are correct, errors publish nothing |
| round-trip | page_authoring_properties_preserve_unknown_source | Breaks, colour, watermarks and borders reopen and render with opaque bytes retained |
| regression | incomplete_page_size_is_reported_consistently | Native/Python/CLI shared findings agree after F-X203 integration |

Add cases to existing native regression and Python test entrypoints. Build a
fresh installed extension from the actual worker source, run strict mypy and
stubtest, and record exact pinned Word/LibreOffice render controls in
deterministic font mode. Reproduce the two reported geometry/footer defects
on the claimed Base before production changes.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Units: HLD01 units and pinned truncation, test equivalent EMU inputs.
- Parser/serializer: HLD04 and HLD06 conventions, namespace qualification,
  schema sequence and exact opaque-subtree preservation.
- Layout: HLD08 section and note pagination, deterministic native controls.
- Published API: additive pre-1.0 authoring, reviewed local-source packaging
  dry run and actual affected archive records below the size ceiling.
- PyO3: HLD10 handles, isolated installed tests, strict typing and stubtest,
  plus both WASM target checks for the changed common native facade.
- Oracle: pin Word/LibreOffice versions and retain actual compared output.

## Hash harness

The geometry and even-story repairs intentionally change output for authored
new sections and enabled even stories. Identify any affected hash fixture
before implementation and declare its exact delta in an approved revision.
Unrelated output must remain unchanged. No baseline movement is authorized
by this draft. Geometry-changing commits are labeled separately from wrappers.

## Exclusive file claims

Existing rdocx document/paragraph/run modules, rdocx-oxml section/settings
models if needed, rdocx-py document/paragraph/run/formatting modules and stubs,
existing native and installed Python test entrypoints, validation consumers,
and the HLD sections above. Resolve collisions with X198, X194, X200, X202,
X203, X192 and F-283 before assigning the wave. No new source module or crate.

## Implementation checklist

- [x] Finalize exact live Section/Story handle signatures and native gaps.
- [x] Approve validation ownership and measured expected hash changes.
- [x] Claim after source conflicts clear and reproduce actual defects.
- [x] Implement geometry and even-story atomicity, then rich APIs.
- [x] Run all Issue304 acceptance, scoped riders and independent microscope.
- [x] Prepare a validated feature-local handoff.

## Open questions

Resolved by the live/snapshot, note/section and checked related-story contracts
below. Admit ordinary rich related-story tables/cells through qualified
same-owner staged source spans, not main-body ordinals or detached wrappers.
New watermark authoring is real DrawingML text/picture behind header text,
with exact named style/opacity and checked owner relationships. Existing VML
compatibility methods remain accurately exposed. Native/LibreOffice output,
installed binding, late failure and optional claimed Google import evidence
are implementation gates, not waived tests. F-X206's completed token provider
supplies originating owner/cause authority once before this source claim.

## Resolved authoring contracts

## Preserve snapshots and add live owners

`PySection` is the existing snapshot at `crates/rdocx-py/src/document.rs:795`. `Document.sections` returns a tuple of those snapshots at :2502. `PyStory` is explicitly frozen, comparable and manually constructible at :459, and `PyHeaderFooterVariant` is a snapshot at :737. Preserve these constructors, equality, fields, existing tuple return and existing `update_section` calls. Do not silently convert those public values into live handles.

Add `Document.section(index: int) -> SectionHandle` in the existing document binding file. This is a concrete owner handle with the document object, section ordinal and captured document revision. Its geometry getters and setters delegate to the same checked native properties used by `update_section`. Its six accessors are `header`, `first_page_header`, `even_page_header`, `footer`, `first_page_footer`, `even_page_footer`, each returning `HeaderFooter`. Add `different_first_page_header_footer: bool` and note-policy setters on this handle. No new module, trait, generic, crate or forwarding-only builder is needed.

`HeaderFooter` owns a section slot, kind and variant, rather than a detached XML snapshot. Expose `is_linked_to_previous: bool`, `paragraphs`, `tables`, `add_paragraph(text: str = "") -> Paragraph`, `add_table(rows: int, cols: int) -> Table`. Resolve the actual effective story through `Document::section_story(section_index, kind, hdr_type) -> Result<Option<SectionStory>>` at native document.rs:23432. Read access never creates a part. An absent slot has empty collections. First mutation creates its explicit story if none exists. Mutation of an inherited slot uses the effective owning part, matching linked header/footer semantics. Setting linkage false uses `unlink_section_story` at :23576 when inherited, or `create_section_story` at :23480 when absent. Setting true uses `inherit_section_story` at :23558. Creating an explicit empty slot is not the same as removing its reference, per `remove_section_story` at :23619.

Handles reject another document by object identity and reject changed revision before ordinal/path resolution. A mutating handle refreshes its own revision after a successful structural publication, so subsequent calls on that owner can continue. Previously fetched paragraph, run and table handles remain stale and must be fetched again. Failed edits change neither revisions nor source. Keep the pre-existing `create_section_story` revision policy intact for its old entrypoint, and document the new live owner policy separately.

## Rich story authoring needs a concrete checked resolver

Current `ParagraphLocation` covers only body and table cells at `crates/rdocx-py/src/paragraph.rs:12`. `PyParagraph` stores `ContentPath` at :129. `PyRun::edit` resolves only those same two cases at run.rs:60. Shared `PathSeg` at `crates/oxml-py-support/src/lib.rs:10` has no story discriminator. Adding a HeaderFooter class alone cannot make these existing Paragraph/Run operations use a footer.

Extend the existing local concrete paragraph/table location representation with a checked story location carrying native `ContentLocation` plus the document revision. Do not encode a footer index as a body index, add a parallel detached paragraph class or retain a borrowed Rust facade across a Python call. Reuse the ordinary Python Paragraph, Run, Table, Cell and formatting classes after their owner resolver understands the actual story part. Every picture and hyperlink must allocate its relationship in that owning part.

Available native primitives are `story_items(&StoryId) -> Result<Vec<StoryItemRef>>` at document.rs:16916, `story_range_paragraph_snapshot(&ContentLocation)` at :17047, `ContentFragment::paragraph(CT_P)` and `ContentFragment::table(CT_Tbl)` at :1234 and :1239, and `insert_content(&ContentLocation, ContentFragment) -> Result<()>` at :18289. The last stages, validates a direct-child boundary, closes inserted namespaces, reconciles and reopens before publishing. Use it for owner-level paragraph/table insertion.

There is no current public checked arbitrary rich story paragraph mutation facade. `story_paragraph_mut` at :16052 is private and its current path admission is narrower than every related-story cell. Add one concrete public checked replacement operation in the existing document module, proposed `replace_story_content(&ContentLocation, ContentFragment) -> Result<()>`, with same-kind validation, source-owner validation, unchanged surrounding bytes, relationship validity, namespace closure and one staged reopen. Use it for actual paragraph and table mutations, not a body transfer. A paragraph replacement must retain the original parsed opaque children and attributes. Table cells resolve within that checked table owner. Do not advertise nested routes until existing physical span resolution admits and tests them.

Pictures need one native run-boundary operation. Existing `insert_picture_to_story(&StoryId, Option<&ContentLocation>, &[u8], &str, Option<Length>, Option<Length>) -> Result<ContentLocation>` at :17496 inserts a picture paragraph, not inside a selected run. Proposed `insert_picture_at(&StoryRunPosition, &[u8], &str, Option<Length>, Option<Length>) -> Result<()>` stages media, the owner relationship and the checked inline run together. Reuse current image dimension probing and validation. Python `Run.add_picture` passes the checked run boundary, rather than appending a new paragraph. This operation is justified by the requested rich footer use case.

Generic PAGE and NUMPAGES authoring can use a proposed public `insert_field(&StoryRunPosition, Field) -> Result<()>` in existing field.rs. Existing `insert_checked_story_field` at field.rs:619 already stages a complex field and calls `insert_story_run_staged`. Preserve its validation, allow the requested cached result and run properties, and avoid making callers depend on a private helper. Native `Run::add_field` at run.rs:507 works for an already resolved ordinary run. PAGE X of NUMPAGES must preserve separate runs and style rather than flattening footer text.

## Checked note creation at run boundaries

Current `create_footnote(&ContentLocation, &str) -> Result<i32>` and `create_endnote` at native document.rs:18745 and :18798 admit only a direct body paragraph and append a reference run at its end. Custom-mark creation at :18878 calls those operations and modifies the last marker. Neither currently promises an arbitrary run boundary. The legacy `add_footnote` at :18709 has an unchecked expect and is not the binding target.

Add `create_footnote_at(&StoryRunPosition, &str) -> Result<i32>` and `create_endnote_at(&StoryRunPosition, &str) -> Result<i32>`, plus custom-mark variants only where the requested API needs them. Keep old paragraph APIs unchanged. New checked position creation uses the existing private `insert_story_run_staged(&StoryRunPosition, CT_R) -> Result<()>` at :16176, which obtains the physical owner source, closes namespace scope, parses the selected paragraph, validates `insert_accepted_run`, and replaces only that span. Allocate the note record and reference in one candidate before the sole reopen/publication. Do not create a note first, commit, then move its reference. Reject header/footer/note reference locations as invalid note hosts. Admit checked main-story paragraphs, including table cells or modeled controls only when their physical path resolver proves ownership. Unsupported opaque and invalid routes refuse atomically.

Python proposed `Document.add_footnote(reference: Paragraph | Run, text: str = "") -> FootnoteEndnote`, and `add_endnote` equivalent. A paragraph means its accepted end boundary, a run means the boundary immediately after that accepted run. Explicit same-document identity and captured revision are required before conversion. Existing `StoryRunPosition` accepts a paragraph or StoryItem at Python document.rs:552, but a detached snapshot alone does not prove document identity. The returned concrete note owner retains its note id and family, provides rich paragraphs/tables through the same owner resolver, and has `delete()` using native `remove_footnote` or `remove_endnote` at :19509 and :19514. Note deletion must invalidate both reference and related-story handles.

Numbering uses existing `NotePolicy` at native document.rs:168, `NoteRestart` at :130, `Document::set_note_policy` at :24390 and `Section::set_note_policy` at :3406. Expose format, positive start, restart and family-valid placement. Reuse native refusal of zero start and incompatible placement, do not hand-write numbering XML in Python. Preserve custom marks, separator records and existing unrelated note owners.

## Geometry and unit choices

Actual layout defaults are native `rdocx-layout/src/engine.rs:11096`: width 612pt, height 792pt, each margin 72pt, header/footer distances 36pt. `section_page_geometry` at :11223 uses gutter zero when absent and applies it according to existing mirror/gutter settings. Therefore persist new geometry in integer twips: 12240 by 15840, margins 1440, distances 720 and gutter zero. Do not inherit a gutter-adjusted margin and also persist the gutter, which would apply it twice.

Before native `insert_section_in_place` at document.rs:23892 changes ownership, select prior section index minus one, current first for insertion at zero, or defaults when no section exists. Resolve each optional geometry field against the defaults above. Copy orientation with geometry, so a landscape predecessor remains landscape. Copy only page width/height, four margins, gutter, header/footer distances and orientation. Existing ownership, columns, page-number starts, title-page flag, break type, note policy, borders and opaque metadata stay on their existing sections. The new section receives none of those unrelated owners or metadata. This is a deliberate geometry inheritance rule, not a claim that rendering currently searches previous sections for missing pgSz.

`Section::set_orientation` at :3230 already swaps present dimensions only when necessary. Keep it. Apply complete dimensions before orientation when a caller supplies both. Do not change incomplete existing source merely by opening or reading it. Emit an incomplete-pgSz validation finding instead.

Python shared.py:11 defines Length as an EMU-valued int subclass. Inches, Cm, Mm, Pt and Emu exist at :57 onward. There is currently no Twips subclass. Add `Twips` in this existing file and export/stub it consistently, constructing `int(twips * 635)` under the same integer semantics as other Length classes. Existing integer EMU inputs and every existing subclass remain accepted. Reject bool, nonintegral values and overflow explicitly. Conversion to signed 32-bit twips truncates toward zero at 635 EMU per twip, as required by HLD01:68. Test positive, negative, fractional-EMU boundaries and each property's own positive/nonnegative constraint rather than changing the global unit representation.

## Even settings and remaining page authoring

Existing `even_and_odd_headers() -> bool` and `set_even_and_odd_headers(bool) -> Result<()>` at native document.rs:24357 and :24364 are the settings API. Add the setting within the same staged candidate as `create_section_story` for an Even variant. Cover both a newly allocated story and an already-direct story when the setting was disabled. A failure publishes neither story nor settings. Expose the requested Python `settings.odd_and_even_pages_header_footer` property through an existing-document-owned concrete settings handle in document.rs.

Native `Run::add_break(BreakKind)` at run.rs:488 already handles breaks. Expose its exact line/page/column choices through the checked body/cell/story run resolver. Page color needs a staged native operation that changes retained `CT_Document.background_xml` and the qualified settings displayBackgroundShape element together. This property currently has no native page-color setter. Keep unknown background/settings attributes and children, write schema order and reconcile settings relationship through existing staging. Do not treat the allowlisted settings child at rdocx-oxml/settings.rs:2472 as an already available typed setter.

Native page borders are Section methods at document.rs:3134 and :3479. Existing watermarks are `set_text_watermark` at :20855, `set_text_watermark_for` at :20876 and `set_image_watermark` at :20913, all VML. Expose them accurately. The requested Google-compatible DrawingML header watermark is an additional implementation path, not already provided by those VML methods. Reuse native DrawingML background anchor semantics and owning-header relationship allocation, retain other header children, and obtain actual Word/LibreOffice plus Google import evidence before claiming that compatibility. No Google UI or external upload is authorized by this read-only research task.

## Shared X203 validation ownership

Aligned with the X203 researcher. X201 owns these minimal concrete native types in existing document.rs and exports from existing lib.rs:

```rust
pub enum ValidationSeverity { Error, Warning }
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub message: String,
}
pub struct ValidationReport { pub findings: Vec<ValidationFinding> }
impl ValidationReport { pub fn is_valid(&self) -> bool }
impl Document { pub fn validate_sections(&self) -> Vec<ValidationFinding> }
```

Incomplete explicit pgSz, with exactly one of width or height, is a Warning in section ordinal order, proposed stable message `section {ordinal} has incomplete page size: width or height is missing`. Completely absent pgSz is default geometry and produces no incomplete finding. Negative or zero explicit dimensions produce an Error with a separate stable message. Read source values without filling or writing them. These findings do not reuse AccessibilityIssue, which is a different audit at native document.rs:30704.

X203 owns `validate_document_bytes(&[u8]) -> Result<ValidationReport>` and `validate_document_path(&Path) -> Result<ValidationReport>`, existing CLI assembly migration, Python module bytes/path wrappers and immutable current-Document validation. Existing OPC-open failure remains Result Err, per CLI commands.rs:2188. Existing native Document-open failure becomes the current error finding at :2290 after collected package findings. Report order stays existing errors followed by warnings. X203 includes section findings in that order without calling or depending on the CLI library. No duplicate report representation or forwarding report wrapper.

## Required focused evidence and actual barriers

Reproduce new-section orientation and Even creation defects on the claimed Base before production edits. Matrix: empty/first/middle/append insertion, partial source geometry, portrait and landscape, odd/even/mirrored/gutter placement, checked units and failed atomic setters. Keep unrelated section refs, page-number starts, columns and unknown XML exact through save/reopen.

Build a footer entirely through installed Python: PAGE X of NUMPAGES, styled runs, inline picture, table and cell text. Assert each part relationship targets actual footer media and no body contamination. Exercise default/first/Even inherited and unlinked slots across three sections, mutation/refetch behavior, stale and foreign owners, malformed references and refused atomic operations. Freshly build/install the worker wheel, strict mypy and stubtest are required.

Notes require main paragraph and run-boundary controls, accepted inserted runs, controls/cells where admitted, reference positions, custom marks, id allocation, numbering restarts, separators, deletion with references, failed foreign/stale/header host and preserved other owners. Test source-preserving namespace aliases, unknown attributes/children and schema sequence. Use actual pinned python-docx object-model comparisons for sections and footer relationships where its API overlaps. Pin Word and LibreOffice for independent rendered page-number, orientation, first/Even story and note controls, deterministic font mode for native baselines. Both WASM target checks and actual affected package archive measurements remain earned riders.

Genuine unresolved acceptance evidence: existing VML watermark methods do not establish the requested Google-compatible DrawingML path, and current rich related-story cell paths need proved physical ownership admission before a public claim. Neither is a reason to invent another approval gate. Parent can approve the concrete additive design, then implementation must obtain real controls and refuse unsupported routes honestly. No baseline change is authorized here. Actual harness deltas must be measured and separately reviewed.

## Approved related-story and DrawingML output contract

## X201 rich related-story cells

Admit cells in the actual linked header/footer and footnote/endnote parts through a checked physical owner, not an ordinal borrowed from the main document. Existing story_sources at document.rs:15867 resolves related parts through actual relationships. story_source_and_owner at15960 accepts TableCell in those sources and validates kind, owner index and fingerprint. The namespace-qualified scanner permits cell/textbox owners at8713. story_range_paragraph_source at16952 resolves one checked paragraph item or a bounded content-control paragraph path. These are real reusable source mechanisms, not a completed Python rich authoring API.

The current mutable borrow helper is unsuitable for that API: story_paragraph_mut at16140 computes a modeled cell route only for the main part at16199 and refuses other parts at16205. Python ParagraphLocation at paragraph.rs:12 stores only Body or main-table cell ordinals. Choose the existing plan's concrete story-aware location alternative carrying a native ContentLocation plus originating document authority/revision. Extend the existing Paragraph/Run/Table/Cell classes and their common owner resolver once. It will require removing ParagraphLocation's Copy assumption when it gains owned location data, not introducing a second class hierarchy or generic path engine.

For each access, validate document identity/revision before physical resolution, resolve the current qualified owner and unique item/span, parse the latest selected paragraph/table, apply the requested typed edit, then replace that same owner span on a staged candidate. The proposed replace_story_content operation in existing document.rs must enforce same-kind replacement, complete containing-cell/source correspondence, namespace closure and owner-scoped relationships, prepare/reopen and one publication. Native insert_content at18377 already demonstrates the source-splice/reopen transaction, while set_story_source_xml at7779 reconciles existing modeled part state. Do not retain a borrowed native paragraph across Python calls or silently route a footer cell into the body table vector.

Supported initial admission is direct paragraphs/tables and independently checked cell owners inside related-story tables, including nested tables where the scanner proves their unique enclosing physical cell. Supported modeled control paragraphs use the existing bounded control path. Refuse ambiguous/opaque ownership, unsupported path depth or stale/foreign owners before mutation. This refusal protects imported unsupported content, but does not excuse failure to implement the ordinary rich footer table/cell case explicitly required by Issue304. Note references themselves remain allowed only at admitted main-story hosts, not in a footer/note merely because those parts support rich editing.

No native capture is required to decide this ownership contract. The implementation gate is one installed Python footer containing styled PAGE/NUMPAGES runs, a logo, a table/cell paragraph and a nested-table admitted route, across linked/unlinked variants. Assert owning-part media/hyperlink targets, unknown XML preservation, save/reopen/render, no body contamination, failed/refetched handles and late-failure atomicity. Use original-source namespace tests. Do not treat scanner availability alone as a passed rich-cell test.

## X201 DrawingML watermark output policy

Keep legacy native VML methods and their existing behavior. Current set_text_watermark_for at document.rs:20963 explicitly constructs VmlWatermark::Text at20983. Expose them accurately. New rich section watermark authoring selects a DrawingML anchored text box or picture in the section's actual header variant, behind text, page-relative positioning, no text wrap, checked dimensions/rotation/colour/opacity and reserved drawing identity. Reuse the existing native file and ownership/transaction code. Do not insert the anchor in the main body, convert text to pixels, or replace unrelated header content.

Concrete reuse: authored_text_box_alternate_content at742 already writes qualified WPS DrawingML plus VML fallback, and add_text_box_to_story at17789 validates, reserves identity and writes to the chosen story before reopen. CT_Anchor.background at rdocx-oxml/drawing.rs:473 supplies page-relative behind-text picture geometry. These are implementation ingredients, not a ready watermark writer: the current text helper accepts plain text and does not carry all TextWatermarkOptions styling/opacity. The new path must retain real text and add the named formatting in its actual run/shape representation. Word2010 text fill is a run-property extension, not ordinary w:color alpha, as documented by [Microsoft's w14:textFill reference](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.linq.w14.textfill?view=openxml-3.0.1). Validate exact extension/schema order and actual renderer handling rather than silently discarding opacity.

Stage header creation/linkage policy, media relationships for picture watermarks, and both physical textbox representations atomically. Replace only API-owned watermark identity, preserve unrelated drawings/text/producer watermark owners and prune only unreferenced API media. If a representation's requested feature cannot be faithfully authored/rendered, return a named atomic error rather than fall back silently to VML-only output. Existing VML-only setters remain a documented compatibility path. Image and text new section paths share owner/anchor mechanics but are substantive writers, not forwarding-only new facades.

Issue304's exact text says Google import is best effort and prefers a DrawingML picture or text box behind text in the header. Approve that concrete output policy now. Native Word/LibreOffice normal reopen/render is a required implementation gate. An actual permitted Google import study is needed only before a demonstrated Google-compatibility claim, not an infinite preapproval prerequisite or a fabricated CI guarantee. No external upload occurred in this task.

## PR328 authenticated source assessment

PR328 closes Issue304 at30c2e61cbdec5499546997df94b6c94e3a1e446d. Full
38-file diff,75 base/head Git blobs, six commits and complete empty PR/Issue304
discussions are retained in /private/tmp/S91-PR328-intake. Independent root
checks authenticate104 artifact hashes/sizes and all75 blob identities.
No contributor code was executed and no new F-ID is created.

Reuse qualified native geometry, footer field, even/first visibility, story
table-width, note style/separator and wrapping-clear controls in existing
files. Contributor story.rs is not an authorized new module. Its:518 read
creates missing stories, while this contract requires empty read-only results.
Its section snapshots with attached ownership and packed section/story slots
do not replace the separate live SectionHandle and F-X206 causal provider.
Avoid a second story-specific Table/Cell hierarchy. Native document.rs:3818
clones columns/page borders/grid beyond the selected geometry inheritance.
Narrow that copy to the approved properties. Python document.rs:2772 rejects
legitimate tiny EMU values using an inferred unit threshold. Use exact checked
length extraction and range constraints instead.

Contributor notes support last-run direct body references and integer IDs,
not rich admitted run/main-table/control owners. VML-only watermark setters
remain honestly labelled and do not meet DrawingML watermark acceptance.
Story and even-header visibility mutations commit separately at story.rs:174
and :259. Stage content, relationships and settings together before one
publication. Native validation remains the shared F-X203-consumed policy,
not the contributor's CLI-only incomplete-dimension heuristic.

The note-reference renderer change at engine.rs:9607 prevents a second
superscript shrink. It is intentional behavior, requiring its own labelled
commit, native styled-note control and deterministic delta review. Existing
49-fixture claims do not prove unchanged note behavior. Full source assessment
and cited gaps remain in the retained assessment, with no local test or
acceptance claim.

## PR328 advanced-head reconciliation, 2026-10-10

Immutable head `381f76683fc28780219dd21e06bf701b63b22d49` replaces
`30c2e61cbdec5499546997df94b6c94e3a1e446d`. Root authenticated the manifest
and all41 source records by Git blob identity, bytes and SHA-256 and inspected
the exact three-path delta. Assessment and receipt are retained in
`/private/tmp/S91-PR328-381f766-intake`. Issue304 remains F-X201.

Reuse default Header/Footer paragraph-style selection inside the approved
single staged native insertion. Explicit style wins, lookup returns the actual
style ID by ID/name, absent style stays None. Add all six story slots,
localized IDs/names, explicit Title, absent styles and saved/reopened pStyle
controls in the existing test entrypoint. Validate before publication and
prove failure preserves bytes and tokens. Absent reads still must not create
parts, and owner/lifetime and rich story acceptance remain mandatory. The
contributed multi-call insertion and new story module are not authorized by
this intake. Contributor tests and HLD changes are proposals, not local proof.

## PR343 contribution reuse

Reuse PR343's assessed native story image allocation and content-type lookup only after reconciliation with the actual checked physical owner and one staged publication. The exact donor head is c6a78f683f327be8d7674af509874f74945080f3, with immutable assessment at /private/tmp/S91-PR343-intake/assessment.md. Related-story rich picture authoring remains the existing acceptance, including header, footer, notes and admitted cells or text boxes. Donor tests embed media relationships without inserting or painting a drawing. Actual owner-scoped drawings, saved/reopened media, stale/foreign/malformed refusals, aliases and native render controls remain required. Run picture binding integration belongs to F-X202 and consumes this ownership work once.

## User-approved opacity boundary, 2026-10-12

Actual native Word controls reverse the authored w14 opacity endpoints, while
the 50 percent control paints correctly in Word and LibreOffice. The user
approved a temporary restriction of new DrawingML text watermark authoring to
opacity 0.5. Other finite opacity values return a named error before document,
header, relationship, drawing identity or handle state changes. Existing VML
compatibility setters retain their full range. All other F-X201 acceptance
remains unchanged.

Use real styled WPS text at opacity 0.5 with a genuine equivalent native
WordArt textpath fallback. Project only the selected branch for painting and
preserve the complete physical owner for strict source identity, replacement
and repeated save/reopen. Tests prove one painted watermark, all inactive
branches retained, foreign namespace refusal and atomic rejection on absent
and existing owners for zero, one and intermediate unsupported opacities.
Actual Word and LibreOffice default-opacity open, render and reopen remain
mandatory. Retain the failed full-range controls as evidence.

F-X219 in S92 owns full-range interoperability. This explicit user-approved
boundary supersedes the earlier full-range DrawingML text opacity requirement
without reopening S91 intake or claiming that the endpoint defects are fixed.
