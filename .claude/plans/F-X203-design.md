# F-X203, Complete Python Word document operations and validation

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X196, F-X201, F-X212

The design is approved. Implementation waits for every formal dependency
to be completed and for the global sprint design and file-claim barriers. Source citations were checked
against canonical head `360b5b1cf77d19e7fe5c4cc26152dd55a00d12c1`.

## Problem

Issue306 requests Python parity with existing native and CLI document
operations. Native exports and templates exist, but Python document.rs exposes
only core_properties at2102 among the requested property groups. Native
Document::text at `crates/rdocx/src/document.rs:19671` is main-body text and is
not CLI all-story text. CLI commands.rs:146 combines body plus readable stories,
with a separate malformed-story policy at230. CLI validation at2181 privately
assembles structural and advisory findings.

Native insert_document at document.rs:26198 is infallible and uses expect on
preflight. Checked import_fragment at18411 stages dependency closure with style
conflict policy. Exports ODT/RTF/EPUB return bytes plus loss diagnostics, not
bare bytes. A Python parity plan must not hide diagnostics, call a panic-based
insertion wrapper or treat existing body-only text as every-story output.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-X203 and full Issue306 criteria.
- `docs/hld/10-bindings-spec.md`, Python API shape, The invalidation problem,
  handled loudly, Native Word facade stability and CLIs, including validation
  findings, accepted-view text, properties, fragment closure and templates.
- `docs/hld/03-architecture.md`, Three families, one workspace and Why these
  seams, for native validation ownership without a facade-to-CLI dependency.
- `docs/hld/08-rendering-spec.md`, Word OfficeMath layout, Word revision views
  and Word section geometry and page numbering.
- `docs/hld/12-testing-strategy.md`, Test taxonomy, Binding tests and The Word
  render fidelity gate.
- `docs/hld/04-opc-and-packaging.md`, The package and deterministic saves.

## Approach

Expose existing native capabilities through concrete checked binding objects
in existing files. Inventory all public Rust behavior and declared Python names
on the claimed Base, then write genuine installed-wheel missing-operation
regressions. Reuse integrated X196 visible paragraph validation and X201 section
story rules. Do not redefine their acceptance in a second implementation.

### Exports, text and counts

```python
Document.to_html() -> str
Document.to_markdown() -> str
Document.text() -> str
Document.text_report() -> TextReport
Document.to_odt() -> ExportResult
Document.to_rtf() -> ExportResult
Document.to_epub() -> ExportResult
Document.word_count() -> int
Document.character_count(*, include_spaces: bool = True) -> int
Document.page_count() -> int
```

ExportResult owns bytes and immutable typed diagnostic records. Reuse native
to_html/to_markdown at document.rs:28085/28097 and existing ODT/RTF/EPUB write
results at odt.rs:137, rtf.rs:85 and epub.rs:64. Do not discard the existing
path/message diagnostics from lossy converters. The approved installed names are to_odt, to_rtf and to_epub.
Bare HTML/Markdown retain the existing native subset and one provider. The
explicit Issue332 emitter corrections below amend the previous Markdown
behavior, without an independent Python formatter.

Extract shared accepted-view text presentation from existing CLI into native
existing document.rs/lib.rs, with one owned TextReport containing rendered
plain text, scope and diagnostics. Its deterministic renderer matches CLI
body text, story ordering, separators and trailing newlines exactly. CLI
adopts the same function and retains stderr warning behavior. If another
story is malformed, retain body plus the same degraded main-only scope and
warning, instead of silently skipping a single owner or changing CLI policy.
Python text() returns the exact CLI stdout projection. text_report exposes
diagnostics separately so degraded extraction is inspectable. No subprocess,
temp-file CLI invocation or rdocx dependency on rdocx-cli.

Counts must have explicit scope. Existing word_count at document.rs:29025 uses
accepted main-body paragraphs, including table/control text, and whitespace
tokens. Proposed word_count preserves this native contract. Character count
uses Unicode scalar characters, with documented whitespace handling, across
the same accepted body projection. Do not count CLI presentation separators or
metadata story labels as content. All-story counts, if requested instead, need
explicit names and a shared native policy. Page count comes from fresh
deterministic layout, not cached app-properties pages. Reuse F-X195 caller-font
selection if caller font options are part of the final method contract.

### Templates and document assembly

```python
Document.render_template(data: Mapping[str, JsonValue]) -> int
Document.insert_document(other: Document, *, at: int,
    policy: FragmentConflictPolicy | None = None) -> None
Document.import_fragment(destination: ContentLocation,
    fragment: DocumentFragment, *, policy: FragmentConflictPolicy | None = None) -> None
FragmentConflictPolicy.reuse_equivalent() -> FragmentConflictPolicy
FragmentConflictPolicy.rename_all() -> FragmentConflictPolicy
FragmentConflictPolicy.with_style_reuse(reuse: bool) -> FragmentConflictPolicy
FragmentConflictPolicy.with_numbering_reuse(reuse: bool) -> FragmentConflictPolicy
FragmentConflictPolicy.with_related_part_reuse(reuse: bool) -> FragmentConflictPolicy
```

Native FragmentConflictPolicy at document.rs:1036 is a concrete non-exhaustive
struct, not an enum. Its constructors at1044/1053 reuse equivalent supported
dependencies or allocate fresh identities, with independent style/numbering/
related-leaf toggles. None means the native default reuse_equivalent policy.
Do not invent a 'fail' variant or a narrower style-only policy that discards
numbering and related-part decisions. JsonValue is a recursive supported
JSON scalar/list/string-keyed-object type alias in the existing stub.
DocumentFragment and ContentLocation reuse existing native owned fragment and
checked story boundary semantics. Bind only required constructors/export/read
metadata, with concrete package identity and frozen source ownership.

render_template at document.rs:27201 already delegates to the staged native
engine. Convert Python JSON-compatible scalars/arrays/objects explicitly.
Check bool before integer, bound integers to the native JSON representation,
reject nonfinite floats, bytes, cyclic containers and unsupported objects.
Never call arbitrary str() or attribute getters. Conversion completes before
mutable access. Scalar tags crossing runs, nested for/if, missing paths and
native atomic errors retain native behavior and exception mapping.

