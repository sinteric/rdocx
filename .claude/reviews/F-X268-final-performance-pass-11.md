# F-X268, final performance, pass 11

**Reviewed**: Frozen working diff against `461eee97e1aa58b460bce4658f5a58453eaf9351` on `sprint/s95.4`. Six files, 1481 insertions and 146 deletions. Focused continuation of pass 10 covering the picture insertion oracle and its canonicalization assumption. The approved contract, seven variants and per-pass bounds are unchanged. No new product logic was introduced.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Source review does not establish runtime qualification.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Oracle assessment

The failed preceding run is preserved as an actual failed test. `/private/tmp/s954-a9-native-count-pass9.log` records the assertion failure in picture insertion and a failed result after 241.23 seconds. It is not a count qualification pass. The removed assertion required every prior body item to remain exactly equal after removing the new picture paragraph. That includes generated drawing and relationship identities, which existing canonicalization can renumber when a new picture precedes an earlier insertion.

The current oracle at `crates/rdocx/src/document.rs:49690` compares the measured result with an actual reference insertion from the same pre-action document, using the existing `CanonicalBlockProofOff` guard. The guard is available under `cfg(test)` independently of debug assertions at `crates/rdocx/src/document.rs:6749`. It disables offered canonical block proofs and source-block shortcuts, retaining whole-path processing. The existing canonicalization at `crates/rdocx/src/document.rs:22219` handles authored story-picture relationships and drawing identities. Therefore requiring equality with the reference result, rather than requiring old generated identities, is consistent with the established behavior.

The reference is cloned before measurement without executing document queries. Its insertion runs only after `a9_lookup_sample` has disabled counting and frozen the returned vector. The reference does not replace the measured checked API call. Staged reopening creates its own parsed record and main-body-scan storage, so post-call reference execution does not populate the measured output document's new opening. The switch restores its previous value after reference execution.

The comparison includes the complete document model and exact `package.parts`. This is stronger than checking only the resized or inserted occurrence. It should be described as a canonical-block-proof-off reference, rather than a wholly independent implementation of the public mutation.

Independent semantic checks remain at `crates/rdocx/src/document.rs:49682` and `crates/rdocx/src/document.rs:49710`: body count grows by one, the exact immediate slot is a paragraph, the returned location resolves to that physical direct-body slot, and the independent picture inventory contains exactly one occurrence there with the requested dimensions and PNG bytes. These checks reject a no-op, wrong destination, append-at-end and wrong-size result even if the measured and reference calls share mutation code. Dropping the old-ID invariant does not remove those discriminators.

The reordered family list at `crates/rdocx/src/document.rs:49863` retains all seven variants. Each family still gets fresh byte openings, a separate cold sample, two changed warmups, and the changed warm sample. The same five-times bound with zero additive allowance remains applied to every recorded pass and unit. No prior finding is reopened by this source change.

## Other aspects

No new findings in correctness, contract scope, panics, OOXML preservation, error ordering or structure within this focused delta. The prior product audit and the pass 10 control, cell, resize, comment and cold-isolation assessments remain applicable. There is no product behavior change to approve in this oracle correction.

## Source identity and pending evidence

- `crates/rdocx/src/document.rs`: `24f618c41e27cc9fa03ab77264068b6412669ee23f4cf23ce0b15a50e12a14fa`.
- `crates/rdocx/src/content_control.rs`: `baea0de088d90910c7e3e9a3525d8fd11fb7c86b135d386124c6f14f416dfc18`.

At inspection, `/private/tmp/s954-a9-native-count-pass10.log` had begun picture insertion cold 250 and had no completed test result. The current native count run uses the test profile with optimization level zero, debug assertions disabled and an 8 MiB test-thread stack. It supplies neither release timings nor debug agreement evidence. The preceding debug batch's stack overflow is not a debug proof pass, and its rerun remains pending.

Completed seven-family counts, separate debug controls, installed-package timings, memory measurements, native oracles and final integrated gates remain required. The reviewer ran no builds or tests and changed no source.
