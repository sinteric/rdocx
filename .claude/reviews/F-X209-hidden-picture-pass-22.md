# F-X209, hidden-picture correction, pass 22

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable hidden-picture-source70-capsule, four complete before/after carriers and 7,541-byte incremental diff. Scope is the Picture hidden reader, picture-only resolver omission, associated approved boundary/HLD and deciding controls.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. The defect is in the new test input construction, not an established production failure.

## Defects

### D1, alias/default fixture replacement corrupts namespace URIs

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/hidden-picture-source70-capsule/after/crates/rpptx-oxml/src/picture.rs:1652`
`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/hidden-picture-source70-capsule/after/crates/rpptx-oxml/src/picture.rs:1675`

base.replace("p:", prefix) operates over the entire serialized picture, not just QName spellings. It also matches the final p: in every http:// namespace URI. For q: the namespace values become httq://..., and the default-prefix iteration produces htt//.... The alias input therefore fails parsing with xmlns:p conflicts with the fixed writer namespace before hidden() is exercised. Actual source70 reader control fails at1675 with that exact error. Replace only the intended QName tokens, retain all namespace URI bytes and explicitly establish expanded-name setup. Numeric-entity and genuine local-rebound marker operands should join the corrected qualified fixture as already requested by ROOT. The original failure must remain retained, without inferring a getter failure from this invalid input.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0 production findings. Contract: 0 production findings. Panics: D1 is a test setup unwrap, 0 production findings. OOXML: D1 in fixture namespace construction, 0 production findings. Tests: D1. Structure: 0 findings.

## Bounded source observations

The getter reads the already selected direct cNvPr source owner, overlays retained root/nonvisual bindings and local bindings, verifies the PresentationML expanded name, and accepts only its unqualified decoded hidden attribute with true or 1. The existing non_visual_drawing_text helper decodes and normalizes XML entity values. Foreign attributes cannot supply the flag. The selected raw drawing properties and all writer state remain unchanged. The namespace helper dependency was authenticated against the source70 map before inspection.

The resolver returns no paint only in ShapeTreeChild::Picture when hidden() is true. Other shape kinds, missing/false pictures, effective inherited transforms and visible picture projection are unchanged. The approved plan/HLD amendment describes this exact bounded behavior. The six retained LibreOffice native PDF probes and source attribution explain the input's qualified hidden=1 materialization, but do not establish a current product rendering pass.

## Authentication and actual limits

Capsule receipt SHA256 adad1886cd1e5565f05c33302ecab3498d86a8880a37c3f1826a3596af5c9f37. All 14 local records and 5 external references authenticated by size and SHA256. The capsule was not modified or rebased during this review. The earlier source69 duplicate test-import compile failure remains retained.

Actual-native-hidden-GREEN-2 is a separate source70 successor receipt, SHA256 a811425173e6fd165731b2e082312c7d1ca656ff5666218aab4b4487a35d0de2. Both raw logs match the receipt. Reader selector exits101 and reports 0 passed, 1 failed, 0 ignored. Resolver selector exits0 and reports 1 passed, 0 failed, 0 ignored. Source map3740ee7a13ee0736235cef36a27b4d480602f7c30c78d3df25301fa79c4334eb matches source70, sourceAfterMatches and ownedLeaseSafeToRelease are true, groups naturally gone/reaped with no signals.

Alias/default, entity and rebound reader runtime acceptance remains unqualified. Fresh wheel, installed original failed saved-deck successor, native output comparison, scoped Clippy and whole-feature closure remain separate gates. No source or runtime changes were made. This review ends at this report.
