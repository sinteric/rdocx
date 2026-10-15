# F-X205, Complete Python PowerPoint text fill and export parity

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X194, F-X195, F-X197, F-X206
**Issue**: 308, captured in S91-intake-issues.json on 2026-10-09
**Prerequisites**: F-X194 and F-X197
**Implementation**: completed in the claimed codex worker, scoped evidence and prepared handoff

## Problem

Issue308 reports Python omissions and two default-template defects. Its claim
that every omission already exists in native Rust needs qualification against
actual source. Some requested exports and writable metadata lack a native
facade, and notes PDF is already exposed in Python.

The complete issue inventory is:

- Fresh Presentation has no inherited template author, lastModifiedBy,
  description or 2013 timestamps. Created/modified dates are supplied at save
  or use deterministic values in deterministic mode.
- Fresh presentation, master and all11 layouts use Office 16:9 geometry,
  rather than a 16:9 page with 4:3 title and body rectangles.
- Cell.text_frame and its paragraphs, fonts and alignment, reused from X194.
- Font baseline for super/subscript, character spacing, language, East Asian
  and complex-script typefaces.
- Auto-number bullet scheme and start, bullet colour, size and font.
- Linear/path gradient fills with stops/angle, pattern and picture fills,
  solid-fill alpha transparency, and gradient/picture slide backgrounds.
- ColourFormat.theme_color, brightness and type with MSO_THEME_COLOR, plus
  nested font.color.rgb, reused from the amended X197 contract.
- Six table flags first_row, last_row, first_col, last_col, horz_banding and
  vert_banding, and the table style id.
- Presentation core, application and custom properties.
- Section reading and setting.
- Media insert with poster frame, listing, extraction, replacement and removal.
- Per-slide JPEG/TIFF, notes/handout PDF, PDF/A, ODP reading and writing.
- Save/reopen, visually meaningful render evidence, exact stubs, python-pptx
  names where available, installed strict mypy and stubtest.

Google Slides compatibility remains best effort. Explicit RGB solid alpha and
common numbering schemes are preferred. Linear gradients may import, while
path gradients and patterns may degrade. Do not turn that advisory into a CI
claim. Issue295 is satisfied once through X194, not implemented again here.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Python API shape and checked handle invalidation.
- `docs/hld/02-scope-and-non-goals.md`, native and Python capability rows.
- `docs/hld/04-opc-and-packaging.md`, The package and owned relationships.
- `docs/hld/08-rendering-spec.md`, Text in a shape, Tables and deterministic output.
- `docs/hld/14-development-backlog.md`, F-X205 and the full linked issue acceptance.

## Approach

Extend existing rpptx-py text.rs font property read/update functions. Native
TextRunMut.properties/set_properties are lib.rs:9369 and :9379, and
paragraph defaults have default_run_properties_mut at :9332.
CT_TextCharacterProperties at oxml-drawing/src/text/paragraph.rs:610 exposes
spacing:617, baseline:618, latin:620, east_asian:621 and complex_script:622.
Baseline currently uses Option<String>, so public numeric conversion must
validate the DrawingML percentage representation rather than pass arbitrary
strings. Spacing uses TextPointValue. Preserve scalar unit truncation and
unknown property bytes. Language is not a typed field in this structure.
Imported lang is a retained raw attribute, so writable language requires a
small typed model extension with round-trip preservation tests. This is an
actual native gap, not an ordinary wrapper.

TextBullet at oxml-drawing/src/text/bullet.rs:24 owns colour, size, font and
choice. TextAutoNumber at :369 has scheme and optional u16 start_at.
Native TextParagraph.set_bullet exists at lib.rs:9353. Expose concrete
checked fields on the existing paragraph bullet interface, retaining the
current character/none reading and picture-bullet preservation boundary.
Replacing the bullet choice must not accidentally discard separately edited
colour, size or font. Follow existing field merge semantics deliberately.

Fill at oxml-drawing/src/fill.rs:88 already has NoFill, Solid, Gradient,
Pattern and Blip. Existing binding dml.rs current_fill/write_fill and
PyFillFormat are the extension points, with native ShapeMut.set_fill at
lib.rs:7891 and SlideMut.set_background at :7017. Picture fills require a
native package operation allocating the image part and relationship for the
actual owning slide. Do not author relationship XML in Python or use an image
relationship belonging to another slide. Reuse existing MediaStore and
native image insertion mechanics where safe. Replacement/removal must prune
only unreferenced relationships and media parts.

Existing ColorFormat is dml.rs:412. X197 must own shared RGB extraction,
theme/type/brightness semantics, supported enum shims, font format sources
and preservation of unrelated transforms. Solid transparency is fill alpha,
not a second global colour conversion. Retain line and shadow-specific alpha
semantics and fore_color prerequisites. Unset getters do not solidify fills.

Native TableRef and TableMut have the six flags at lib.rs:8210 through :8250
and :8454 through :8527. Their existing Python table.rs handles can expose
the requested python-pptx aliases. CT_TableProperties already has style_id at
oxml-drawing/src/table.rs:56, but the current facade lacks a style id getter
or setter in the inspected TableRef/Mut methods. Add those minimal facade
operations and preserve unknown table property bytes and style references.
Changing banding/style formatting must not invalidate grid or cell handles.

