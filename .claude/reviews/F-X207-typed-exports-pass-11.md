# F-X207, typed export inventory, pass 11

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

crates/rpptx-py/python/rpptx/_rpptx.pyi:36 omits AutofitRefreshResult and AutofitUpdate from __all__, despite defining them at lines150 and183 and exporting both at runtime. Actual installed stubtest loga260990ad7327affb4d014b0b3af2a0eeae964866716a91ed0d9b433dc465a68 reports precisely these two missing names, with no names present only in the stub. Add both existing classes to the stub export list. Keep the existing runtime export inventory and class definitions. No allowlist or new class is justified.

## Smells

None.

## Nitpicks

None.

## Not found

Actual receiptc54681d50a1780c320c0fcea978367a9c4077fdd9983790390335a50b6add4a7 proves all15 focused installed cases passed, including genuine late-unfit atomicity. Word strict typing and stubtest and PowerPoint strict typing passed. PowerPoint stubtest failed and post-origin command remains unrun. This does not prove whole-feature completion. Only this review file was written during review. Return to a separate stub-only correction, authentic refreshed wheel packaging and relevant installed typing renewal.
