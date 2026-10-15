# F-X207, Persist PowerPoint autofit and implement fit_text

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X194, F-X195

## Problem

Issue310 asks for stored fit results after template filling, not just a renderer that temporarily shrinks text. Native TextFrameMut::set_autofit_mode at crates/rpptx/src/lib.rs:9098 keeps or creates a bare NormalAutofit flag and ShapeAutofit changes no extent. Existing F-100 is done and correctly renders stored values and trusts stored shape extents. This story adds explicit authoring operations without silently mutating during rendering or ordinary saves.

ShapeTextLayout at crates/rpptx-render/src/lib.rs:214 already reports font_scale, usable bounds, text height and overflow from the shared renderer. It does not report computed line-space reduction. Bare autofit at crates/rpptx-render/src/text.rs:474 uses the current 2.5 percent ladder with no generated spacing reduction. Existing TextFrame and ShapeMut borrow only their text body or shape child, so they cannot independently resolve the owning deck's theme, inheritance and actual fonts. Python PyTextFrame at crates/rpptx-py/src/text.rs:133 retains presentation ownership and ContentPath, which permits delegation to a presentation-owned operation.

## Spec reference

- docs/hld/14-development-backlog.md, F-X207, existing F-100, F-X194, F-X195 and F-X204 ownership.
- docs/hld/08-rendering-spec.md, Text in a shape, Autofit and Slide text layout as data.
- docs/hld/06-presentationml-model.md, Public facade, The shape tree, Placeholders and Preservation strategy.
- docs/hld/07-inheritance-and-resolution.md, The chains: Position and size, Body properties, The nine-level list style and Fonts.
- docs/hld/10-bindings-spec.md, Python API shape and Native PowerPoint text layout.
- docs/hld/12-testing-strategy.md, Test taxonomy, Binding tests and the render fidelity gate.
- docs/hld/01-glossary.md, units, and docs/hld/15-build-and-toolchain.md, Deterministic rendering and Packaging.

## Approach

Implement explicit Presentation.refresh_autofit and checked TextFrame.fit_text
in existing files. Ordinary save, render and text-layout remain read-only.
Use the concrete owner inventory, signatures and every-frame accounting below,
with F-X195 caller fonts prepared once and immutable staged measurement.
No new module, trait, generic, dependency or competing font/owner resolver.

For explicit Normal refresh, ignore previous fitted values and try this exact
ordered integer fontScale/lnSpcReduction list:
100000/0,100000/10000,92500/0,92500/10000,92500/20000,85000/20000,
77500/20000,70000/20000,62500/20000,55000/20000,47500/20000,40000/20000,
32500/20000,25000/20000. This is a bounded PowerPoint-informed producer
policy, not a universal reconstruction of its private search. Every admitted
candidate must pass two actual shared-layout trials with the same fonts,
source body, owning context, bounds, wrap and spacing. Q rounds effective
run/default/end sizes by positive whole-point half-up and uses scale1.
S retains nominal sizes and uses the candidate fractional stored scale.
Do not derive either trial by multiplying completed measurements. Reject
nonpositive rounded sizes and all unrepresentable inputs before publication.

Both trials require the real available-width predicate and physical text
extent within the usable height plus0.01pt. Accumulate bounds from actual
participating shaped segments, selected face metrics, bullet markers and
baseline shifts before discarded provenance. Use maximum baseline-plus-
descent extent and preserve enabled first/last paragraph edge spacing.
Measure unanchored extent first, then apply the actual anchor consistently.
The existing summed line-advance height remains a separate fact. Reports
name Q native-informed trial and S persisted-render containment separately.
Do not call a guessed line.font_size/font-family lookup sufficient for mixed
text or bullets. Preserve exact ordinary62500 glyph12.5/bullet6.25 semantics.

Persist the first candidate passing both trials in every admitted context,
with original nominal sizes and both integer attributes. If none passes,
refuse the entire staged operation. The genuine BExact28 native floor is
still unfit. Its safe refusal is an explicit native-policy divergence, not a
successful native pair match. Do not write a falsely successful floor.

Shape-to-fit uses actual persisted content measurement at fixed usable width,
checked canonical EMU arithmetic and the full shape/group affine transform.
Preserve the physical top/center/bottom anchor and permit growth or shrink.
No unauthenticated1034/1024 multiplier or axis-aligned rotation shortcut.
Shared owner resize requires identical quantized local bounds and anchor
adjustment in every context, otherwise atomic named refusal.

fit_text(None) preserves effective families and bold/italic while writing one
largest supported positive integer point size at most max_size. Explicit
family writes that family. Descend through every permitted integer size,
using the same actual fixed-frame S layout and fonts. Reject Python bool,
noninteger and out-of-range values rather than coerce them. Preserve colors,
links, fields, unknown properties, text and owner identity. On success write
wrap-on/no-autofit and applicable direct/default/end sizes. Empty text is a
no-op. Unfit-at1 refuses atomically. None-family is a documented extension to
python-pptx, with oracle comparisons only for coincident selected semantics.

Stage all owner updates on one clone. Preflight every source correspondence,
measurement, numeric conversion, relationship and serialization/reopen before
a single publication and success-only causal invalidation. Actual requested
and selected face facts come from F-X195/F-X210, including same-pass owned
diagnostics. The concrete native results serve Python and F-X204 CLI once.
Prerequisite completion, scoped tests, native/control acceptance and review
remain required before integration. No implementation has passed here.

## Rejected alternatives

- Mutate fitted values during read-only rendering or ordinary saves.
- Invent coupled line-spacing reduction without pinned native evidence.
- Duplicate CLI fitting instead of exposing the same Python-visible API.

## Test plan

**Test gate**: regression.

- Prove four public missing behaviors on the claimed Base through installed APIs: bare normal flags save without computed attributes, shape autofit saves without changed bounds, fit_text is absent, and refresh_autofit is absent. Existing render-time scaling is a positive control, not the missing stored-operation gate.
- Generate a small deterministic native PowerPoint oracle set and pin app/build, platform, source package SHA, actual font bytes/version, exact recomputation action and output package SHA. Extract native scale/reduction/extent numerically. Compare stored values exactly when quantization agrees, with explicitly reviewed unit/coordinate tolerances otherwise. python-pptx1.0.2 is a structural fit_text oracle, not an autofit-recomputation oracle.
- Retain the existing render tests for62500 scale, stored line-space reduction on percentages but not exact point spacing, no-autofit overflow, bare ladder and trusted spAutoFit extent. They must remain correct outside explicit authoring calls.
- Verify save/reopen retains computed normAutofit attributes, no duplicate autofit children, correct schema order and all genuine unknown XML bytes/namespaces. Namespace aliases and rebound/foreign children must not acquire typed semantics.
- Verify shape growth/shrinkage for top/center/bottom anchors, insets, wrapped and unwrapped text, explicit breaks, mixed run styles, bullets, fields, empty paragraphs, rotations/flips, vertical text and nested group transforms. Assert actual physical anchor coordinates and PDF/PNG containment.
- Verify fit_text stores the largest permitted size, candidate-plus-one overflows, max_size cap, explicit family selection, None-family policy, wrap-on/no-autofit result and preserved colors, bold/italic, links and text. Differential checks use pinned python-pptx1.0.2 where semantics intentionally coincide, with documented divergence elsewhere.
- Verify deck-wide refresh after repeated template fills, inherited flags, idempotence, stable identities, warm/fresh equality and original bytes on malformed/unsupported/atomic refusals. Include every approved location class and explicit unsupported-location reporting.
- Caller-only font controls must make refresh, text-layout reports and rendered output agree. Missing requested families produce the same approximation diagnostics. Use actual variable700 instances from F-X195, not regular outlines with bold labels.
- Add tests to existing rpptx integration.rs, existing rpptx-render text/lib test modules and rpptx-py test_documented_examples.py and typing_smoke.py. No new test binary or tracked binary fixture. Installed runtime, strict typing and stubtest cover both new Python methods and returned snapshots.
- Run scoped changed-crate fmt, lint, tests, policy and deterministic hash gates plus exact native oracles, both WASM target checks, verified patched affected-package dry runs, archive limits and actual updated measurements. Integrated full verification remains the sprint gate.

