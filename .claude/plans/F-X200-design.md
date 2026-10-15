# F-X200, Complete Python Word formatting and list parity

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X197
**Issues**: 303 and321, captured in S91-intake-issues.json on 2026-10-09
**Prerequisite**: F-X197
**Implementation**: not claimed, no source changes or new gate evidence

## Problem

Issue303 requests installed Python parity for everyday Word formatting that
mostly exists in the native facade. The complete acceptance inventory is:

- Run.font superscript, subscript, small_caps, all_caps, character spacing,
  double_strike and hidden.
- Font.language, East Asian and complex-script font names, and rtl.
- Paragraph.paragraph_format.tab_stops with add, clear and ordered listing,
  including alignment and leader.
- Paragraph borders top, bottom, left, right and between, and shading.
- Paragraph outline level and bidi.
- add_style and set_style alignment, line spacing, underline, keep-with-next,
  keep-together, page-break-before, borders, shading and tab stops, extending
  existing font, size, bold, italic, colour, spacing and indent arguments.
- Document default font and size in w:docDefaults, without restyling Normal.
- add_bullet_list_item and add_numbered_list_item with level.
- List restart and start overrides through a new instance of one definition.
- Checklist glyph presets for unchecked and checked bullets.
- Both bindings' shared colour input and nested font.color format contract,
  delivered once by F-X197, including one conversion in oxml-py-support.
- Getter and setter save/reopen parity, meaningful render/PDF evidence,
  handwritten stubs, strict mypy and stubtest, python-docx names where they
  exist, and accurate HLD02 Python capability rows.

Google Docs import is best effort. Author w:vertAlign, real w:tabs, plain
w:numPr numbering and supported paragraph borders. RGB assignment writes a
literal colour. Do not invent a theme-only colour from RGB. Issues264 and281
remain excluded. The prerequisite may reuse their existing native work.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Python API shape and checked handle invalidation.
- `docs/hld/02-scope-and-non-goals.md`, native and Python capability rows.
- `docs/hld/04-opc-and-packaging.md`, The package and owned relationships.
- `docs/hld/08-rendering-spec.md`, Text in a shape, Tables and deterministic output.
- `docs/hld/14-development-backlog.md`, F-X200 and the full linked issue acceptance.

## Approach

Extend FontSnapshot, FontUpdate and PyFont in
crates/rdocx-py/src/formatting.rs:62, :103 and :126. Preserve its existing
AcceptedRunPath and ParagraphLocation resolution through run_snapshot:178
and apply_run_update:205. Adding one run property must work in accepted
control, table and body runs, not just direct paragraph children.

Native Run setters already cover language at run.rs:727, script font slots
at :685, language slots at :1013 and :1021, rtl at :985, character spacing
at :909, super/subscript at :887 and :898, and caps, strike and hidden at
:854 through :942. RunRef has corresponding property projections for many
fields, including vert_align:1362 and slot_font:1378. Some builder-era
methods accept only bool or unconditionally set a value. Add the smallest
Option setter/getter pairs needed for direct inheritance-aware Python
properties. Do not turn None into false. Validate spacing range before
conversion and preserve canonical truncation toward zero.

Extend ParagraphSnapshot, ParagraphUpdate and PyParagraphFormat in the same
binding file at :378, :432 and :457. Native Paragraph already has
set_border_value at paragraph.rs:1992, set_shading_value:2052, tab_stop:2067,
set_tab_stop:2081, remove_tab_stop:2108, clear_tab_stops:2128 and
set_outline_level_value:2240. ParagraphRef exposes border:3112 and bidi:3185.
Use those checked paths. Any missing immutable projection or clear operation
is a concrete native gap to fill, not permission to bypass the facade.

Styles are configured in document.rs:2751 and :2871. Native StyleBuilder
paragraph_properties at style.rs:415 and run_properties:463 accept the
existing concrete CT_PPr and CT_RPr. Stage a single checked property update
per style operation, preserving imported unsupported attributes and children.
Do not add a generic property application framework or many forwarding
wrappers. Validate the entire input before creating or mutating the style.

NumberingLevelOverride already exists at native document.rs:29971 with
level, start and optional replacement. add_numbering_instance:22207 stages
and validates its override slice. Python document.rs:2977 currently always
passes an empty slice, so restart/start is an exposure gap. ListLevel and
existing PyListLevel supply checklist glyph configuration. The native
one-call list helpers exist at :21809 and :21868, but their allocation paths
contain expect and the level is not validated before allocation. Check the
requested level before invoking them or provide a checked native operation
if required for imported malformed graphs.

No public native Document default-font/default-size operation was found.
doc_defaults currently appears in style resolution, including style.rs:977
and :1011. Add a small checked native read/update projection over the
existing defaults model. It must preserve paragraph defaults, unrelated
run defaults and unknown XML. Normal and explicitly configured styles remain
unchanged. This item is not merely a binding wrapper.

## Proposed Python contracts to settle in design

Use inheritance-aware Optional properties for direct font booleans and
language/font-name slots. superscript and subscript share one w:vertAlign
source, so setting one true replaces the other. Setting one false must not
clear a different active vertical mode. None clears direct formatting.
Character spacing should accept documented Length and return Length,
without an undocumented point or EMU reinterpretation.

Expose a live tab_stops collection on paragraph_format. Follow the requested
python-docx spelling add_tab_stop(position, alignment, leader). Iteration
returns typed positions, alignment and leader, ordered as the actual source
requires. The draft does not prescribe a new file or generic collection.
A new concrete collection class inside existing formatting.rs is justified
by the named public API, but its exact mutation invalidation needs review.

Border and shading methods should reuse current table/cell naming where
python-docx has no public counterpart. Their colour arguments use F-X197's
one shared validator. Preserve auto only where the existing native contract
supports it. Unset getters do not create empty property nodes.

Extend add_numbering_instance with explicit overrides and a convenience
start/level form only if it reduces actual caller work. Reject conflicting
forms rather than guessing. The final design must state its exact signature.
Checklist presets produce real bullet definitions using the supplied glyph,
not ordinary text characters preceding the paragraph.