Implement insert_document via the checked fragment/dependency closure route,
or a minimal checked native try_insert_document in existing document.rs that
reuses that route. Do not bind the legacy infallible expect wrapper at26198.
Validate at against the documented direct body index rather than silently
clamping Python mistakes. Stage imported styles/numbering/media/comments/notes,
IDs and section ownership, with the existing equivalent-reuse/rename policy.
Preserve source Document and opaque dependency bytes. Structural success bumps
revision once and invalidates old destination handles. Failure bumps nothing.

### Properties and settings

```python
Document.custom_properties: CustomProperties
CustomProperties.get(name: str) -> CustomPropertyValue | None
CustomProperties.set(name: str, value: CustomPropertyValue) -> None
CustomProperties.delete(name: str) -> bool
Document.app_properties: AppProperties
Document.settings: Settings
Settings.track_revisions: bool | None
```

Native custom_properties/set/remove at document.rs:25950/25965/26011 and
application getters/setters at25911/25919 already stage checked mutations.
Reuse `oxml-core/src/custom_properties.rs:22` typed CustomPropertyValue and
actual AppProperties fields. Define string, bool, integer, floating and date
mapping exactly, preserving property IDs/types on updates. Reject bool-as-int,
out-of-range integers, nonfinite floats, invalid dates and invalid names before
mutation. Missing delete is a defined no-op. Date timezone/serialization choice
must match the native supported value contract.

AppProperties exposes company, manager, template and supported native count
fields without conflating cached producer counts with calculated counts.
Preserve unsupported fields and producer XML. Settings is document-owned and
read-only retrieval does not create a settings part or increment revision.
Native track_revisions getter/set/remove at document.rs:24956/24961 supplies
presence-sensitive None/false/true. Assignment and removal use staged native
settings mutation, preserving schema position and opaque settings children.
This sets the tracking flag only. F-291 owns tracked-edit authoring.

### Validation parity

Move the existing CLI structural/style/numbering/comment/metadata advisory
assembly into a shared native routine in existing files. Proposed result:

```rust
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub message: String,
}
pub struct ValidationReport {
    pub findings: Vec<ValidationFinding>,
}
pub fn validate_document_bytes(bytes: &[u8]) -> Result<ValidationReport>;
```

Exact message/order and validity projection match the CLI. Byte-level checking
is needed to diagnose a package that cannot produce a Document. Avoid assuming
a bound open Document can reproduce invalid unopened input. Python exposes
Document.validate() over an immutable serialized snapshot and a bytes/path
validation entrypoint if needed for full CLI malformed-package parity. Snapshot
validation must not change source, public revision or held-handle usability.
CLI formatting and exit status stay local to commands.rs, consuming the report.
Malformed XML findings retain named part errors and open failure, not an opaque
panic or swallowed scan error. Reuse X196's final opaque-aware visible-paragraph
logic and X201 checked section findings, not its earlier unreconciled draft.

### Equations and content controls

```python
Paragraph.add_equation(source: str, *, format: Literal['latex', 'mathml']) -> Equation
Paragraph.equations() -> tuple[Equation, ...]
Equation.to_latex() -> MathConversionResult
Equation.to_mathml() -> MathConversionResult
Document.content_controls() -> tuple[ContentControl, ...]
Document.set_content_control_value_by_tag(tag: str, value: str) -> int
Document.set_content_control_value_by_alias(alias: str, value: str) -> int
```

Native equations are paragraph-owned, paragraph.rs:960/981. Conversion already
exists at math.rs:115/137/182/200 and returns MathConversionResult diagnostics
for bounded LaTex/MathML support. Preserve those diagnostics, validate before
owner mutation and use native OfficeMath serialization/schema order. Do not
claim arbitrary TeX execution or invent an unrelated body insertion contract.
Equation snapshots and source owner identity follow existing handle rules.

Native content_control.rs:69 inventory is typed main-body controls, with
recursive accepted ownership. Expose tag/alias/type/text with frozen immutable
records and document that scope. The by-tag/by-alias setters at95/102 already
stage display/custom XML binding updates. Reuse exactly, including duplicate
selection and unsupported binding refusal. Do not reinterpret list results as
mutable detached XML handles. No content-control creation, dropdown authoring
or tracked-change creation is included. If all-story inventory is required,
that native expansion must be explicitly designed and tested, not assumed.

## Rejected alternatives

- Document.text wrapper advertised as all-story text when it is body-only.
- Running the CLI from Python or adding a facade-to-CLI dependency.
- Discarding export/math loss diagnostics to offer a convenient bare value.
- Binding the expect-based native insert_document wrapper directly.
- Calling arbitrary Python str() during template data conversion.
- Cached producer pages called fresh layout page count.
- Detached property/settings/control snapshots pretending to be mutable owners.
- A new source file/module for a forwarding binding abstraction.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | test_issue306_missing_operations_on_installed_base | Genuine missing-method failures for every advertised operation before production edits |
| regression | python_markdown_html_match_native_cli | Sample corpus accepted content, headings/lists/tables/links/bold/italic, exact Python/native Rust bare HTML and Markdown bytes. CLI equals those exports plus independently source-authored related-story appendices, with direct full equality when no appendix exists |
| regression | python_all_story_text_matches_cli | Body tables/controls, headers/footers/notes/comments/text boxes, ordering/markers/newlines, accepted revisions |
| regression | malformed_story_text_keeps_cli_degraded_scope | Same retained body, scope and diagnostic, no silent loss or source mutation |
| regression | python_exports_retain_loss_diagnostics | ODT/RTF/EPUB bytes open with native converter, unsupported source reports path/message, deterministic output |
| regression | counts_have_explicit_content_and_layout_scope | Unicode scalars, spaces, native word tokens, cached-vs-calculated pages, deterministic layout |
| regression | python_template_json_conversion_and_native_blocks | Split-run scalars, nested for/if, invalid/cyclic/unsupported values and native failure byte equality |
| regression | checked_document_fragment_import_closes_dependencies | Styles/policies, images/numbering/notes/comments/custom XML, source unchanged, stale handles and atomic failure |
| regression | typed_custom_and_app_properties_roundtrip | Bool/int/date/string/float distinctions, IDs, delete no-op, invalid bounds and opaque producer fields |
| regression | python_validation_matches_cli_findings | Complete finding ordering/message/severity and verdict, X196 pictures/opaque controls, X201 sections, malformed unopened packages |
| regression | equation_conversion_and_insertion_preserve_diagnostics | Supported LaTex/MathML, unsupported constructs, owner stale/invalid rollback, OMML reopen/render |
| regression | content_control_inventory_and_binding_are_checked | Tag/alias/type/text, duplicate selectors, custom XML update, unsupported binding byte equality |
| regression | track_revisions_is_presence_sensitive | None/false/true, readonly no part creation, schema placement and opaque settings preservation |
| binding | issue306_runtime_stubs_and_revision_contract | Fresh current worker wheel, neutral CWD, strict mypy/stubtest, correct types/errors and live/stale ownership |
| round-trip | issue306_all_mutations_save_and_reopen | Every criterion survives native reopen, source bytes/closure and appropriate deterministic render |