## HLD impact

- docs/hld/06-presentationml-model.md
- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/14-development-backlog.md

HLD08 must distinguish explicit authoring refresh from unchanged render-time trust, and state the approved scale/reduction algorithm, owner/location and anchor policies. HLD10 documents concrete native and installed Python surfaces. Do not weaken the existing stored-value rendering contract to implement refresh.

## Risk routing

- Layout/shaping requires deterministic font bytes and measured shared-render containment controls. Read HLD08.
- Unit conversion requires checked range and frame-anchor controls while retaining existing truncating constructors. Read HLD01 and CLAUDE.md Things that are deliberately wrong.
- OOXML serialization requires prefix-tolerant read, canonical write, xsd:sequence and exact unmodelled XML round trips. Read HLD04 and HLD06.
- Published native API/report changes require stated additive or source-breaking semver impact, affected package verified dry-runs with the entire local patch graph, source membership and10MiB bounds. A public ShapeTextLayout field addition is pre-1.0 source-breaking for external literals and must be stated if selected.
- Python/WASM requires installed runtime, typing/stubtest and both wasm checks. Workspace Rust tests exclude both extension-module binding crates.
- External oracle riders require exact PowerPoint app/build and recorded recomputation, deterministic font inputs and pinned python-pptx1.0.2. No unobserved native pass claims.
- No new dependency, trait, generic, module, crate, feature flag or implementation file is proposed. A public concrete report has named current native/Python/CLI consumers and no builder.

## Hash harness

Ordinary rendering and save without explicit refresh should remain unchanged. Authored refresh/fit output intentionally differs from input when it writes scales, reductions, run sizes or bounds. If native evidence requires correcting the bare renderer's candidate algorithm, enumerate and independently review each actual deterministic harness delta before the exclusive baseline moves. Preserve attribution from F-X195 and other stories rather than blend their font changes into this story's delta.

## Implementation checklist

- [x] Align explicit refresh route, location/owner policy and fit_text None-family semantics.
- [x] Pin and capture native scale/reduction/anchor ground truth before choosing the missing algorithm.
- [x] Prove all public Base failures and retain positive render controls.
- [x] Reuse caller-font and owner infrastructure from prerequisites.
- [x] Implement shared fitting results, atomic materialization and anchored resizing.
- [x] Add installed Python methods and exact typing/runtime snapshot surfaces.
- [x] Verify all approved location classes, refusal controls and save/reopen preservation.
- [x] Complete scoped/oracle/packaging/WASM gates and independent zero-finding review.
- [x] Declare actual harness behavior and update HLD impact files.

## Open questions

None. Explicit-only refresh, None-family preservation, strict integer size
validation, full physical-owner inventory, common shared-context candidate
intersection and identical-resize-or-refusal are selected. The actual22-control
Q/S diagnostic below settles the bounded observed-step producer policy.
Unsupported flagged owners and unresolved shared contexts refuse atomically.
These are explicit implementation gates, not authority to ignore frames.

## Qualified autofit owner contract

## Inputs and concrete location

Reuse exactly X195 font preparation once per operation. Native input is font_files:&[(&str,&[u8])], and Python refresh_autofit and TextFrame.fit_text accept fonts=None, font_dir=None and font_dirs=None. Ordered directories, lexical file order, later same-rank precedence, explicit fonts last, embedded family/style identity, caller-only versus bundled fallback and invalid-directory failures remain the one X195 rule. Do not create a second fit loader or omit font_dirs now that313 is included. Copy caller byte ownership before detached work. All measurement trials use one prepared font selection, including genuine variable weight/italic instances and requested/actual family diagnostics.

Use a concrete frame location, not only slide_index+shape_id. Proposed existing-file native type `TextFrameLocation` carries owner_part:String, child_path:Vec<usize>, cell:Option<(usize,usize)> and checked owner fingerprint. Its constructor is private and inventory returns it. Context-specific reports also carry slide_index:Option<usize>, owner_kind and instantiated context identity. The source child ordinal path identifies group descendants, and cell coordinates identify a table text body. Shape ID is report metadata, not sole mutation identity. Duplicate IDs or failed owner correspondence are checked errors. No new generic path engine, source file or trait.

Proposed native signatures:

- Presentation::refresh_autofit(&mut self,font_files:&[(&str,&[u8])])->Result<AutofitRefreshResult>.
- Presentation::fit_text_at(&mut self,location:&TextFrameLocation,font_family:Option<&str>,max_size:u16,font_files:&[(&str,&[u8])])->Result<AutofitUpdate>.
- Presentation::fit_report(&self,font_files:&[(&str,&[u8])])->Result<FitReport>.

Python TextFrame.fit_text delegates its existing checked ContentPath to the concrete native location. Keep the initially proposed native slide-index/shape-ID method only as an optional convenience if it resolves exactly one same-owner frame and actually has another consumer. Do not add a forwarding wrapper merely to preserve a draft signature. Native location fields/checks can instead reuse a reviewed existing X206 owner locator if it is integrated by claim time. The draft X206 explicitly says no stable-ID resolver yet, plan:195, so that provider is not assumed implemented.

## Inventory and every-frame accounting

Inventory scans qualified typed owner parts in stable package ownership order, then shape-tree child/source order. Include ordinary shapes, populated placeholders, nested groups and table-cell text bodies. Inventory includes empty text and flag presence, not only visible characters. Existing text_layout_deterministic at rpptx/lib.rs:1199 omits master/layout shapes, SmartArt, table cells and empty frames, so it is not this inventory.

One physical owned text body is one mutation target. Slide-local inherited body flags are resolved through actual layout/master context, then their computed answer is materialized on that slide-local owner. Never change a shared master merely because a slide inherits a flag. Directly owned master/layout frames with flags are inventoried separately, along with every consuming slide context. Notes-slide and notes-master frame owners are inventoried separately using the existing notes resolution/theme/page geometry, not slide geometry. SmartArt/opaque unsupported text ownership is recorded with a named unsupported location, not ignored as though it had no flag.

For a directly shared master/layout/notes-master frame, evaluate every applicable context before mutation. Normal shrink may use only a common candidate pair that passes every context. Choose the least shrinking candidate in the approved candidate order satisfying their intersection. Shared shape-to-fit resize may be written only when required local bounds and anchor adjustment are identical in every context after canonical EMU quantization. If they differ, refuse the entire refresh with owner/context details, rather than select a global arbitrary size or duplicate shared XML into slide-local shapes. This is a conservative exact ownership rule, not an optimization based on similar rendered text. An unused flagged shared owner without a resolvable context is recorded as not-measured and makes refresh refuse atomically with physical owner details. A diagnostic-only report may retain that status. No arbitrary context or successful every-frame claim is invented.

Table-cell normal shrink and uniform fit_text use the actual shared cell layout box and resolved paragraph style. Shape-to-fit flag on a table cell has no independently owned shape extent. Refuse with a named unsupported-cell-size policy until a reviewed row/table geometry transaction exists, rather than changing just text box height or falsely claiming one cell changed its enclosing row correctly. This is a real every-frame acceptance barrier if any required imported flagged cell uses that mode. Direct X209 master/layout authoring is not reimplemented here.

Empty flagged frames are accounted for with status='empty', no sizing/text mutation and no fabricated lines or zero scale. Their existing flag/raw properties remain. Empty fit_text is a no-op. Invalid retained XML, nonpositive usable box, duplicate IDs, unsupported flagged owner or unfit floor returns an atomic refresh error that names its physical location. A diagnostic-only report may enumerate unsupported owners, but successful refresh cannot claim it measured them.

## Fitting facts and thresholds

Use the existing shared shaping/line breaking/stacking/inheritance/theme and page-field value path, with width_factor fixed1.0 for authoring and fit report. Reuse 0.01-point acceptance tolerance from StackedText::fits at rpptx-render/text.rs:439 and ink-width checks at666. Both width and height must pass. Do not fit by height alone, estimate font width or count characters. Display rotations and text directions use their actual reading frame, ShapeTextLayout documentation at rpptx-render/lib.rs:214. Retain glyph ink handling, bullet extents, field values, indentation, insets and exact paragraph spacing.

