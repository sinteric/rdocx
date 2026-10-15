# F-X209, picture uniqueness correction, pass 13

**Reviewer**: root, independent read-only bounded review.
**Reviewed**: immutable picture-unique-source59-capsule, exact native source58 to59 correction. One existing carrier, 13 added and one removed line. Source58 test-only duplicate-ID controls and the complete native after carrier provide context. The approved physical owner, preservation and native API contracts govern this pass.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Pass12 direct duplicate-ID discrimination is improved, but the new read preflight has a narrower traversal domain than the supported picture lookup.

## Authentication

Receipt SHA `623a9fd658675eac136539894782ab986917e51b1322665162477236abe512b6` authenticates all13 records, each independently checked for hash and size. Reconstructed before/after unified diff matches correction.diff exactly, SHA `f43e7bd8f547eea2363d54940ca9adb7ee16bf79514eee2bb3c450e62ada38e0`. Immutable before/after carrier hashes are `d62244aa86bce4044d5bc493ff149d2acf27fd5437b5c945b9dbe9bd3b994b7b` and `eb90c1501b0317f339d1440db921c50069040130ab3221d6bfd493f2fa8a50f5`.

The unchanged shape-tree provider was separately authenticated against the source59 map. No moving source was used to infer the correction. No runtime was executed by this reviewer.

## Defects

### D1, uniqueness preflight omits the fallback members accepted by picture reads

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/picture-unique-source59-capsule/lib.after.rs:6772`

The new read preflight requires shape_id_count to equal one, but that helper recurses only into GroupShape at immutable line9638. The actual picture finder explicitly recurses into AlternateContent.selected_fallback at line8543. The unchanged ShapeTreeChild.non_visual_id provider at `crates/rpptx-oxml/src/shape_tree.rs:53` returns an alternate chart-choice identity, not its picture fallback identity.

A valid sole selected fallback picture therefore has lookup success but count zero, causing picture_image_at and the existing slide picture_image API to reject a previously supported read. A top-level picture and a fallback picture with the same ID can instead have count one while find_picture chooses the first encountered member. That retains the arbitrary-target ambiguity the correction is intended to remove.

This is a static code-path finding, not an observed new runtime result. The existing documented read contract explicitly includes alternate-content fallbacks at immutable line6762. Use picture ambiguity admission whose traversal and logical owner domain matches the actual supported lookup. Retain safe refusal for unsupported alternate-content mutation rather than allowing a broader counter to reach the mutable finder, which lacks fallback traversal at line8549. Preserve the raw alternate representations and existing direct owner contracts.

Add deciding controls for a sole qualified fallback read and fallback/top-level or grouped ID ambiguity in the actual owner scope, including exact whole-package retention on replacement refusal. Ordinary direct and nested-group valid reads, current duplicate tests and original failure evidence remain required.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: D1 only in the new uniqueness admission domain.
- Contract: D1 only, supported fallback reads regress and fallback ambiguity is missed.
- Panics: no additional finding in the bounded changed guard. A correction must not expose the existing unsupported mutable fallback path.
- OOXML: no additional finding in the guard. No serializer changes are introduced by this correction.
- Tests: D1 coverage requirement above. Existing direct duplicate and whole-package assertions remain unchanged.
- Structure: 0 findings. Two actual APIs reuse an existing concrete helper, without new module, trait, generic or dependency.

## Actual evidence and limits

Original source58 duplicate-ID Base receipt `eda199098827c887215e419892490483a7032a8f9935dbbf968491f69909ac74` and raw log `366f67b24772f8a75209cb56410b8cb77590317c29bf587716206fdff79caab2` authenticate an actual0PASS1FAIL at F209_AMBIGUOUS_PICTURE_READ_REFUSED Master(0). Source59 successor receipt `55b93f71f5226ad513cd0db4092264c31a0f308fb3a3de879842a1dcc9ff758d` and raw log `d63d327e29128b0e76f51e5ca61e54c154be49bfa11e110c4e5ade0957901f30` authenticate actual1PASS0FAIL0IGNORE for the unchanged deciding selector, including master/layout duplicate refusal, existing valid owner/link/move/remove/timing/save/reopen and atomic controls. Source guards match, groups are naturally gone and leaders reaped with no signals.

That observed direct-owner result remains bounded credit. No fallback runtime, installed current wheel, native Office paint, whole-feature or sprint closure acceptance is claimed. Root notified the worker before the pending install. No source, test, runtime, checkpoint or ledger was modified during this review. Review ends at this report.