Reuse existing Python test_core/typing_smoke and native regression entrypoints.
Reference native fragment conflict regression_test.rs:37569, atomic incomplete
closure:37691, full-story dependency graph:46119 and owner-pair matrix:46683.
Reuse template paired text boxes:21373 and control boundary refusal:21992,
content-control binding controls:20160 through20356 and existing property tests.
Rebase citations before claiming. No new test binary or binary fixture.

Pinned native Word opening/rendering and existing LibreOffice/export oracles
validate only the planned exact cases. Google Docs custom-property loss is
documented, and supported OMML/control/Markdown import is best effort. Sample
corpus Python MD, text and validation parity is required, not optional.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

Update native ownership of shared validation/text, equation/count semantics if
needed, current Python APIs and invalidation, complete parity/diagnostic tests,
and actual delivered story scope. No fictional Word HLD07 document exists.

## Risk routing

- Parser/serializer: HLD04 The package and HLD06 Preservation strategy, exact
  opaque subtree/namespace/sequence controls for properties/settings/equations
  and dependency imports, malformed source refusal without partial mutation.
- Architecture: HLD03 dependency graph, native shared validation has no CLI
  dependency and neutral oxml crates gain no facade dependency.
- Public API: HLD10 exact new types/signatures, pre-1.0 source impact declared,
  no new file/module/trait, verified locally patched workspace dry-run packaging
  with --allow-dirty for scoped work and all archives below10MiB.
- PyO3: fresh installed full declared suite, strict mypy/stubtest and both WASM
  checks. Binding crates remain excluded from Rust workspace test binaries.
- Layout: HLD08 page-count/equation controls use deterministic fonts. Reuse
  existing page geometry and F-X195 caller-font selection, no system-font
  baseline or cached producer count substituted for actual layout.
- External oracle: differential-testing skill, pinned Word/LibreOffice/converter
  versions and exact bounded input/output receipts. Corpus parity compares the
  same immutable package through native CLI and current Python extension.
- Archive carriers, if changed source inventory requires them: HLD15 Packaging
  and release command, actual local affected records only, same tolerances and
  inventories, no publication or manifest/version changes.

## Hash harness

Expected49 existing deterministic outputs unchanged for binding exposure and
shared validation/text refactoring. Keep native template/import semantics.
Issues332/334 below explicitly amend Markdown and fresh compatible creation
semantics, with separate measured reviewed behavior deltas. If another native
defect requires behavior changes, record its
separate approved expected delta and exact affected entries before proceeding.
Neither parity wrappers nor diagnostic projection justify an unexplained hash
delta or a second baseline writer.

## Exclusive file claims and integration

Proposed native existing document.rs/lib.rs plus only necessary existing
content_control/math/export modules. Shared validation and text extraction
touch existing rdocx-cli commands.rs and its integration entrypoint, after
X196 integration and remediation. Python existing document.rs/paragraph.rs/
lib.rs, _rdocx.pyi and current test/typing entrypoints. Final exact claim list
follows public API decisions, with no new source files. Serialize document.rs,
binding registration/stub and HLD overlaps with X200/X201/X202. X204 consumes
these document operations and owns broader one-shot commands. Workers never
write sprint totals. Fresh dependency-prefix source is the test Base.

## Implementation checklist

- [x] Integrate reviewed X196/X201 and resolve final count/text/validation scope.
- [x] Freeze typed signatures, diagnostics and exact source ownership.
- [x] Prove each missing installed Base operation and corpus parity failure.
- [x] Share native CLI validation/text without changing its report semantics.
- [x] Bind exports/templates/checked assembly/properties/equations/controls/settings.
- [x] Fix Issue332 in the shared native Markdown emitter with preservation controls.
- [x] Set Issue334 fresh WordCompatible mode15 without normalizing imported documents.
- [x] Prove source preservation, revision and malformed atomic controls.
- [x] Run complete sample-corpus parity and declared runtime/typing/oracle gates.
- [x] Run scoped hash, package, repository-policy, WASM and risk riders.
- [x] Finish independent zero-defect, zero-smell review, HLD and valid handoff.

## Open questions

Resolved. The sections below define count scope, ordered diagnostic records,
checked insertion, live metadata, display equations and CustomFileTime.
Use the exact to_odt/to_rtf/to_epub signatures above with typed ExportResult,
not bare bytes or truncated strings. Undefined property dates stay inspectable
without normalization. F-X212 owns the qualified paragraph_visibility scanner
and is a formal prerequisite. Consume that Result API once, rather than
extracting a second F-X196 scanner. Dependency completion and installed Base
reproductions are implementation gates, not reasons to leave resolved design
questions open. Shared document.rs and CLI validation edits are serialized
with F-X201/F-X212 and the later F-X204 consumer.