For fit_text, max_size is a positive integer in the existing DrawingML size range, with explicit Python bool/nonintegral/overflow rejection. Validate against the actual native size setter before candidate trials rather than assuming u16 proves all schema bounds. Try each permitted integer point size descending from max_size through1. Choose the first that passes the same fixed-frame test. Exhaustive bounded descending search avoids an unproved monotonic binary-search assumption caused by line-wrap changes. Candidate-plus-one must fail unless the max cap selected the result.

font_family=None preserves effective per-run family and bold/italic while writing one chosen point size. Explicit family writes that family on affected native direct runs, fields and paragraph/default/end properties without replacing text. Preserve color, hyperlink, unknown attributes, ordered text choices and original XML owners. Set wrap on and explicit no-autofit only after success. The method is formatting and geometry, not text replacement. Unfit-at1 refuses with original bytes unchanged. Native python-pptx comparison only applies to deliberately matching explicit-family/default and stored-shape semantics, not automatic claim of identical None-family behavior.

Normal-autofit recomputation ignores previous stored values for explicit refresh. Ordinary bare and stored rendering retain their existing contracts. The exact ordered integer candidate list and dual Q/S admission in Approach are the selected authoring policy. Historical2.5-percent diagnostic references below do not override it or claim universal native steps.

Store a selected approved pair as decimal integer strings fontScale and lnSpcReduction in NormalAutofit, oxml-drawing/text/body.rs:225. Both values are present for a successfully recomputed nonempty frame, including100000/0 when unshrunk. Couple measurement and storage from the same concrete result. No formula lnSpcReduction=1-fontScale is assumed. Existing stored reduction applies to percentage/default spacing but not exact-point spacing, text.rs:930. Generated coupled candidates must preserve this existing stored-value interpretation. If floor still overflows, mutation refuses rather than storing a falsely successful25-percent pair.

For SHAPE_TO_FIT_TEXT, measure at current effective text sizes, usable width, wrap and paragraph settings, ignoring renderer-side shrink. New local height is measured content extent plus actual insets and supported leading/trailing rules. Allow growth and shrinkage. Preserve local top/bottom/center anchor exactly, transform through existing shape/group affine conversion and retain rotation/flip/vertical direction. Canonical EMU truncation stays unchanged. No axis-aligned world-space shortcut for rotated/grouped shapes. If inverse transform or coordinates cannot be represented, refuse before any mutation. Native oracle must verify anchor facts and group/rotation controls separately from the coupled shrink study.

## Reports and mutation generations

AutofitUpdate includes checked location, owner/context, shape ID/name metadata, mode, changed/status, old/new local bounds and canonical scale/reduction or written uniform point size, plus ordered requested/actual font diagnostics. AutofitRefreshResult contains updates in physical inventory order and diagnostics. Same-size/same-attributes refresh is idempotent and publishes no mutation. One genuinely changed deck-wide refresh is one staged package transaction, serialized/reopened before publication. A late failure changes no part or generation.

Use X194/X206 handle partition: formatting and extent changes do not alter indexed slide/shape/table/text topology, so no structural or text-topology revision bump is justified for successful refresh or fit_text alone. Existing Slide/Shape/TextFrame/Paragraph/Run handles remain live when their logical paths are unchanged. Invalidate native completed layout cache once after successful formatting/geometry publication. Do not bump text_revisions merely because font sizes were written. If implementation actually creates/removes logical paragraph/run owners to fit, reject that approach or explicitly revise its contract instead of silently applying blanket staleness. Immutable report snapshots remain historical values and are not mutation selectors.

Read-only FitReport returns frames in instantiated slide/notes context order with their physical location, current measured scale/spacing, overflow, hypothetical needed_scale/reduction, bounds, status and diagnostics. Needed means a total hypothetical scale relative to original effective run sizes, not an additional ratio multiplied by current stored scale. It uses the same approved solver as refresh without persisting it. Unfit-at-approved-floor returns needed_scale=None with reason and remains overflow. Unsupported owners are explicit unmeasured records. F-X204 CLI consumes that common provider and must not turn unsupported into successful fit0 or run a second solver.

## Smallest coupled PowerPoint oracle study

Prepare one controlled slide with four otherwise identical fixed-size unrotated inline text boxes, explicit family/style/point size, no bullets, no theme-dependent fonts, same width/insets, and fresh bare normAutofit. Use a real installed native font whose exact original face bytes/version/hash are recorded and feed those same bytes to rdocx/rpptx measurement. No font install or UI capture is implied by this research task.

- A: several explicit text lines in one paragraph with100-percent/default line spacing, height chosen to overflow slightly at full size.
- B: identical text and bounds with exact point line spacing equal to the initial nominal spacing.
- C: identical text/bounds with150-percent spacing.
- D: identical text and default spacing but split into multiple paragraphs with explicit before/after spacing.

A/B discriminates whether native writes/uses a line-spacing reduction distinct from font shrink, because the current renderer intentionally does not reduce exact point spacing. C discriminates proportional reduction, absolute percent-point reduction or a scale-only answer. D discriminates reducing line spacing versus paragraph spacing. Fixed width and explicit breaks isolate vertical shrink. No wrap uncertainty should be introduced into this first study. Avoid a hand-calculated expected pair.

Capture actual pinned PowerPoint build, generated source SHA, app-selected fonts and exact native edit action that triggers recomputation, then save. Extract each fontScale/lnSpcReduction, actual extents/insets/spacing and complete owner XML from the saved package. Close/reopen/save without editing, capture another SHA and compare the exact values/owners. Merely opening/saving a bare flag is not evidence of recomputation. Do not manually author the expected attributes into the oracle output.

If A/B/C/D do not discriminate quantization, use a second two-height intervention on the same A text with all other inputs unchanged. The two heights should straddle an observed native step, determined from the first actual receipt. Only then choose a candidate lattice and coupling rule. Additional mixed-size/bullet/wrapped controls validate generalization after an actual rule has been identified. Finite observed matches do not prove arbitrary parity, so any remaining discrepancy stays an explicit acceptance barrier.

Separately, a three-box shape-autofit control shares text/size but differs top/center/bottom anchor. Record old/new local bounds and native anchor points. Add one rotated grouped frame only after unrotated anchor results are understood. This keeps coupling and coordinate diagnostics independent.

## Actual unresolved acceptance barriers

Native coupled scale/reduction algorithm is still unmeasured. Current F100 ladder with zero reduction cannot be asserted as the requested PowerPoint algorithm. Shared-context shape resize, unsupported cell/SmartArt/unused shared owners need actual admission and oracle evidence before every-frame completion. These are bounded provider design/evidence barriers, not user preference questions or authorization to approximate. Root can approve ownership/refusal policy, but no plan approval substitutes for required native ground truth.

Canonical read HEAD: `5caf36fd631fb32e2a00acb26b1625d0b99b92ca`. Source SHA receipts:

- `crates/rpptx/src/lib.rs`: `9e203e0e9e3fe71c8b0c9d070a1a0cffa6f757fbd849c88c46702ed910996b6b`.
- `crates/rpptx-render/src/text.rs`: `2a7570f8d9fcdbd1933625a4148ca4ae7d76f2f9773e1d25da821339043323d4`.
- `crates/rpptx-render/src/lib.rs`: `98f646dbb7e10806140b0079298165e79c71c3dcb571e03548fea23f223a641b`.
- `crates/oxml-drawing/src/text/body.rs`: `d9db6dd0a2dc2b61eaa7512c630ba7d33d27762eceb683f7e9a676f9541dc9cb`.

## Authenticated coupled shrink observations

## Method and provenance

Microsoft PowerPoint `16.113.3`, build `16.113.26092714`, verified from its installed bundle metadata and used through native CUA. The scope is generated local controls only. The prior native-authored `/private/tmp/S91-X205-native-blank.pptx` was read and copied into an independent ignored study. No original control, user document, application setting, production source, tracked file or Cargo artifact was modified.

