# F-X239, python-pptx notes names and refusal of silent zero values

**Status**: completed
**Sprint**: S94
**Size**: M
**Depends on**: none

Approved after S93 closed and S94 opened from main3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190. Source citations were prepared against the unchanged runtime inputs of S93. No S94 runtime evidence is claimed by this design. Establish the exact feature Base when claiming its worktree.

## Problem

Slide.notes_text exists at crates/rpptx-py/src/slide.rs:666,678, but the two exact Issue379 scripts need a side-effect-free has_notes_slide predicate and the writable notes_slide.notes_text_frame.text chain. Native SlideRef.notes_text distinguishes no notes from an existing empty notes part at crates/rpptx/src/lib.rs:21223. Presentation.set_notes_text already creates the native notes slide when absent at7596. Existing PyTextFrame is shape/cell path-based at crates/rpptx-py/src/text.rs:268 and cannot simply be handed a slide path and called a notes owner.

The five frozen follow-up assignments currently truncate or store zero through Word Font.set_size at crates/rdocx-py/src/formatting.rs:531 and PPT spacing conversions at crates/rpptx-py/src/text.rs:89,98,786,1056,1074,1085,2338. The story explicitly assigns all five, including the Word setter, to F-X239. Refusal must precede any source or owner publication and name the unit to use. Preserve the deliberate native constructor truncation and every unrelated valid unit behavior.

## Spec reference

- docs/hld/14-development-backlog.md, "F-X239, python-pptx notes names and refusal of silent zero values", size M, no dependencies, test gate regression.
- docs/hld/10-bindings-spec.md, "Python API shape", "Python colour inputs and live font formats", "Native PowerPoint notes and handout export", "PowerPoint Element XML surface" and "Live Word sections and rich related owners", specifically the checked Length/owner and notes-preservation contracts.
- docs/hld/01-glossary.md, "Units", half-point Word font size, centipoint PPT spacing and EMU conversion.
- docs/hld/05-drawingml-model.md, "Colour, the part everyone gets wrong", "Do not touch the Word path" and "Direct script and spacing properties".
- docs/hld/04-opc-and-packaging.md, "The package" and "Checked PowerPoint XML transactions", plus docs/hld/06-presentationml-model.md, "Public facade", for actual notes relationships and malformed/ambiguous ownership refusal.
- docs/hld/12-testing-strategy.md, "Test taxonomy", "PowerPoint checked XML qualification" and "Integrated Python compatibility qualification". The PR333 corpus remains separate from F-X211.
- docs/hld/15-build-and-toolchain.md, "Toolchain pinning", "Packaging", "Feature flags" and "CI job matrix".

## Approach

### Frozen issue inputs and source boundaries

Use the same authenticated /private/tmp/S94-parity-inputs inventory as F-X238. Original commit193819ca0a1dcf3415017c0be1186fe8df6bcb86 has122 authenticated files:76 Word scripts,44 PPT scripts and2 runners. Preparation checked every SHA256 and Git blob SHA1 and archive SHA25635f454637169c3859f40a01e4ab9aae9e1dc714441f33e58ff9aa9b6abf217b9. The original issue body, accepted split and exact five-assignment script are preserved separately. Reauthenticate at execution. Never edit source bodies or the original runner.

No new product file, module, crate, trait, generic, feature flag or dependency is proposed. Add concrete owner-aware notes classes within existing slide.rs and register/export them through existing lib.rs and package/stub files. Their responsibility is checked physical slide identity, absent/body distinctions and prepared publication, not merely forwarding calls through an unvalidated wrapper. Do not expand general shape TextFrame paths to pretend a notes body is a slide shape.

### Notes names, including absent versus empty source

Proposed Python signatures:

```python
Slide.has_notes_slide -> bool                    # read only, no allocation
Slide.notes_slide -> NotesSlide                  # creates actual notes if absent
NotesSlide.notes_text_frame -> NotesTextFrame | None
NotesTextFrame.text -> str                        # read and write
```

has_notes_slide reads actual notes ownership, equivalent to SlideRef.notes_text().is_some(), not bool(notes_text). Existing notes with an empty body are true. A missing notes part is false and remains missing after any number of predicate reads. Validate the held slide before each operation.