## Resolved document operation contracts


Keep `Document.word_count() -> int` identical to native whitespace token counting over accepted main-body paragraph visitation. Native document.rs:29025 sums each paragraph's split_whitespace count, including tables and typed controls and excluding removed rows. Do not replace this with tokenizing concatenated text, since accepted paragraph joins can change tokens.

Define `Document.character_count(*, include_spaces: bool = True) -> int` as the sum of Unicode scalar values in the same accepted paragraph text visitation. True includes actual text whitespace. False excludes every character satisfying Rust char::is_whitespace, explicitly including tabs, line breaks and nonbreaking spaces. Combining marks count individually, supplementary-plane characters count once, and no grapheme or UTF-16 interpretation is claimed. Synthetic paragraph newlines, table tabs and story labels do not count. Native text formatting inserts these separators at document.rs:11775, document.rs:11801 and document.rs:11809, so counting Document.text() is unsuitable. Add exact scalar/whitespace examples to the stub and HLD.

`Document.text() -> str` remains the new CLI presentation parity operation, not the source for those body counts. `Document.text_report() -> TextReport` exposes immutable text, scope Literal['main','all-supported-stories'], revision_view Literal['accepted'] and ordered diagnostics. Retain exact CLI text labels, grouping and newlines, commands.rs:206. On failed other-story extraction, return main text and one omission diagnostic with the same detail used by the CLI warning, commands.rs:230. CLI consumes the shared result and writes its diagnostic to stderr. No all-story count API is silently added.

`Document.page_count() -> int` uses fresh deterministic native layout and returns actual layout pages. Producer app-properties pages are separate cached metadata. Final font-option signature must reuse integrated F-X195 options rather than inventing a second font selection rule.

## Diagnostic result names

Use format-specific concrete immutable result types:

- `OdtExportResult.bytes: bytes`, `diagnostics: tuple[OdtDiagnostic, ...]`, each diagnostic path/message.
- `RtfExportResult.bytes: bytes`, `diagnostics: tuple[RtfDiagnostic, ...]`, each diagnostic offset/destination/message.
- `EpubExportResult.bytes: bytes`, `diagnostics: tuple[EpubDiagnostic, ...]`, each diagnostic path/message.
- `MathConversionResult.value: str`, `diagnostics: tuple[MathConversionDiagnostic, ...]`, each diagnostic path/message.
- `TextDiagnostic.message: str`, retained native extraction detail. TextReport diagnostics are immutable ordered values.

Important correction: native RTF diagnostics are NOT path/message records. rtf.rs:33 has offset:usize and destination:Option<String>. Native odt.rs:46, epub.rs:51 and math.rs:30 use path/message. Do not discard or synthesize RTF source location fields to fit a generic ExportResult. The bindings expose existing native results using these exact public Python names, with no new Rust generic or forwarding module.

Signatures: `Document.to_odt() -> OdtExportResult`, `to_rtf() -> RtfExportResult`, `to_epub() -> EpubExportResult`. Keep errors as RdocxError and diagnostics as successful-result data.

## Validation input and ownership

Proposed native existing-file surface:

```rust
pub enum ValidationSeverity { Error, Warning }
pub struct ValidationFinding { pub severity: ValidationSeverity, pub message: String }
pub struct ValidationReport { pub findings: Vec<ValidationFinding> }
impl ValidationReport { pub fn is_valid(&self) -> bool; }
pub fn validate_document_bytes(bytes: &[u8]) -> Result<ValidationReport>;
pub fn validate_document_path(path: &std::path::Path) -> Result<ValidationReport>;
impl Document { pub fn validate_sections(&self) -> Vec<ValidationFinding>; }
```

These signatures are proposals only. X201 owns the minimal report types and checked section helper. X203 owns the full CLI assembly relocation and bytes/path entrypoints, using the X201 helper without a second section scanner. Native facade never depends on CLI. Existing document.rs/lib.rs are sufficient, with no new module.

Python module signatures: `validate_bytes(data: bytes) -> ValidationReport` and `validate_file(path: str | os.PathLike[str]) -> ValidationReport`. Also `Document.validate() -> ValidationReport` over a serialized immutable snapshot. bytes must not require an opened Document. Path conversion is explicit and does not invoke arbitrary object stringification. Snapshot serialization uses native to_bytes staging (document.rs:14674), not mutations of the source revision or handles.

Preserve actual CLI failure boundaries. OpcPackage::open failure propagates at commands.rs:2188, so non-ZIP/failed archive decoding and path I/O remain RdocxError. Once package inspection succeeds, failed Document opening adds the existing error finding at commands.rs:2290 only if independent package errors are already present. Without those errors, it remains Result Err. This supports malformed XML and missing-owner findings without claiming every arbitrary byte sequence produces a report. Preserve exact messages and deterministic ordering of all existing error groups, then warnings. A Warning does not make is_valid false. CLI alone formats stdout/stderr and exit status. Bytes input and equivalent path input must agree on findings where both decode.

## Checked whole-document import

Bind the actual concrete FragmentConflictPolicy struct, document.rs:1036, with reuse_equivalent/rename_all and three independent immutable-returning toggles at1044,1053,1062,1068,1074. None selects native Default at1092. Do not invent fail or replace these controls with style-only policy.

Proposed `DocumentFragment.from_range(document, start: ContentLocation, end: ContentLocation, *, include_final_section_properties: bool = False) -> DocumentFragment` directly reuses native document.rs:1144. It captures a nonempty half-open range of direct items from one checked story. Section-inclusive closure is allowed only at the final main-body boundary. Fragment remains owned and usable independently of later source edits. Existing native ContentLocation::new/end are at1278/1288.

Proposed `Document.import_fragment(destination, fragment, *, policy=None) -> None` directly reuses checked native import_fragment at18411, which stages the supported dependency closure and reopens before publication at18458. Do not wrap infallible insert_document at26198, which calls expect and clamps.