## Shared ownership and exclusive paths

F-X197 owns colour extraction, RGBColor behaviour, colour formats, theme and
brightness semantics, their stubs and colour contract documentation. X200
only consumes the integrated shared implementation for its newly exposed
paragraph/style colours. It does not duplicate an extractor.

Proposed exclusive implementation paths are existing rdocx-py formatting.rs,
document.rs and current handwritten stubs/enums, plus native run.rs,
paragraph.rs, style.rs and document.rs only for demonstrated missing checked
operations. Existing Word test_formatting_tables.py, test_core.py and typing
entrypoints supply the regression locations. Other stories use native
document.rs, so ownership must be claimed and edits serialized or integrated
before this worker starts. No tracked file is created by this proposal.

## Rejected alternatives

- Duplicate prerequisite colour/table APIs instead of reusing their actual result.
- Treat the issue's claimed native parity as proof where source inspection shows gaps.
- Drop unknown source or stale-path checks to make the missing API appear usable.

## Test plan

**Test gate**: regression.


Start with an installed Base reproduction matrix. Each missing property,
helper and style argument must fail through the installed public API before
production changes. Distinguish AttributeError, rejected keyword and actual
incorrect output. Reuse F-X197 evidence for nested colours.

Suggested tests in existing entrypoints:

| Test | Evidence |
|---|---|
| test_word_extended_font_properties_round_trip | Every font property, true/false/None, mutually exclusive vertical modes and script slots |
| test_extended_font_properties_cover_accepted_nested_runs | Existing accepted body/control/table run paths and stale controls |
| test_paragraph_tabs_borders_shading_and_bidi_round_trip | All requested edges, tabs, alignment/leader and scalar getter facts |
| test_tab_stops_preserve_unknown_xml_and_fail_atomically | Foreign siblings, clearing, invalid position/enum and unchanged revision |
| test_extended_style_keywords_preserve_existing_style_graph | Create/update, inheritance, missing values and rejected input atomicity |
| test_document_defaults_do_not_restyle_normal | Defaults round trip and unchanged explicit style properties |
| test_one_call_lists_and_checklist_presets_round_trip | Level checks, glyphs, actual numPr and definition/instance ownership |
| test_numbering_start_overrides_reuse_one_definition | Correct lvlOverride/startOverride, save/reopen and clear/update controls |
| test_plain_numbering_instances_match_authenticated_word | Separate native oracle experiment before any continuation change |
| test_extended_python_word_formatting_reaches_render_and_pdf | Deterministic rendering of the visually meaningful requested features |

Extend tests already present at test_formatting_tables.py:88, :639, :793,
:904, :926 and :970. Native tests stay in existing run/paragraph/style
modules and existing integration or regression entrypoints. No new test
binary. Compare supported public facts with pinned python-docx, then inspect
package bytes for non-public properties. Save/reopen twice and assert exact
unsupported XML bytes remain unchanged. Render tests use deterministic fonts.
Add accurate stub types and installed strict mypy/stubtest evidence for new
properties, collections and arguments.

## Risk routing

Read HLD01 units and CLAUDE truncation rules for spacing/tabs. Read HLD04 and
HLD06 for parser/serializer ordering and prefix/raw preservation. Read HLD07
inheritance and resolution, HLD08 deterministic rendering, HLD10
bindings and the invalidation contract, and the pinned differential oracle
skill. Native published API changes earn archive dry-run, size and actual
measurement maintenance. Binding changes earn the documented WASM check and
workspace binding exclusions. Normal scoped verification and independent
microscope remain mandatory.

Expected default hash harness delta is none for wrapper exposure. Newly
authored requested properties change only the new source-built test outputs.
Do not move the baseline. If the authenticated continuation experiment proves
a native rendering defect, propose its separate labelled behaviour commit
with the exact expected delta and exclusive baseline ownership before fixing
it. Tests and claims may not treat an unconfirmed Word assertion as evidence.

## HLD impact

- docs/hld/02-scope-and-non-goals.md, Python capability rows.
- docs/hld/07-inheritance-and-resolution.md, defaults and list resolution contracts if expanded.
- docs/hld/08-rendering-spec.md, authoritative Word numbering continuation and restart contract.
- docs/hld/12-testing-strategy.md, intentional numbering divergence and authenticated controls.
- docs/hld/10-bindings-spec.md, exact font/paragraph/style/default/list signatures and invalidation.
- docs/hld/14-development-backlog.md, resolved acceptance and retained boundaries.

## Hash harness

Wrapper exposure should preserve existing defaults. Actual native semantic
changes require measured declared deltas and separate labelled commits before
any exclusive baseline update. The risk discussion above identifies those
possible changes. No baseline movement or implementation is approved by this draft.

### Measured namespace closure delta

The actual current49-entry harness differs in exactly two document XML entries.
Source-aware save/reopen retains a self-contained Word namespace on raw border
and tab owners. feature_showcase changes
ada34bba6c97288dce7f60a96b32f2d20c3ea2254f6f82e08c660a39ecfc3b54
to9d3418ef8ef11cbcd64acb40b6f2d54648f985050138181bfb32531c8eecf48f,
with three pBdr and four tabs declarations. report changes
bc7d4995931e61ff1fc1c0f004349e64b483428b8c0a5b652a47c46ccb50915e
to8df95515887d2b7c7f5604db9c99f5cf2dcbce6a2ea2e202799adf6885c68ec4,
with one pBdr declaration. Independent pass62 reconstructs both old parts
exactly by removing those eight declarations and proves expanded XML tree
equality. All remaining47 entries, including PDF and image output, match.
There is no measured numbering delta. Retain actual imported namespace closure
rather than strip raw bindings without a parent scope. These precise two XML
changes are the declared expected delta. A labelled source attribution commit,
exclusive two-entry baseline update and actual current hash retry remain required
before scoped acceptance. The failed original receipt remains unchanged.

## Implementation checklist