Match pinned python-pptx1.0.2's notes_slide access: its source at installed pptx/slide.py:196,205 distinguishes the pure presence predicate from lazy creation. If absent, stage existing Presentation.set_notes_text(index, "") to create actual native notes/master relationships. Existing notes must never be rewritten simply to obtain a wrapper. Allocate/validate the returned wrapper against prepared owner state before publishing the creation. Any native creation, wrapper allocation, ownership or revision error leaves source and parent handles unchanged.

notes_text_frame follows the actual body-placeholder distinction. Pinned pptx/slide.py:151 returns None when the notes placeholder is absent, unlike an existing empty body. Add this bounded read-only projection in existing rpptx/src/lib.rs SlideRef, because scalar `notes_text` aggregates body text and does not identify a single writable frame:

```rust
pub fn notes_text_frame_text(&self) -> Result<Option<String>>;
```

Use the already-modelled source, with one exact admission rule: no notes part or no effective body placeholder anywhere gives None. Exactly one direct effective Body shape, with no additional grouped/AlternateContent body, gives Some of that specific body's text. If its txBody is absent, return Some("") without creating it. The existing setter may create txBody on later write. Multiple effective bodies, a body only in grouped/AlternateContent source, or direct-versus-effective ownership disagreement refuse this new chain with a precise ambiguity/ownership error. Do not silently choose one or return joined text. Retain scalar `Slide.notes_text` as the named existing aggregate-read alternative, and explain that its existing setter uses first writable-body replacement rather than claiming it removes ambiguity. Existing APIs keep their behavior.

The projection must inspect the same direct body selected by existing first_notes_body_mut at rpptx/src/lib.rs:23670 after the admission check. Existing scalar reads at lib.rs:21223 and notes_parts.rs:151/:409 aggregate effective body text, including alternate fallbacks, whereas mutation at lib.rs:20436 selects its first writable body and does not descend into AlternateContent. That mismatch is why the bounded projection is necessary. NotesTextFrame.text reads the projection and validates the exact admission again immediately before staging the existing set_notes_text provider. Revalidate every operation. A held frame must refuse if its selected owner becomes absent or ambiguous. Existing no-body notes return None from NotesSlide.notes_text_frame, not a fabricated empty frame. The projection has one current consumer and adds no new parser, module, file or rich-notes authoring semantics.

NotesTextFrame provides exactly scalar .text replacement. Preserve outside-body placeholders, header/footer fields, master/theme graph, relationships and unrelated opaque source. Within the selected body retain only what existing CT_TextBody::set_text guarantees: body-level state, first-paragraph formatting and its existing template/reconciliation behavior. The native setter at lib.rs:20440 calls that provider. The provider at oxml-drawing/src/text/mod.rs:434 drains old paragraphs and derives new paragraphs from first formatting. Do not promise preservation of arbitrary rich run distributions or every original paragraph's formatting. Match/qualify the pinned scalar text behavior and explicitly document any existing source-format difference rather than adding new preservation semantics. No full rich TextFrame API or other-placeholder editing is promised.

Reads are source-preserving. Repeated text is a no-op only when the existing provider produces no source delta, not merely because aggregate text compares equal. Publish actual changes using prepare_owners/publish_owners at presentation.rs:3156/:3392, as existing slide.rs:678 does. The existing text_changed_owners at presentation.rs:3809 preserves unrelated body owners. For lazy creation allocate the returned owner-bound wrapper against prepared candidate state before publication. Failure leaves original notes/master graph and held owners unchanged.

Bind wrappers to the actual checked slide owner, not a permanent numeric index. A moved surviving slide follows its physical owner. Removed, replaced, ambiguous or stale parents refuse before reading/writing another slide's notes. Warmed note wrappers and existing shape/table/text wrappers must follow the current preservation/retirement contract without test re-fetch workarounds. No foreign wrapper can supply authority to another presentation. Do not return a wrapper before successful prepared publication of an absent notes part.

### Theme alias and explicitly scoped color enum import

Add MSO_THEME_COLOR_INDEX = MSO_THEME_COLOR in existing python/rpptx/enum/dml.py:81, update __all__, existing package exports if applicable and _rpptx.pyi. Preserve exact alias and member identity, not a second enum with equal integers. Test direct import and identity with pinned python-pptx1.0.2.

