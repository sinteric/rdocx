# F-X206 HLD union, pass 58

**Reviewed:** Incremental R2 HLD correction `/private/tmp/S91-F-X206-HLD-union-r2-prepared`, receipt SHA-256 `08a7fb7d604c03c89e58bc686448dd62fb8ba0b466462e5c81e9f724d530d970`. All eight approved HLD candidates and their preimages were authenticated. Only HLD10 changes relative to R1. Full diff SHA-256 `676463b19450528d3a850754dea9b1f453d37025dd3be8257db700e588273ba7`. Incremental diff SHA-256 `78d4119601b2e533ddd80e084d798ce7a89e7599a778192804dcb35dcd857552`.

**Verdict:** 1 defect, 0 smells, 0 nitpicks. No adoption or runtime performed.

## Defects

### D1, slide replacement prose is not backed by the bound source union

`/private/tmp/S91-F-X206-HLD-union-r2-prepared/candidate/docs/hld/10-bindings-spec.md:863`

The replacement paragraph now says both binding routes stage a cloned presentation and finish checked owner preparation before publishing. At the same candidate file:869 it names Slide.try_replace_text as the retirement cause and promises affected-descendant retirement without global invalidation.

The bound R16 plus cfg prospective map ce60 retains `crates/rpptx-py/src/slide.rs` SHA-256 `c5032bbb031ff75d8cc1ac231ced1ade76d816c00205cfef86fb0f3edb3b846e`, exactly the actual a9 worker file. R16's candidate manifest does not modify this carrier. Actual `/private/tmp/rdocx-f-x206-codex/crates/rpptx-py/src/slide.rs:259` takes a mutable reference to the live presentation inner and calls native try_replace_slide_text. At that source file:267 it performs a bare revisions.bump for a nonzero count. This binding route contains no cloned-presentation staging, prepare_owners, publish_owners or Slide.try_replace_text retirement-cause operation.

Trigger: a nonzero replacement through Slide.try_replace_text. The HLD promises a concrete owner-publication implementation that is absent from its declared source union. Native selected-slide validation is distinct from the newly claimed binding-owner preparation mechanism. Reconcile the paragraph with the actual source, or independently implement and review the approved staged owner semantics before treating this HLD clause as delivered behavior. Do not weaken the approved owner-survival contract merely to match this remaining implementation gap.

Prior pass 57 cited a staged slide implementation at slide.rs:275 and :284. Those citations are not supported by the actual a9 carrier or the bound ce60 map. That source assertion cannot be inherited as proof for this R2 paragraph. This pass records the discrepancy explicitly rather than silently accepting the prior citation.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects and checked nonfindings

- **Correctness:** D1 above. The group-add paragraph agrees with prospective shape.rs:1375 through :1396, which stages a cloned presentation, prepares unchanged owners, allocates the returned Python shape against successor authority and then publishes. It no longer falsely requires re-fetching the group collection after addition.
- **Contract:** D1 above. The text-frame paragraph agrees with prospective text.rs:357 through :375: cloned candidate, count validation, selected text-changed owners, the TextFrame.try_replace_text operation label and publication only for a nonzero count. The absent-cell-body zero/count-mismatch behavior agrees with prospective text.rs:335 through :351. The eight-file HLD impact list remains exact.
- **Panics:** 0 findings. This documentation proposal introduces no executable operation or arithmetic. It cannot supply missing implementation or runtime credit.
- **OOXML:** 0 additional findings. The correction does not change geometry, namespaces, source order, opaque ownership, picture crop or preservation clauses from the prior reviewed candidates. No normalization or foreign-owner promotion is added.
- **Tests:** 0 additional findings. No test, oracle or acceptance result changes. Actual Base, current GREEN, installed coverage, qualified native application and no-repair evidence, recovery/caps/highlight, archives, hash attribution and scoped policy remain separate mandatory gates.
- **Structure:** 0 findings. Exactly eight existing approved HLD carriers. Only one candidate differs from R1. No new product file, type, module, dependency or API.

## Authentication and integration boundaries

Every sealed artifact hash matches. All eight candidate and preimage identities match the manifest, and physical worker HLD preimages agree with the actual a9 map. The prospective source map retains all 2714 records and differs from declared ce60 code source only at HLD02/03/05/06/07/08/10/14. F195, F200 and F213 current canonical clauses remain localized integration boundaries. Complete older worker HLD08/10/14 candidates cannot replace newer canonical files wholesale. R2 does not claim metrics, archive measurements, runtime acceptance or feature completion.

This is an incremental HLD/source-binding audit. Unchanged prior HLD and independently reviewed source slices retain their earlier authority. No new independent audit of historically self-authored production is claimed. Only this report was written under ROOT's explicit review-write lease. Return the lease to ROOT and end this pass.