Artifacts are in `/private/tmp/S91-F-X207-native-autofit`. `create_controls.py` constructs correctly ordered DrawingML shapes inside the copied native package. The initial four shapes have bare `a:normAutofit` with no earned fontScale or lnSpcReduction attributes. `input-receipt.json` records the exact native template/input hashes, every original box/property, and Arial face bytes/version/hash. A second generated input reuses only the A100 shape properties with two different heights. Its exact source/input hash and change are in `height-input-receipt.json`.

PowerPoint opened both inputs without repair. Save As made separate native copies before any editing. For each control, native thumbnail and Tab selection entered the shape, F2 entered text editing, Command-End moved to the final text position, and an actual period was typed. Escape left text editing, and Command-S saved native recomputation. Saved XML proves each originally seven-line control ends in `Line seven.` and retains its original paragraph/break count. The scale pair was not inserted by the generator or a post-save XML writer.

Native local `Best for printing` PDF export was selected explicitly. Each saved generated deck was closed, normally reopened by its exact local path, saved without editing, and exported again locally. `audit_capture.py` and `capture-receipt.json` assert frame/text-body, notes owner and scale-pair identity across reopen, identical extracted PDF text and unchanged original input hashes. Every input/saved copy, PDF, extracted text and owner XML has bytes/SHA-256 in the receipt. The initial/final saved package snapshots are retained separately.

Both generated decks were closed only after exact generated URL and `Saved to my Mac` proof. PowerPoint had no remaining document window after closure. The UI lease is released. The existing Word APA6 study was not touched.

## Original four controls

All four use Arial regular, explicit size2800 hundredths of a point, black text, no bullets, identical seven short lines, fixed width5029200 EMU (5.5 inches), height2057400 EMU (2.25 inches), all four insets45720 EMU (3.6 points), wrap square and top anchor. A/B/C use one paragraph and six explicit breaks. D uses seven paragraphs. No line wraps are needed to distinguish these short explicit lines.

| Control | Line spacing | Paragraph spacing | Native fontScale | Native lnSpcReduction |
| --- | --- | --- | --- | --- |
| A100 | explicit100000 percent | before/after0 | 77500 | 20000 |
| BExact28 | exact2800 points | before/after0 | 25000 | 20000 |
| C150 | explicit150000 percent | before/after0 | 47500 | 20000 |
| DParagraph | explicit100000 percent | before/after600 points | 40000 | 20000 |

PowerPoint's UI reported Arial, original28 before editing and effective displayed sizes22/7/13/11 after recomputation. Native XML retains nominal28-point run/default properties plus the scale pair. The PDF content stream uses transforms corresponding to actual drawn22/7/13/11-point glyphs. Those observed drawn sizes differ from multiplying28 by each stored scale without quantization. Do not assume arbitrary mixed-run quantization from four observations.

The reopened PDF was rasterized with pinned Poppler26.01.0 and visually inspected. A/C/D fit inside their outline in this native capture. B's last line is visibly below the outline. B reaches native25-percent font floor with20-percent reduction and still overflows. Therefore the native floor pair cannot be treated as proof of fit. This directly supports an explicit unfit report/refusal in the authoring solver rather than calling every stored native pair a successful fit.

`coupled-bbox.html` records actual native word bounds. B's baseline gaps stay about28 points, with print-grid rounding, despite its20-percent stored reduction. This supports the existing exact-point-spacing interpretation: the reduction does not turn exact28 into a reduced percentage spacing. It also explains why shrinking glyphs cannot fit this fixed box's seven exact-spaced lines.

A and C's measured baseline spans are127-ish and122-ish points over six gaps, despite differing spacing and shrink. Normalizing by observed drawn glyph sizes is consistent with an absolute percentage-point line-spacing reduction rather than blindly multiplying150 percent by0.8. This is a bounded inference from the actual PDF, not a universal verified spacing equation. Paragraph/default font quantization and print-grid hinting prevent claiming an exact arbitrary-stack formula from these spans. The complete PDF content stream and bbox artifact are retained for independent examination.

## Two-height discriminator

The follow-up has two otherwise identical A100 boxes with fresh bare normAutofit, height2377440 EMU (2.6 inches) and2651760 EMU (2.9 inches). Width, all insets, text, explicit breaks, font/style, spacing, wrap and anchor remain identical to A100. Each received the same real native final-period edit.

| Control | Height | Native fontScale | Native lnSpcReduction |
| --- | --- | --- | --- |
| AHeight260 | 2.6 inches | 92500 | 20000 |
| AHeight290 | 2.9 inches | 92500 | 10000 |

Both pairs persist across normal reopen/save and repeated native PDF export. Same fontScale with different reduction rules out a fixed reduction function based only on fontScale, including `lnSpcReduction = 1 - fontScale` and a clipped variant. The first four observations all at20-percent reduction could not discriminate that point.

All six observed scales lie on the existing2.5-percent lattice, and all reductions are10 or20 percent. This is compatible with bounded candidate search that trades line spacing against font size, but does not establish the complete lattice, reduction cap or candidate priority for arbitrary paragraphs, bullets, sizes, inherited settings, wrapping and fonts. Choosing a universal font-first or reduction-first order from six pairs remains unjustified. The old scale-only computation with zero reduction cannot claim parity either.

## Font authentication

The selected UI face is Arial. The emitted PDF's embedded subset identifies ArialMT and retains head font revision6.8000030517578125, units-per-em2048, global bbox(-1361,-665,4096,2129) and hhea metrics(1854,-434,67). These match PowerPoint's bundled `/Applications/Microsoft PowerPoint.app/Contents/Resources/DFonts/arial.ttf`, version6.80i, SHA-256 `15663c7e8495594c6136eb51b11983eb8f5bbcdd07bb2c327e0d1e83415b0f06`.

The known macOS `/System/Library/Fonts/Supplemental/Arial.ttf` is a different face revision5.0099945 with different head bbox and SHA-256 `525979822591a3447cfc49d943d6f7683508e25543407871c0ed8fed05fd2bd9`. `font-resolution-receipt.json` records the subset and both known installed candidates. This authenticates the emitted bundled face revision instead of assuming identical family names mean identical font bytes. The original bundled bytes/version/name tables are recorded for deterministic caller-provided measurement. PDF subset files, drawing-content stream and font-command receipt are retained as ignored evidence. No font was installed or system setting changed.

## Native notes-master evidence for PR318

The native template already contained a notes master. The study added a genuine speaker note through PowerPoint's native Notes pane on the controlled first slide, then saved and reopened. This created an actual notes slide and relationships without changing coupled frame properties. It is structural notes ownership evidence, not a notes-page rendering pass.

Native slide size is12192000x6858000 EMU (16:9). Native notes size is6858000x9144000 EMU.

| Owner | Placeholder | Offset x,y | Extent cx,cy |
| --- | --- | --- | --- |
| notesMaster1 Slide Image Placeholder3 | sldImg idx2 | 685800,1143000 | 5486400,3086100 |
| notesMaster1 Notes Placeholder4 | body idx3, quarter | 685800,4400550 | 5486400,3600450 |
| notesMaster1 Header Placeholder1 | hdr quarter | 0,0 | 2971800,458788 |
| notesMaster1 Date Placeholder2 | dt idx1 | 3884613,0 | 2971800,458788 |
| notesMaster1 Footer Placeholder5 | ftr idx4, quarter | 0,8685213 | 2971800,458787 |
| notesMaster1 Slide Number Placeholder6 | sldNum idx5, quarter | 3884613,8685213 | 2971800,458787 |

The native-created notesSlide1 has sldImg without explicit idx, body idx1, and sldNum idx5. All three have no direct xfrm. The body therefore cannot be paired with notes-master geometry using equal idx alone: its native idx1 differs from the master's body idx3. Actual type/ownership resolution remains required.