For `insert_document(other, *, at: int, policy=None)`, add a minimal checked native `try_insert_document(&mut self, index: usize, other: &Document, policy: FragmentConflictPolicy) -> Result<()>` only if whole-body fragment capture cannot express the complete existing insertion contract. Define at as physical direct body-child index, including preserved body items, not the filtered paragraph index. Accept 0 through body length inclusive and reject larger/negative Python indices. Destination end remains before final sectPr. Ordinary insertion does not silently import final source sectPr. Section-owning input content and empty source need explicit regression fixtures before choosing their compatibility behavior. This is a real implementation question: document.rs:26248 currently clones body content and merges styles/numbering, while the checked fragment route has stronger dependency and section-owner rules. Do not claim the two are already equivalent.

## Custom properties and date policy

Native CustomPropertyValue::FileTime is String, custom_properties.rs:34. Parsing reads its text at307, and writing returns that text at403. There is no native calendar/timezone validation or UTC normalization. This is the actual native policy and should remain unchanged for existing packages.

Choose a concrete `CustomProperty` immutable read snapshot carrying name, pid, fmtid, type and value, alongside a document-owned CustomProperties collection proxy. `get(name) -> CustomProperty | None` avoids conflating a missing property with native Empty. Expose unsupported Raw values as read-only bytes with explicit type, never silently map them to None or ordinary strings. Preserve lpstr versus lpwstr when updating an existing string, PID, fmtid and ordering. New strings use Lpwstr and a newly available PID >=2. Native set_custom_property validates name/PID collision, document.rs:25965, but does not preserve those metadata automatically if passed a freshly constructed replacement.

For newly authored dates, prefer an explicit `CustomFileTime(value: str)` with a documented checked RFC3339 authoring subset and no timezone conversion, rather than accepting datetime implicitly. Require an explicit Z or numeric offset, valid Gregorian date/time, optional fractional seconds, no timezone-naive value and no leap second. Preserve accepted lexical offset/fraction spelling when written. A preexisting native FileTime outside that authoring subset remains inspectable and unchanged by unrelated writes. This adds Python authoring validation, not a claim that native String already validates dates. The exact CustomFileTime name and grammar are selected by this design revision. If datetime convenience is requested later, its timezone/precision normalization is a separate explicit conversion, not hidden in property access.

Bool dispatch precedes int, I4 is signed32-bit, R8 rejects nonfinite newly authored values, unsupported object conversions fail before mutation. Missing deletion is a no-op. Explicit native Empty and Raw need distinct reading representations.

## App-properties ownership

Choose a document-owned AppProperties proxy, following the existing CoreProperties proxy pattern at rdocx-py/document.rs:1305. Each setter borrows the owning document, clones the latest native model or uses default only when absent, changes one supported field and invokes native set_application_properties. That preserves private child_order/extra_xml/extra_namespaces from app_properties.rs:152, and avoids stale whole-snapshot overwrite.

Expose supported native fields with their actual Optional types, app_properties.rs:130. Cached pages/words/characters are metadata, never calculated counts. Getter access does not create a part. Assigning the same value is a no-op. A failed application_version shape is rejected by native document.rs:25919 without publication. Metadata writes retain existing nonstructural Python handle usability, matching CoreProperties behavior at rdocx-py/document.rs:1321. Do not expose a detached mutable model as though mutations reached the document.

## Equations and content-control boundaries

Proposed `Paragraph.add_equation(source: str, *, format: Literal['latex','mathml']) -> EquationInsertResult`, where the immutable result has equation:Equation and diagnostics:tuple[MathConversionDiagnostic,...]. Returning only Equation would discard native import diagnostics, math.rs:115/182. Convert before mutable document borrow, then append through paragraph-owned native add_equation at paragraph.rs:981. Initial convenience insertion creates inline OfficeMath using native OfficeMath::inline, rdocx-oxml/math.rs:227. Do not invent display conversion by treating source whitespace as layout intent. Any display convenience needs an explicit display parameter and native supported shape.

`Paragraph.equations() -> tuple[Equation, ...]` follows native source order, paragraph.rs:960. Equation is an immutable owned snapshot, not a mutable detached proxy. `Equation.to_latex()/to_mathml() -> MathConversionResult` retain diagnostics from the existing native conversion. A native display equation may contain multiple CT_OMath children, so conversion must preserve each equation boundary or report a bounded diagnostic rather than silently selecting its first child. The follow-up below resolves display conversion with an ordered per-equation result.

`Document.content_controls() -> tuple[ContentControl, ...]` means every typed main-body control in native recursive document order, including paragraph, table and nested controls. Native content_control.rs:69 delegates CT_Body inventory. rdocx-oxml/document.rs:2671 skips raw body XML and enumerates typed controls. It covers typed main-body controls. Related stories and accepted-revision filtering are separate native capabilities. Return immutable tag/alias/id/type/text snapshots. Native by-tag/by-alias setters at content_control.rs:95/102 retain duplicate-selection, custom XML binding and atomic refusal semantics. Do not expand all-story control discovery silently under Python parity.

## Evidence and remaining decisions

No native Word, converter or Python oracle was run for this appendix. References describe current source only. The concrete insertion and display conversion decisions are resolved by the follow-up below. The contracts below supersede the initial singular equation result and conditional insertion proposal above. Source-wave ownership must still be reconciled with X201/X202 before approval.

## Source receipt

Canonical read HEAD: `33ec5f18e83b71028d0b8e33f233ef07c4f4965c`. Working-source hashes:

- `crates/rdocx/src/document.rs`: `3b8eec9fc3980a9dc743be2faef39d1126c819be102a61b58a4f6ae798a59620`
- `crates/rdocx/src/content_control.rs`: `c240eddc91a81cd833afa5ebfc1cf7c27c987801824faf5d4457278d55ff3c0c`
- `crates/rdocx/src/paragraph.rs`: `83198917fce911d18c1cdcf732c9f6e992feae279cf3092f1c43a48084a6da0c`
- `crates/rdocx/src/math.rs`: `fbe4314cf25a23ff48aac68419ea3e83cc081878f5b325f474df668812842cab`
- `crates/rdocx-cli/src/commands.rs`: `72b89954de46fda52b2287e3914f84230ec354d798999aa457ebfc57c3c1b6a4`
- `crates/oxml-core/src/custom_properties.rs`: `201fdf81d0615c2cc2d2a64e6f09bab1eadef72b0af879a2710fc46848aaa8cf`
- `crates/oxml-core/src/app_properties.rs`: `176cd6dae588964906821d7e1d5a79e578acac1ddb2c2162aa76652aae9f3daa`
- `crates/rdocx-py/src/document.rs`: `07566b1b259fe7fc736e22e842b999d898907db8d3cd1da2f14832bc207e5aa3`

