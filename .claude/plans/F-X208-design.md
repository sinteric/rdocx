# F-X208, Author PowerPoint footer fields themes and transitions

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X197, F-X206

The design is approved. Implementation waits for completed dependencies and exclusive source claims. Issue311 is
owned here. Issue312 theme editing remains F-X209 and shared one-shot CLI
commands remain F-X204. F-X197 is the approved colour-format prerequisite.
Do not reinterpret PR315 caller-font metadata, font ownership or rendering
claims as footer evidence. Its intake evidence remains unchanged.

## Problem

- Issue311 evidence is `.claude/scratch/S91-intake-issues.json`, updated
  2026-10-09T18:42:46Z. Acceptance requires the default template and a branded
  potx, one Python call and one CLI call producing slide numbers plus footer
  with title-slide suppression, correct PDF output and native PowerPoint
  opening without repair. It also requires date modes, placeholder format,
  theme reading and the complete requested common transition set.
- `crates/rpptx/src/lib.rs:3253` creates slides. At 3272 it unconditionally
  excludes dt/ftr/sldNum and only copies type/index for other placeholders.
  A hf-aware route must clone sufficient existing placeholder geometry,
  properties and text formatting, preserve idx inheritance and allocate
  fresh slide-local shape IDs. Copying foreign relationship-bearing content
  requires owned dependency remapping or explicit atomic refusal.
- `crates/rpptx-layout/src/context.rs:286` and 291 derive independent master
  and layout latent policies. At 3670 an inherited placeholder is emitted
  only if its source hf permits it, it is occupied, and no deeper matching
  placeholder already wins. This is the actual present renderer behavior,
  not a confirmed PowerPoint rule. HLD06:1101 presently requires these latent
  placeholders to be inherited and never cloned. This section must change
  only after actual native comparison.
- `crates/rpptx-layout/src/context.rs:3815` defines occupancy from nonempty
  plain text. An uncached slidenum latent field may be filtered here before
  `crates/rpptx-render/src/text.rs:156` can substitute page_number. That
  renderer already substitutes slidenum regardless of cached text, so a
  duplicate field evaluator is unnecessary. Inspect inherited masking and
  duplicate-selection occupancy together with the visibility fix.
- `crates/rpptx-oxml/src/presentation.rs:152` does not type firstSlideNum.
  Root attributes remain opaque. Add presence-sensitive typed handling at the
  model boundary, and pass the displayed number separately from physical
  page identity to static and timeline render paths. Zero is a supported
  input. Do not change physical page numbering, selection, or output naming.
- `crates/rpptx-oxml/src/slide_parts.rs:201` CT_Slide has no p:hf. Layout and
  master roots have it. A per-slide public header/footer state is derived
  from slide-owned placeholders and fields. Do not serialize an invalid
  slide-root p:hf. Native capture must establish which legal master/layout
  settings the dialog writes for deck-wide and per-slide operations.
- `crates/oxml-drawing/src/text/paragraph.rs:1359` already models a:fld,
  GUID/type/properties/optional text and opaque children. Add checked field
  construction in this existing module and facade/Python paragraph routes.
  Allocate GUIDs uniquely and deterministically under the existing identity
  convention. Preserve run formatting and paragraph schema child order.
- `crates/rpptx-oxml/src/timing.rs:1175` has typed Cut/Fade/Wipe/Push/Zoom/Morph
  but Split/Cover/Uncover are Other. CT_SlideTransition at1198 retains raw XML,
  and to_xml at1316 returns raw_xml, so changing public fields alone will NOT
  persist transition edits. Existing set_speed and morph editing demonstrate
  retained-XML patching. Author canonical supported transitions with checked
  properties, and preserve unrelated children and extensions when editing.

## Mandatory native oracle before renderer changes

Microsoft PowerPoint was located through read-only CUA getApp. It is at its
Home/open-recent chooser, with no deck opened by this task. No menu, document
or UI mutation was performed. Pin the app version/build before capture.

Prepare finite synthetic decks outside tracked source and record hashes and
full package provenance. Use footer/date/number strings unique by owning
master/layout/slide and identical geometry so native PDF text and raster
visibility distinguish ownership. Use a normal and a title layout.

