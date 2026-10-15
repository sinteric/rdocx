# F-X197, typing tests, pass 4

**Reviewed**: Root-authored movement of two existing getter assert_type lines
in crates/rdocx-py/tests/typing_smoke.py, on work/f-x197-codex at
b96c7888b7d99bf1ab6c720ea5ba318c7103d611. No statement was removed.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this incremental test review.
No Cargo, import, mypy invocation, source fix or formatting was performed.
The full implementation retains independent all-aspect review2 and the
previous fixture correction retains tests-pass3.

## Authentication and contract

Read the approved197 plan, workflow, microscope and current progress.
Installed strict typing and retained declared getter/setter signatures remain
mandatory. The feature remains in progress, with original actual failures
retained rather than reclassified as success.

Current typing source SHA-256 is
2bf4f31d53f129bd95f7b61cdb5339b204b6a9255d1f505443f8eb4737224233.
Independently reversed only the two-line movement in memory and recovered
original R2 and R3 SHA-256
a300ba2f6fb0e2ef12eced4fde9cebf0f2ff0039720c76361c17f800379d2c14.
Every other21 frozen197 file remains byte-identical to the R3 freeze, including
all production, manifests, stubs, package Python and previous cell test fix.

Read the original R11 scoped receipt and authenticated its three command logs.
Its actual preparation SHA is
153b99a29fed4bb8c9d80c4bc34ea8eb453195c446b4c97725762aee66000b36,
not the earlier unexecuted informational preparation. Installed provenance
revalidation succeeded. Word scoped155 passed, then strict mypy exited1 with
one error at old typing_smoke.py:390, expression None versus RGBColor|None.
No Word stubtest, PPT scoped/typing/stub or full suite result followed.
The failed log SHA is
d6e946c25edf976de8b5178e85af53d47a3760e0a74c102ad10e838fdf4f1c90.
Root separately reports the corrected strict mypy checked seven files with
exit0. That is its bounded typing proof, not execution by this review or a
complete renewed scoped gate.

## Assessment

At crates/rdocx-py/tests/typing_smoke.py:383 through :385 the unchanged
assertions check ColorFormat, RGBColor|None and str|None before assignments.
This tests the declared getter types rather than mypy's later flow-narrowed
None value after font.color.rgb=None. The stub declarations remain
crates/rdocx-py/python/rdocx/_rdocx.pyi:977 and :983.

All subsequent accepted setter inputs remain at typing_smoke.py:386 through
:390: hashed string, legacy triple, RGBColor, None and supported theme enum.
No assertion was widened, ignored or replaced, and no setter or public
signature changed. Moving declarations before assignments is the correct
separation between declared getter shape and assignment acceptance.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: the movement avoids incidental flow narrowing while retaining declared return-type checks.
- Contract: getter declarations and every accepted assignment remain checked.
- Panics: no production path changed.
- OOXML: no parser, serializer or package input changed.
- Tests: no assertion removal, ignore or weakened type introduced.
- Structure: two existing lines moved in one existing carrier, with no new helper or module.

## Remaining qualification

Renew the guarded scoped/full preparation with both reviewed test-only deltas
and authenticate original R10 compiled/install equivalence explicitly. Preserve
R10 and R11 failed receipts and successful intermediate command counts. No
wheel rebuild is required solely for this typing-test movement. Remaining
scoped, stub, PPT, full and feature gates still need actual execution. Only
this report was written, and the pass ends here.
