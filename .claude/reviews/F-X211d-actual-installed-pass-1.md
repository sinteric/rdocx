# F-X211d, actual installed failures, pass 1

**Scope**: Actual source4007d483 installed run with28 passing and4 failing controls. Read-only diagnosis of header authority and the two new merge-refusal exception assertions. Earlier static pass2 remains retained and supplies no installed credit.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. One actual header publication defect and one new test exception-contract defect account for the four failures.

## Defects

### D1. HeaderFooterAppend has no selected owner but requires one

`crates/rdocx-py/src/document.rs:2075` routes validated HeaderFooter.add_paragraph through the new unit HeaderFooterAppend publication. The selected-owner match at line4809 has no arm for that variant and falls through to None at line4833. The preflight at line4840 exempts only Conservative and AppendBody. Consequently selected_index is absent and line4845 raises checked XML owner has no current physical authority.

The actual method validates the HeaderFooter handle at `crates/rdocx-py/src/document.rs:2004`, stages creation/insertion, and reaches this failing preflight before prepared result capture or publication. The observed failure is therefore not evidence of a failing prepared paragraph or furniture-scope capture. Those later paths are unreached and still need actual qualification after repair.

Both actual failing controls stop at this same authority barrier: existing shared-physical-header control `crates/rdocx-py/tests/test_core.py:6124` and corrective header control at line6495. The required successful section/header survival is explicit at `.claude/plans/F-X211d-design.md:45` through line52. Repair must identify the existing validated Section owner for the publication or provide equally concrete aggregate authority. Do not broadly exempt unchecked mutations from owner preflight. Preserve section ancestry, shared physical-part provenance, prepared result allocation, rollback and the existing furniture retirement controls.

### D2. New merge-refusal tests assert the wrong public exception class

`crates/rdocx-py/tests/test_core.py:6553` expects ValueError for the native opaque-metadata refusal. The checked native guard at `crates/rdocx/src/table.rs:1120` returns Error::Other. The binding retains the existing translation at `crates/rdocx-py/src/table.rs:2163`, and `crates/rdocx-py/src/lib.rs:116` through line130 explicitly maps this error to public RdocxError.

The approved existing native topology/serialization mapping is stated at `docs/hld/10-bindings-spec.md:370`, and the Python-validation versus native-setter distinction at line402 requires RdocxError for a native rejection. The d plan at `.claude/plans/F-X211d-design.md:49` reuses the checked physical native operation. It does not amend the public error hierarchy to ValueError.

Both actual metadata cases refuse with the precise opaque-cell-metadata message in RdocxError. This is the expected native exception contract, not evidence that the metadata guard still discards bytes. The new regression should assert the exact RdocxError and retain its message, package identity, held-owner, no-op and reopen assertions at `crates/rdocx-py/tests/test_core.py:6551` through line6560. Replacing it with a broad Exception catch or removing downstream assertions would weaken the test. The original frozen corpus remains unchanged.

The actual run stops at this wrong-class assertion, so its later package/owner/reopen checks were not reached. Native predecessor/successor evidence remains separate. No installed atomic-success credit is inferred here.

## Smells

None.

## Nitpicks

None.

## Evidence and boundary

Read the exact terminal installed log `/private/tmp/S91-F-X211d-current-Word-producer-2/actual-installed-1/current-Word-residual-and-owner-controls.log`, SHA `f026bc53750ed75a218577f0f7842becd040d739dd544c391c9da3c043910a58`. It records28 passing and4 failing controls, without ignores. The run remains failed. This report does not replay cases, score a retry, alter frozen100 bodies or change their denominator.

No source, test, runtime, CP environment, output artifact, dependency, Git state or native UI was modified. No build, tests, installation or runtime diagnostic was executed. Only this untracked, unstaged worker review report was written under the exclusive lease. Review ends and returns control for separate remediation and actual successor verification.
