# F-X208 whole working review, pass 7

Reviewer: fonts_resume, independent of the F-X208 implementation.

**Verdict: 4 defects, 0 smells, 0 nitpicks.** This review covers the complete frozen 27-carrier source24 implementation against the approved F-X208 plan, its spec references and seven HLD impact files. It is not feature completion or permission to replace required acceptance gates.

## Authenticated input

Immutable capsule `/private/tmp/S91-F-X208-working-source24-capsule/receipt.json`, SHA-256 `b20283b316d1c0a0f5562b9afa7f09c422fd6cad185a1f4d0bcf21cd560793e5`. Source-production24 map SHA-256 `1f33f1195e93e69d55c2ccb5f16f126ccc0964d9189f96a4cd85d05a81a5fd92`. Complete diff SHA-256 `9cb5d037d207a980455685c32028c47967a8fc8cee265f22e7975d2e675ab5c8`. Every compressed candidate, uncompressed hash and size was independently authenticated. All 27 source candidates also matched the frozen worker bytes when inspected. HEAD is `a6f3f1c1c014f510f5ff79827eae02974f574d01` on `work/f-x208-codex`. The capsule retains original HEAD and index evidence.

Citations below refer to the uncompressed source24 candidates under this capsule. Native lib.rs SHA-256 `c43b24b76132f4f6842bd00c1738993935ee09a03072b448a786dbeabe639a7c`, layout context.rs `83d34c8f685c57cc552e898feb91160ec9a74e3f4feef1287b19e1e133827c4a`, Python slide.rs `69ddf0a61dee8b09286bea6a2b1bd60af01418c340389bcc4548289e91f87bb3`. The reviewed native, layout, renderer, model, Python, CLI, tests and HLD carriers comprise the whole feature diff, not only earlier focused slices.

## Defects

### D1. Footer authoring publishes templates without usable effective geometry

`crates/rpptx/src/lib.rs:15746`, `crates/rpptx/src/lib.rs:15839`, `crates/rpptx/src/lib.rs:16033`, `crates/rpptx-layout/src/context.rs:1359`.

The template selection and copying paths validate qualified placeholder existence, duplicate counts and relationship transport, but do not resolve or validate usable effective geometry. latent_copy explicitly ignores from_master. Both apply_to_slide and new_slide_latent_shapes can therefore publish a qualified footer template whose own and corresponding inherited transforms have no usable extent. commit_candidate at native lib.rs:3570 serializes and reopens that result. It does not establish renderable geometry. The renderer subsequently returns no resolved shape when transform_values fails at context.rs:1359, so the operation succeeds while the requested footer disappears.

The approved plan at `.claude/plans/F-X208-design.md:336` requires missing or ambiguous template geometry to refuse atomically. Resolve the source-owned layout/master geometry, including the qualified corresponding owner and supported group context, before publication. A valid inherited-geometry positive and missing/ambiguous geometry negatives must decide this boundary, with unchanged complete package bytes on refusal. Do not synthesize coordinates or infer that a successfully reopened package paints the requested content.

### D2. The compatible datetime alias loses its existing cached value

`crates/rpptx/src/lib.rs:15708`, `crates/rpptx/src/lib.rs:15713`, `crates/rpptx/src/lib.rs:15525`.

The automatic-date no-op branch compares field types literally. A retained imported field of type datetime with cached text, followed by set_header_footer using date="auto", the default datetime1 and no replacement cache, fails that comparison. It creates a new datetime1 field and clears its text. The explicit formatter recognizes datetime as the datetime1 alias at native lib.rs:15463, but this compatibility is not applied when retaining an existing cache.

The approved cache contract at `.claude/plans/F-X208-design.md:334` requires compatible existing automatic caches and imported aliases to survive absent replacement text. Compare the supported semantic alias identity for this retention decision while preserving imported source when no change is needed. Keep incompatible format replacement and an explicitly empty replacement cache distinct. Add a deciding datetime-to-datetime1 control with cached text, save/reopen and repeated assignment.

### D3. HLD05 advertises an unsupported field type

`docs/hld/05-drawingml-model.md:467`, `crates/rpptx/src/lib.rs:15393`, `crates/rpptx/src/lib.rs:15502`.

The new current-intent paragraph says field insertion supports slidename. The authoritative native list contains exactly slidenum, datetime and datetime1 through datetime13, and text_field refuses other values. The Python route delegates to that native operation. There is no slidename authoring implementation in the inspected source. Correct the HLD inventory to the approved, actually supported set. This finding requests documentation alignment, not an extra unapproved field feature.

### D4. Uncached automatic fields render silently without the required report