1. No slide-owned latent placeholders, occupied master placeholders only.
   Compare absent hf, empty hf and each explicit enabled/disabled flag.
2. Occupied layout placeholders only with the same hf presence/flag matrix.
   Include conflicting master/layout flags and layout showMasterSp false.
3. Both master and layout placeholders with distinguishable strings and
   matching versus distinct idx. Include empty and uncached-field variants.
4. Add slide-owned placeholders with master/layout flags disabled, cached and
   uncached slidenum, and firstSlideNum absent, zero and a nondefault positive
   number across at least two slides.
5. Native new-slide action on templates with each relevant hf state. Inspect
   generated slide XML, copied fields, cached text, IDs and placeholder idx.
6. Native Header and Footer dialog Apply and Apply to All, with footer/date
   fixed/automatic/off and title suppression. Save and reopen normally, then
   export native PDF. Inspect legal master/layout flag locations, per-slide
   ownership, generated datetime type and title identification.

Freeze before/native-saved/reopened outputs and actual action receipts. Compare
native PDF text and page rasters with deterministic rpptx renders of the same
immutable deck. Opening alone is not rendering evidence. A save can rewrite
or materialize placeholders, so distinguish original versus native-saved
package behavior. No hf-only conclusion or renderer baseline delta is approved
until these controls establish the actual native rule.

## Spec reference

- `docs/hld/06-presentationml-model.md`, Adding a slide, Placeholders and Public facade.
- `docs/hld/08-rendering-spec.md`, Text in a shape and displayed field values.
- `docs/hld/10-bindings-spec.md`, Python API shape, Native PowerPoint text layout and CLIs.
- `docs/hld/02-scope-and-non-goals.md`, explicit transition-authoring boundary.
- `docs/hld/14-development-backlog.md`, F-X208 and Issue311 acceptance.

## Approach

Use one concrete header/footer options value with slide number bool, footer
optional text, date off/fixed/automatic-format and skip-title bool. Native
Presentation and checked SlideMut methods stage all selected slides and
required legal package edits, validate identities/fields and publish once.
The Python `prs.set_header_footer(...)` and slide.header_footer interface use
those same operations. Validate wrong types, unsupported date formats and
invalid XML text before mutable access. Public applied state must distinguish
absence from false, fixed from automatic and local override from inheritance.

Title suppression should use the native-confirmed layout/placeholder rule,
not slide index zero or a display-name string assumption. Repeated calls must
be idempotent, preserve unrelated slide-owned shapes and avoid duplicate
latent placeholders. Off removes only the identified owned footer field
placeholders, never an ordinary textbox with matching text. A template with
missing footer geometry needs an explicit documented synthesis or atomic
refusal contract, decided from the default/branded acceptance captures.

Python shape.is_placeholder and placeholder_format.type/idx should reuse
existing ShapeRef/placeholder collection identity and stale-handle rules.
PP_PLACEHOLDER follows existing python-pptx names and integer values.

Theme reading resolves master through presentation relationships, then theme
through that master's relationship, with multi-master tests. Expose name,
twelve resolved RGB scheme slots and major/minor latin/EA/CS fonts. Reuse the
amended F-X197 RGBColor/ColorFormat and theme vocabulary. Reserve owner identity
by master part, not a detached first-theme cache or slide index. X209 later
adds mutation and apply_theme, including theme write-back and lifecycle.
Do not create a second incompatible theme proxy that X209 must replace.

Transitions use none/fade/push/wipe/split/cover/uncover/cut/zoom. None removes
the selected transition, not all animation timing. Validate effect-specific
direction vocabularies, finite nonnegative durations and advance-after values,
with checked conversion to milliseconds. Preserve presence-sensitive click
advancement. Apply-to-all stages every selected slide and rolls back if any
refuses. Unknown existing effects remain preserved and readable with explicit
unsupported metadata. Add typed model/render support for the requested missing
effects rather than pretending Other is implemented. Static output should
remain unchanged except reviewed footer/number fixes.