Presentation.core_properties/core_properties_mut exist at lib.rs:1927 and
:1932. Application/custom property read/write facade operations were not
found in native rpptx. Their OPC models can be reused, but package ownership,
part creation, relationship/content-type management and staged atomicity
must first exist in the native facade. Do not expose internal package access
as Python's properties API. Use concrete snapshots/live properties consistent
with existing Word public contracts where they genuinely match.

Native sections/set_sections at :2479/:2484 and media lifecycle at :4217,
:4230, :4349, :4449 and :4475 supply checked native operations. Media insert
is owner-aware and must keep its signature, poster fallback and supported
formats explicit. Do not infer that a generic add_movie path supports every
codec or playback policy.

Native PDF/A, notes PDF and handout PDF exist at lib.rs:1292, :1302 and
:1318. Python presentation.rs:258 ALREADY exposes to_notes_pdf and :265
exposes render_all_notes. Reuse their evidence and strengthen the acceptance
tests where needed. ODP public methods exist in rpptx/src/odp.rs:71 and
:102, including typed loss/diagnostic reports. Python must retain those
reports instead of silently hiding lossy conversion. Native per-slide
JPEG/TIFF facade methods were not found, though PNG methods exist at
lib.rs:1177 and :1184. Choose a small native image encoding operation using
existing image support rather than presenting those items as wrapper-only.

## Fresh-template corrections

PyPresentation.new at presentation.rs:188 delegates directly to native
Presentation::new. Native lib.rs:818 returns from_bytes(DEFAULT_TEMPLATE).
Its file-level documentation says only slide size/type changed from the
python-pptx template. Therefore the bugs originate in native fresh-template
creation and the bundled template, not a binding-only constructor.

First build source-derived package assertions for existing template core.xml
and all master/layout placeholder rectangles. Compare supported geometry
with an authenticated Office 16:9 document, retaining named provenance and
scale units. A width-inside-slide check alone is too weak to prove Office
rectangles. Update only the fresh template or its fresh-constructor
normalization, preserving arbitrary imported metadata and geometry.

The final design must choose one explicit metadata policy. The issue permits
save-time timestamps or a deterministic constant. Avoid injecting current
time into to_bytes, which is currently the deterministic OPC writer at
lib.rs:969. A deterministic default is the smallest compatible approach,
subject to reviewing the fresh authoring API and existing metadata promise.
Fresh empty author fields versus absent fields must be stated and tested.
No overwrite of user-provided core properties at save.

This behavioural change must carry a separately labelled commit and exact
expected package/render delta. It may affect native, Python and WASM fresh
presentations. Coordinate exclusive hash baseline ownership with the root.
Do not hide it inside wrapper exposure or an asset regeneration.

## Ownership and files

X194 owns cell text frames. X197 owns shared conversion and all colour-format
and theme/brightness changes, including enum/stub migration. X206 owns alt
text, grouping, glued connectors, picture placeholders and general live-owner
handle contracts. X205 may expose new formatting only after their integrated
prerequisites and uses their revision validation unchanged.

Proposed implementation paths are existing rpptx-py text.rs, dml.rs,
table.rs, presentation.rs and slide.rs, their current handwritten stub and
enum shims, native rpptx/lib.rs and odp.rs as needed, and existing DrawingML
text/fill/table models for demonstrated gaps. The actual template asset is
crates/rpptx/assets/default.pptx. No new file or module is proposed. Claim
those paths serially with X206 and X197 because their semantic areas overlap.
Native PDF/export/model changes also need review with any active export story.

## Rejected alternatives

- Duplicate prerequisite colour/table APIs instead of reusing their actual result.
- Treat the issue's claimed native parity as proof where source inspection shows gaps.
- Drop unknown source or stale-path checks to make the missing API appear usable.

## Test plan

**Test gate**: regression.


Reproduce every missing operation against an installed Base. Record existing
to_notes_pdf as present and passing or actually defective, not a fabricated
missing-method failure. Source-build fresh documents and media controls.
Do not reuse a contributor output as the expected oracle.

Suggested tests in current test_documented_examples.py and native entrypoints:

| Test | Evidence |
|---|---|
| test_fresh_presentation_has_fresh_core_properties | Exact package metadata, user mutation and imported controls |
| test_fresh_master_and_all_layouts_use_office_widescreen_geometry | Authenticated rectangles, 16:9 slide and save/reopen |
| test_extended_run_properties_round_trip_through_python_pptx | Baseline, spacing, language and all script typefaces |
| test_numbered_bullet_attributes_preserve_unedited_choices | Scheme/start/colour/font/size, clearing and unknown controls |
| test_gradient_pattern_picture_and_alpha_fills_round_trip | Shape/slide owners, all fill choices and real rendered evidence |
| test_picture_fill_relationship_lifecycle_is_owner_scoped | Replacement/removal, shared image, cross-slide and no-orphan failures |
| test_table_style_and_six_banding_flags_round_trip | Native properties and correct conditional rendered regions |
| test_presentation_properties_and_sections_round_trip | All supported data types, invalid input and source preservation |
| test_media_insert_extract_replace_remove_preserves_poster_ownership | Actual lifecycle, parts/relationships and malformed atomicity |
| test_python_export_formats_use_native_deterministic_results | JPEG/TIFF, existing notes, handout, PDF/A and ODP reports |
| test_new_python_parity_getters_are_read_only | No property creation, transform changes or revision bump |