- [x] Resolve technical contracts and actual native gaps against prerequisites.
- [x] Approve complete issue acceptance, source claims and measured output policy.
- [x] Claim only after actual prerequisites and file conflicts clear.
- [x] Prove installed public Base regressions before production changes.
- [x] Implement every requested capability, stubs and existing-entrypoint tests.
- [x] Run scoped gates and exact applicable oracle/risk riders.
- [x] Obtain independent zero-defect, zero-smell review and prepare handoff.

## Open questions

Resolved by the selected Issue303 Word-parity scope and authenticated
counter contracts below. Definition-owned counts, concrete override consumption,
ancestor rearming and deferred section reset are specified independently.
F-283 consumes completed F-X200 once. The deliberate compatibility change
requires a separately labelled behavior commit, source-built native expected
values and reviewed actual harness attribution. F-X197 completion and shared
file/baseline claims remain source gates. Checkbox glyph coverage follows
F-X210's exact authorized provider bytes, never an inferred asset approval.

## Resolved technical decisions

## Oracle provenance

The requested /private/tmp/s77-fx141-venv/bin/python executed Python 3.12.13, but its sys.path omitted that venv's site-packages and import docx failed. Used the already installed /private/tmp/S91-F-X198-python/bin/python instead, asserted docx.__version__ == '1.2.0', inspected its exact installed source and executed 24 super/subscript transition cases. No substitute Word oracle was used.

Pinned oracle source root for citations below: /private/tmp/S91-F-X198-python/lib/python3.12/site-packages/docx/. Native repository citations refer to the source checkpoint above.

### Decision 1: vertical modes and direct properties

Match python-docx Font.superscript and Font.subscript names and Optional[bool] facts. Oracle text/font.py:346 and :364 delegate to oxml/text/font.py:241 and :263.

| Assignment | Existing direct mode | Resulting direct w:vertAlign |
|---|---|---|
| superscript=True | any | superscript |
| subscript=True | any | subscript |
| superscript=False | superscript | absent |
| superscript=False | subscript, baseline, absent | unchanged |
| subscript=False | subscript | absent |
| subscript=False | superscript, baseline, absent | unchanged |
| either=None | any | absent |

Getters return None if vertAlign absent, otherwise a boolean comparison with the corresponding token. Thus direct baseline yields False for both. Never report effective inherited formatting as a direct value. The executed 24-case matrix confirmed every row above. Preserve repository read/clear no-op discipline: unlike python-docx's get_or_add_rPr setter, clearing absent direct state must not insert empty nodes.

Existing native Run::set_superscript and set_subscript at run.rs:887/:898 support only True. Add minimal Option<bool> value setters with the matrix above. RunRef::vert_align at :1362 supplies getter facts. Preserve unrelated raised/lowered position rather than conflating w:position with the requested w:vertAlign.

Add inheritance-aware Optional booleans for small_caps, all_caps, double_strike and hidden, using existing toggle pattern. Existing bool-only methods at run.rs:854, :865, :876 and :942 need Option value counterparts. rtl already has set_rtl_value :985. None clears, False writes explicit false for independent toggles.

Exact proposed additional Font names: character_spacing, language, language_east_asia, language_bidi, name_east_asia, name_complex_script, rtl. Length/integer EMU character spacing returns Length|None. Check signed i32 twips before conversion and use integer division toward zero. Native Run::set_character_spacing :909 currently narrows through as_twips and lacks clear. add a checked optional operation. Script names use set_slot_font at :685, languages use :727/:1013/:1021. retain slot theme clearing only for the changed slot.

Correction to issue narration: native Run::set_font_value at :666 sets all four slots, and existing Python style font_name writes all four at rdocx-py/src/document.rs:1003. Do not silently change those established general-name semantics. The new slot properties allow independently changing East Asian/complex-script names.

### Decision 2: document defaults

Use Document.default_font_name -> str|None and Document.default_font_size -> Length|None, both writable with None clearing their own direct facts. No python-docx public document-default equivalent exists, so these are repository naming decisions consistent with existing style font_name/font_size and Font.size. Do not expose CT_RPr in Python.

Add four concrete native operations in existing document.rs:

- default_font_name(&self) -> Option<&str>
- set_default_font_name_value(&mut self, value: Option<&str>) -> Result<()>
- default_font_size(&self) -> Option<Length>
- set_default_font_size_value(&mut self, value: Option<Length>) -> Result<()>

Read the explicit ASCII slot and sz from self.styles.doc_defaults.rpr. General font assignment follows the existing four-slot general setter semantics, clearing corresponding theme references, preserving font hint and all nonfont fields. Size assignment writes normal and complex-script sizes as the current general font size setter does. Validate schema-representable nonnegative half-points before any mutation. Absent getters and absent None clears create nothing. Stage updates using existing Document candidate/style-part reservation, commit only after success, and invalidate layout without bumping paragraph topology. Normal and named styles are not restyled.

Concrete preservation rider: CT_DocDefaults at rdocx-oxml/src/styles.rs:564 only holds rpr/ppr. Its parser skips unknown docDefaults children at :593 and rPrDefault wrapper children at :623. Therefore a setter over the current model cannot honestly promise preservation of unknown wrapper source. Extend that existing model/parser/writer minimally to retain source attributes and unknown child slots in those exact wrappers, reusing existing raw/order facilities. This is a demonstrated existing-model gap, not a new generic XML mutator or file. Add its existing styles.rs tests to the exclusive path/risk list. Native CT_RPr/CT_PPr unknown fields must also remain untouched. Authenticate malformed namespace controls and empty/default clearing serialization rather than claim preservation from fields alone.

### Decision 3: paragraph tabs, borders, shading and style keywords

#### Tabs

Use paragraph_format.tab_stops as a live collection over the existing paragraph ContentPath/AcceptedRunPath resolution, implemented in existing formatting.rs. Read access creates no pPr or tabs. Exact requested public operations:

- add_tab_stop(position: Length|int, alignment=WD_TAB_ALIGNMENT.LEFT, leader=WD_TAB_LEADER.SPACES) -> TabStop
- clear_all() -> None
- len, iteration, integer indexing including negative indices, del collection[index]
- Returned TabStop exposes position: Length, alignment: WD_TAB_ALIGNMENT, leader: WD_TAB_LEADER as read-only owned facts.