Original Issue379 also explicitly imports MSO_COLOR_TYPE, while the accepted follow-up/backlog sentence mentions only the theme alias. The selected scope is additive import-only enum support in that same original issue contract, in existing dml.py. Pin the actual python-pptx1.0.2 member names/values: RGB1, SCHEME2, HSL101, PRESET102, SCRGB103 and SYSTEM104. This is not permission to change ColorFormat.type, whose current string-literal result is declared in _rpptx.pyi:907 and implemented in src/dml.rs:1298. Do not silently switch existing callers to IntEnum returns or claim HSL/scRGB authoring support. The return-type difference remains an explicit documented residual. Root selects this bounded import-only addition as authorized routine original-issue scope. It is not a new user-blocking question. Keep the ColorFormat.type string/None difference explicit in HLD and closure accounting. No full enum migration or new color semantics is promised.

### Exact five silent-zero refusals

Freeze and execute /private/tmp/S94-parity-inputs/issue379-five-zero-assignments-original.py unchanged. Preserve its original bytes and output. It catches its own exceptions, so exit zero is not proof: every exact label must report a raised error with the required unit pointer, and no label may report accepted.

| Exact assignment | Existing conversion | Required pre-publication refusal and positive controls |
|---|---|---|
| rdocx run.font.size = 12 | formatting.rs:531 accepts i64 EMU, converts to points, then Word stores half-points. Current output is expanded Word sz/val0. | ValueError explicitly says bare int is EMU, Word stores0.5point steps and use rdocx.shared.Pt(12), or a Length at least Pt(0.5). Check nonzero inputs that the existing conversion would quantize to zero before apply. Keep None clearing, valid Lengths and native truncation unchanged. Do not use F-X238's new run aliases to bypass the existing Font check. |
| rpptx paragraph.space_before = 6 | paragraph_spacing at text.rs:786 and points_spacing at89 divide by127EMU/centipoint. Current source writes spcBef/spcPts val0. | ValueError names EMU,0.01point storage and rpptx.util.Pt(6), or the already-supported explicit float line multiple where appropriate. Reject nonzero EMU values that become zero. Preserve meaningful explicit zero spacing, None clear, existing bounds and valid Length/float controls. |
| rpptx paragraph.space_after = 6 | Same helper, setter text.rs:1085, current spcAft/spcPts val0. | Same unit-qualified refusal, with complete independent before/after atomicity checks. Keep before/after fields distinct in serialized assertions. |
| rpptx run.font.spacing = 2 | text.rs:2338 truncates EMU/127 to centipoints, currently spc0. | ValueError names character spacing, EMU,0.01point storage and rpptx.util.Pt(2). Apply nonzero-to-zero rejection symmetrically to small negative operands. Preserve explicit zero, None and valid negative spacing such as Pt(-2), without rounding or forbidding all negative values. |
| rpptx paragraph.line_spacing = 0 | text.rs:1056 routes non-Length values through lines_spacing at98 and currently writes spcPct0. | ValueError names a positive line multiple such as1.5 or a Length such as rpptx.util.Pt(18). Reject zero/nonpositive actual line-spacing values before update. Preserve existing negative refusal, finite/bounds validation and positive point/multiple controls. Role-specific validation must not accidentally reject legitimate zero space_before/space_after through the shared helper. |

The changed public numeric paths reject bool/type confusion before mutation and retain their existing accepted unit representation. Keep all arithmetic/truncation in the existing conversion policy. No changes to Length, Twips, Emu or Points constructors, native font sizes or renderer geometry are authorized. Do not raise a tiny value to a representable one or silently coerce integers to points. Preserve imported pre-existing zero-valued XML on read/save unless the user invokes a setter covered by this policy. Any broader range-policy change outside these specific zero-quantization/line constraints needs explicit justification and separate test coverage, not an incidental cleanup.

Exact storage-boundary controls are part of the contract, using integer EMU operands and the existing conversions:

