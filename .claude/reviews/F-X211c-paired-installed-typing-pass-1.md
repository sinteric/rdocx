# F-X211c, paired installed typing, pass 1

**Reviewed**: Existing 20-line typing-smoke addition and the actual current paired installed result. Source8cb3, fresh Word0daf/PPT1e6d. No source or runtime changes.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, union assertion follows a narrowing assignment

`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/tests/typing_smoke.py:786` assigns the string bullet character. The assertion at line787 then demands str or bool or None. The actual strict mypy log reports str at that expression, consistent with the assignment. The public getter and setter retain the declared union at `crates/rpptx-py/python/rpptx/_rpptx.pyi:1060`. This is a typing-control placement defect, not a reason to widen the narrowed expression or change the runtime API.

Inspect the unassigned getter's union before the string assignment, retaining the existing string, False and None setter controls. Requalify the corrected strict typing control and the remaining native/root stub checks with truthful source and artifact provenance. Preserve the current failed gate and do not call the paired installed stage successful.

## Smells

None.

## Nitpicks

None.

## Actual evidence and boundary

Independently rehashed actual receipt b34954274ff69c86463a7dcbe9f960c692f3f1a347855d4eb9700a2dacc19c75 and strict mypy log b099ba20cbd27459fab1b553662e56afaa65c028e98424f798825515ada140fa under `/private/tmp/S91-F-X211c-final-unknown-paired-producer-1/actual-installed-1`. All50 runtime cases passed. Word strict mypy, native stubtest and root stubtest passed. The PPT strict mypy row failed, so its later stubtest rows were not reached. Whole100 and full acceptance remain pending.

Correctness and contract inspection found no separate runtime or public-stub defect in this bounded failure. No code, tests, source capsule, CP, UI, Git or frozen corpus was changed. Only this review was written. Review ends and returns control for distinct remediation.