The last choice deliberately limits the projection to the issue's add/clear/list acceptance. It does not claim python-docx's additional writable TabStop.position/alignment/leader API. A frozen three-field snapshot in existing formatting.rs has concrete consumers: add result, iteration and indexed inspection. Consequently no live indexed child can silently alias another tab after reorder, no tab revision framework is needed, and collection operations leave parent paragraph identity live. Document this subset in stubs/HLD. If mutable individual tab setters are added to scope later, design their identity/invalidation explicitly first.

Oracle text/tabstops.py:53 defines add signature and permits negative positions. :67 defines clear_all. :31 indexing. :20 deletion. :43 iteration uses source XML order. Oracle oxml/text/parfmt.py:383 inserts new tabs before the first greater position, preserving equal-position insertion order. Oracle TabStop.leader at text/tabstops.py:105 accepts None as SPACES. Model absent leader reads SPACES, preserving source absence on a read. Native TabStopRef::leader at paragraph.rs:642 conflates missing and unsupported tokens, so add a concrete source projection or complete its modeled variants before applying the default. Add/None leader writes the repository equivalent of the oracle default. Add WD_TAB_ALIGNMENT/WD_TAB_LEADER to existing enum/text.py, not new files.

Native paragraph.rs:1645/:1670 currently append, while :2081 rewrites in place without reordering. Provide a checked native insertion path for this named Python add consumer that inserts by position and validates signed twips before narrowing. Retain imported source order and unknown alignment/leader tokens instead of sorting the entire existing list. Extend existing native TabAlignment/TabLeader projections to expose the already modeled Bar/Clear/Num and Heavy/MiddleDot variants rather than misreport them as absent. The pinned oracle also exports LIST=list, END=end and START=start. Existing low-level ST_TabJc at rdocx-oxml/src/shared.rs:432 rejects list and normalizes start/end to left/right at :446/:448. Add concrete List/Start/End variants to that existing enum/parser/writer and the facade to expose and preserve the exact requested oracle vocabulary. No new enum file is needed. Any genuinely unsupported token must remain preserved and reads must report an explicit unsupported-value error rather than guess. Use native removal :2108 and clear :2128. Source-owned snapshots remain readable after later mutations, while the live collection validates its parent path on every operation.

#### Borders and shading

Follow existing table/cell names where python-docx has no equivalent:

- paragraph_format.set_border(edge, style, *, size, color)
- paragraph_format.border(edge) -> tuple[str, int|None, RGBColor|str|None]|None
- paragraph_format.clear_border(edge), clear_borders()
- paragraph_format.shading -> RGBColor|'auto'|None, writable RGBColor|str|None
- paragraph_format.outline_level -> int|None, writable checked 0..9|None
- paragraph_format.rtl -> bool|None

Edges: top, bottom, left, right, between, bar, using actual ParagraphBorderEdge slots. size is eighths of a point, matching current table/cell spelling at rdocx-py/src/table.rs:454/:1200. Colour uses integrated F-X197 shared extraction and surface-specific auto handling. Border tuple comes from native ParagraphBorderRef :229, including style/size/color. do not silently replace unsupported existing style values. Preserve space and foreign border attributes when changing the named fields. Clear removes one edge, and last-edge clear removes pBdr, using paragraph.rs:1992.

Shading is the direct fill projection at ParagraphRef::shading_fill :2960. A fill edit preserves imported pattern, foreground and unknown shading attributes. Existing Paragraph::set_shading_value :2052 replaces the entire shading struct, so add a narrow native fill-value update rather than reuse that destructive operation. None removes direct shading, absent clear is no-op. Pattern editing is not required by this acceptance and no speculative shading class is needed.

### Styles

Extend both existing add_style/set_style keyword sets with alignment, line_spacing, underline, keep_with_next, keep_together, page_break_before, borders, shading, tab_stops. Existing font/spacing/indent names remain unchanged. Exact input shapes:

- alignment: WD_ALIGN_PARAGRAPH|None
- line_spacing: Length|float|None, using the existing ParagraphFormat spacing interpretation
- underline: bool|WD_UNDERLINE|None, using existing Font underline conversion
- keep_with_next, keep_together, page_break_before: bool|None
- borders: Mapping[str, tuple[str,int,RGBColor|str]]|None
- shading: RGBColor|str|None
- tab_stops: Sequence[tuple[Length|int,WD_TAB_ALIGNMENT,WD_TAB_LEADER|None]]|None

Retain established keyword omission semantics: None for these style convenience kwargs means no supplied override, False is explicit false. Empty borders/tab_stops are explicit clear-all inputs. Do not introduce a public UNSET object merely to change old None behavior. If explicit clearing of every scalar style property becomes required, use an explicit checked clear operation as a separately documented addition rather than overloading omission.

Validate the whole input before style creation/update. Copy existing relevant properties, apply only named supplied changes, then submit one existing StyleBuilder update. Native style.rs:415 and :463 already accept concrete property structs. current Python StyleFormatting document.rs:981 constructs new structs, so replacing properties blindly would drop imported state. Preserve unrelated children, attributes, schema order, aliases and run/paragraph defaults. Character styles reject supplied paragraph-only inputs before graph mutation. No new style-property framework is needed.

### Decision 4: explicit numbering restart and checked list helpers

Use the smallest required public signature:

add_numbering_instance(definition_id: int, *, start: int|None=None, level: int=0) -> int

When start supplied, construct one existing native NumberingLevelOverride::new(level).start(start), then call the existing staged Document::add_numbering_instance at document.rs:22207. When start is omitted, pass the existing empty override slice. Its rendering consumes the explicitly revised shared-definition continuation contract below. Reject nonzero level without start as meaningless, invalid level outside 0..8, negative or unrepresentable start (zero remains valid in the existing native u32 contract), missing definition and malformed numbering graph before source commit. No new public override type, parallel overrides/start forms or convenience framework is needed for Issue303's restart/start acceptance. Full replacement-level editing belongs the separately scoped numbering inspection work.