Reuse existing Python tests at :798, :1016 and :3870 for notes, :1494 for
bullets, :1627 for text parity, :2328 for fills/lines and :2412 for tables.
Use existing typing_smoke.py and _rpptx.pyi for installed strict typing and
stubtest. Native tests stay in lib.rs modules and current integration or
regression files. No new test binary. Add prefix-alias/default namespace,
foreign/rebound namespace, malformed bounds, exact raw sibling and twice
save/reopen controls for each promoted model field.

Visually meaningful tests render with deterministic fonts and use pinned
PowerPoint or LibreOffice evidence appropriately. A package round trip alone
does not establish render coverage. Existing notes and other exports may
reuse prerequisite regression evidence, but all acceptance items still need
an explicit capability/evidence mapping. PowerPoint open-without-repair is a
native application check, not a claim from python-pptx parsing.

## Risk routing

Matched rows: unit conversion, theme/colour transforms, parser/serializer,
layout/rendering, published native APIs, PyO3 bindings, dependency edges,
external oracles and packaged assets. Read HLD01, HLD03, HLD04, HLD05,
HLD06, HLD07, HLD08, HLD10 and HLD15, plus pinned differential testing.
Do not change the legacy Word tint implementation. Respect schema child
order, exact unsupported subtree preservation, real archive measurement and
size checks, WASM compilation and documented workspace binding exclusions.
No new feature flag is justified. Normal scoped gate and independent
microscope remain required.

Wrapper-only additions expect an unchanged default hash harness. The fresh
template correction is an intentional default behaviour delta and needs an
approved expected package/render change, a separate labelled commit and
exclusive baseline ownership. No automatic re-recording is authorized by
this proposal. Reconcile integrated archives from actual bytes, not tuples
copied from another worker.

## HLD impact

- docs/hld/02-scope-and-non-goals.md, accurate Python parity/export rows.
- docs/hld/05-drawingml-model.md, language and any new typed fill/colour contracts.
- docs/hld/06-presentationml-model.md, fresh metadata/template, table style and property contracts.
- docs/hld/07-inheritance-and-resolution.md, template geometry/default resolution changes if needed.
- docs/hld/08-rendering-spec.md, deterministic export contracts and actual supported output.
- docs/hld/10-bindings-spec.md, exact public signatures, return reports and handle behaviour.
- docs/hld/14-development-backlog.md, complete acceptance and prerequisites.
- docs/hld/15-build-and-toolchain.md, actual template provenance and packaged asset changes.

## Hash harness

Wrapper exposure should preserve existing defaults. Actual native semantic
changes require measured declared deltas and separate labelled commits before
any exclusive baseline update. The risk discussion above identifies those
possible changes. No baseline movement is approved by this design. Implementation waits for completed dependencies and exclusive source claims.

## Implementation checklist

- [x] Resolve technical contracts and actual native gaps against prerequisites.
- [x] Approve complete issue acceptance, source claims and measured output policy.
- [x] Claim only after actual prerequisites and file conflicts clear.
- [x] Prove installed public Base regressions before production changes.
- [x] Implement every requested capability, stubs and existing-entrypoint tests.
- [x] Run scoped gates and exact applicable oracle/risk riders.
- [x] Obtain independent zero-defect, zero-smell review and prepare handoff.

## Open questions

Resolved by the concrete formatting/fill/property/export contract below,
fixed fresh metadata and authenticated native geometry. Notes/handout absence
is a read-only synthetic-surface case, not malformed-owner fallback. Exact
common JPEG/TIFF/ODP result signatures and same-provider font inputs are
selected. Language/gradient Base failures, output comparison and package/hash
review remain required implementation tests, not claims of completed parity.
No outstanding user product choice or speculative new source module remains.

## PR318 contribution intake