Exact extracted owner files use names `coupled-autofit-after-reopen-ppt_notesMasters_notesMaster1.xml`, `coupled-autofit-after-reopen-ppt_notesSlides_notesSlide1.xml` and corresponding `_rels` filenames. `capture-receipt.json` includes the complete relationship XML and every placeholder attribute/transform. These are unchanged across normal reopen/save. The visible notes-page PDF was not captured, so PR318 visible notes layout acceptance remains pending. Do not mistake the ordinary slide PDF for a notes-page render.

## Remaining provider decision barrier

F-X207 now has native real-edit counterexamples and persisted scale-pair evidence. It still needs a measured, approved candidate ordering and shaping/stacking parity sufficient to reproduce the stored pair from the same exact font and text inputs. Mixed size/font runs, default versus explicit100-percent spacing, inherited flags, bullets, wrapped text and shared-owner contexts remain separate generalization controls. No universal formula, successful refresh gate, renderer parity or deterministic baseline update is asserted by this receipt. Root may reconcile the concrete observed pairs into the plan while retaining those bounded acceptance barriers.

## Bounded native coupling analysis

## Proven spacing semantics

Current source already implements absolute percentage-point subtraction, not proportional reduction. crates/rpptx-render/src/text.rs:936 calculates SINGLE_SPACING_EM * (factor - line_spacing_reduction), and exact spacing at938 stays unchanged. The omitted percent case at940 treats original factor1.0. SINGLE_SPACING_EM is1.2 at21. The low-level actual line height at crates/oxml-layout/src/line.rs:1918 is font_size*multiple and :1919 exact points. There is no multiplication bug to fix in this path.

Primary Microsoft documentation, ECMA-376 remarks for NormalAutoFit, says the reduction is subtracted from the original line spacing and applies only to percentage line spacing. It gives original-font scaling with default100% and reduction default0%. Source: [Microsoft NormalAutoFit](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.normalautofit?view=openxml-3.0.1). This describes stored attribute semantics, not the producer's algorithm for choosing attributes.

Native PowerPoint16.113.3/build16.113.26092714 exported Quartz PDFs normally before/after reopen. Arial original font at /Applications/Microsoft PowerPoint.app/Contents/Resources/DFonts/arial.ttf has version6.80i, unitsPerEm2048, source SHA15663c7e8495594c6136eb51b11983eb8f5bbcdd07bb2c327e0d1e83415b0f06. PDF subset revision/global metrics match that installed face, not system Arial5.01. Font-resolution receipt explicitly limits this to emitted-face identification, not a blanket OS resolver trace. Use that exact caller-font face through X195 for the bounded comparison.

Direct content-object3 matrices identify emitted font sizes: A10022pt, BExact287pt, C15013pt, DParagraph11pt. PDF font size is CTM0.24 multiplied by text matrix91.66666/29.16667/54.16666/45.83333, not the literal Tf1. These differ from unrounded stored-scale products21.7/7/13.3/11.2pt. This is an actual export quantization observation, not permission to globally round native renderer fonts. Height controls'26pt is an inference from identical Line glyph bbox and matching face metrics, separately labeled in the calculation receipt.

| Control | Native stored scale/reduction | Mean six native baseline pitches, pt | Prediction with emitted point size and absolute subtraction, pt |
| --- | --- | --- | --- |
| A100 | .775/.20 | 21.1600017 | 22*1.2*(1-.2)=21.12 |
| BExact28 | .25/.20 | 28.0000017 | exact28, unaffected |
| C150 | .475/.20 | 20.36 | 13*1.2*(1.5-.2)=20.28 |
| DParagraph | .40/.20 | 22.52 | 11*1.2*(1-.2)+6+6=22.56 |
| AHeight260 | .925/.20 | 25.0000017 | 26*1.2*(1-.2)=24.96 |
| AHeight290 | .925/.10 | 28.1600017 | 26*1.2*(1-.1)=28.08 |

The six physical baseline steps are retained individually. They vary at0.24pt rendering increments and some differing-line glyph metrics, so averages are informative and must not be represented as exact constant per-line distances. C150 decisively distinguishes rules: proportional reduction predicts13*1.2*1.5*.8=18.72pt,1.64pt below observed mean, whereas absolute subtraction differs by.08pt. D's observed pitch is consistent with unchanged exact6pt before/after paragraph spacing. It does not prove before/after rules for omitted spcFirstLast, percentage paragraph spacing, mixed font sizes or bullets.

## Actual candidate grid versus unknown ordering

The six outputs lie on the existing2.5%-font-scale grid from1.0 through.25, with reduction values.10/.20. A candidate representation can therefore use integer scale ticks100000-2500*k for k0..30 and integer reduction ticks0/10000/20000. These exact integer attributes avoid float-grid accumulation. This is a bounded candidate lattice containing the observed answers. It does NOT prove intermediate reduction values impossible, a20% universal cap, the allowed complete producer scale range, or the order in which candidates are tested.

Same fontScale92500 with lnSpcReduction20000 at height260 and10000 at height290 rules out reduction=f(scale) alone. At least the measured frame height/context participates. Comparing independent one-dimensional maxima also cannot decide a coupled objective. A concrete scale-first lexicographic order is falsified by these controls under the current renderer's actual acceptance equations, not merely unproven:

- Inner heights: the first four use2.25in*72 minus two.05in insets=154.8pt. Height260=180pt, height290=201.6pt.
- For the uniform seven lines with explicit100% spacing and fixed6pt spaces on D, current arithmetic is H_A=7*28*s*1.2*(1-r), H_C=7*28*s*1.2*(1.5-r), H_D=7*28*s*1.2*(1-r)+72. The72pt is six adjacent before/after pairs. These are numeric source-model deductions, not a native rendering run.
- With r=.2, largest source-grid fit would be A.800(H150.528), C.500(H152.88), D.425(H151.968), height260.950(H178.752). Native selected.775/.475/.400/.925 respectively. Each native selected scale is one2.5% step lower than this simplified source prediction.
- Height290 with r=.1 admits source-grid.950(H201.096), yet native writes.925/.1. With r=.2 it admits even.975(H183.456), so maximizing font scale before minimizing reduction would select a distinctly different pair. A purely global reduction-first order would prefer substantially smaller fonts with r0, also not these native pairs.

It would be easy to invent one extra scale rung, a hidden padding constant, a weighted utility or an initial.925 trial to force these answers. None is identified by the controls. Do not encode any such empirical patch as the PowerPoint coupled algorithm. Integer font-point quantization can change fit thresholds, but it does not by itself establish producer search ordering. The current renderer uses scaled original run sizes rather than those observed Quartz integer font emissions (text.rs:652/:656), so exact stored-pair parity and exported PDF metric parity need separate comparisons.

A valid concrete solver design can retain one prepared font manager and one shaped original text inventory, evaluate every integer candidate without mutating any owner, and retain actual StackedText.fits results at text.rs:439, checking both width and height. The solver must accept a documented/native-confirmed candidate ordering. The present evidence only licenses the representation and evaluator, not a final ordering. Refresh/fit_report must consume the SAME eventual ordering and fit threshold. Persist the exact selected integer pair only after every shared owner context passes. Reports retain ordered font diagnostics and the tested pair. No unproved ordering can be silently substituted to mark the design complete.

## The exact-spacing floor is a positive unfit discriminator

BExact28's seven line advances remain28pt, regardless of fontScale or reduction. Current source full line-height sum is196pt, greater than154.8pt for every candidate. Native selected floor.25/.20 and still drew lines past the frame: actual baseline235.68pt and last word bbox lower extent lie beyond frame bottom205.2pt and content bottom201.6pt. The saved native pair is evidence of producer floor behavior, not of successful fitting. It must not become a successful fit status.

Keep canonical atomic unfit contract. An operation requiring all requested normal frames to fit must refuse before publication when B cannot fit at the approved floor. Read-only report returns overflow=true, needed_scale=None and an exact-spacing reason. Do not shrink exact28pt spacing, truncate/drop lines, raise the floor, classify native floor persistence as successful parity, or partially write other frames. Native floor-reproduction can be independently asserted as an observed serialization control while the product deliberately reports failure, with that exact divergence documented.