Expose add_bullet_list_item(text, level=0) and add_numbered_list_item(text, level=0), returning the new Paragraph at the current revision. A checked native counterpart IS required, not just a Python level check. Native document.rs:21809/:21868 call fallible reserve_numbering_bundle and reserve_numbering_ids through expect. reserve_numbering_bundle at :21713 observes an imported package graph and can fail. Level validation cannot prevent those failures.

Add try_add_bullet_list_item and try_add_numbered_list_item returning Result<Paragraph<'_>> in existing document.rs. Preserve successful existing output. Convert existing staged expect paths to propagated errors and validate 0..8 before allocation. The existing infallible helpers may delegate preserving their historical signature, but Python consumes the checked operations. No unwind catcher, duplicated unchecked path or speculative trait is justified. Malformed imported graph/relationship and exhausted IDs must leave bytes, graph and revisions unchanged.

Checklist preset uses existing PyListLevel: ListLevel.checklist(*, checked=False) returns ListLevel(format='bullet', text='☑' if checked else '☐'). Use actual bullet definitions and numbering references. No ordinary-text prefix, new checklist type or mandatory unrelated helper. Deterministic glyph coverage and actual native PDF/raster output remain required.

### Tests and remaining oracle barrier

Add the executed super/sub transition matrix to existing test_formatting_tables.py, including explicit baseline, direct absent/inherited facts and None-no-node controls. Add tabs sorting/equal positions/negative positions/index errors/None leader/read-only source, full paragraph slot paths, style atomic validation/preservation, document-default no-restyle/foreign-wrapper preservation, numbering startOverride XML and malformed imported helper atomicity controls. Existing entrypoints only, installed runtime and strict mypy/stubtest required. Font and paragraph edits preserve parent text topology, existing structural edits remain loudly stale. Consume F-X197 once for all new colours.

The authenticated Word evidence below resolves the original plain-instance, interruption and multilevel experiment. Keep current production behavior unchanged until the explicit existing compatibility contract, additional affected constructs and measured output policy are reconciled. Any intentional renderer behavior change requires a labeled commit and reviewed hash delta.

Existing rdocx-oxml/src/styles.rs is an additional exclusive source owner
for minimal default-wrapper preservation. Run parser/serializer namespace
and raw child-order riders on that repair. No new module is proposed.

## Authenticated Word numbering result

Native Word 16.113.2, build 16.113.26092012, displayed and exported all
21 labeled numbering controls consistently before and after normal reopening.
The corrected input puts all abstract definitions before concrete instances,
uses distinct independent definition identities, and opened without repair.
Plain fresh instances sharing one abstract definition printed 1,2,3,4.
Explicit level-zero startOverride1 printed 1,2,1,2. The same instance
continued after an unnumbered interruption. Multilevel controls printed
1 / 1.1 / 1.2 / 2 / 2.1, then a plain fresh instance printed 3 / 3.1,
and an explicit root restart printed 1 / 1.1. Both exported PDFs preserve
the same 21-number map. Native display and pinned Poppler 26.01.0 extraction
are authoritative here, since accessibility list numerals differed.

Evidence and hashes are in the existing ignored
.claude/scratch/F-X200-word-numbering-oracle.md and the local
S91-F-X200-word-numbering/audit-receipt.json. The preliminary malformed
interleaved-definition fixture is excluded from acceptance. These controls
do not establish returning to old instances after overrides, style-linked
lists or every lvlRestart variant. Add controls if a change touches them.

The current independent-numId rendering is an intentional existing contract
in HLD12 and HLD08, with an explicit style-resolver test. This evidence
therefore requires a deliberate compatibility-contract decision and a
separate labeled behavior commit, rather than treating current behavior as
an unexplained defect. Final approval must reconcile that decision, actual
affected fixture keys and F-283 resolved-counter dependency before claiming
X200. Implementation waits for completed dependencies and exclusive claims. No baseline update is preapproved.

## Selected continuation scope and scheduling

The accepted Issue303 parity scope supersedes the prior intentional
independent-numId counter behavior. F-X200 owns the authoritative native
NumberingState change in existing rdocx-layout/src/style_resolver.rs. It must
update advance, current, ancestor substitution, deeper and section restart,
and transaction state lifecycle together where the authenticated controls
require it. Preserve concrete numId in ResolvedNumbering as source identity.
Do not change relative_to from visible-marker evidence alone. F-283 now
requires completed F-X200 and proves its own numbered REF switch semantics.

Before production edits, capture return-to-old-instance, plain-after-override,
non-one and child start overrides, replacement-level-only, cross-instance
lvlRestart and section controls. A global abstract-id key substitution is
not a sufficient design. Preserve separate abstract definitions and source
traversal facts. Existing independence tests become explicitly labeled Word
parity regressions. HLD08 and HLD12 must describe the new current contract.
Native layout, deterministic render, package and hash riders apply alongside
binding riders. Use a separate intentional behavior commit. No exact fixture
delta is claimed before measurement. Baseline ownership remains exclusive.
The bounded native state decision is recorded below. The prerequisite and final source-wave approval still gate implementation.

## Final authenticated counter contract


Microsoft Word `16.113.2`, build `16.113.26092012`. Pinned python-docx `1.2.0` creates ignored controls only. Pinned Poppler `26.01.0` extracts native printed PDFs. Native CUA opened the generated input, saved a separate local Word copy, exported local `Best for printing` PDF, closed only the saved generated copy, reopened that exact copy normally, then exported a second local printing PDF. No repair prompt, F9, numbering edit, online conversion, production layout inference or settings change. All input definitions precede instances and use distinct nsid values.

Artifacts are under `/private/tmp/S91-F-X200-word-numbering`:

- `create_section_levels.py`, `level-section-input-references.json`, `audit_level_section.py`, `level-section-audit-receipt.json`: 35 markers, five preserved distinct definitions, ten instances.
- `create_ancestor_discriminator.py`, `ancestor-input-references.json`, `audit_ancestor.py`, `ancestor-audit-receipt.json`: eight markers, one definition, two instances.
- Both have immutable original inputs, independent Word copies, first and reopened native PDFs/text, and source/saved numbering and document XML. Receipts include exact input/saved reference mappings, nsid, level, replacement and startOverride contracts, file bytes and SHA-256.
- Original input hashes are unchanged. Saved Word renumbers IDs but preserves definition ownership, paragraph levels and explicit overrides. All 43 actual PDF markers are identical before and after reopen. These are actual printed numbers, not synthesized AX list numerals.