## Follow-up: checked whole-document insertion resolved

Choose one checked native `try_insert_document(&mut self, index: usize, other: &Document, policy: FragmentConflictPolicy) -> Result<()>` in existing document.rs. Bind Python insert_document to it, never to the existing infallible wrapper. The helper composes the existing checked capture/import route. It does not duplicate dependency import or use the old staged clone-and-style merge as a fallback.

1. Resolve the destination physical body owner and its direct content list. Define index as that list's ordinal, excluding final body sectPr but including preserved direct children. Reject index greater than the direct count even when the source is empty. Map a nonfinal direct ordinal to the corresponding full projected scanner location, not `ContentLocation::new(..., vec![index])` blindly. `validated_content_boundary` at document.rs:7494 indexes scan_story_items first and only then translates to direct ordinal. Nested controls can make these indices differ. Use ContentLocation::end for index equal to count.
2. Obtain the source's staged authoritative main body and direct content list using existing internal scanner helpers. An empty list is a valid no-op after destination/source preflight. Return without importing unused styles/parts, changing source or destination bytes, incrementing revision, or invalidating handles. Existing from_range intentionally requires a nonempty range, document.rs:1169, and package_authoritative_body_fragment rejects an empty list at document.rs:6952. Do not broaden either public primitive solely to express this no-op.
3. For a nonempty source, capture its first direct item through ContentLocation::end(body), with include_final_section_properties=false, via DocumentFragment::from_range at document.rs:1144. Source remains unmodified because capture uses staging. Import with the requested complete native policy using import_fragment at document.rs:18411. That route stages closure and reopens before a single publication.
4. Preserve paragraph-owned and nested section particles already in selected body blocks. Import the related references through the existing dependency closure, which scans the selected authoritative content, field.rs:3240, and records its referenced internal parts at field.rs:3270. Do not strip paragraph sectPr to make insertion simpler. Unreconcilable schema/dependency ownership is an atomic checked error from the existing route, never a panic or legacy merge fallback.
5. Ordinary insert_document excludes the source's terminal body sectPr. The destination terminal body sectPr remains final. To explicitly include the source final section, callers use DocumentFragment.from_range(..., include_final_section_properties=True) and import_fragment. Existing native package_authoritative_body_fragment at document.rs:6971 wraps an included final section in a new paragraph's pPr, retaining the destination final section. Native field.rs:3192 performs the corresponding typed dependency projection. This existing explicit API avoids another insert_document flag or ambiguous geometry replacement.

Existing source test regression_test.rs:37645 constructs a final-section-inclusive fragment and asserts that import yields two sections. This is source evidence only, not a newly observed test pass. Add checked insertion regressions for genuinely empty body plus terminal sectPr, empty invalid destination index, sections-only source no-op, internal paragraph section with header/footer dependencies, insertion before/middle/end, preserved direct children and nested controls whose projected positions differ from direct ordinals, unsupported closure rollback, source byte equality, and success versus no-op handle revision behavior. The Issue306 convenience operation remains insertion of selected body content rather than an implicit replacement of destination geometry.

## Follow-up: display equation conversion boundaries resolved

Keep Paragraph.equations inventory one snapshot per native OfficeMath occurrence. Add immutable `Equation.is_display: bool` and use one boundary-preserving result shape for both inline and display conversion:

```python
Equation.to_latex() -> EquationConversionResult
Equation.to_mathml() -> EquationConversionResult
EquationConversionResult.is_display: bool
EquationConversionResult.equations: tuple[MathConversionResult, ...]
EquationConversionResult.diagnostics: tuple[MathConversionDiagnostic, ...]
```

For inline OfficeMath, equations has one result. For display OfficeMath, it has one result for each CT_OMath child in native source order. CT_OMathPara stores that ordered Vec at rdocx-oxml/math.rs:113 and pushes each parsed oMath at153. No flattening, implicit separators, first-child selection, equation-array reconstruction or joined TeX string is needed. This corrects the earlier singular Equation.to_latex/to_mathml -> MathConversionResult proposal while retaining MathConversionResult as the concrete per-child value/diagnostic record. Native converters at rdocx/math.rs:137 and200 remain unchanged.

For each child, pass its cloned expressions through MathArgument::new and existing converters. Keep converter diagnostic paths unchanged within their child's result, where tuple position identifies the boundary. This expression projection alone loses CT_OMath-level preserved attributes or raw siblings, because CT_OMath owns private Preservation independently of its expressions, rdocx-oxml/math.rs:37/65. Before projection, clone the child and clear expressions, then call its public has_unsupported_content at83. If that isolated container still retains content, attach an explicit conversion loss diagnostic to that child's result. Existing math.rs:1436 already uses the analogous clone/clear/check pattern for MathArgument. Do not claim that cloning expressions preserved the outer container.

At the display wrapper, clone CT_OMathPara and clear equations. Its public has_unsupported_content at175 detects opaque wrapper/property content independently of child content. Emit a wrapper-level diagnostic if that remainder has unsupported content. Also emit a wrapper-level layout-loss diagnostic when its typed justification is present, since canonical per-child MathML or LaTeX export does not represent native oMathParaPr group justification, rdocx-oxml/math.rs:1144. The is_display flag retains display ownership classification without pretending these plain conversion values preserve all layout metadata. A read-only Equation snapshot retains the complete cloned OfficeMath for native document preservation. Conversion never rewrites its owning paragraph or strips stored OMML.

