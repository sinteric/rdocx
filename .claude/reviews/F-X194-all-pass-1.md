# F-X194, all, pass 1

**Reviewed**: Frozen working diff against Base400e52b09c60f44ea20f0de7cd05240ed6952b5f, 17 existing files, 646 insertions and 81 deletions.
**Diff SHA-256**: a2797a94b91e1878a3f3091ed7a7ec80245544527e83ef80281f220c394c5e2e
**Source receipt SHA-256**: 585bb4fff873ca57879e700f1c1ddc6ef6cdd86ade649955a006bb72730474fb
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: cell-aware immutable and mutable resolution at
  `crates/rpptx-py/src/text.rs:129` and `:140` retains complete grid-cell
  ownership and grouped shape traversal. The getter at
  `crates/rpptx-py/src/table.rs:629` validates the cell without materializing
  its optional body. Topology publication at text.rs:151 preserves global
  shape behavior while selecting the cell-text counter for cell descendants.
  Add-paragraph and add-run return paths at :343 and :684 carry the new
  generation, while collection descendants at :874 and :1116 clone both
  generations. Run and font value writes do not bump either generation.
  Cell merge/split bumps at table.rs:662 and :670 follow native success,
  preserving grid handles and leaving failed staged operations unchanged.
- Contract: the complete approved F-X194 plan and its cited HLD01 units,
  HLD05 text/table model, HLD08 placement, HLD10 binding ownership and HLD14
  story sections were checked. `crates/rdocx/src/table.rs:803` implements the
  selected optional signed setter with validation before property creation
  and no property creation on clear. Existing shape captures remain None at
  `crates/oxml-py-support/src/lib.rs:44`. Shared field names match the approved
  later F-X206 partition. `crates/rpptx-py/src/text.rs:1065` refuses cell
  hyperlink assignment before native relationship publication, while reads
  at :1025 resolve the cell body. No native cell hyperlink API or separate
  Word placement policy is introduced.
- Panics: `crates/rdocx/src/table.rs:413` uses division and checked signed
  conversion, including i64 extremes, without overflow-prone negation.
  `crates/rpptx/src/lib.rs:8653` indexes private table-cell coordinates that
  the existing consuming constructor checks at :8292. The exclusive native
  mutable borrow prevents intervening grid mutation. The borrowed accessor
  at :8558 ties its result to the actual cell lifetime. Python path and
  generation validation precedes resolution, so stale nested paths refuse
  before native mutation. No new untrusted unwrap or expect is introduced.
- OOXML: `crates/rdocx/src/table.rs:808` changes only the modeled direct dxa
  indentation and :810 clears only that property. Existing writers retain
  schema order and opaque properties. Cell text authoring reuses the same
  TextFrame/Paragraph/Run machinery as shape text. Explicit merge continuation
  addressing remains separate, as exercised at
  `crates/rpptx-py/tests/test_documented_examples.py:4782`. The missing-body
  counted-replacement preflight at text.rs:296 rejects empty placeholders
  and mismatches before the consuming accessor can create a body.
- Tests: the missing-API gate at
  `crates/rpptx-py/tests/test_documented_examples.py:4694` and Word indent gate
  at `crates/rdocx-py/tests/test_formatting_tables.py:1064` are meaningful
  against the unchanged Base surface. Retained actual installed Base probes
  report AttributeError for each API, with original environment provenance
  kept distinct from the claimed Base and matching production-source guards.
  Companion tests cover nested stale handles, successful return generations,
  other-cell invalidation, failed/no-match source preservation, merge/split,
  grouped tables, imported hyperlinks and signed bounds/truncation. Native
  signed authoring at `crates/rdocx/tests/integration_test.rs:24368` round trips
  and tests absent clearing, extreme rejection and exact endpoints. Existing
  floating placement at :21188 and margin/compatibility matrix at
  `crates/rdocx/tests/regression_test.rs:56980` now include signed values.
  PDF/raster controls at documented_examples.py:4871 use the actual Python
  deterministic render routes at presentation.rs:223 and :236.
- Structure: the 17-file diff adds no module, file, crate, dependency, feature
  flag, trait, generic parameter or new integration binary. Resolution remains
  concrete in existing text.rs, and the native consuming cell accessor follows
  the existing shape accessor pattern. `docs/hld/10-bindings-spec.md:646`
  describes current cell ownership and :669 keeps bodyPr anchor distinct from
  the F-X206 cell tcPr placement contract. No sprint ledger, baseline or corpus
  mutation is included.

## Verification boundary

This is an independent source and test-design review, not scoped verification.
Both frozen diff identity and all 17 file hashes matched before the report.
The worker still owns actual installed final Python tests, typing/stub checks,
full native/scoped lint, WASM, unchanged hash comparison, archive measurements
and applicable verified publication dry-runs. The mixed-source interrupted
Base harness attempt and initial compile failure remain distinct evidence,
not passing gates. No Cargo, formatter, runtime import or specimen was run by
this review. Only this review file was written.