## Smallest next discriminator and implementation gates

Before claiming the native ordering is settled, capture a near-full-size boundary using the identical seven lines, same exact Arial face, width, insets and explicit100% spacing. Heights3.2/3.3/3.4in give inner223.2/230.4/237.6pt, straddling the unshrunk source235.2pt. Native edits/recompute must establish whether first overflow chooses initial.925, a.975 first rung, reduction before shrink, or another joint rule. A controlled150% spacing partner distinguishes compression priority at the same font scale. Keep exact output/source receipts and ordinary reopen. These are bounded research operations to be scheduled with the current native UI owner, not a question for the user.

Then replay every actual pair and next-boundary output with the exact original font bytes through the current measurement provider. Compare line baseline advances, glyph point sizes, physical first/last bounds and fit decisions separately. A current source-model mismatch is not solved by replacing the native oracle with the library. Expand only where a new production branch actually requires mixed sizes, percentage paragraph spacing, wrapping, bullets or shared master/layout contexts. Record every unsupported/unmeasured case, never silently skip it. Existing saved six-pair controls can be promoted to authored regression expected values only after the chosen algorithm's true source-model fit correspondence is established. No universal six-control parity, scoped gate, baseline update or source approval is asserted here.

## Authenticated near-full shrink threshold

## Input and native actions

The prior bare-normAutofit A100 seven-line control was copied from `/private/tmp/S91-F-X207-native-autofit/coupled-autofit-input.pptx`. Three identical text bodies retain Arial regular28-point run/default properties, explicit100-percent line spacing, zero paragraph before/after spacing, square wrap, top anchor, width5029200 EMU and four45720 EMU insets. Only height changes between the controls. Offsets differ to place all three on one slide and do not alter their internal text geometry.

Input heights are2926080,3017520 and3108960 EMU, exactly3.2,3.3 and3.4 inches. Every initial normAutofit has no attributes. The body SHA-256 is identical for all three. Exact original package and source package hashes are in `input-receipt.json`.

Microsoft PowerPoint16.113.3, build16.113.26092714, opened the generated input without repair. Save As created a separate native copy. Each measured box was selected at its observed frame border, F2 entered text editing, Command-End moved to the final position, and a period was genuinely typed. Escape left editing. An explicit native toolbar Save persisted all three results. Saved XML proves exactly seven lines with the final text `Line seven.` in every measured control.

An initial thumbnail/Tab attempt had selected the unrelated second template slide and typed a period into its title. That local action was immediately undone before any Save. Subsequent edits used the visible measured frame borders. Saved XML for the second slide contains no period title, and that slide is outside all measurement assertions. A coordinate attempt on a ribbon Section button was dismissed with Escape and made no document change.

Native local Best for printing PDF was explicitly selected for both exports. The saved generated deck was safely closed and normally reopened using its observed recent-file entry in the exact generated directory. An explicit native Save after reopen and a second local PDF export produced the comparison. The original input and prior source package remain byte unchanged.

## Persisted native pairs

| Control | Height inches | Actual normAutofit attributes | Effective fontScale | Effective lnSpcReduction |
| --- | --- | --- | --- | --- |
| AHeight320 | 3.2 | lnSpcReduction=10000 | 100000 | 10000 |
| AHeight330 | 3.3 | lnSpcReduction=10000 | 100000 | 10000 |
| AHeight340 | 3.4 | none | 100000 | 0 |

fontScale absent means the defined100-percent default. It is not an unknown observation or a hand-authored100000 attribute. All three native UI font-size readings remained28. Nominal28-point run/default attributes, width, insets, anchor, paragraph/break count and line-spacing properties remain present. Native slide1.xml is byte identical across normal reopen/save.

These observations show that the native engine can apply10-percent line-spacing reduction with no font-size reduction at3.2 and3.3 inches, and remove that reduction at3.4 inches for this exact control. Together with the earlier2.9-inch92500/10000 pair, this constrains a candidate policy but does not prove all intermediate scale candidates or the universal candidate priority.

## Actual native PDF sizes and baselines

The page-one PDF text commands use a0.24 scale and116.6667 text-matrix scale with font-size1. This yields28.000008 points at emitted decimal precision for all three controls. There is no hidden26-point or27-point actual drawn size. The actual page-one font resource is ArialMT, authenticated by the embedded subset's head revision6.8000030517578125 and units-per-em2048. This matches the previously authenticated PowerPoint-bundled Arial6.80i. An Aptos subset belongs to the preserved second template slide and is not used by the measured first-page text commands.

AHeight320 and AHeight330 have top-origin baselines41.99999,71.99999,102,132.96,162.96,192.96,222.96 points. Their six gaps are30,30.00001,30.96,30,30,30 points. Both start at the same slide y offset.

AHeight340 has baselines304.08,338.16,371.28,405.12,439.2,472.08,506.16 points. Its gaps are34.08,33.12,33.84,34.08,32.88,34.08 points. Its different slide y offset accounts for the absolute baseline positions. The observed reduction changes the stack spacing while preserving actual28-point glyphs. Print-grid variation is retained rather than replaced by an invented exact gap equation.

First and reopened PDFs have identical actual font sizes and all twenty-one baselines. Their raw page-one content streams differ: the first export splits the newly typed final period into another text command, while the reopened export combines that line into one command. This is a bounded native export representation difference, not identical PDF bytes. The receipt explicitly records stream identity as false and geometry identity as true. Both complete content streams are retained.

Pinned Poppler26.01.0 produced bbox HTML and a rasterized reopened page, which was visually inspected. All three seven-line frames fit visibly within their outlines in this native render. This observation does not supply arbitrary mixed-font, inherited, wrapped or bullet solver acceptance.

## Evidence and cleanup

`/private/tmp/S91-F-X207-native-threshold` retains the input/provenance receipt, native before/after reopen packages, both native PDFs, extracted slide owner XML, complete page-one text commands, embedded subset fonts, bbox HTML, rendered PNG, `audit.py` and `capture-receipt.json`. The receipt includes bytes/SHA-256 values, exact persisted attributes and defaults, every frame/property/text value, actual PDF font sizes/baselines, and source/input immutability assertions. `audit.py` recomputes the assertions without opening PowerPoint.

The final generated deck was closed only after its exact generated URL and Saved to my Mac state were visible. PowerPoint returned to new/recent files with no document remaining. The UI lease is released. No tracked file, production source, Cargo artifact, native application setting, font or user document was modified.

## Native 150-percent near-full partners

The three previously captured100-percent frames now have matching150-percent
partners. Inputs differ only in the three paragraph spacing values. Native
PowerPoint16.113.3 build16.113.26092714 real final-period edits produce
70000/20000 at heights3.2 and3.3 inches and77500/20000 at3.4 inches. Their
100-percent counterparts are100000/10000,100000/10000 and100000/0.
The complete artifact receipt is
/private/tmp/S91-F-X207-native-threshold150/capture-receipt.json, independently
checked before this record. The ignored appendix is
.claude/scratch/F-X207-native-threshold150-oracle.md.

Saved slide XML, actual PDF glyph sizes and all21 baselines agree across
normal reopen/save. Original inputs are unchanged. Native draws about20/20/22pt
while saved nominal runs remain28pt. This is an observed export quantization,
not a global font-size rounding rule. Raw PDF streams differ because the
initial separately typed period is coalesced on reopen. Do not claim byte
identity for those streams. Both ArialMT subsets authenticate to the previously
identified native bundled Arial generation.

Across all12 current coupled/near-full observations, every reduced scale is
100000 minus7500 times an integer. The earlier2500 lattice contains those
values but does not prove the native step. Under the current simplified
source height equations, the candidate sequence100000/0,100000/10000,
92500/10000,92500/20000,85000/20000,77500/20000 and subsequent7500 scale
steps reproduces these bounded saved pairs. This is a new discriminating
hypothesis, not a demonstrated producer search order. No hidden padding,
weighted objective or universal spacing cap is approved. Exact-spacing floor
B remains unfit and must not become a successful result.

