# F-X268, final performance, pass 8

**Reviewed**: Frozen working diff against `461eee97e1aa58b460bce4658f5a58453eaf9351` on `sprint/s95.4`. Six files, 1338 insertions and 146 deletions. Product changes are in the two existing Rust files. The other changes are the approved plan and HLD 03, 04 and 12.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. Both defects concern required acceptance evidence. No product correctness defect was established in this pass.

## Defects

### D1, counted calls lack independent per-action semantic checks

`crates/rdocx/src/document.rs:49613`
`crates/rdocx/src/document.rs:49670`
`crates/rdocx/src/document.rs:49790`
`crates/rdocx/src/document.rs:49806`

The seven-variant A9 gate samples mutations but checks their semantic results only after all four calls. Some checks do not establish the requested mutation at all. For control moves, the final assertion checks the number of controls, followed by common serialization and reopening checks. A path that validates the source and destination, returns success, and leaves the control at its original location can satisfy these assertions while still producing counted work. The gate never establishes the selected control's identity, type, actual destination owner and paragraph-relative position, or its departure from the original position.

The cell case also admits a concrete wrong-owner result. Every fixture cell starts with `alpha` at `crates/rdocx/src/document.rs:49465`. Steps alternate `alpha` to `omega` and back, twice. A faulty path editing an earlier cell rather than the selected later cell can report one replacement on every call and restore that earlier cell to `alpha`. The selected cell remains unchanged and still satisfies the final `contains("alpha")` assertion. This is a source-established weakness in the new acceptance test, not a reviewer-executed mutant result.

The final-only resize, insertion and comment checks at `crates/rdocx/src/document.rs:49780`, `crates/rdocx/src/document.rs:49787` and `crates/rdocx/src/document.rs:49793` likewise do not independently demonstrate that each measured call and each declared changed warmup performed its intended edit. A skipped intermediate mutation can leave the final state correct and allow a later sampled call to exercise an unchanged-state path.

Freeze each measured work vector before running an independent semantic oracle outside the counted window. For each action, establish the exact intended target and result, plus unchanged unrelated owners where appropriate. For cells, check the requested value and the other cells after every call. For moves, check identity, type, resolved source and actual destination position after every call. For the other families, check the corresponding endpoints, dimensions or insertion semantics. This must cover the cold call, both changed warmups and the measured warm call.

### D2, later cold cases inherit caches warmed by earlier families

`crates/rdocx/src/document.rs:49722`
`crates/rdocx/src/document.rs:49732`
`crates/rdocx/src/document.rs:21459`
`crates/rdocx/src/document.rs:21460`

The gate constructs its two fixtures once, then creates each family's purported cold documents with `clone_for_staging`. That method shares both the parsed-state Arc and the main-body-scan Arc with the fixture. Earlier families can populate those shared interiors through their measured operations, preparation and postcondition queries. The common picture and story queries beginning at `crates/rdocx/src/document.rs:49759` also run before subsequent families clone the same fixtures.

Consequently, later cold cases can start with facts retained from unrelated earlier cases. A cache-dependent path can qualify as cold without paying its first-use work, so this test does not prove the required cold versus warm distinction. Logical document cloning does not isolate the derived cache state.

Give every family an independent fresh opening, or otherwise prove that its starting cache state is fresh. Its own normal target acquisition may remain outside the measured window. Keep each family's warm phase on its own document, with the two independently verified changed warmups. This finding does not require counting test setup or semantic oracles.

## Smells

None.

## Nitpicks

None.

## Not found

No additional findings in product correctness, panic handling, OOXML preservation, error ordering or structural rules in the reviewed scope. The audit traced immutable namespace span sharing, borrowed actual parsed children, complete document matching including late unavailable children, exact resource-checked opening-writer proof, the restricted carried-record shortcut, and all three physical-cell-route consumers with their old fallback and pointer checks. Source-model equality remains distinct from writer eligibility. The static owner-name admission preserves the existing qualified set. Private field and root/context fallbacks remain present.

Instrumentation was inspected at attempted reads, byte/event work and rejected candidates. The production empty-frame scan remains counted. Reference-only debug comparisons pause counting. No additional counter-coverage defect was established beyond the two fixture defects above.

## Evidence and limits

The exact reviewed source hashes are:

- `crates/rdocx/src/document.rs`: `7d2ea96db7aa0ce992d9ca0aee40d42f6878833d94ab99a5362705fa8f7a75ad`.
- `crates/rdocx/src/content_control.rs`: `baea0de088d90910c7e3e9a3525d8fd11fb7c86b135d386124c6f14f416dfc18`.

Consumed existing logs without running builds or tests. `/private/tmp/s954-shared-proof-immutable-green.log` records four passing checks in 0.53 seconds. `/private/tmp/s954-controlled-cell-routing-balanced.log` records the balanced route sensitivity test passing in 14.81 seconds, including a real old-route bound failure. The earlier unbalanced fixture failure and initial incomplete full A9 run are not passes. The first namespace RED was a setup compilation failure, while its subsequent pointer RED and the cached-child-copy RED exercise the intended invariants.

At the final inspection, `/private/tmp/s954-a9-counted-work-balanced.log` had no completed full-test result. Its completed direct-cell samples do not qualify the remaining families. No full A9 pass or timing pass is claimed here. The installed V5 release still has six reported timing misses. The separate optimized profiling artifact has different text and is diagnostic evidence only. Memory measurements, installed-package timing qualification, native Word evidence and final integrated gates remain pending as recorded by the implementer.

The reviewer made no source edits and ran no builds or tests. Remediation belongs to the subsequent implementation phase.
