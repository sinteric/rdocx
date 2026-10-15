# F-X207 provider composition, pass 11

**Reviewed**: External provider composition proposal, 23 existing carriers, 5,352 added lines and 25 deleted lines. This pass reviews incremental provider integration and semantic overlaps, not the entire historical F-X207 implementation.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority and scope

The reviewed proposal is `/private/tmp/S91-F-X207-current8d3eef-completion-prepared/preservation-current-r11/provider-composition-proposal`. Its composition diff SHA-256 is `7ba6a2624966dbb7f95a8ad87ae1f26deb34c4cd66494fdf1b0f2be895e7984d`, manifest SHA-256 is `1f321ddb63e5840dcea30aebf744a15f5f757b852aa0d5aaf770f1e20c1bc147`, and 3,062-record provider guard SHA-256 is `173ca361ef8977213701c951ab0137a0c52f31defb469bd71324f5d2ef578c25`. All 23 claim, provider, owned and candidate file hashes match the manifest. Captured provider HEAD is `22503882a7cb20281c54bd6ad875a2a6f1808616`.

Read the approved F-X207 design, current canonical in-progress status, progress record, workflow, microscope contract and proposal decisions. Retain earlier independent whole-feature and provider reviews for unchanged behavior. The reviewer previously implemented a narrow F-X207 unused-shared-owner accounting remediation and worked on F-X200/F-X195 proposals. This is independent review of the new composition proposal, not a new independent qualification of those historical authored slices.

## Defects

### D1, the owned-shape resolver omits the provider's required member count

`candidates/crates/rpptx-layout/src/context.rs:457`, relative to the proposal directory.

The new `resolve_owned_text_shape` constructs `ResolvedSlide` with size, background, shapes and diagnostics, but omits `unresolved_member_count`. The actual captured F-X206 provider type requires that public `usize` field at `/private/tmp/rdocx-f-x206-codex/crates/rpptx-layout/src/lib.rs:135`. The ordinary composed constructor already initializes it at `candidates/crates/rpptx-layout/src/context.rs:564`.

Consequently the composed layout crate cannot type-check this new constructor. Explicit physical-owner resolution used by fitting and reporting is not executable. The external syntax check cannot establish struct-literal completeness. Preserve the provider's member accounting contract when reconciling this constructor. No compiler execution occurred during review.

## Integration dispositions

- Text-body provenance: `candidates/crates/oxml-drawing/src/text/mod.rs:171` reuses the existing source-root and body-property carrier. Its writer at line298 retains unchanged body properties and closes generated children against the retained wrapper scope. The conditional empty-like shell at line454 retains that same provenance for an opaque autofit choice. No second wrapper state was introduced.
- Opaque autofit edits: `candidates/crates/oxml-drawing/src/text/body.rs:423` captures unknown attributes and accepted comments. The source writer at line544 retains unchanged source or changes only owned choice names and unqualified scale/reduction attributes. `ensure_empty_element` at line811 refuses unsupported PI, CDATA and child payload rather than silently treating them as modeled empty choices. The new lexical and inherited-wrapper controls retain foreign attributes, entities, comments and reopen assertions. No additional preservation defect was established in this incremental seam.
- Affine composition: `candidates/crates/rpptx-render/src/lib.rs:1970` retains the actual painter's `flip.then(rotation)` order, followed by translation and parent transform. `candidates/crates/rpptx/src/lib.rs:12917` measures both anchors through that painter transform and inverts the resolved parent mapping. No competing reconstructed transform order was added.
- Diagnostics and publication: the existing F-X210 renderer-owned diagnostic suffix remains at `candidates/crates/rpptx-render/src/lib.rs:357`. Existing native export and timeline fact handling is retained. Authoring prepares its own manager and contribution scopes at `candidates/crates/rpptx/src/lib.rs:12684` and line12717. Package publication remains staged and occurs after serialized reopen at line1939. No new persistent diagnostic registry or provisional source publication was introduced.
- Bindings: existing causal owner APIs are retained. Caller fonts use the existing collector, made crate-visible at `candidates/crates/rpptx-py/src/presentation.rs:1700`. The added result classes are registered and expose frozen snapshots with tuple diagnostics. `fit_text` uses the existing checked ContentPath without adding a topology revision for formatting alone.
- HLD and measurements: four HLD additions preserve provider sections. Four proposed archive tuples remain historical owned measurements, as DECISIONS.md explicitly states. They do not qualify actual composed archives.

## Not found

No additional correctness or contract defects were established in the reviewed integration seams. Panics: 0 findings. OOXML preservation and child order: 0 additional findings. Test assertion weakening: 0 findings. Structure: 0 findings. Smells: 0. Nitpicks: 0.

## Limits and next deciding controls

This source-only review grants no composed compilation, native, installed, archive, policy or paint acceptance. After correction, the selected qualified namespace and opaque-choice controls must actually run on the composed source. Combined rotation, flip and nonuniform group anchor controls remain due. Historical native and paint results remain bound to their original source, and original failed experiments remain failed. No code, runtime, environment, index or candidate was modified during this pass.