Existing invalid/default empty display is not normalized into one empty equation: CT_OMathPara serialization rejects no-equation display at rdocx-oxml/math.rs:189. For a nonempty display, every child still gets a result even if a bounded converter returns empty text and diagnostics. This preserves cardinality as well as ordering.

Paragraph.add_equation source conversion remains `-> EquationInsertResult` with import diagnostics and one inline OfficeMath child. Existing native conversions return a MathArgument, while OfficeMath::inline consumes its expressions at rdocx-oxml/math.rs:227. No new multi-equation authoring syntax or display inference is introduced. Initial insertion's format selector remains latex or mathml. A display authoring extension is outside this binding parity convenience, but reading and converting existing multi-equation display content is covered completely by the tuple result.

Add source-built tests for two distinct display children including repeated identical children, typed group justification, opaque wrapper attributes/children, opaque CT_OMath-level siblings, nested expression diagnostics, one failed bounded conversion among successful siblings, unchanged source bytes after export, insertion diagnostics and save/reopen equation order. No oracle or test has run for this design follow-up.

## Shared validation coordination receipt

Fonts agent's `.claude/scratch/F-X201-authoring-decisions.md` proposes the same minimal native report types and section helper. Its proposed incomplete explicit pgSz warning and zero/negative explicit dimension error are X201-owned. X203 consumes them in the existing errors-then-warnings parity projection, retaining the CLI archive-open error boundary. Root reconciles exact stable messages and source claims. No duplicate section implementation or facade-to-CLI edge is proposed.

## Approved provider boundary

The selected operations stay in existing native document.rs/lib.rs, existing
Python document/property projections and current CLI source/test entrypoints.
No proposed contributor inspection.rs file is authorized. Provider completion
is required before the source claim. Native typed export diagnostics retain
path, code and loss information through the installed projection.

## Complete PR325 and advanced-head reconciliation

PR325 closes Issue306 and attaches to this existing F-ID. Complete original
headfbeb5a5b4eaea022fb8c24d21862b9fa2cd57a61 and advanced head
87d4b71c1ed211ecf34156d39f18a69cb4fd44ad are captured in
/private/tmp/S91-PR325-intake and /private/tmp/S91-PR325-87d4b71-intake.
All26 cumulative feature paths are unchanged by the advance, which only
classifies inspection as operations in the existing workflow test. No feature
recapture or duplicate implementation is required. Retain useful shared
CLI/native text and validation work, but implement the approved typed export
reports, qualified physical owners, checked insertion and live property
contracts. Bare export bytes and truncated warnings do not satisfy those
contracts. Adapt useful extraction inside existing files without adopting
contributor inspection.rs. Formal X212 completion supplies paragraph visibility.
Root's delegated verification authenticated both complete captures and Git
blob identities, including all26 unchanged head blobs against the new tree.

## Checkbox extension from stacked PR330

PR330 head274e3ca82b3db0393f0b3e0833b640ae2bbf9dc8 includes an additive
checkbox extension over this feature's authenticated PR325 ancestor. Retain
its shared native/Python control operation here once, alongside the already
approved typed property and tag binding operations. The assessment is in
/private/tmp/S91-PR330-intake/assessment.md. No second CLI control model or
new feature is created.

The contributor writer at rdocx-oxml/src/content_control.rs:268 and:275
selects a raw prefix and removes children by local name. Adapt it to the
qualified Word2010 checked namespace, retaining foreign same-name children,
attributes and opaque order verbatim. Bind checkedState/uncheckedState font
and scalar together instead of dropping the state font at native
content_control.rs:193/:199. Preserve actual supplied-face precedence and
source XML, using X195/X210's shared qualified font semantics. Default
U2612 and U2610 are in F-X210's selected3968-byte subset, explicitly
approved by the user on2026-10-10. This PR does not supply separate approval.
Tests must
cover foreign namespaces, distinct state fonts, explicit custom state glyphs,
unsupported control kinds, live owner identity and late atomic refusal.

## Issues332 and334 native provider amendments