- Word font:6350EMU is0.5pt and stores one half-point. 6349EMU stores zero under current conversion and must refuse. Reject any nonzero input whose existing HalfPoint conversion is zero, including negative values that saturate to zero. Preserve explicit integer0's existing setter behavior and None clearing. Do not invent a new general positivity policy. HalfPoint::from_pt at oxml-core/src/units.rs:79 remains unchanged.
- PPT before/after:126EMU refuses,127EMU stores one centipoint. Explicit0 remains valid, None clears, negatives retain current refusal. Check fields separately and retain admitted float line-multiple behavior.
- PPT character spacing:±126EMU refuses,±127EMU stores±1centipoint. Explicit0, None and meaningful signed values retain their behavior.
- PPT line spacing:explicit numeric0 and Length(0) refuse. Positive Length126EMU also refuses because it serializes0centipoints, while127EMU is positive. Positive float0.000001 rounds to0 through existing lines_spacing and must refuse before update. 0.000005 rounds to one percentage unit and remains admitted, with positive ordinary1.5/Pt(18) controls. Check computed storage output rather than a changed rounding rule. Zero space_before/after must remain valid despite shared helpers.
- Actual bool inputs refuse as unit/type confusion in every changed setter before integer/float extraction, including True in Word size/character spacing and False in all five paths. Preserve accepted actual Length, int, float and None roles. Handwritten public types remain unchanged. Implementation may inspect Bound<PyAny> internally to distinguish bool. No native constructor, truncation, rounding or renderer change is authorized.

For each refusal capture complete serialized package bytes and current owner validity before and after, using both a fresh source and pre-existing nonzero values. Warm the font/paragraph wrappers and prove refused assignments cannot clear a value, allocate rPr/pPr, advance revisions, retire unrelated owners or mutate relationships. A subsequent valid Pt assignment through the same warmed wrapper must still work. Use expanded namespace names for OOXML assertions, not only literal prefix strings or a check of internal helper output.

### Feature source leases

F-X238 owns the Word binding document/paragraph/run/formatting/table files, existing Word enum/package/stub carriers and Word test_python_docx_parity.py/typing_smoke.py. Its third divergence uses existing PowerPoint slide.rs and documented-example/typing carriers. F-X247 owns separate native math/conversion and Word test_core.py math controls in wave1, with no binding API, stub or native PPT edit lease.

F-X239 receives Word formatting.rs and parity tests after F-X238 completes or explicitly checkpoints and releases those lanes while blocked. It owns PPT notes/slide/text/lib/enum/stub carriers and existing documented-example/typing controls, plus the bounded native projection in rpptx/src/lib.rs and existing crates/rpptx/tests/integration.rs. F-X241 subsequently receives Word document/core/stub and CLI form carriers. F-X240 receives both families and both CLIs last. The exact four-wave schedule is recorded below. No simultaneous source, test, package catalog or HLD semantic-section edits are allowed.

## Rejected alternatives

- Defining has_notes_slide as bool(notes_text), which loses empty existing notes and may create source through a lazy getter.
- Returning ordinary shape TextFrame over a slide path or routing by a stale numeric slide index.
- Parsing ZIP/XML independently in bindings to discover notes ownership, rather than reading the existing native model.
- A forwarding-only notes wrapper that does not validate owner lifetime or stage publication.
- Changing ColorFormat.type from strings to enums as an undocumented side effect of adding an import alias.
- Rounding tiny EMU values, changing native constructors, treating bare integers as points or applying line-spacing positivity to valid zero before/after spacing.
- Editing the frozen five-assignment script or calling its self-catching exit zero an acceptance pass.

## Test plan

The **test gate** is **regression**, verbatim from the backlog. Add pinned structural differential and installed artifact riders. Existing Word test_python_docx_parity.py holds Word refusal controls. Existing PPT test_documented_examples.py and typing_smoke.py hold notes/units/import controls. Lease existing crates/rpptx/tests/integration.rs for native projection/source controls and existing rpptx/src/lib.rs in-source tests if needed. F-X247 has no PPT native lease. F-X241 native tests remain Word-only. No new integration entrypoint. No new product/test file or new Rust integration binary.

