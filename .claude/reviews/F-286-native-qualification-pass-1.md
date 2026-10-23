# F-286, native-qualification, pass 1

**Reviewed**: `--working` on `sprint/s95.3`, frozen HEAD `7e2e47926cc0229bdf10087b30f060e5a04621bd`. Four scoped files, 247 additions and 5 deletions: existing `test_core.py` (+150), F-286 plan (+44/-1), HLD12 (+26), and CURRENT_SPRINT native checkpoint/factual correction (+27/-4). The unrelated release-target correction in CURRENT_SPRINT is excluded. No product source delta.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this qualification increment.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence examined

Read AGENTS.md, CLAUDE.md, WORKFLOW.md, microscope command, canonical differential-testing guidance, approved F-286 plan, latest scratch progress, existing feature review, sprint/backlog status and cited HLD contracts. F-286 remains approved and in-progress.

Inspected retained native VBA sources, producer receipts, original packages, diagnostic scripts and observations under `.claude/scratch/S95.3-takeover-evidence/native-word-originals/`, `native-word-differential/` and `native-word-consumption/`. Small Python probes ran with bytecode disabled and no file writes. All 58 provenance entries matched their source and retained SHA-256 and size. All 46 entries from `provenance-initial.json` retain their earlier hashes.

Original Word documents reside outside git at `/Users/atulsharma/Library/Containers/com.microsoft.Word/Data/Documents/rdocx-s95.3-oracle-20261022`. The retained producer is Microsoft Word for Mac 16.114 (26101123), observed through native About. Imported `.bas` sources use Documents.Add, ContentControls.Add, native operations and SaveAs2. Unreliable editor paste and failed read-only RepeatV1/V2 attempts are historical failures, not successful producers. Fresh writable V3 supplies the successful equivalent-operation evidence.

Consumed `/tmp/s95.3-native-word-installed-tests.log`, recording 2 passed and 488 deselected. Independently hashed the active release extension in `/tmp/s95.3-opaque-release-probe`, confirming `9b9234fa1969f29c38e22a725dc3e78df253b6e30a8792273a7fb8b569b4b8b5` against the retained artifact manifest. No tests, builds, broad suites or native UI were run in this review.

## Not found

- **Input authentication**: the fixture pins producer/version/build and hashes actual recorded source bytes before use, including native expected outputs. Missing bundle is a skip, not native success (`crates/rdocx-py/tests/test_core.py:6371`).
- **Typed replacement discrimination**: comparisons include control identity, ordered type payload trees, text and drawing counts. List display/value differ. Refined date XML independently confirms `yyyy-MM-dd` versus `dd/MM/yyyy`, before and after replacement. These inputs reject unchanged text, wrong selection, date-format collapse and payload loss (`crates/rdocx-py/tests/test_core.py:6404`, `crates/rdocx-py/tests/test_core.py:6425`).
- **Atomic refusal**: picture and three native group inputs require an exception and byte-identical library serialization after refusal. Native receipts record refusal and retained before/after control XML is unchanged (`crates/rdocx-py/tests/test_core.py:6434`).
- **Repeat discrimination**: distinct First/Second/Third text with only First bold rejects wrong template, lost bold and wrong insertion order. Assertions retain existing identities, require a fresh unique clone identity, and preserve the correct survivors after middle deletion. Independently checked retained XML against `distinct-repeat-observations.json` (`crates/rdocx-py/tests/test_core.py:6482`, `crates/rdocx-py/tests/test_core.py:6498`).
- **Provenance boundaries**: twelve library-authored consumer inputs and native saved outputs independently match recorded hashes and type/payload/id/tag/text/drawing/media facts. RichText omission and date lid=en-GB are explicit comparison normalizations. Group contentLocked addition is recorded separately. OpenAndRepair=False and native OPEN/SAVED receipts support consumption acceptance, not strict structural parity or original Word authorship (`.claude/plans/F-286-design.md:213`).

## Pending and excluded

Q24 remains explicitly pending. Native default repeat copies the addressed item. The successful equivalent appends after the last item and copies first.Range.FormattedText. Native last-item deletion removes the section, while the approved library API refuses atomically. The test asserts this difference and the plan preserves the blanket differential gate pending a decision (`crates/rdocx-py/tests/test_core.py:6503`, `.claude/plans/F-286-design.md:234`). This existing pending decision is not a new finding.

The comparisons qualify their stated observables, not every XML property, rendering or the entire A1 through A8 acceptance matrix. No whole-feature acceptance, B1 closure, acceptance waiver, fresh sprint verification or sprint review pass 4 is supplied. No external issue closes. Historical cleanup evidence is retained separately from active artifacts.