Both generated documents were closed only after exact generated URL and `Saved to my Mac` proof. The existing APA6 study remains open and unchanged. Word UI lease released. No source, tracked file, Cargo, user document or setting writes.

## New exact observations

IDs below are input IDs. Receipts map saved IDs.

| Control | Definition and IDs | Before section | After section | Contract |
| --- | --- | --- | --- | --- |
| P | abstract50 root start3, child start4, default restart. Plain1201, child override1202 start7 | 3,3.7 | used child override first:3.4, then3.5, plain child3.6, plain root4, override child4.4 | Child-first section restart initializes ancestor root3. Already-used override does not reapply. A root advancement by another instance resets the shared child to base4 without rearming that override. |
| Q | abstract51 root3, child4, default restart. Plain1211, unused child override1212 start9 | 3,3.4 | unused child first:3.4, second3.9, plain child3.10, plain root4, override child4.4 | First child after section uses base4 without consuming unused override9. Second use applies9. Root advancement by another instance resets shared child to base4, preserving that override's consumed state. |
| R | abstract52 section opt-in, root3, child4 with lvlRestart0. Plain1301, root override1302 start8 | 3,3.4,8,8.5 | used override root3, child3.6, plain root4, child4.7 | Section restart resets root to3 but preserves carried child count5. Neither section nor ordinary root advancement resets a lvlRestart0 child. |
| S | abstract53 opt-out, otherwise same as R. Plain1401, root override1402 start8 | 3,3.4,8,8.5 | used override root9, child9.6, plain root10, child10.7 | Opt-out retains root and child state across section break. |
| T | abstract54 section opt-in, root start3. Plain1501, root override1502 start8 | 3,8 | used override first3, second4, plain5 | Section restart uses non-one base start3 rather than hardcoded1 or consumed override8. |

### Ordinary ancestor owner discriminator U

Abstract60 has default child restart and base starts1. Plain1601 and child override1602 start7 refer to that same definition.

| Physical paragraph | Concrete numId | Level | Printed marker |
| --- | --- | --- | --- |
| U_PLAIN_ROOT_1 | 1601 | 0 | 1. |
| U_OVERRIDE_CHILD_1 | 1602 | 1 | 1.7. |
| U_PLAIN_ROOT_2 | 1601 | 0 | 2. |
| U_OVERRIDE_CHILD_AFTER_PLAIN_ROOT | 1602 | 1 | 2.1. |
| U_OVERRIDE_ROOT_3 | 1602 | 0 | 3. |
| U_OVERRIDE_CHILD_AFTER_SAME_ROOT | 1602 | 1 | 3.7. |
| U_PLAIN_ROOT_4 | 1601 | 0 | 4. |
| U_OVERRIDE_CHILD_AFTER_OTHER_ROOT_AGAIN | 1602 | 1 | 4.1. |

This distinguishes shared counter reset from concrete override-use reset. A root from any instance resets the shared child count under the base restart policy. An already-used child override is rearmed only by an ancestor paragraph from its own concrete numId. Cross-instance root paragraphs do not rearm it. Earlier G observed same-instance rearming. U now proves its ownership boundary without section behavior confounding the result.

## Recommended smallest concrete state

Keep the existing `NumberingState` and existing native rendering path. Do not add a numbering framework, independent field counter, trait or new module.

1. Shared counts: `HashMap<(abstract_num_id, ilvl), u32>`, replacing the concrete counter owner at `crates/rdocx-layout/src/style_resolver.rs:191`. Plain/old instances share counts, while separate distinct abstract definitions remain separate. Resolve definition ownership in the existing resolution path at line512.
2. Concrete explicit-override use: a set keyed by `(num_id, ilvl)` in that same state. First ordinary use applies explicit `startOverride` to the shared count and marks consumed. Returning to a consumed override advances the shared count. Replacement-level `start` is not an explicit override. Concrete display formatting stays in the existing effective level path at lines534..550.
3. Ancestor reset: under base lvlRestart policy at lines238..248, remove the shared deeper count across the definition. Rearm only `(the advancing paragraph's concrete num_id, reset_level)` overrides. Do not clear consumed override state for other instances. Keep lvlRestart0, configured owner thresholds and ignored replacement-level restart.
4. Section restart: mark each opted-in previously initialized definition as pending for its next numbered use. When consumed, initialize the shared root from its base start and apply the base root-triggered deeper restart policy. Preserve carried deeper counts where lvlRestart0 or owner policy says not to reset. Preserve all consumed concrete override-use state. Skip applying explicit override for the first numbered item in this definition after the section break without consuming an unused override. After that first item, ordinary explicit-override rules resume.

The pending section state is concrete today behavior required by N/O/P/Q/T. Initializing ancestor root when the first physical item is at child level is required by P/Q: the first child shows root3 and the next physical root is4. It is not sufficient to use root3 only as a formatting fallback while leaving its count absent. Conversely R proves that root restart cannot indiscriminately discard all deeper counts. Section initialization must be distinguished from an ordinary ancestor paragraph, since the section event must preserve override-use state rather than rearming the current concrete ID.

The section method currently clears concrete counters at `style_resolver.rs:255`. Its implementation should become the measured deferred definition event above, not a blind map-key rename. `number_context` at line593 and `format_lvl_text` at line772 must consistently read definition-owned ancestor counts while rendering the selected concrete level. `references_only` at line300 must begin with empty shared counts, override-use and pending section state so note-width retries preserve existing once-only consumption. Clone semantics must include the concrete state fields.

Do not change `ResolvedNumbering.num_id` or `relative_to` at line149. Actual concrete source identity remains distinct from shared count ownership. F-283 owns its authenticated REF switch oracle. Do not infer or implement nsid coalescing.

## Implementation regressions and compatibility

