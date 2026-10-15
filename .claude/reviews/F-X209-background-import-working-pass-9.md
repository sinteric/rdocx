# F-X209, background and import correction, pass 9

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: immutable background-import-source42-capsule, two complete native/OXML carriers. Scope is pass7 missing-reference preflight remediation and effective background detachment/reference replacement with owner-scoped image cleanup. Later source45 opaque controls and moving common-owner work are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior pass7 D1 is resolved. Background reference conversion still has the D1 below.

## Authentication

Receipt SHA `db7496782487c275ca0d0beaaa5f3780ab5d13f78e58800186abe80492ba3515` authenticates all 22 records. Native diff SHA `0b02267651254b617eabfc882795cc49c6f7ce742eb5a9a5d53465e7c1e8d84e`, OXML diff SHA `800c4bddf028c7ca9fd7fd9c1b28e5566f56a260074342e509d86f502f0e6e7e`. Copied after hashes are `24ed4b2c8d11d062f82112fcbc8ac50562ebedfc30455c6e4d2d16b67db7fbdb` and `0e6d47f57281c7f7d5d470a11da9b999ca2473189e7dc2786ed19d8d6a782382`. Source42 map SHA `95b6b75d33c5eb06457ae4886bc15423799a35c4d67009d0d784e438cb5812a6`. Approved F-X209 preservation/background/import contracts and canonical microscope/workflow govern this pass.

## Defects

### D1, reference conversion admits opaque attributes belonging to another schema owner

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/background-import-source42-capsule/slide_parts.rs.after:806`

`background_reference_replaceable` uses the same union of val/lastClr/r/g/b/hue/sat/lum for every admitted DrawingML color and transform descendant. These attributes are not modeled on all of those owners. For example, a retained `<a:tint val="50000" lastClr="ABCDEF"/>` beneath bgRef has an unmodelled lastClr attribute on a tint owner. The guard accepts it because lastClr is valid on a different color owner. `replace_fill` then replaces the entire reference range at line 139 and discards that opaque attribute instead of refusing.

The approved contract preserves unmodelled source and uses named atomic refusal for an unsupported reference-to-fill conversion. Root-only unknown-attribute checks at lines 119-129 do not inspect this descendant. Admit attributes by the actual qualified color/transform owner, and refuse unmodelled attributes or payload before the selected reference is replaced. Add a deciding misplaced-known-schema-attribute control with exact package/source retention, alongside the existing ordinary reference positive.

The worker confirmed no downstream per-element guard exists. Later source45 unknown x-attribute/child tests do not decide this counterexample and are not part of this immutable review input.

## Resolved prior finding

Pass7 D1 omitted reference inventory over retained imported XML. `background-import-source42-capsule/lib.rs.after:3927` now collects actual namespace-qualified references from every selected master/layout/theme XML part and checks that each resolves in that part's relationship scope before allocation. Missing refs produce the owning-part diagnostic at line 3937. Same-spelled foreign attributes remain outside the existing qualified provider. The extended control imports each of the three owner kinds with an aliased qualified missing embed and asserts refusal plus unchanged destination whole-package bytes.

## Other inspected behavior

`background-import-source42-capsule/lib.rs.after:3381` validates the actual slide/layout owner and no-op state, initializes the candidate with staged current source, finds the current effective layout/master background, closes its inherited namespace/MC scope, and remaps only qualified image relationship references into the destination owner. It refuses missing, external or non-image dependencies. Restoring inheritance clears the explicit background and releases only previously qualified owner image references that are now unused. Other owners' references keep shared image parts reachable.

The OXML background keeps one raw serialization source and separately tracks safe direct-fill/reference ranges. Ordinary reference-to-properties replacement preserves the bg wrapper and sibling ranges. Foreign or unknown child/attribute checks otherwise refuse in the copied guard. The guard defect above remains material despite the ordinary image-detachment control passing.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: D1 only in background reference replacement admission.
- Contract: D1 only, opaque descendant attributes can be discarded.
- Panics: 0 findings in the bounded slice. Valid owner checks precede selection and namespace-stack expectations retain the initial frame.
- OOXML: D1 only. Qualified import refs and background namespace/MC/image-scope transport otherwise have no findings.
- Tests: no additional findings. Owner-specific opaque attribute coverage is part of D1. Existing ordinary detachment/reopen/repeat/restore and invalid-owner atomic assertions remain.
- Structure: 0 findings. Existing carriers and concrete helpers, no new module, dependency, trait or generic framework.

## Actual evidence and limits

Actual source40 missing-reference row in receipt SHA `7cc5b3d406e1860a4b59c28f68a287befa0459c3b41972da2e366b27b20fe63b` passed exactly one test, covering master/layout/theme missing refs. Its authenticated log SHA is `8247c0736e93f26ece2fedbc19aac314cf41e94891549fb7e32ba6f1efeb1d96`. The same combined attempt remains failed because background detachment exited101.

Source42 background successor receipt SHA `03404d6321ace6f350a4602bc6764b16c40bf9ce5488609cae217266364dbca2` passed exactly one test, raw log SHA `4ec87a333558fa6386475c2f2d966da1909cecc5d301b33611a008560f2f9ac8`. SourceAfterMatches/owned cleanup qualify natural ESRCH and no signals. Earlier attempts6 and8 and the other failed row in7 remain retained. Check22 completed with exit0. No failed wrapper is credited as clean.

No product, source or runtime changes were made by this reviewer. No new installed, native Office/render, later opaque-control or whole-feature acceptance is awarded. Review ends at this report.