Footer CLI owns `rpptx footer FILE --slide-number --footer TEXT --date
auto|TEXT|off --skip-title` plus established safe output selection and refusal
of input overwrite. All operations must be Python-visible. X208 also extends
existing inspect text/JSON with masters, layouts by master, theme colours/fonts
and hf presence/flags. Keep existing metadata and slide details compatible.
F-X204 owns other general CLI commands and consumes this surface.

## Rejected alternatives

- Write p:hf on the slide root, which has no such legal modeled element.
- Add another slidenum evaluator when the existing evaluator already works.
- Infer PowerPoint inheritance from LibreOffice behavior without native capture.

## Test plan

**Test gate**: regression.


Before implementation, fresh installed public Base regressions for every
missing Python operation and native source-built regression for each reported
rendering defect. Reuse existing test modules and entrypoints, no new binary.

Required matrix includes default/branded potx, multiple masters, title/normal
layouts, missing/empty/conflicting hf, uncached/cached field values, index-zero
displayed numbering, repeated calls, per-slide overrides and deck-wide atomic
failure. Save/reopen must retain qualified names, raw producer children,
relationships, GUID uniqueness and placeholder idx. Compare field text and
PDF/PNG output under deterministic fonts with pinned PowerPoint captures.
Transitions require every effect/direction, boundaries, malformed input,
all-slide rollback, retained unrelated timing and supported timeline samples.
Python stubs, strict mypy and stubtest cover all types and held/stale handles.
CLI help/reference, footer and inspect compatibility tests use existing
integration entrypoint. Google Slides import is best effort, with honest
notes on automatic dates and hf-only compatibility, not an inferred CI pass.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/05-drawingml-model.md`
- `docs/hld/06-presentationml-model.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

HLD sections: 02 specific transition-authoring non-goal, 05 DrawingML field
and theme API, 06 facade/footer rules and transition persistence, 08 displayed
number/latent visibility and timeline effects, 10 Rust/Python/CLI surfaces,
12 exact oracle matrix and 14 complete story contract. Review each section
against the final planned diff before approving this list.

## Risk routing

Matched rows: parser/serializer HLD04+06, raw subtree byte preservation and schema
ordering. Rendering HLD08 deterministic fonts and explicitly declared harness
delta after native proof. Public API HLD10 semver, full local patched package
dry run and archive limits. PyO3 and both WASM targets. Units HLD01 existing
truncation conventions for transition milliseconds, no incidental rounding.
External oracle skill requires pinned PowerPoint version/build and truthful
capture receipts. No new crate, module, source file or speculative abstraction.

## Hash harness

Existing static transition output should remain unchanged. Footer occupancy
and displayed-number fixes can change deterministic output only after native
proof and explicit reviewed delta declaration. No baseline move is approved
by this design.

## Implementation checklist

- [x] Capture and pin native PowerPoint footer/new-slide behavior.
- [x] Resolve legal owner state, title suppression and theme proxy contracts.
- [x] Approve exact source claims, primitives and expected output changes.
- [x] Reproduce public defects on the claimed Base before implementation.
- [x] Implement full Issue311 native/Python/CLI acceptance and existing tests.
- [x] Run scoped verification and native oracle/risk riders.
- [x] Obtain clean independent review and prepare validated handoff.

## Open questions

Resolved by the native Apply/ApplyAll controls and ten-case legal-owner matrix,
plus the exact live footer/title/date/cache/transition contract below. Author
slide-owned placeholders using qualified supported template geometry, preserve
legal owner metadata and refuse ambiguity atomically. Same-pass displayed
numbers and deterministic field identities use the existing providers.
Branded/multi-master/new-slide native and local output comparisons remain
implementation gates. No ambient date clock, guessed geometry or new module
is authorized.

## Actual native design evidence

PowerPoint16.113.3 build16.113.26092714 produced the controlled blank
16:9 baseline and authenticated Apply/ApplyAll captures in
/private/tmp/S91-X208-native-footer-controls, with input/output hashes and
owner facts. Apply on the title slide with suppression false creates three
slide-owned placeholders. Fixed dates are ordinary text, slide numbers are
a:fld type slidenum. Setting start0 writes presentation firstSlideNum=0 and
the visible first-slide field0. A newly added slide after Apply only has no
footer/date/number placeholders.