Three further150-percent height controls around the first reduction/shrink
boundary are scheduled to distinguish full-scale20000 from92500/10000 and
whether92500/0 is tried. Actual font-provider measurement and mixed/wrapped
contexts remain required before selecting a general algorithm. Refresh and
fit-report must ultimately use one approved evaluator and ordering. These
observations do not authorize production, a baseline change or completion.

## Taller percent-spacing boundary and fit criterion

Three150-percent single-frame controls at4.35/4.5/4.65 inches produce native
92500/10000,92500/10000 and100000/10000 respectively. Each fixture explicitly
removes the other two controlled first-slide frames, preserves the active
frame x/y/body/width/insets, and changes only active height. Every other input
part, including the unrelated second slide, matches the original source.
The complete receipt and checked artifact inventory are
/private/tmp/S91-F-X207-native-threshold150-tall/receipt.json. The ignored
appendix is .claude/scratch/F-X207-native-threshold150-tall-oracle.md.

All three native saved slide XML files and measured glyph sizes/baselines
are identical across normal reopen/save. Actual PDF fonts are about26/26/28pt.
Raw page streams differ through the already documented period coalescing.
All original inputs are unchanged and generated decks saved and closed.

The4.65-inch result is a genuine fit-criterion discriminator. Its inner height
is327.6pt, while the current seven full line advances at100000/10000 sum to
329.28pt. Therefore the previous full-advance arithmetic cannot reproduce
this native accepted pair. Do not attribute that difference to search ordering
or force it with hidden padding. Existing source baseline_offset and pinned
Arial descent predict a last-baseline-plus-font-descent extent about323.4536pt,
a distinct measured criterion candidate. Actual glyph ink and final-line
leading need independent evaluation before changing renderer fit semantics.

The next bounded4.53/4.57/4.61-inch controls distinguish those end-extent
criteria and whether92500/0 is eligible. After those captures, pause further
native sweeps until an actual exact-font source measurement probe compares
all saved controls through existing layout_shape_text. Keep the current
reported overflow as evidence. Alternate end-extent metrics are diagnostics,
not silently substituted successful results or an approved implementation.

## Final bounded spacing boundary

The final150-percent controls at4.53/4.57/4.61 inches produce92500/10000,
92500/0 and100000/10000. Reduction0 and full scale100000 are omitted in
native XML where they are defaults. Their exact final artifact receipt is
/private/tmp/S91-F-X207-native-threshold150-boundary/receipt.json and the
ignored appendix is .claude/scratch/F-X207-native-threshold150-boundary-oracle.md.
Artifact hashes are independently checked. Every saved first-slide XML and
all actual PDF glyph sizes/baselines agree across normal reopen/save.
Input/source hashes remain unchanged and unrelated input parts match source.
All three final page rasters were inspected and generated decks saved/closed.

The92500/0 middle result falsifies the earlier candidate sequence that omitted
that trial. With inner height321.84pt, its seven full line advances total
326.34pt, while existing-source baseline plus pinned font descent predicts
about320.1736pt. Full100000/10000 at4.61 inches similarly distinguishes full
advances329.28pt from end extent323.4536pt within324.72pt. These controls
support testing end-extent semantics rather than hidden padding, but actual
source measurement remains necessary. Native PDF font sizes about26/26/28pt
are reported separately from unrounded scaled nominal sizes.

There are now18 bounded native coupled/near-full observations. Pause native
sweeps until the prepared exact-font diagnostic runs through the current
public layout_shape_text with immutable saved decks and the authenticated
installed Arial bytes read in place. Retain current renderer overflow and
last-baseline-plus-font-descent as separate outputs. Glyph ink is unmeasured
where the public API does not expose it. No general candidate order or
production change is approved by these finite boundary observations.

## Actual source measurement and bounded solver hypothesis

Successful standalone diagnostic session53221 and compiled-binary complete
grid session77863 both have actual exit0. Source, inputs, output and corrected
canonical-pinned lock hashes are retained under
/private/tmp/S91-F-X207-source-measurement. The original interrupted fresh-lock
resolution and temporary hash-formatter failure remain explicit failed runs.
No repository source, dependency or baseline changed. Original Arial bytes
are read in place, SHA15663c7e8495594c6136eb51b11983eb8f5bbcdd07bb2c327e0d1e83415b0f06.

Across18 authenticated frames, a vertical-only last-baseline plus actual
selected-font descent criterion matches17 native selected pairs under the
bounded sparse-order hypothesis. Existing full-advance fitting matches14.
H465/H457/H461 distinguish terminal metric from full line advance. Metric
end is not glyph ink, and vertical-only success does not prove width fitting.
BExact28 remains genuinely unfit at native floor25000/20000:190.483398pt
metric end versus154.8pt usable height. Preserve unchanged atomic refusal
for that control and explicitly document the native-floor-writing divergence.
Never count it as an eighteenth successful fit.

The sparse sequence100000/0,100000/10000,92500/0,92500/10000,92500/20000,
then7500 scale decrements with20000 reduction down to25000 is a measured
hypothesis, not a proved general native search. It conflicts with the earlier
2.5-percent draft ladder and must be selected only with explicit policy and
reviewed meaningful discriminators before production implementation.
Next focused tests distinguish terminal descenders and last paragraph-after
space, omitted100000/20000 and85000/10000 competitors, a real width/wrap
threshold, mixed/default run styles, late transactional failure, shared owner
contexts and resize anchors. Do not repeat a broad height sweep or redefine
exact28-point spacing to force this control to fit. One evaluator and actual
provider must serve refresh, reporting and render diagnostics.

Detailed read-only decision memo is
/private/tmp/S91-F-X207-decision/design-resolution.md, with authenticated
18-case measured-summary.json and source/input/output read receipt. Existing
explicit refresh, None-family preservation, physical inventory and conservative
shared-context policies remain selected contracts. General terminal extent
and candidate admission still need this bounded final technical resolution.

## Final size and terminal controls

Four additional genuine PowerPoint16.113.3 build16.113.26092714 controls
are authenticated under /private/tmp/S91-F-X207-next-bounded-control.
The30-entry final manifest has SHA
f717573bceeaab91ed246b927ae1670373e280cfe2de88ac468bba49f16138ca.
Normal reopening and saving preserves both native slide parts byte exactly.
PDF content bytes differ, while authenticated embedded Arial6.80 subsets,
measured baselines and drawn sizes agree. No PDF byte-equality claim follows.

The28pt reference, terminal-descender variant and last-paragraph-after8pt
variant all store92500/0 and draw about26pt. A20pt control with proportional
usable height stores92500/10000 and draws about19pt. Source sizes remain
28/20pt. These native choices falsify size-invariant reduction inference.
Terminal after-space does not change the native choice in this one control,
which does not establish arbitrary paragraph-spacing semantics.

Root ran the existing compiled exact-font diagnostic with actual exit0.
measured-four-controls.json has SHA
14283fba0e81b9e42d1e0759415ad42362947c560af5bd0d8d954a820bfd777a.
The existing unrounded terminal-metric hypothesis selects92500/0 for all
four cases and therefore misses the native20pt choice. Full-line-advance
fitting selects92500/10000 for all four and misses the three28pt choices.
Neither result is adopted as a successful production solver.

A bounded whole-point half-up arithmetic hypothesis explains this difference:
20 times0.925 becomes19pt, whereas28 times0.925 becomes26pt. It is not a
shared-layout execution or universal mixed-run rounding proof. The prepared
optional quantized-candidate diagnostic preserves the original source and
measurements, binds22 genuine owners, and recomputes the actual shared
layout of rounded explicit-size clones. Compile and actual execution remain
required before selecting this producer policy. Width, wrapped text, bullets,
mixed/default run styles, shared owners and anchored resizing remain actual
implementation acceptance gates. No additional height sweep is needed.

## PR329 source reconciliation

PR329 closes Issue310 atd6cd56d6554119ea4cfc4b4897a56d660dbe44af. Full
18-file source capture, four commits and empty complete discussions are
authenticated in /private/tmp/S91-PR329-intake. This is source assessment,
not execution or native parity acceptance. Reuse shared stacking, explicit
refresh/fit APIs and concrete owned plans within existing files.

