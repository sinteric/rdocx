# F-X211d, microscope, pass 2

**Reviewed**: Frozen3348-file map `4007d483e8f1473e733ea80e1dd904220d3031307ed37a01f26b3ad7adf12a67` and corrective patch `ff88286c2a449aeb4f210dc6edf5c81c83a3acee25621df97b09133822354d14`. Independently authenticated every mapped worker file's bytes, size and mode. Compared against pass1 map0cf61d2a. Only native table.rs and the existing native/Python test entrypoints changed since that review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the reviewed source. Pass1 D1 is resolved. Installed binding and complete feature acceptance remain separate gates.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D1 resolution

`crates/rdocx/src/table.rs:1108` now checks opaque tcPr and nested border stores for each selected row's non-first cells, which are the actual cells subsequently removed at line1147. The guard at line1112 covers CT_TcPr.extra_xml. Line1113 covers CT_TblBorders.extra_xml. Refusal happens before candidate mutation or publication, with the existing direct-cell metadata, vertical-merge, topology and boundary guards retained.

Each row's first selected cell remains physical, and its properties remain in place when span, width and vertical merge are updated. The new offset guard does not prohibit retaining its opaque property or border children. The same-cell no-op still returns at `crates/rdocx/src/table.rs:1091` before the metadata-removal guard. No silent deletion, new normalization or blanket metadata rewrite was introduced.

The genuine pre-fix native RED receipt `.claude/scratch/F-X211d-merge-opaque-defining-RED.json`, SHA `e02d0c18d3898267dadc246d229b0c0adca159d2023ee7705431f2d08b6c1f38`, records both original unsafe successes. The native control at `crates/rdocx/tests/integration_test.rs:25399` constructs the two actual opaque stores separately, checks exact package no-op and atomic refusal, obtains the reopened document's own current table location at line25443, and checks retained opaque bytes after reopen at line25453. This corrects the earlier test-helper use of a foreign source location without weakening the preservation assertion. The failed helper successor remains retained separately.

Read the actual final native successor log `.claude/scratch/F-X211d-merge-opaque-successor-2.log`, SHA `cf3457da669b8efe3a972b393cfc5d5e449a5c98263b0bfb708f8ff865e19fe6`. It reports all four d controls passing, with zero failures or ignores. The two added Python nodes at `crates/rdocx-py/tests/test_core.py:6536` check package identity, held owner survival, no-op object identity and reopened opaque bytes. They are source-reviewed here, not credited as installed runtime passes.

## Unchanged reviewed composition

Pass1's root-record namespace closure, checked prepared merge return/provenance, section/header and paragraph note/equation survival, four canonical margin aliases, lazy source-qualified built-in table style, inherited c Word carriers and root-admitted shared stale formatter are byte-identical in this3348-file comparison. Their bounded pass1 no-finding dispositions stand. The metadata omission was the only pass1 source defect, and this review found no further correctness, preservation, atomicity, owner, dependency or structural finding in its repair.

## Acceptance boundary

This report is an independent source review. It does not admit a wheel installation, infer Python test execution, clear an unrelated repository-policy failure or qualify the complete frozen corpus/native obligations. Original failures, source provenance and the100-script denominator remain unchanged. Root and the worker must complete the remaining actual scoped gates and installed controls separately.

No source, test, runtime, CP environment, output artifact, dependency or Git state was changed. No tests, build, installation or native UI were executed during review. Only this untracked, unstaged worker review file was written under the delegated exclusive lease. Review ends and returns control to orchestration.