ApplyAll with automatic date, footer and numbers plus title suppression
removes the title slide placeholders, writes all three on the ordinary slide,
updates master/layout placeholder content and stores legal master hf hdr=0.
Neither slide has a root p:hf. Automatic date uses a:fld type datetime1 in
this native locale control. Normal same-path reopen/Save preserves all14
slide/layout/master XML parts byte-for-byte and firstSlideNum0, with no
repair dialog. The ordinary slide visibly shows date, footer and number1.

A source-built comparison removes only the three ordinary-slide owned
placeholders, retaining exact master/layout inputs. Native normal open shows
none of the three footer fields and Save does not materialize them. This
contradicts unconditional latent inheritance for this concrete control.
Use explicit owned-placeholder authoring and a separately reviewed renderer
correction, with intentional hash deltas declared before implementation.
This evidence is one blank-theme control, not the complete hf matrix,
branded potx or native PDF acceptance. Keep those remaining gates.

## Ten-control native footer ownership matrix

PowerPoint16.113.3 build16.113.26092714 accepted ten controlled inputs that
preserve the native OPC graph and existing part/master/slide IDs, modifying
only existing owner XML. The read-only appendix is
.claude/scratch/F-X208-footer-matrix-oracle.md. The complete owner facts,
40 artifact hashes and immutable original source hash are recorded in
/private/tmp/S91-F-X208-native-matrix/receipt.json and independently checked.
All ten generated documents were saved and closed. Three prior synthetic
multi-master inputs failed native validation and were cancelled without
repair. They provide no visibility evidence.

With zero slide shapes, occupied dt/ftr/sldNum placeholders on master or
layout remain invisible under absent, empty, enabled and disabled legal hf,
opposing master/layout flags, exact matching indices2/3/4, distinct indices,
and title layouts with showSpecialPlsOnTitleSld1. Normal native save/reopen
does not materialize slide placeholders. Enabled hf normalizes to empty hf
because true is the default. Disabled flags remain explicit0. The ordinary
master-text positive control removes only its ftr placeholder marker and
renders that footer text despite disabled hf. Therefore do not treat occupied
latent footer placeholders as ordinary inherited master/layout text.

Every accepted case has explicit local Best for printing PDF before and after
normal reopen/save. All ten extracted page-text pairs agree. Visible positive
and blank conflict pages were rasterized and inspected. No intermediate owner
XML snapshot was frozen before the second save, so this matrix does not assert
byte-identical owner XML or PDFs across reopen. Preserve the earlier captured
Apply/ApplyAll owner evidence separately. Historical ConflictSame actually
has distinct native indices10/11/12 on layout. ConflictMatchingIdx supplies
the real equal-index discriminator. Do not infer semantics from filenames.

This resolves the bounded latent-placeholder visibility gap. Authoring must
create/update slide-owned footer placeholders through the approved candidate,
with legal flags only on their actual supported owners. Title suppression,
date cache context, authored templates with supported existing slide fields,
notes-page output and transition contracts retain their separate tests.
No source, renderer baseline or completion verdict is changed by this study.


## Rolling PR324 linked contribution