Reject implicit autofit marks before save/to_bytes/PDF/PNG/layout at contributor
presentation.rs:185 and :253. Ordinary output remains read-only. Contributor
text.rs:525 globally rounds stored Normal sizes and replaces the62500
regression. Preserve exact F-100 fractional stored rendering. Its candidate
table at:539 omits our authentic92500/0 controls, and :580 writes an unfit
floor. Retain bounded candidate evidence and atomic refusal. Sequential
apply_autofit_plan writes and unchecked float/EMU conversions at lib.rs:1443
and :1494 require checked clone-stage/reopen publication once. The1034/1024
geometry factor at render/lib.rs:484 has no authenticated local native
receipt and cannot become universal resize policy. Caller-font preparation
uses the actual F-X195 context for every measurement and output. The claimed
285 frames/244 matches are contributor assertions, not our oracle results.

## Actual22-control Q/S measurement and design approval

Root held the exclusive Cargo lease after F-X199 policy true exit. Standalone
diagnostic session26661 exited0, compiling its one main in7.96 seconds and
running actual shared-layout Q/S candidates for all22 authentic owner inputs.
Source SHA2567f244080b5119e49103a9c13356be6d4a37f92d17610096f4e7bed4f051f77dc
and complete result/provenance are retained in
/private/tmp/S91-F-X207-source-measurement/quantized-22-result-receipt.json.
No production source, stored-render semantics or baseline changed.

All21 fitting controls select their exact native saved scale/reduction pair
through genuine re-shaping in both trials, including92500/0 at H457 and
92500/10000 for proportional20pt. BExact28 has no fitting candidate and is
retained as the explicit atomic-refusal control. Every quantized trial
passed source-operand restoration and uses the exact authentic Arial6.80i
SHA15663c7e8495594c6136eb51b11983eb8f5bbcdd07bb2c327e0d1e83415b0f06.
There were zero quantized admission errors. Native glyph-size rounding and
fractional rpptx rendering remain distinct reported facts.

This diagnostic evaluates uniform explicit20/28pt Arial controls. Horizontal
line-bounds containment is a bounded check, not arbitrary-content available-
width acceptance or mixed/font/bullet parity. The old production full-advance
overflow result remains separately recorded. Its metric terminal extent
check is source measurement, not an implemented production overflow change.
The selected physical-segment evaluator, general wrap/mixed/bullet/field/
edge-spacing tests, Q-only falsely fitting round-down rejection, actual PDF
containment, every-frame inventory, checked affine sizing and transactional
late-failure controls remain mandatory implementation/completion gates.
Design is approved without claiming those future tests already passed.

## Concrete owned Python result schema

Presentation.refresh_autofit returns one immutable AutofitRefreshResult:
updates is tuple[AutofitUpdate,...], diagnostics is tuple[str,...] of exact
same-pass owned messages, and changed is bool. Each immutable update exposes
owner_part:str, owner_kind:str, slide_index:int|None, shape_id:int|None,
cell:tuple[int,int]|None, mode:str, status:str, font_scale:int|None,
line_spacing_reduction:int|None, point_size:int|None, old_bounds/new_bounds
as four-integer EMU tuples (left,top,width,height), and changed:bool.
Mode is normal, shape or none. Status is updated, unchanged or empty.
Normal nonempty updates expose the exact saved integer pair. Uniform fit
exposes the chosen integer point_size and mode none. Empty updates leave
fitting values absent. No floats masquerade as serialized integer values.
TextFrame.fit_text returns its one AutofitUpdate with owned diagnostics
attached as diagnostics:tuple[str,...]. Native structures retain the same
facts plus concrete typed location, mode and owned Diagnostic vectors.
FitReport separates native-informed Q facts from actual persisted S facts.
The Python result projection and F-X204 CLI use these actual recorded facts,
not a second measurement or global last-render registry. Existing-file
classes/structures are justified by these two current named consumers.

## Actual Shape resize and size controls, 2026-10-11

Native PowerPoint16.113.4, build16.113.26100421, opened generated local
Shape-autofit controls without repair. The earlier Normal oracle remains
bound to16.113.3. Separate native copies received genuine final-period edits
and normal save, close, reopen and save operations. The exact Arial6.80i
source face remains SHA15663c7e8495594c6136eb51b11983eb8f5bbcdd07bb2c327e0d1e83415b0f06.
No expected resized geometry was authored into the inputs.

The top, centre and bottom controls share three20-point lines, all four
45720EMU insets, width3657600EMU, initial height457200EMU and y914400EMU.
Native height becomes1015663EMU for each. Native y values are914400,
635169 and355937EMU respectively. Top and bottom anchors remain exact,
while the centre differs by0.5EMU. The native first-slide XML is byte exact
after normal reopen. Both local print PDFs retain identical text. Their
page-one content streams differ in run and kerning representation. Pinned
Poppler26.01.0 at144DPI finds48 differing pixels out of1920x1080, with
mean absolute channel difference0.0029513889. This is measured native
export variation, not a product paint pass or a selected tolerance.

The complete capture receipt is
`/private/tmp/S91-F-X207-native-shape-anchor-root-1/capture-receipt.json`,
SHAdeebceecaaf90ef173a8355518c7a44405705558d3cdd291bf95c5dd239f0aa6.
An actual source-built comparison preserves all three source anchors and
the exact text, but computes height1005840EMU. Its9823EMU smaller height
also explains its centre and bottom position differences. This does not
establish native sizing parity. Preserve the diagnostic as an acceptance
boundary rather than introducing an unexplained geometry multiplier.

A second native capture varies font size and line count, with explicit
matching end properties, top anchors, the same insets and width3200400EMU.
Its actual heights are400110EMU for20-point one-line text,707886EMU for
20-point two-line text,830997EMU for16-point three-line text,1200329EMU
for24-point three-line text and1015663EMU for20-point three-line text.
Every control received a genuine edit and normal reopen. First-slide bytes
remain exact. These results distinguish line-count and size-dependent
allocation from a fixed9823EMU correction. No PDF was captured for this
second study. Its exact receipt is
`/private/tmp/S91-F-X207-native-shape-sizing-root-2/capture-receipt.json`,
SHA29673a53ae7f2f4c4a7019eaec1c98d079157c7962b6da923588c6301ddd0d12.

These standalone unrotated observations do not prove grouped or rotated
Shape resizing, mixed-font or bullet paint, installed implementation gates
or whole-feature completion. The selected physical-segment contract and
all remaining plan gates still apply. Additional sizing policy must follow
actual provider and paint evidence, not a fitted constant.


## Final scoped qualification

The implementation checklist is earned by the original meaningful public Base, native22 and physical-font authorities plus the independently reviewed final preservation/provider composition. The completed F206 provider remains intact, including owner routing, flip-then-rotate, diagnostic vectors and namespace-root provenance. Its combined nested affine anchor and inherited namespace controls pass on the actual composed source. The approved9823EMU allocated-versus-physical distinction follows original source/font geometry and painted containment, with no allocation-equality claim or baseline correction.

Current fresh PPT build, exact installed15, two opaque/inherited successes, three foreign-element atomic refusals, strict typing and stubtest pass. All current Word members remain exact. Four affected current verified package archives pass with the full local patch graph and source/member proofs. Current metadata remeasurement, affected policy, prose, adapter synchronization, formatting, focused Clippy and both WASM checks pass. The actual deterministic harness matches all49 entries against unchanged baseline2bd99ae6. Final qualification pass15 records the independent review chain and retained-source boundaries. No integrated whole-sprint gate is claimed.

Evidence and original failed receipts are retained under /private/tmp/S91-F-X207-completed206-localized-rebind-1 and the existing F207 progress/source authority paths. Ordinary output remains unchanged. Explicit refresh and fit intentionally write only approved owned values, sizes and anchored bounds. The implementation commit is labelled for that deliberate authoring behavior and does not blend it into a move or borrowed provider change.