| Category | Proposed existing-file test case | Asserts |
|---|---|---|
| regression | fx239_notes_frame_owner_is_exact_or_refused | One direct body reads and writes that same body. No-body returns None. Multiple, grouped-only and AlternateContent/direct mismatches refuse without changing any graph, byte or held owner. Existing aggregate notes_text behavior remains unchanged and is named accurately |
| regression | fx239_notes_names_distinguish_absent_empty_and_missing_body | Pure predicate leaves package untouched. Lazy notes creation follows native source. Existing empty notes counts true. Missing body gives None rather than an invented frame. Existing populated notes read identically. |
| differential | fx239_notes_names_match_pinned_python_pptx | Pin python-pptx==1.0.2 and compare absent/empty/populated/missing-body and lazy-creation outcomes, actual notes/master relationships and normalized source with expanded names. Exact unchanged notes_read_all and notes_slide_text bodies and saved checks run. |
| round-trip | fx239_notes_text_preserves_owned_and_opaque_source | Set/read/save/reopen text through held NotesSlide/NotesTextFrame, preserve outside-body header/footer fields, unrelated placeholders/relationships and opaque XML. Selected-body formatting follows the existing scalar first-paragraph/template behavior, without a rich-preservation promise. Reads and repeated values do not create unintended source or owner revisions. |
| regression | fx239_notes_wrappers_follow_physical_slide_owners | Surviving slide move, removed/replaced slide, re-opened presentation, warmed wrappers and unrelated held shape/text/table objects. Old wrappers cannot target a new slide at the same index. Refusals are atomic without re-fetching away a defect. |
| regression | fx239_five_exact_original_assignments_raise_unit_errors | Execute the exact frozen original script, independently require all5 labels to report raised errors with EMU/Pt or positive-multiple guidance. No accepted label, no edited body and no success inferred from script exit alone. |
| regression | fx239_quantization_refusals_are_source_and_revision_atomic | Fresh/preexisting nonzero/warm-wrapper matrix for all5 assignments, before/after package identity, unrelated XML/owner validity, no new zero nodes, and subsequent positive Pt operations. Preserve valid negative character spacing and meaningful zero before/after. |
| differential | fx239_valid_unit_controls_retain_pinned_structure | python-docx==1.2.0 and python-pptx==1.0.2 for valid explicit Length/line-multiple controls. Assert exact half-point, centipoint and percentage values after reopen. The invalid-input refusals intentionally differ and must be separately asserted, not oracle-normalized away. |
| regression | fx239_theme_alias_and_color_import_scope_are_explicit | Alias is object-identical with identical members. MSO_COLOR_TYPE imports with pinned values while current ColorFormat.type remains the documented string/None contract. No claim of extra authoring support. |
| integration | fx239_immutable_ppt_corpus_and_independent_requirement_report | Authenticate122-file source inventory and run original44 PPT cases with original runner unchanged. Preserve raw report and independently account for notes/import/refusal requirements. The original76 Word corpus/report and stale merge diagnostic remain separate, without editing markers or mixing F-X211 scores. |
| integration | fx239_fresh_installed_ppt_and_word_refusal | Build locked actual PPT wheel in a fresh venv, run complete PPT suite and focused installed Word refusal/alias interactions using the current Word wheel. Authenticate wheel RECORD, installed native/package/stubs, source/commands and selected proof inputs. Final integrated sprint runs both complete families on the union source. |
| integration | fx239_notes_exports_and_stubs_are_consistent | mypy==2.3.0 --strict and installed stubtest for PPT new surfaces/enum exports. Existing Word size signature is unchanged, but the final integrated Word typing/stubtest remains mandatory. No Word stub/README edit is planned. |

Capture actual RED against the old installed releases for notes names and five silent-zero outputs before repair. Independently reopen and observe those original zero nodes. A semantic mutation must reintroduce each zero write or lose an actual notes body and be rejected by the same controls. No pass, benchmark, native producer run or numeric new suite count is claimed by this draft.

## HLD impact

- docs/hld/06-presentationml-model.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

Apply canonical .claude/skills/risk-routing.md and differential-testing.md. Matched rows add obligations to scoped verification and the final union.

- Unit conversion: read HLD01 "Units" and CLAUDE.md "Things that are deliberately wrong". Keep constructors/truncation untouched, cover exact storage-boundary values, signed tiny spacing and role-specific zero behavior. Declare unchanged hash expectation.
- WASM or PyO3 bindings: read HLD10. Run cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm and installed/typing gates. Workspace cargo tests exclude both extension-module crates.
- External oracle comparison: pin python-docx==1.2.0 and python-pptx==1.0.2, record actual interpreter/wheel/source identity, compare normalized expanded-name structures and intentionally different refusal semantics separately. No oracle runtime dependency in product code.
- Public API: concrete notes classes and any bounded native read-only projection are additive pre-1.0 surfaces. Preserve existing types except the expressly approved new enum import. Run applicable affected verified archive dry runs with local reviewed patches and the10MiB size gate, plus real wheel builds/install/exports/RECORD checks. Refresh exact measured inventory only where changed. No feature or Cargo default is changed here.
- Parser/serializer adjacency: read HLD04 and HLD06. Existing native notes providers own graph changes. Qualify missing/duplicate/external relationships, namespace aliases, exact opaque preservation, schema sequence, and creation/write failures before source/revision publication. Do not add a second binding parser.
- Theme color alias: read HLD05. No tint/shade, inheritance mapping, theme resolution or historical Word color function changes. The alias/import must not change current color getters, setters or rendering.
- New construct rule: new owner-aware concrete classes live in existing files and implement validation/publication responsibilities. No forwarding-only abstraction, trait, generic, feature, module or product file is proposed. Any unexpected need for a new file/module returns to approval first.