[PR324](https://github.com/tensorbee/rdocx/pull/324) closes Issue311 and belongs
to this existing F-ID. Initial head is b4786dc9211f36a48e64b38d4277a07b21e11354.
It contributes slide-owned footer/date/number placeholders, firstSlideNum,
fields, theme reads, common transitions, Python and CLI operations and
layout child-order corrections. Complete immutable source assessment is
in progress. Its native and test results are contributor claims to verify
against our authenticated controls and actual integrated implementation.
Do not adopt new source files, the libc dependency, public struct changes,
archive values or output hashes implicitly. Shared colours remain X197 and
theme editing remains X209. No additional F-ID is created.

## Completed immutable PR324 assessment

The complete35-file PR324 source and68 Git blobs were authenticated at
b4786dc9211f36a48e64b38d4277a07b21e11354, retained under
/private/tmp/S91-PR324-intake. ReceiptSHA
8bef6a0de587509bee618e5a04649d90d3b9ebc2e677d41275b83282c9f9d180.
Reuse useful footer fields, legal hf ownership, firstSlideNum, theme reads
and child-order corrections. Transition edits must preserve sound and opaque
siblings instead of replacing the complete subtree. Cover supported transition
variants and timing/units explicitly. Footer writes preflight the whole OPC
candidate, generate unique field IDs and resolve current owner/theme state.
Preserve placeholder identity, transformed geometry and qualified relationships.
The copied new files, libc dependency and public result field are not implicitly
authorized. Contributor hashes remain inputs to inspect, not local gate evidence.

## Approved F-X208 final technical contract

Replace the stale Open questions at plan:224 with the resolved native owner contract. Ten accepted graph-preserving controls show occupied master/layout dt/ftr/sldNum placeholders are invisible without slide-owned placeholders under absent/empty/on/off/conflicting hf, equal/distinct idx and title settings. Ordinary master text remains visible. The Apply/ApplyAll captures prove actual slide-owned fields, legal master/layout hf, firstSlideNum0 and title suppression. No slide-root p:hf is legal. These bounded native facts close the design question, while full branded/multi-master/cached-field implementation controls remain required.

Select native HeaderFooter/HeaderFooterDate and existing-file operations as contribution ingredients, not new header_footer.rs/transition.rs modules or libc. Presentation.set_header_footer(slide_number=True,footer=None,date=None,hide_on_title=True,date_format="datetime1",*,date_cache=None)->None is the exact complete-replacement Python operation. date None disables, date="auto" creates an automatic field, every other string is fixed text. date_cache: str|None explicitly supplies automatic cached text. No ambient clock or silent US locale calculation. Automatic field type supports datetime1..datetime13 and retained imported aliases without claiming universal locale rendering. Empty supplied cache is distinct from absent. Without cache preserve a compatible existing automatic cache, otherwise author an uncached field and report that static rendering has no computed date. Fixed date/footer text validate XML before access. A separate field helper can format explicitly supplied Gregorian year/month/day for datetime1..7 with documented en-US output, refusing time-only formats unless an explicit time context exists. Native PowerPoint may refresh automatic fields on open. Do not use its captured current date as a deterministic expected value.

Slide.header_footer is a live owner proxy with the contributor spellings slide_number/footer/date/date_format, adding read-only presence/mode/source metadata so absence is not flattened to false. Setters patch only that field on the latest state. Local operations change only that slide's owned placeholders. Deck operation stages every slide plus legal owner flags once. Patch existing master/layout known flags while retaining hdr and opaque fields, unless a separately explicit header operation requests them. Repeated identical assignment is a source-proven no-op. Missing/duplicate or ambiguous template geometry refuses atomically rather than guessed synthesis. Resolve supported layout placeholder first, then qualified corresponding master placeholder by type/idx. retain grouped geometry and remap owner relationships or refuse unsupported carriers. Default and branded acceptance fixtures must supply supported footer geometry. Ordinary matching-text shapes never qualify.

Title means the linked layout's qualified type="title", not position zero or localized name. hide_on_title removes only owned latent placeholders on those slides and updates legal layout/master title flags consistently. A missing/unknown layout type is not guessed to be title. New-slide authoring uses the selected layout/master effective legal enabled state and qualified template placeholders. an absent hf alone does not invent enabled authored fields. This is an explicit authoring policy, not a claim that all native templates materialize identically. Native new-slide variants remain operation tests, not a design-approval wait.

firstSlideNum is optional typed root state, absent defaults1 and explicit0 is valid. Python slide_number_start:int|None getter/setter retains presence. Use checked schema range and overflow, displayed=first+physical_index. Carry displayed numbers to static/timeline slide field evaluation, without changing physical page IDs, output selection/names or notes/handout page numbering. GUIDs are deterministic, unique within the complete presentation field identity inventory, not fixed constants reused per field kind. Preserve retained IDs and allocate new ones through the existing accepted allocator. Qualified a:fld and schema child order are shared authoring primitives.

Transitions keep contributor live Slide.transition spellings type/direction/duration/advance_on_click/advance_after and apply_to_all. Preserve explicit optional presence in native/read metadata. Durations finite nonnegative seconds convert by checked truncation to milliseconds. Only the selected qualified property is patched, retaining sound, extensions, unrelated transition attributes and timing. None removes transition only. Implement typed/timeline none/fade/push/wipe/split/cover/uncover/cut/zoom with effect-specific directions and atomic all-slide rollback. Unknown retained effects remain readable with unsupported metadata. PR324 Other effects and whole-subtree replacement are implementation gaps, not an approved shortcut.

Theme reads use the already approved X209 master-part live owner contract and X197 colours, not detached snapshots. X208 can implement its read extraction once and consume X209's resolved lifecycle without waiting for provider completion to approve design. Formal source claim depends on actual providers, shared file leases and global draft barrier. CLI footer/inspect remains X208's declared surface, coordinated with X204 serialization, output refusal and reference tests. No new F-ID, module/dependency, baseline movement or unpublished contributor test claim is authorized by this memo.

## Latest PR324 head equivalence

PR324 now points to232b5dbb63a29cfe72462a1716d2b17d79a7d252. Its entire immutable tree
6be56b2375623e373ab8e6d0507a7d91904c65e2 is identical to the previous assessed head,
with the same parent commit. Diverged final commit identity introduces zero
source changes. Full current discussions and reused source blob hashes are
authenticated in /private/tmp/S91-PR324-232b5db-intake/receipt.json.
The prior source assessment and existing feature ownership remain applicable.
No duplicate work, asset approval, baseline or contributor pass claim changes.

## Qualified provider parser closure

The existing comment and reply text-body readers pass the captured enclosing
namespace bindings into CT_TextBody. Diagram text uses the same selected-root
parser with its actual diagram scope. Presentation defaultTextStyle uses the
existing CT_TextListStyle parser with inherited bindings. The same concrete
route applies to slide master title, body and other styles and notes master
notesStyle. Physical source
declarations and opaque children remain retained separately from namespace
lookup, and unsafe canonical rewrites still refuse. SmartArt edited text
extracts its qualified root structurally, preserving root attributes instead
of assuming a literal attribute-free opening token. No synthetic namespace
declarations are injected into stored source owners.

The affected existing regression controls are
comment_fallback_shadows_and_inherited_reply_drawing_namespace_survive,
list_owned_comment_shadows_and_typed_drawing_children_keep_their_namespaces,
unchanged_text_bodies_keep_inherited_a_raw_bytes_and_unsafe_rewrite_fails,
edited_smartart_text_retains_safe_root_attributes_and_namespaces,
modelled_nested_elements_reject_fixed_prefix_rebinding and
ordinary_shape_properties_round_trip_in_schema_order. The last control checks
expanded namespace identity and schema order while retaining the imported
text-body alias. These repairs close the existing qualified preservation
contract needed by the field and footer parser and serializer gates.

## Scoped completion evidence

The claimed Base public controls and native latent-owner discriminator reached
real semantic failures before implementation. Subsequent native footer, field,
numbering, policy, transition, clone and positive-geometry controls passed.
Independent whole pass7 and bounded correction passes9 and11 close the
implementation findings. Pass12 authenticates the affected archive metadata.
All earlier failed receipts remain retained, with no failed wrapper relabelled.

The final qualified source has complete OXML156 and DrawingML179 passes,
seven-crate check and Clippy, retained affected native family and corpus tests,
current seven installed public cases with zero skips, strict mypy and stubtest.
Both WASM targets, public docs and all49 deterministic hashes pass. Existing
Office-only ignored tests remain uncredited. ROOT authenticated branded two
masters, POTX, native new-slide, footer and fixed-date operations with native
save/reopen and six visual pages using PowerPoint16.113.4. This is bounded
Office acceptance, without a pixel identity or Google Slides claim.

Six affected verified dry-runs retain actual archives and complete source,
mode, membership, manifest and VCS proof. Metadata-only remeasurement retains
exact normalized totals with compressed deltas within64bytes. Current scoped
README cases pass, and workflow policy reports141tests with two declared
skips. Formatting, prose, adapter and diff checks pass. Evidence is indexed
in .claude/scratch/F-X208-start-1/scoped-evidence-final-preparation.json and
actual-current-metadata-gates-2/receipt.json. Source45 metadata changes leave
all compiled, rendered and installed inputs byte-exact. No baseline moves.