PR [318](https://github.com/tensorbee/rdocx/pull/318) entered rolling intake at
immutable head b1aa842bf531a65c70a49d4ef282033d26e64020, based on733ce6f7.
It closes Issue308 and is attached to existing F-X205. It contributes text,
fill, table, metadata, media and export binding surfaces plus the existing
16:9 template repair. Its stated App/custom-property omission remains part
of F-X205 acceptance. Shared colours retain F-X197 ownership, cell text
retains F-X194 ownership and structural-handle improvements retain F-X211
ownership. Assess all24 changed files and the actual native template before
reuse. Contributor tests, archive values and artifact hashes are claims to
verify locally, not completed sprint evidence. No additional F-ID or asset
inventory is created by this contribution.

## Residual metadata and fill contract

## Fresh metadata policy

Choose a deterministic fresh-authoring default, `2000-01-01T00:00:00Z`, for BOTH created and modified. Author/creator, last_modified_by and description are absent. No ambient clock, user account or inherited python-pptx author is supplied. Set these explicit values on the fresh default-template authoring route only, so native Presentation::new and Python Presentation() agree. This is a fixed timestamp, not the empty dates currently asserted by PR318 tests. Existing Issue308 permits deterministic dates instead of save-time clock values. Document that new presentations use deterministic metadata by default even when a renderer's font mode is not selected.

Ordinary save does not update either date or lastModifiedBy. User-authored dates remain exact existing native String values and imported presentations are never normalized. A caller can set new dates explicitly through core properties. No new save hook, datetime dependency or deterministic feature flag is needed. Default_template core.xml can carry the same neutral values, but do not add a second normalization path that also modifies imported packages. Constructor and save/reopen tests assert exact timestamps, repeat-byte determinism, absent stale metadata and retained imported/user override values. Fresh geometry and fresh metadata are separately labelled intentional behavior changes with measured expected artifacts, not copied contributor hashes.

PR318's default template currently has an empty coreProperties root and its tests require None dates, captured assessment section Template projection. Amend those expectations to the approved explicit timestamps. Do not call absence a deterministic date or silently revise actual308 wording.

## Additive native property facade

Use existing oxml-core CoreProperties/AppProperties/CustomProperty/CustomPropertyValue models and existing rpptx lib.rs. Proposed additive facade:

```rust
Presentation::set_core_properties(&mut self,properties:CoreProperties)->Result<()>;
Presentation::remove_core_properties(&mut self)->Result<Option<CoreProperties>>;
Presentation::application_properties(&self)->Option<&AppProperties>;
Presentation::set_application_properties(&mut self,properties:AppProperties)->Result<()>;
Presentation::remove_application_properties(&mut self)->Result<Option<AppProperties>>;
Presentation::custom_properties(&self)->&[CustomProperty];
Presentation::custom_property(&self,name:&str)->Option<&CustomProperty>;
Presentation::set_custom_property(&mut self,property:CustomProperty)->Result<()>;
Presentation::remove_custom_property(&mut self,name:&str)->Result<Option<CustomProperty>>;
```

These are proposals, not methods already provided by PR318. Existing core_properties/core_properties_mut at native lib.rs:1927/1932 can remain compatible, but new Python writes consume checked set/remove operations instead of mutating dirty state before fallible serialization.

Resolve actual package-root relationship targets, not hardcoded docProps paths. Preserve current owner part if present. On absent ownership, allocate the conventional part only if unoccupied, with exact content type and package relationship, refusing producer-part collision or ambiguous duplicate relationship ownership. Preserve relationship IDs and part names on updates. Remove only the requested owned property part, relationship and content-type override, with no unrelated part deletion. No-op missing removal does not mutate.

AppProperties already contains private child order, raw XML and namespace preservation, app_properties.rs:152. Clone the latest full model, change one field and stage its serialization, retaining those internals. Validate application_version against the existing Word XX.YYYY rule, Word document.rs:25919, because both consume the same OPC property. Keep actual optional field types, including cached pages/words/counts versus measured layout facts. Do not normalize retained cached counts or replace absent strings with defaults.

Custom property identity and typed mapping follows the reviewed X203 contract: Lpstr/Lpwstr, I4 signed32-bit, finite new R8, Bool, explicit lexical FileTime, Empty and read-only Raw. Preserve existing name, PID, fmtid and order on value update. Allocate fresh PID>=2 without collision on add. Case-sensitive names, invalid XML name/value rejection, bool-before-int conversion, missing-delete no-op and unsupported Raw preservation are required. Native FileTime is String with no timezone normalization, oxml-core/custom_properties.rs:34/403. Share approved Python CustomFileTime/immutable CustomProperty result semantics rather than create an incompatible PPT-only datetime rule.

Every property operation clones, validates all inputs, updates its owner, serializes/reopens and uses commit_candidate, native lib.rs:2553, only after complete success. Metadata edits do not change slide/shape/table/text topology and therefore bump neither structural nor text generation. Held content handles and property proxies remain usable on success/no-op, and all bytes/generations remain unchanged on failure.

Core preservation needs explicit care: CoreProperties at oxml-core/core_properties.rs:10 contains known values but no opaque child/order fields, unlike AppProperties. Whole-model to_xml therefore cannot by itself preserve unknown imported core XML. Prefer a qualified retained-core-part patch in native rpptx existing file: compare old/new known fields, replace only changed qualified owned children, preserve all other bytes/namespace context and append/remove in valid OPC property order. Refuse ambiguous duplicate owned fields or wrong namespace rather than patch by local spelling. Keep the native model consistent via staged reopen. Do not add binding-owned XML writes or assume the existing mutable core accessor already guarantees opaque preservation. If a shared CoreProperties model preservation change is chosen instead, state its public literal impact and coordinate it with Word, rather than silently add a private struct field that breaks existing literals.

## Python live property owners

Proposed `Presentation.core_properties`, `.app_properties`, `.custom_properties` are concrete document-owned live proxies in existing presentation.rs. Each holds originating PyPresentation, reads current native values, clones latest native model for setters and calls checked facade operations. This matches Word PyCoreProperties at rdocx-py/document.rs:1305, not a detached mutable snapshot pretending to affect the deck.

Core properties keep python-pptx field names including author/comments/last_modified_by/created/modified, mapped deliberately to creator/description. App fields use existing native optional types. Custom get returns immutable property snapshot, not None for native Empty, and get absence is None. Expose get/set/delete plus immutable ordered iteration. Getters never create parts or dirty a presentation. Replacing/removing a metadata part does not invalidate its owner proxy. Strict stubs identify actual optional/string/date adapter semantics before implementation.

Reuse X197's one RGB extractor and live ColorFormat source matrix for gradients, pattern foreground/background, bullet colour, font/line/shadow and alpha. Do not retain PR318's separate text.rs color_argument parser. Its raw lang accessor/set_language already edits preserved lang in place without a public struct field, oxml-drawing/text/paragraph.rs:835/844, so adopt that minimal model approach subject to qualified unknown-attribute controls. The proposed new450-line language enum source and direct oxml-drawing dependency in PR318 remain unapproved additions here. Reuse existing enum/stub surfaces or explicitly reconcile their provenance before any file/dependency adoption.

Every new JPEG/TIFF/PDF-A/notes/handout/ODP render or measurement entrypoint uses the integrated X195 fonts/font_dir/font_dirs options and requested/actual face diagnostics. Native/Python deterministic defaults remain exactly the existing no-input behavior. Do not install fonts, bypass variation selection or call a no-font wrapper from a caller-font API. Notes PDF already exists, so test its actual parity rather than claim a newly added missing capability.

## Picture fills, media and pruning transaction

PR318 add_slide_image at captured lib.rs:3461 and release_unused_slide_images at3480 are reusable allocation/reference/pruning ingredients. Their binding composition is not atomic: dml.rs:535 allocates first, then writes fill, while write_fill at141 publishes before fallible cleanup. Replace that sequence with one native checked transaction in existing rpptx lib.rs.

Proposed `set_picture_fill(&mut self,owner:&FillOwnerLocation,image_data:&[u8],filename:&str)->Result<()>` and `set_fill_at(&mut self,owner:&FillOwnerLocation,fill:Fill)->Result<()>`. FillOwnerLocation is the existing concrete qualified owner route or an explicitly approved minimal location reused with X206/X209, covering slide background, shape and table cell, with owner fingerprint/path. No generic dynamic owner registry. Lines/cell borders refuse picture fill before media allocation.

The candidate resolves actual owning part, imports media through existing MediaStore, allocates the owner relationship, sets the correct BlipFill/Stretch source, scans current retained references, removes only unused owned image relationships and unreachable parts, refreshes its media store, then serializes/reopens and commits once. No relationship from another slide is borrowed. If current owner source/reference scan fails, the original presentation remains unchanged. Shared references from picture, fill, background or other part keep the media alive. External linked fills are not fetched. Same existing fill assignment is a defined no-op if complete source/metadata equivalence is proven.

Do not use guessed string search for reference pruning. PR318 cleanup already relies on owner reference discovery and reachable-part pruning, captured lib.rs:3480, but move it inside the candidate before publication. Retain exact native unsupported/malformed refusal and content-type/signature rules. Each later failure test compares complete original package bytes, media list and handle generations.

Native add_media already stages ownership and settings, captured lib.rs:4359. Reuse the full existing signature, including source input, poster, bounds, playback and owner-aware relationship graph. Python add_movie-style convenience parses all input before mutable access and returns a newly captured shape only after successful insertion. Insertion/removal is structural and follows X206 generations. Replacement with the same logical shape retains its shape owner where the reviewed native operation does, while text descendants are not bumped by a media-only replacement. Removal must prune only unreferenced media/poster parts and retain shared relationships. Root integrates tested X206 ownership once, not a parallel implementation here.

## Stable gradient stop owners

Do not adopt PR318 GradientStops::restructure's unconditional whole-presentation revision bump at dml.rs:770/776. It stales the collection and unrelated slide/shape handles and cannot distinguish identical stops through index changes. Fill owner and GradientStops collection remain live across append/delete/reorder of that same gradient. Existing stop/color handles remain bound to their original logical stop across edits of other stops. A removed stop fails loudly and never binds the next stop at its old index.

X211 owns this stable-stop identity adaptation. Use one concrete per-fill transient stop-identity vector in existing PyPresentation state or its approved existing owner state, indexed by the checked fill owner. Assign monotonic nonreused IDs when the stop inventory is first captured, maintain them in exact parallel with the native stop vector on successful insert/remove/move, and let stop/color handles retain the token rather than only an ordinal. No OOXML runtime ID attributes are written, no matching by position/colour/raw values, and no generic registry/trait is introduced. Clear/reseed identity only on a genuine fill-kind/whole-gradient replacement, making old stop handles explicitly stale while the FillFormat owner survives.

Property edits to stop colour/position preserve identity. If position changes reorder stops, reorder the parallel IDs in the same transaction. Duplicate positions/colours remain distinct by token, and append places a new duplicate after existing equal-position stops, matching the contributor's insertion comparison at dml.rs:813. Delete below two stops refuses before mutation. New stop is returned after successful native update with its own fresh token. Failures/no-ops do not advance IDs or revision. Shape topology removal remains governed by X206/X211 checked owner paths, which invalidate removed owners before token resolution. Coordinate one final state shape with X211, not two divergent stop-lifetime implementations.

## Missing native template and export oracle

Retained PR318 template comparison proves master/layout geometry against the authenticated native baseline, but that native baseline has no notesMaster part, assessment Template projection. Its notes-master rectangle changes therefore lack actual native source provenance. Ask the existing root/bindings PowerPoint oracle owner to create one controlled new16:9 deck, add nonempty speaker notes to one slide, save normally, freeze package/app/action hashes, close/reopen/save without edits and freeze again. No external/user presentation is involved. Inventory native notesSz, notesMaster and notesSlide placeholders by qualified type/idx, their xfrm and notes master/theme relationships. Compare slide-image aspect and body/footer bounds against native notes-page render, not the slide's16:9 dimensions alone.

This control needs notes page layout or PDF output to prove visible notes geometry. A plain saved notesMaster part is structural evidence only. Native handout successful export needs an actual native handout owner/control separately. PR318's Python export tests check mostly headers/nonempty bytes and absent-master refusal, assessment Contributor test limits, so they cannot establish successful notes/handout/JPEG/TIFF/PDF-A rendering acceptance. Pinned LibreOffice26.2.5.2 and deterministic native outputs remain required, with exact caller-font face/version receipts.

Pattern/path gradient rendering is still unsupported according to contribution assessment. A structure-only fill round trip cannot satisfy actual308 render acceptance. The writer must implement the approved native render support or record an explicit scoped diagnostic boundary before complete acceptance, with actual visible pixels/control comparison. Google Slides best-effort import remains distinct from native render evidence.

## Required verification and ownership

No local PR318 pass is claimed. Capture each genuine missing installed Base operation first, reuse X194 cell resolver/X197 colours/X195 fonts/X206 live owner policy, then adapt contributor slices under the sole writer claim. Test fresh fixed-date geometry separately from imported properties, opaque owner XML, malformed atomic allocation/prune failures, repeated identical stable stops, strict typing/stubtest, real export pixels/page count and native open/save/reopen. Contributor archive tuples and default hashes are foreign evidence, not values to copy. Actual affected members/sizes and expected artifact/hash deltas are measured under root's lease after accepted local source. No new module, file, crate or dependency is authorized by this appendix.

Canonical read HEAD: `4c2423b53735c833dc7fbb3910909389ea80922e`. Source SHA receipts:

- `crates/rpptx/src/lib.rs`: `9e203e0e9e3fe71c8b0c9d070a1a0cffa6f757fbd849c88c46702ed910996b6b`.
- `crates/oxml-core/src/core_properties.rs`: `32eac8c83f4b1f736260376c256d14005994a5e7df2c2541f95b6a65cea79455`.
- `crates/oxml-core/src/app_properties.rs`: `176cd6dae588964906821d7e1d5a79e578acac1ddb2c2162aa76652aae9f3daa`.
- `crates/oxml-core/src/custom_properties.rs`: `201fdf81d0615c2cc2d2a64e6f09bab1eadef72b0af879a2710fc46848aaa8cf`.

## Independent native notes geometry comparison

An authenticated PowerPoint16.113.3 control with an actual entered speaker
note was saved, normally reopened and saved without edits. The notes master
and notes slide owners remained structurally identical across that reopen.
The saved deck SHA256 is438f65214a485dd9dbe6e7978be0120ca9b0f43e05573b19938e0488f2f6debf.
Root independently projected all six qualified notes-master placeholders
against the immutable PR318 template. Slide-image idx2 geometry exactly
matches offset685800/1143000 and extent5486400/3086100. Body idx3 exactly
matches offset685800/4400550 and extent5486400/3600450. Both notesSz values
are6858000/9144000. Header/date/footer/number heights differ by1587 or1588
EMU, so the entire master is not claimed identical. Keep those observed
native values as the comparison control, with source namespace and absence
facts retained. The genuine notes slide has no direct placeholder transforms.

Full projection and input hashes are in
.claude/scratch/S91-PR318-native-notes-comparison.json,
SHA25613106840fe671a24696e933d8fbf6f5c055a05297d43c94904431b85a34102d5.
This fills the structural provenance gap for the key slide-image/body
rectangles. It does not establish notes-page visual output, handout export,
renderer parity or full acceptance. Those implementation gates remain due.

## PR318 incremental head refresh

The contribution advanced to bfdd6ab732e8c0003ea8c6110e3c891fb098d5b3.
Its one additional commit changes only the F158 PowerPoint artifact SHA
constant in crates/rdocx/src/document.rs. Complete source is retained in
.claude/scratch/S91-PR318-bfdd6ab-source, with Git blob3dc84d1eb439407426a8d48b67303e938316cecd,
1674441 bytes and SHA256ed032cbd76c76d260758d1cac37294b77e5bc22d8bc2c37c0aa7a497d7a7cfab.
Prior24-file assessment remains valid for unchanged source. Recompute the
actual affected local authored artifact after reviewed template adoption.
A foreign expected hash is not evidence that our artifact passes. Preserve
immutable cross-viewer control hashes and separately justify any current
authored-output expectation update. No source or baseline adoption occurred.

## PR318 second incremental head refresh

PR318 advanced to b8e212f8db1841a5533646aa7190847724c73733 with one additional
commit and one path beyond bfdd6ab: crates/rpptx/tests/integration.rs. The
complete1069136-byte Git blob95945f1d397581cb377e67179ca12417a2aa542b has
SHA2562c1b4f0a94c6c319b01e9393a126e06ca37c731289ac257eced840b111d56f58.
The complete non-truncated head tree authenticates its path/size. Exact
metadata, complete incremental diff and source assessment are retained in
/private/tmp/S91-PR318-b8e212f-intake. Prior24-file source assessment remains
valid. The new full PR base diff has25 paths. No new F-ID is added.

The sole additional edit updates M21_CURRENT_MINIMAL_SOURCE_SHA256 for the
currently generated SmartArt fallback deck. Existing Unsupported SmartArt
text and single ink-band checks remain unchanged. This is another derived
current-template expectation. Independently regenerate the actual local
current artifact after accepted template and fixed-date metadata changes,
then justify and review its expectation. The foreign7d19b5b4 hash is not a
passing local result. Legacy unsigned/static and corrected signed native
manifest constants remain immutable and are untouched by this commit.
No rendering code, baseline or source adoption occurs in this intake.


## Actual native notes and handout positive exports

The bounded PowerPoint16.113.3 study now includes actual local Print to PDF,
using Notes and Hand-outs with two slides per page. Evidence is retained in
/private/tmp/S91-F-X205-native-notes-handout/study.md and receipt.json.
Root reverified all45 retained artifact sizes and SHA256 values, plus the
original source SHA438f65214a485dd9dbe6e7978be0120ca9b0f43e05573b19938e0488f2f6debf.
The actual two Notes PDFs each contain two A4 portrait pages, with the real
speaker note below the first slide image. The two Hand-outs PDFs each
contain one A4 portrait page with both slide frames. Each mode's actual page
content streams and extracted text are identical after normal reopen/save.
Notes owner XML and relationships remain identical. No notesSlide2 or
handoutMaster is saved. Native printing synthesizes the blank second notes
page and a default handout surface. Thus the earlier structural-only notes
control now has a positive visible-output counterpart, while successful
native handout printing does not imply a populated handoutMaster part.

The measured notes slide frame is x73.9/y141.625/w447.2/h251.4375 points.
The handout frames are x58/y123.5/w480/h270 and x58/y447.5/w480/h270.
The actual speaker-note body uses Aptos2.0099945 at12pt horizontally, with
native print scaling recorded separately. Arial6.8000031 and handout
Calibri6.1999969 are authenticated from the actual embedded PDF fonts.
These paper/print frame measurements do not replace master EMU geometry.
Do not copy native print dates, fonts or PDF bytes into deterministic baselines.
Use the authenticated owner graph, notes text, page counts and visible
regions for implementation controls, then measure accepted local output
with the shared X195 caller-font provider.

The current render_handout_pages requires a related handoutMaster at
crates/rpptx/src/lib.rs:10241. Native successful default-handout printing
without that part establishes a distinct default-surface compatibility case,
not a passing local result. The approved final technical contract below resolves this absent-owner case,
while preserving malformed existing-owner refusal and imported master
geometry. The study performs no renderer comparison and covers only this
two-slide deck. The original exact-line-spacing overflow remains visible
inside its slide image. Other export and rendering gates remain due.

## Approved F-X205 final technical contract

Close questions2,3 and5 from the already resolved appendices. Fresh native/Python presentations use created=modified="2000-01-01T00:00:00Z", with creator/author, lastModifiedBy and description absent. Save never injects an ambient clock or replaces imported/user fields. The additive checked set/remove property facade at plan:310-318, retained-core XML patch at329 and live property owners at333 are the chosen policy. Custom property value, PID/fmtid/order and FileTime semantics reuse X203. Solid-fill transparency belongs to its alpha transform, not a new RGB conversion. X197 owns colours and brightness, X195 owns all caller-font entrypoints, X194 owns cell text, and X211 owns stable gradient-stop identity. F-X195 is a formal font-provider dependency. X205 owns its stable gradient-stop identity once, with the shared X211 contract defining later corpus validation rather than creating a dependency cycle. Do not create a new colour parser or lifetime implementation.

Close question1 with the contributor's concrete established spellings, adapted to checked operations and existing enum files:

- Font.baseline: float|None, finite fraction[-1,1], truncation to DrawingML100000 units. Font.spacing: Length|None getter, int|None setter in existing EMU convention, checked conversion to text hundredths of a point. Font.language: str|None, Font.east_asian_name and complex_script_name: str|None. Clear only the selected field. Validate XML text and preserve opaque attributes. Do not adopt the new language enum module. Latin uses existing Font.name.
- Paragraph.auto_number: the complete existing DrawingML TextAutoNumber scheme strings, exposed as a Literal in the existing stub, not a new enum module. auto_number_start: int|None with schema range1..32767. bullet_color uses X197's RGBColor/hex extractor, bullet_font str|None, bullet_size finite fraction0.25..4.0|None. Clear removes only that owned property. Choice edits preserve colour/font/size. Imported point-size bullets remain intact on read and unrelated edits, with no claim that None proves follow-text when the raw kind is points.
- FillFormat retains solid/background/gradient/patterned/picture(image_file), pattern enum in the existing enum file, gradient_angle finite degrees, gradient_path Literal[circle,rect,shape]|None, gradient_stops with append/get/delete, and stop.position finite[0,1]. None path means linear. Existing stop identities and whole-fill replacement follow the plan:355 contract. Add FillFormat.transparency: float|None, finite[0,1], defined as1-opacity for a solid fill. Setter requires existing solid fill and replaces only its alpha transform. Getter never creates a fill. Preserve line/shadow semantics.
- Core/app/custom proxies retain the plan:333 names and actual native optional types. created/modified use native lexical String values, not an implicit datetime/timezone conversion. Core author maps creator and comments maps description. Custom get returns immutable typed property|None, set/delete are checked, iteration immutable and ordered. Section getter tuple[Section,...], set_sections(list[tuple[str,list[int]]]) reuses current native validation and stable slide-ID conversion.
- ShapeCollection.add_movie(movie_file,left,top,width,height,poster_frame_image=None,mime_type=None)->Shape uses existing native add_media settings and defaults. Slide.media->tuple[MediaInfo,...], extract_media(shape_id)->bytes|None, replace_media(shape_id,media_file,mime_type=None)->None and remove_media(shape_id)->None remain owner-aware. All input validation, fill allocation and reference pruning precede one native publication.

Close questions3/4 export signatures using the captured PR318 stub: render_slides(*,dpi=150.0,format="png",quality=None,transparent=False,slides=None), PNG/JPEG returns list[bytes] with one image per selected slide, TIFF returns one multipage bytes value in selected order. This format-dependent return is explicit in overloads, not hidden. Existing single-slide PNG remains compatible. Add minimal checked JPEG/TIFF encoding in current native lib.rs using existing image support and shared layout/font pipeline. JPEG quality applies only to JPEG, transparency refuses for JPEG, invalid format/DPI/quality/selection fails before rendering. to_pdfa(profile="pdfa-2b")->bytes supports exactly pdfa-2b/pdfa-3b. to_handout_pdf(slides_per_page=6)->bytes and render_all_handouts(slides_per_page=6,dpi=150.0)->list[bytes] support exactly1,2,3,4,6,9. Existing to_notes_pdf/render_all_notes remain. Add the integrated X195 keyword font controls to every rendering route. Preserve report-bearing native OdpReadResult/OdpWriteResult: from_odp(path|bytes)->tuple[Presentation,list[tuple[path,message]]], to_odp()->tuple[bytes,list[tuple[path,message]]], save_odp(path)->list[tuple[path,message]], retaining ordered diagnostics exactly, not bytes-only convenience that discards loss.

Close the real default-surface edge: notes/handout export is read-only and succeeds when a legal owner is absent. Existing valid owner geometry/theme is authoritative. An absent handoutMaster creates an in-memory neutral handout surface using existing HandoutLayout and export_page_size/handout_targets, not a saved package part. An absent notesSlide yields a blank note body for that slide, as already supported. If the whole notesMaster is absent, use an in-memory fresh notes surface from the approved native-authenticated notes geometry. Do not mask a present malformed, duplicate, dangling, wrong-content-type or external owner as absence. Preserve refusal for those cases. Use presentation notesSz for logical page dimensions, rather than importing the macOS A4 paper setting. No ambient print date is injected. Synthetic defaults must not mutate to_bytes, metadata or owner generations.

Geometry is resolved from the authenticated template comparison and notes comparison, not page containment guesses. Fresh master/all11-layout rectangles use the authenticated Office16:9 projection. Notes slide-image/body exact rectangles are recorded at plan:389-394. For differing header/date/footer/number heights, choose the recorded actual native values rather than claim PR318 byte equivalence. Imported geometry is unchanged. Native Print Notes/Hand-outs now supplies positive visible evidence: two notes pages including one blank-body page and one two-frame handout, even with no notesSlide2 or handoutMaster. This closes the provenance question. It does not prove local rendering.

Question6 is an implementation acceptance decision: implement actual requested supported linear/path gradient and pattern rendering plus baseline/spacing/script-font/language propagation where current output demonstrably lacks it. Structure-only round trips and Google Slides advisory degradation do not satisfy native visible acceptance. Keep explicit diagnostics for imported unsupported cases without silently reducing Issue308 requested authoring support. Base repro, pixels, fonts, export signatures, typing, native reopen, package/hash deltas and gates remain implementation obligations.

## Latest PR318 head equivalence

PR318 now points to545cba61206b9a4535bce1e5a77cae7f9f405ebd. Its entire immutable tree
c48923347b2f8d7eb9a95d4fd9f72a70252f7aaf is identical to the previous assessed head,
with the same parent commit. Diverged final commit identity introduces zero
source changes. Full current discussions and reused source blob hashes are
authenticated in /private/tmp/S91-PR318-545cba6-intake/receipt.json.
The prior source assessment and existing feature ownership remain applicable.
No duplicate work, asset approval, baseline or contributor pass claim changes.

## Further PR318 replacement head

The latest head1eab7066b69203e36df3782a2d343ff1ae1defd2 replaces545cba6
with the same full treec48923347b2f8d7eb9a95d4fd9f72a70252f7aaf and
the same parent. Direct tree comparison has zero changed paths. Complete
metadata and reauthenticated25 main-base head blobs are recorded in
/private/tmp/S91-PR318-1eab706-intake/receipt.json. Root verified all25
source identities and17 retained artifacts without mismatch. Prior complete
source assessment and this approved contract remain valid. No duplicate
implementation, revised baseline or new F-ID follows from commit identity
alone. The unchanged contributor chart-hash comment remains a claim rather
than a passing local oracle.

## PR318 advanced-head reconciliation, 2026-10-10

Immutable head `4fc65a50679e6afd12ea6d99b218007cad7d6e5d` replaces
`1eab7066b69203e36df3782a2d343ff1ae1defd2`. Root authenticated the full
capture manifest and all27 source records by Git blob identity, bytes and
SHA-256, then inspected the exact two-path delta. Assessment and receipt are
in `/private/tmp/S91-PR318-4fc65a5-intake`.

Keep Issue308 with F-X205 and colour with F-X197. Preserve the approved
integer/Length EMU truncation contract. The contribution's sub127EMU refusal
cannot distinguish typed Length from an equal integer and its signed abs can
overflow at i64 minimum. Validate before mutation without a magnitude-based
unit guess. Add0,1,126,127,-126,-127 and typed equivalents, plus i64 minimum
and maximum refusal controls with unchanged package bytes and live tokens.
The prior template, export, diagnostic and provider acceptance barriers remain.
No contributor source, tests or gates are accepted by this reconciliation.