## Hash harness

All49 existing entries and shared decoded pixel baselines are expected unchanged. New refusals affect only newly invalid Python setter calls. Existing native rendering/defaults and imported source remain unchanged. Fresh notes authoring fixtures are not shared rendering baseline moves. An unexplained delta blocks completion. This story has no baseline write lease.

## Implementation checklist

- [x] Approve all five S94 designs after opening S94, including the root-selected import-only color scope and exact bounded notes-owner projection.
- [x] Acquire released F-X238 formatting/slide leases and agree conditional F-X241/F-X247 lanes before claims.
- [x] Freeze original122-file corpus and exact five-assignment script identities without a second canonical tracker/corpus.
- [x] Add actual old-artifact RED for missing notes names and silent-zero source, plus independent semantic negatives in existing carriers.
- [x] Implement pure presence, owner-aware lazy notes/body/text surface and preserved publication boundaries using existing native semantics.
- [x] Add identity theme alias and the selected bounded import-only color behavior, preserving existing string ColorFormat.type.
- [x] Validate all five refused writes before mutable publication and retain valid zero, negative spacing, None, Length and float policies by role.
- [x] Execute original raw corpus/script plus independent requirement classification, complete installed/typing/differential and applicable risk gates.
- [x] Obtain zero-defect/zero-smell independent microscope, scoped verify and structured handoff. Integrator owns final full union, ledgers and sprint review.


## Open questions

No unresolved blocking product question remains. The orchestrator approves the concrete approach and risk riders in this plan under the authorized S94 scope. Native and external controls are future gates until actually executed.
The bounded owner-validated notes text path, minimal read-only native projection and original-issue MSO_COLOR_TYPE import are approved. Preserve existing ColorFormat.type string semantics, distinguish absent/ambiguous bodies and retain actual scalar replacement formatting limits. No rich notes API or color return-type migration is approved.

## Approved sprint waves and exclusive resources

Wave1 runs F-X238 and F-X247 in separate worktrees. F-X238 owns Word parity tests and the existing binding naming/stub lanes, plus its PowerPoint XML-divergence pointer. F-X247 owns native math/conversion, the existing OfficeMath oracle, Word core math tests and the sole published-package numeric catalog lease in this wave. Neither edits the other's test entrypoint or source files.

Wave2 runs F-X239. Wave3 runs F-X241. Wave4 runs F-X240. These stories normally serialize because their actual edits share binding carriers and the source file scripts/readme_doctests.py for mandatory genuine archive measurements. If F-X238 is blocked, its fully checkpointed, frozen worker may explicitly release its source, test, HLD and cache leases so independent F-X239 can start from clean canonical source without inheriting or integrating that blocked implementation. F-X247 may continue on its disjoint math lanes, retaining the exclusive numeric-catalog/archive lease until explicitly released. F-X239 defers that shared stage and remains unprepared until every rider passes. F-X241 also waits for actual Word core/test lease release. F-X240 follows validated integrated form source for public fill/sign integration tests. A later F-X238 resume requires explicit lease return, reconciliation against the current approved consumers, a fresh microscope and affected union checks. Operational source leases add no speculative backlog dependency and do not waive any completion gate.

Each worker updates exactly its listed HLD impact under feature-specific semantic section leases before its prepared handoff. The integrator coordinates shared HLD sections and numeric catalog ownership. Shared package observations use genuine archives, and combined observations run again after integration. Workers never update sprint delivery totals. The integrator alone writes AS_BUILT and SPRINT_TRACKER. Keep complete installed environments and target/build caches isolated or exclusively leased. The hash baseline remains exclusive, with no approved new baseline move.
