# F-X256, correctness, pass 3

**Reviewed**: Successor working diff against d75f407b98128f611961c91617ce4cc3da45e905, ten tracked files including the revised approved plan, 509 insertions and 78 deletions. Nine implementation, stub, test and HLD files plus the plan. Review focuses on the remediation and explicitly approved fixture reconciliation since the first freeze, with full contract consistency checked.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass2 D1 is closed. Required checks remain due as stated below.

## Contract and identity

The approved plan copied from aa8d9759 is byte-identical to this worker plan, SHA256e44ab6d4c236c48bd216f55f3117a2350a26e3cd212bd969320cfe1d59d5bebb. Read its finite reconciliation amendment, prior review findings, current progress and F-X249/F-X251 contracts. All ten files match the successor frozen manifest before and after review. Manifest SHA2564cffdd7ee5310feb4f116d57f155270edebd0dbc32c7ee8de617118031ea4d55. Claim-base HEAD and branch remain unchanged. Original pass1 and pass2 hashes were authenticated and neither report was modified.

## Defects

None.

### D1 closure

`crates/rdocx-py/src/document.rs:6450`
`crates/rdocx-py/tests/test_core.py:198`

Actual PyInt classification now precedes checked_section_length and therefore its integer extraction. Bool remains rejected by the unchanged shared helper. Unsupported non-int objects cannot enter their index conversion through this public path. The new two-variant regression separately asserts no callback, actionable TypeError, package equality and live held owners for both returning and raising hooks. Retained d1-red.log shows both variants fail against the actual prior installed wheel. Independent observations retain calls1 in both cases and RuntimeError for the raising hook, with bytes and held owners unchanged. This proves the prior diagnostic/conversion defect without claiming a reentrant document mutation. No shared-helper or other geometry consumer change was introduced.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: the only new production change since the first freeze is public section type-classification order. The previously reviewed full EMU quotient checks, finite multiple/native conversion, Length versus multiple distinction, zero and None behavior, live validated forwarding, native bounds and existing section rollback remain intact. No unsupported-value bypass or native behavior expansion was found.

Tests: exactly seven old collected cases are reconciled under the revised approved plan. Two section fixtures use explicit Emu around unchanged numeric inputs and retain saved geometry, native invalid-gutter and rollback assertions. One missing individual SDT selector now asserts KeyError. Two invalid individual glyph variants now assert ValueError, preserving package and held snapshots. Two body/cell tab variants now assert duplicate-position ValueError and source equality, then use a distinct third position for the original ordered insertion/deletion assertions. Immutable retained tab values, exact reopened values, invalid range refusal and clear/no-op controls remain. Existing native and bulk selector error/count contracts were not changed or widened. No skip, broad exception catch or removed deciding assertion was found.

Panics and OOXML: no new unchecked conversion, indexing, panic path, parser, serializer, schema-order handling or opaque-source behavior was introduced in this successor. The classification fix precedes extraction and publication. The private new regressions exercise error propagation without executing unsupported conversion as an accepted provider.

Structure: no new product file, module, generic, trait, dependency or native API was introduced. The new regression uses one local Python input class for the actual two callback behaviors rather than adding a testing framework.

## Verification boundary

The actual prior candidate full-suite failures and the approved prerequisite/fixture distinction remain preserved. Source review does not qualify missing external inputs or infer fresh successor results. New fresh wheel, full installed suite, typing, stubtest and other scoped checks were still running or pending. The existing worker2 clippy.log records exit101 for collapsible_if at crates/rdocx-py/src/document.rs:6458. That required lint repair and rerun remain outstanding. This report does not call the source scoped-complete or claim that check green. No additional correctness issue is inferred from a pending check.

No reviewer tests, builds, product imports, cache writes, source fixes or remediation occurred. Only this report was written. Prior reports remain byte-identical. The worktree write lease returns to root on delivery. End this pass here, with remaining lint/check work in a separate implementation phase.