[Issue332](https://github.com/tensorbee/rdocx/issues/332) and
[Issue334](https://github.com/tensorbee/rdocx/issues/334) are assigned once
to F-X203. Their read-only source assessment is retained in
/private/tmp/S91-Issues332-334-assessment/assessment.md. They are explicit
native behavior changes, not parity-only wrappers or a repeat of completed
F-X193 inbound HTML mark inheritance. No material question remains.

Issue332 fixes the existing rdocx-html/src/markdown.rs emitter. Move leading
and trailing text whitespace outside owned emphasis markers, preserving
original text. Whitespace-only segments stay undecorated. Coalesce adjacent
segments with equal emitted emphasis within the same link/context, without
merging changed targets, field-format boundaries, hard/page/column breaks
or accepted-view structural boundaries. Preserve hyperlink sanitation,
accepted-view control/deletion rules, tabs and two-space hard-break syntax.
Insert a block separator before a nonempty emitted table following a list,
including accepted content-control tables. Consecutive list items stay one
list and omitted empty tables do not manufacture a block boundary.

Existing test carriers must cover the three original from_html examples and
independently authored run splits. Require intended emphasis, unchanged
logical text/spacing, equal/mixed emphasis, link boundaries, cached fields,
hard breaks and control/list/table transitions. Correct the existing
whitespace-inside-italic field expectation intentionally, retaining its
logical-text and break controls. Bare HTML and source/package snapshots
remain unchanged. Native, Python and CLI exports use the same emitter.
This does not add a full CommonMark implementation or a parser dependency.

Issue334 initializes exactly one qualified compatibilityMode with office
URI and value15 in every freshly authored WordCompatible profile, across
its supported document/template/macro package classes. Use existing typed
settings and set_compatibility_setting before initial serialization in
Document::new_with_profile. Document::new and from_html inherit that policy.
Do not change CT_Settings::new globally or alter Minimal creation. Imported
absent/14/15/16 modes and opaque compatibility facts retain their source
semantics. Explicit caller removal or override remains honored on save.
The existing modern-versus-legacy layout branch remains the single policy.

Test fresh profiles and HTML creation through save/reopen, exactly one
mode15 setting, settings schema order and unchanged package ownership.
Test Minimal, imported absent/legacy/future modes and explicit removal/
overrides separately. Prove modern layout selection for the fresh profile
and retained legacy selection for imported absent/14 modes. Use existing
deterministic table/margin/tab/footnote controls and bounded pinned native
Word opening evidence. Contributor title-bar observations are not a local
Word acceptance result. No automatic promotion of imported fixtures.

Reserve existing rdocx-html/src/markdown.rs and lib.rs in addition to the
existing document/settings and test carriers. HLD03/08/10/12/14 updates
cover this provider ownership, fresh-profile selection and acceptance.
Each issue lands as its own labelled native behavior commit, separately
from binding exposure and each other. Measure exact affected hash entries
and review Markdown/XML/layout/PDF/PNG deltas with deterministic fonts.
Issue334 may change multiple freshly authored samples. No blanket49-entry
unchanged claim applies to these fixes. Preserve all unrelated outputs,
derive actual package inventories/archive rows and retain root's exclusive
baseline lease. No new file, module, dependency or test binary is required.

## Issue339 and PR342 all-story text amendment

Resolve [Issue339](https://github.com/tensorbee/rdocx/issues/339) once with
[PR342](https://github.com/tensorbee/rdocx/pull/342), immutable head
d3dc522510e1250da441ba360601619a29d40103. Complete source assessment is
/private/tmp/S91-PR342-intake/assessment.md, SHA256
46b51f8abec9e4b70e067e75da116a7367dd726e1309957a70d593c760fed6c2.
Reuse qualified direct-run tab/br/cr reads, same-paragraph nested field
visibility and staged visible-control replacement from reviewed source hunks.
Do not replace whole document carriers or copy archive measurement tuples.

All native, CLI plain/JSON and installed Python text views agree with the
accepted Paragraph projection across body, headers/footers, notes/comments,
text boxes, tables and admitted controls. Retain paragraph tab stops and
opaque branches. Cover simple/complex field page/column breaks, start/end
control spellings, nested IF/PAGE instructions and accepted deleted/moveFrom
visibility. F-X201 owns physical story lifecycle. F-X212 owns accepted typed
cache visibility. F-X211 owns the complete cross-paragraph field graph. This
patch alone does not establish that graph. Reconcile their evidence once.

Add controls in existing test carriers for ancestor aliases/default namespace,
local rebinding, foreign same-name elements and type attributes, unknown
attributes/children/entity spellings, repeat/no-op and save/reopen. Replacement
serializes real owned tab/br controls and preserves hidden or instruction
controls. Empty-source insertion retains actual namespace closure. Stale,
invalid or malformed locations refuse atomically with exact source/package
equality. Reproduce public Base before implementation, then execute the
contributor's five native selectors, CLI and Python selectors plus uncovered
controls. Run current installed provenance, typing and applicable risk gates.

HLD03/04/10/12/14 updates describe shared accepted text ownership and source
retention. Tab/break output, nested instruction suppression and visible-control
replacement are explicit native behavior changes, each attributed in reviewed
commits and measured deterministic output deltas. Baseline ownership remains
exclusive. Original Issue306/332/334 acceptance remains required. No new
product file, module, dependency or public mutation path is required.

### Refreshed PR342 contribution, 2026-10-11

PR342 advanced to `9fd0b8da374b8c641e7ab89c0bd3d76bd2f4519a`.
The complete immutable update is assessed at
/private/tmp/S91-PR342-intake/head-9fd0b8da/assessment.md, SHA256
b88a9794b53cf405c52e60eb71dd900829a29204f9b9ad92ce47433fbae65cdb.
Reuse the qualified item-local incomplete and deleted field handling once
within F-X203. Add the contributor native selector
`set_story_text_replaces_the_text_of_unclosed_and_deleted_fields` to the
existing local execution inventory. CLI and Python test carriers did not change.
The approved acceptance and dependencies remain unchanged. Simple-field
page and column breaks still need deciding controls. F-X211 retains complete
cross-paragraph field ownership. Preserve every original namespace, source,
refusal, all-story and installed-consumer acceptance check. Contributor archive
measurements do not replace locally verified package evidence. No source
adoption, runtime acceptance or feature completion is asserted by intake.

## Rolling intake, PR354

The 2026-10-11 19:57 UTC refresh found [PR354](https://github.com/tensorbee/rdocx/pull/354) at head5a2c657eb705ab53048ab5d4e729b9014d6e0c0e. Its compatibility-mode and CommonMark export fixes address Issues332 and334 already assigned to F-X203. Reuse this contribution within that existing story, with no duplicate implementation owner. The exact retrieved patch is c4814b20267e16c75dd36a972a0e87eb7b8e2c07a3889614315125fc2a2f9b0d at /private/tmp/S91-PR354-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR354-intake/assessment.md, SHA7c7dab495057d1bcc0920e94af90231c6aaab76c068f3b1e6780f46c8807d19a. Consumer reconciliation remains pending. Preserve original edge whitespace and terminal hard breaks, retain field boundaries, and change list state only for emitted nonempty table output. Reuse the constructor and Markdown hunks without copying contributor archive numbers. The approved original issue acceptance remains required, including cases the PR explicitly leaves out. Contributor results do not waive local pinned oracle or output-delta review gates.

Intake now contains40 open issues and23 open PRs. Issues264 and281 remain excluded discussion records. No issue or existing PR head changed in this refresh. This intake does not mark F-X203 implemented or complete.

## Existing JSON conversion dependency

The binding's concrete Python JSON converter uses `serde_json.workspace = true` in the existing rdocx-py manifest. The workspace and native facade already use this package. Only the binding dependency edge and corresponding generated lockfile dependency list change. Existing package versions, checksums and features remain fixed. Conversion reads actual dict/list/tuple containers through their native Python storage, checks bool before int, refuses cycles, nonfinite floats and out-of-range integers before any publication. It does not invoke arbitrary stringification or custom object getters.