Use source-built controls in the existing style_resolver unit module and existing public integration entrypoint. E through U are expected-value specifications from the immutable native receipts. Meaningful controls include old/new/returned IDs, root and child overrides, concrete rearm versus shared reset, replacement-only display/start, child lvlRestart0, configured restart owner, opt-in/opt-out, non-one base starts, lower-level-first ancestor initialization, delayed unused override and consumed override continuation across section resets.

Tests must separately assert marker values, ancestor number context and actual concrete reference identity. Existing owner/zero/replacement restart tests at lines1327/1357/1380 remain controls. The independent-instance expectation at line1202 is intentionally superseded and must be labelled in the behavior commit. Same-instance child override reapplication at1291 remains supported, while U adds the cross-instance distinction. Separate abstract definitions remain independent at1448.

Update the existing X200 plan and current HLD07/08/10/12/14 numbering intent as described in the prior reconciliation. Historical F-248 design remains historical evidence. F-283 completion depends on settled X200 numbering output. Hash harness deltas require deterministic rendering, a labelled intentional behavior commit and independent review. No measured library parity, generated fixture acceptance, hash baseline change or gate pass is asserted here.

## Evidence boundary

These controls settle the requested shared-state ownership and section architecture for root3/child4, used and unused child overrides, ordinary same/different concrete ancestor ownership, default/zero/configured restart rules and opted-in/opted-out sections. No nsid coalescing, relative REF, unsupported numbering format, bullet, zero start or continuous-section oracle expansion is inferred. A child-first section event with lvlRestart0 was not directly captured, although separate controls establish child-first initialization and carry preservation. The algorithm above preserves both measured rules and should not invent a special reset for that combination. If production implementation needs another behavioral branch for that combination, obtain its bounded oracle rather than guessing.

## New immutable artifact hashes

| Artifact | Bytes | SHA-256 |
| --- | --- | --- |
| numbering-level-section-input.docx | 36895 | `d8bfbafb0273a8ed9827e7b6d6444db216d1662517ca6d555be0d779d29cdf2b` |
| numbering-level-section-word.docx | 27060 | `1dc99af50f8c882341f52d37d5031cca701cd297bca75b431b29494f4ff9b857` |
| numbering-level-section-first-export.pdf | 22073 | `82e26194be28ffbbe8d941425223b3b2bedd13eb128a1e93f7bdedec9a9c991f` |
| numbering-level-section-reopened-export.pdf | 22073 | `bec1d17129aae7f6a24d401b5f61dfbacfaaffbda5a79cbc9315700fb348cecc` |
| numbering-ancestor-input.docx | 36461 | `d8b4e0a1747f803bbad617f1bd0f0b96d4eff842aa4ad82e0db65c4cd16f9959` |
| numbering-ancestor-word.docx | 26158 | `c54ef320d3f2d77b595a81b6cff02c409106378e23757783297b1a9b02ce8aaf` |
| numbering-ancestor-first-export.pdf | 17349 | `bef1c7c55c2d0b7273e23415928860db3991b8a760a5430126495324a4c54587` |
| numbering-ancestor-reopened-export.pdf | 17349 | `c9f736e2f3dc4e30eb04be2d660e5cc0ced9d8b20acca3ba902ba8d58f7df1d4` |

Uninitialized definitions retain their existing first-use behavior across a
section boundary. The measured section controls all previously initialized
the definition. Do not extend the suppression rule to an unused definition
without its own native control. This preserves behavior outside the observed
contract while the shared-state change is implemented. No style-linked or
nsid coalescing claim is added. The draft awaits completed F-X197 and final
source-wave approval, not another user permission request.

## PR320 and Issue321 reconciliation

PR320 at immutable head252acccbba9adab9264cfb109c4b1d720ed79d4a closes
Issue303 but explicitly leaves shared-definition native counters unchanged.
Its native helper startOverride workaround is not a replacement for the
156-marker authenticated Word state contract above. Issue321 is assigned
to this same F-ID and provides explicit acceptance for ordinary-instance
continuation, first-use explicit restart and sub-level restart preserving
level0. Implement and verify those criteria once under the native owner.
Contributor source is reusable only after full immutable source assessment,
including tab-handle mutability versus the existing snapshot contract,
unsupported tab enums, raw docDefaults retention and checked list operations.
Colours remain F-X197. Contributor passes and hashes are not local gates.

## Immutable contribution assessment decisions

Completed immutable PR320 assessment is retained in .claude/scratch/S91-PR320-252accc-assessment.md. All18 blobs and2585720 source bytes were independently revalidated. Retain the canonical read-only TabStop snapshots. Contributor mutable handles keyed by position alias imported duplicates and can revive after deletion/reinsertion or position reuse. Installed python-docx1.2.0 actually retains two tabs at an identical position and exposes ten alignment and six leader enum members, not the contributed four each. Do not silently narrow spacing or tab positions from i64 to i32. Native list expect paths and separately published restart allocation/paragraph updates require one checked candidate transaction. Preserve docDefaults unknown wrappers and shading pattern/foreground. XML-only small caps is not native rendering acceptance. Issue264 remains excluded even where Issue303 uses overlapping clearable run setters.

## Cross-PR checklist glyph coverage

PR320's checklist preset uses U2610 for unchecked and U2611 for checked.
HarfBuzz14.4.0 coverage of all27 current bundled faces finds both glyphs
missing. The exact approved3916-byte PR319 symbol subset has U2611 but
not U2610. Its pinned full upstream Noto Sans Symbols2 face contains both,
with equal821-unit advances at1000 units per em. The complete read-only
receipt is .claude/scratch/S91-checklist-symbol-coverage.json.

The approved asset retains its exact bytes and hash. Do not treat
its16-target coverage as acceptance for the checklist preset, silently
change its approved-file scope, substitute an unrelated square or emit a
missing glyph. Resolve U2610 through the existing caller-font provider or
an exact separately reviewed asset change when needed. Native PDF/raster
appearance and deterministic fallback acceptance remain explicit joint
F-X200/F-X210 obligations, tested once against their actual provider.

## Resolved checklist provider asset

