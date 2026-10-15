# F-X201 image owner checksum attribution, pass 7

Reviewer: fonts_resume, independent of the implementation.

**Verdict: 1 defect, 0 smells, 0 nitpicks.** This pass diagnoses repeated image watermark replacement only. Pass6 rotation and theme-border remediation, image rendering/refusal completeness, and styled notes are not granted acceptance by this diagnosis.

## Bound evidence

Immutable capsule `/private/tmp/S91-F-X201-image-owner-diagnosis-r2/receipt.json` SHA-256 `d323e39838530b828ff0e159edb26b24086813447f4e7b8caaa73e198cc45ab5`. Source map `77e68e86ab70f6db913bedab5cda8bc925f634773ccdd8647eb9db291a5a240e` and diff `2aac5b97997d0523532428ad57f2f7b20639d22d4ff184b776bca72e9ba6b875` are bound. All36 manifest and receipt references authenticated with zero mismatches. Native document.rs SHA-256 `46908905bb937638055c06e488b29db00758031ca47bb9744196977734e6030a`. Production is identical across diagnosis1/2, with only test diagnostics changed. Citations refer to copied `source/` files in that capsule.

Actual diagnosis2 receipt `8ae6e28f022168343c3905b53a0da06d7fdc7f6d9e2ea59c6aff96565c57d278` retains two child101 rows. The image control fails at regression_test.rs399911 on the second replacement iteration. Both owned groups are gone, leaders reaped, safe release. Styled-note failure is retained but excluded from this review. No failed attempt is relabeled as acceptance.

## Defect

### D1. Replacement hashes a provisional relationship ID before staging renumbers it

`crates/rdocx/src/document.rs:22337`, `crates/rdocx/src/document.rs:22371`, `crates/rdocx/src/document.rs:22387`, `crates/rdocx/src/document.rs:15537`, `crates/rdocx/src/document.rs:14837`, `crates/rdocx/src/document.rs:855`.

The image setter allocates a new image relationship while the old watermark relationship still exists. It builds the owner and its exact-byte source fingerprint using that provisional relationship. After replacement it prunes the old relationship, and prepare_staged_package canonicalizes related-story identifiers. canonicalize_related_story_relationships rewrites actual qualified relationship references at14837 but does not update the watermark marker. The next replacement scans the saved physical source and refuses its own unchanged owner.

The retained first owner XML SHA-256 `0feb36ca92d698889e114956de55f7b5f3edf3b7d8b91731ceed5d7f39db10de` recomputes exactly to declared3623461979753378675. The saved opacity0 owner XML SHA-256 `4d74c691da949d3fffe761f7208721dae45caeb4973c1e85c915094c4ecd0935` declares8778204005819137341 but recomputes10189429556975011758. Using the exact bytewise FNV routine at9942, changing only its `r:embed="rId0"` to `r:embed="rId1"` reconstructs the declared8778204005819137341 exactly. No namespace, opacity, whitespace or producer-content change is required to reconstruct the producer hash.

Bind ownership to the actual final staged owner bytes or ensure this operation does not fingerprint an identifier that subsequent mandatory staging will change. Preserve foreign and opaque-source refusal, actual relationship ownership, pruning and all other identifier invariants. Do not broadly normalize references or accept arbitrary checksum mismatches. Repeated replacement must leave one live image relationship, render the requested opacity/rotation, reopen byte-stably, and permit source-qualified removal. Externally changed owners must remain an atomic refusal.

## Six aspects

- Correctness: D1. Zero additional findings in the bounded attribution.
- Contract: D1. Repeated valid API replacement is rejected despite unchanged owned source. Zero additional findings.
- Panics and errors: 0 findings. The test unwrap exposes the returned named refusal.
- OOXML: 0 separate findings. The mismatch is the qualified relationship reference changed by existing staging, not invalid namespace input. Exact foreign/opaque retention requirements remain.
- Tests: 0 findings. The repeated opacity0 then opacity1 control reaches the real operation and exposes the defect. Initial and post-replacement XML were retained independently.
- Structure: 0 findings. No new registry, trait, module or dependency is needed to explain the cause.

Only standard-library read-only hashing and source inspection were performed. No product imports, tests, Cargo, source edits or environment changes. Only this external review file was written. Return for distinct remediation.
