# F-X268, opened staging, pass 2

**Reviewed**: Mechanism 3 working diff, five source and test files, 1263 insertions and 63 deletions, against worker 19056ee740adf80c1d16a2bacd690c417e2528a0. Root inspected the shared opening record, constructor and clone lifecycle, unchanged-main admission, relationship and namespace inventories, drawing and image canonicalization, save preparation, ordered comparison, three Python publishers, and the added regression assertions. This is a bounded mechanism review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D1 from the first guard pass is closed. The common admission helper now requires cached parsed-model equality as well as successful serialization equality and the unchanged source and namespace context. The redundant canonical namespace fixture reproduced the previous admission error, exit 101, and passed after repair with exact cache-on versus disabled save bytes. Both shortcut callers use that repaired helper.

## Smells

None in this bounded scope.

## Nitpicks

None.

## Evidence and limits

The inspected native batch passed the three opened-main cases, the 50 and 200 paragraph warm-work case, the distinct first-resource-error case, and the related-part case. The affected three-crate clippy passed. Logs are `/tmp/fx268-staging-d1-red.log`, `/tmp/fx268-staging-d1-green.log`, and `/tmp/fx268-staging-d1-clippy.log`. The related comments parser refusal is not a save refusal because the existing writer accepts that package. The older valid-only comparison helper does not prove first-error order. The dedicated ordered resource-error case supplies that narrower evidence.

The previous 56-case installed Python batch predates this native guard repair. Rebuild and targeted Python confirmation remain pending. Child counters are not implemented yet. Whole-model serialization and copying are not zero-work claims. Signature and embedded composition, release fallback evidence, complete A3 and A5 coverage, mechanisms 4 through 8, memory measurements, the full F-ID microscope, and the final integrated union remain pending. This report does not mark the feature complete.