On2026-10-10 the user approved F-X210's exact3968-byte symbol subset
revision, SHA256
a95e7d7421e30e8ad304a0eb48aa18aefabcfec568a0f652c1039d72da999373.
It adds U2610 to the prior16-codepoint asset at the same approved path.
Consume that selected provider after F-X195/F-X210 implementation, without
duplicating the subset or claiming unperformed native glyph acceptance.
No checklist asset question remains.

## PR336 rolling intake, 2026-10-10

PR [336](https://github.com/tensorbee/rdocx/pull/336), immutable head
`c6ab986008782e8a376a9cdd0bda2afd4847cf47`, contributes to Issue321 shared numbering counters.
The same contribution also contains the other story's slice, owned by
F-X211. Adopt accepted changes once per owner.
Complete source and discussion assessment is pending. Its proposed extra
inline-wrapper and cross-control nesting policy is not automatically approved
by intake. Contributor gates and hash expectations are not local evidence.
The approved contract and completion prerequisites remain authoritative.

### Completed immutable PR336 assessment

Shared abstract-definition counter keys are reusable ingredients. The patch's
once-per-num_id override tracking and blanket section resets do not satisfy
approved per-level override consumption, rearm, pending section initialization
or retained lvlRestart0 state. Preserve the authenticated P/Q/R/T/U native
controls. EPUB must use the same selected counter semantics. Reuse only the
conforming slice, then implement and verify the uncovered contract.

Complete assessment: `/private/tmp/S91-PR336-c6ab986-intake/assessment.md`.
All fourteen changed paths and twenty-eight base/head Git blobs were captured
and independently authenticated. No contributor gates were executed or
accepted as local completion evidence. No new module or F-ID is required.

### PR336 updated-head intake

The 2026-10-10 07:27:15 UTC refresh observed head
`935ac165afcd29d34454568ba3b2287a4bac52dc`, replacing the retained assessed
c6ab986 head. Complete incremental source assessment is pending. The new
body claims64 controls,64 inline wrappers,512-element revision refusal and
additional relative-REF numbering changes. These are contribution proposals,
not automatically selected resource limits or locally executed gates. Preserve
the approved contract until actual changed-source assessment is reconciled.
The prior immutable assessment remains historical evidence, not approval of
this new head. Issue331 and321 ownership is unchanged.

### Completed updated-head source assessment

Incremental assessment of immutable935ac165afcd29d34454568ba3b2287a4bac52dc
is complete, superseding the pending checkpoint above. All sixteen changed
full blobs, five reused blobs and seven missing prior versions were
authenticated by Git identity, size and SHA256. Source and discussion
evidence is retained at `/private/tmp/S91-PR336-935ac16-intake/receipt.json`
and its `assessment.md`. No contributor gate is local completion evidence.

Related-story preflight and staged parsing are reusable ingredients, including
comments, footnotes/endnotes, headers/footers and glossary relationships.
Preserve inclusive256 qualified-control admission, exact foreign/rebound
content and the normal-stack lifecycle. Do not inherit proposed64 controls,
64 inline wrappers or512 total XML-element revision depth. The new revision
guard runs after transaction staging and does not establish staging safety.

The updated numbering state still lacks selected per-level override
consumption/rearm and deferred section initialization with retained
lvlRestart0 counts. Preserve native P/Q/R/T/U acceptance. The new
abstract_num_id and relative_to changes affect numbered REF and belong to
F-283's actual native discriminator, not marker-only inference. Preserve
concrete numId identity and account for published struct-literal compatibility
before adopting a new public field. Reuse accepted slices once per owner.

## Existing exporter and style-indirection boundaries

The updated PR336 names existing ODT continuation and abstract numStyleLink
gaps. Exact issue and source reconciliation is authenticated at
`/private/tmp/S91-PR336-935ac16-intake/acceptance-reconciliation/receipt.json`.
Issues303/321 require native counters and render behavior. They do not
establish unrestricted ODT list continuation or abstract numbering-style
indirection acceptance. Preserve that distinction in capability documentation.

Add a bounded loss diagnostic in existing `crates/rdocx/src/odt.rs` when
shared-definition continuation cannot be represented by the existing concrete
list writer. Use existing OdtWriteResult diagnostics, a stable numbered-owner
path and source-built plain A,A,B,B controls. Assert deterministic write/reopen
and diagnostic results, with explicit override and independent-definition
controls so a second concrete instance alone does not falsely imply loss.
Native and EPUB selected counter semantics remain required. Do not claim
all exporters reproduce Word continuation. This is an additional existing
source reservation and focused regression, not a new writer or counter.

Retain raw abstract numStyleLink indirection without invented decimal markers
when its local definition has no usable level. Add numbering save/reopen
preservation and unresolved-marker controls in existing test modules.
Supported paragraph-style links through numPr and level pStyle remain
supported. Full abstract numStyleLink delegation is an existing unsupported
capability, not evidence that those paragraph-style links fail. F-283 retains
cache/diagnostic behavior for unresolved numbering sources. Update the
already-listed capability, resolution and testing HLD sections accordingly.

## Joint checklist font acceptance, 2026-10-11

The current F-X200/F-X210 provider passed the four actual unchecked/checked caller and no-caller controls. R10 completed all35 bounded command rows with exit0, unchanged complete source guards and naturally reaped process groups. Receipt1b5d6206be78327e5a8664590494b0ce638d21659be607649fe2a4b25d1f40ec is /private/tmp/S91-F-X200-F210-checkbox-font-text-experiments-r10-prepared/actual-comparator-1/receipt.json. Original source-derived origins, exact approved3968-byte physical font, reachable painted CID and Unicode marker mapping are authenticated. Independent approved source-font PDF and original contour PNG positives are1.0 for every owner, with all wrong-character, blank and six-point negatives below the unchanged.99 threshold. Full original font experiment A remains distinguishing evidence only. Previous path-raster comparison failures remain failed, without a threshold or geometry change.

This completes the joint checkbox paint obligation. F-X210 still requires its remaining scoped package/license/risk gates, HLD, final review and integration. F-X200 still requires attributed baseline and policy/feature partition closure. No feature is marked complete by this one obligation.