`crates/rpptx/src/lib.rs:15525`, `crates/rpptx-render/src/text.rs:325`, `crates/rpptx-render/src/text.rs:337`.

Authoring intentionally supports an automatic field with no cached text. Rendering then treats every non-slidenum field as ordinary cached text and shapes that value, which is empty for the uncached case. No date-specific layout or output diagnostic reports that the static renderer has no computed date. Exposing an absent cache through the HeaderFooter getter does not report the condition in the render result.

The approved final contract at `.claude/plans/F-X208-design.md:334` explicitly requires this report when there is no compatible or supplied cache. Preserve the clock-free behavior and source XML, but carry a stable report through the actual static render diagnostic path. A deciding control should distinguish absent cache from an explicitly empty cache and a nonempty cache, and verify repeat rendering without ambient date calculation. This is separate from D2, which loses a cache that should already have survived.

## Six aspects

- Correctness: D1 and D2. Zero additional proven correctness findings across footer application, displayed-start assembly, per-master theme lookup, field insertion, transition mutation/composition, Python owner views and CLI publication.
- Contract: D1 through D4. Zero additional contract findings. The implementation retains title-layout qualification, legal owner policy versus painted owner facts, zero displayed start, physical page identity, explicit date inputs and the eight common transition effects.
- Panics and errors: 0 additional findings. Checked indices, field identity allocation, duration/range conversion and fallible candidate preparation remain in place. Deck operations stage owner changes before publication. This does not excuse the missing geometry preflight in D1.
- OOXML: 0 additional findings. The audited paths retain qualified namespace closure, physical template identity, grouped ancestors, known flag patching, ordinary unrelated shapes and opaque header/footer fields. Relationship-bearing unsupported footer carriers refuse. Transition editing preserves sound, extension payload and unselected AlternateContent branches, and refuses unrepresentable type changes. D1 and D2 remain semantic preservation defects despite syntactically legal XML.
- Tests: 0 separate findings. Existing native and public controls exercise owner policy presence, title suppression, source stability, field identity, transition rollback and displayed-start static/timeline behavior. The deciding gaps described in D1, D2 and D4 remain required. Existing passing broad tests cannot establish those absent predicates.
- Structure: 0 findings. Work stays in the 27 existing carriers. Live views resolve physical owners instead of holding detached mutable themes or header/footer state. Publication uses the existing causal owner mechanism. No new crate, module, dependency, speculative trait or forwarding-only abstraction was introduced.

## Earlier findings and acceptance limits

Earlier focused pass1 date-mode D1 is resolved by the Field-aware fixed-text no-op check and the same-visible-string automatic-to-fixed, reopen and repeat control. Pass3 policy D1 is resolved by optional legal hf flags and policy-source projection, separate from materialized presence. Pass5 Zoom-default D1 is resolved by the renderer's absent direction default out, with the deciding control reviewed in pass6. Passes2,4 and6 retain their narrow remediation authority. This whole review does not retrospectively widen those runtime scopes.

The capsule's nine retained runtime receipts and referenced raw log hashes were authenticated. Source24 changed-library and CLI suite receipt `7bdf5f487e233ce3f343eda41f8a470aab3ab4bf95c5addf77d508e3c7c7fe45` records successful rows. Source19 installed/typing successes remain source-qualified to that wheel and do not qualify a final wheel containing the later Zoom renderer repair. Failed installed wrappers remain failed, with their successful individual successors separately identified.

ROOT subsequently reported the source24 integration gate terminal with 315 passes and 28 existing ignores. This is a coordinator observation, not a new execution or independently authenticated receipt in this pass. The current scoped Clippy log at `.claude/scratch/F-X208-start-1/actual-scoped-Clippy-1/changed-crates-Clippy.log:20` and `:40` records derivable Default at native lib.rs:16621 and a useless string conversion at lib.rs:19099. Clippy remains failed and must be closed in the separate remediation phase. Neither the existing ignores nor the failed lint gate receives acceptance credit here.

The ROOT native library open/save/reopen and paint records under `/private/tmp/S91-F-X208-native-library-acceptance-root-1/` describe observed PowerPoint 16.113.4 build 16.113.26100421 results. Their paint receipt expressly excludes repository-renderer pixel identity and automatic-clock expected values. Earlier ten-case native policy observations remain their own historical authority. Fresh final wheel/member provenance, installed and typing qualification after actual source repairs, renderer comparison and all remaining declared hash/WASM/archive/risk gates remain acceptance obligations. This report grants none of those unfinished gates.

Only this external review file was written. No source edits, formatting, tests, product imports or runtime commands were performed. Return control for distinct remediation.
