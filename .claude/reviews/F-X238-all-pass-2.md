# F-X238, all, pass 2

**Reviewed**: Frozen working tree in /private/tmp/rdocx-s94-x238 against claimed Base a644dd9443346223e0003066bbfd33dc7937e3c7,21 tracked files,940 insertions and112 deletions. Follow-up to all-pass-1. Audited the D1 production repair and parity regression, retained the unchanged D2 finding, and reused the prior six-aspect audit for the unchanged implementation. Root-owned prose and approved scope clarifications remain included. No source edits, builds, cache writes, commits or remediation.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. D1 resolved. D2 remains open.

## Defects

### D2, Runtime section-module alias still lacks the typed import path

`crates/rdocx-py/python/rdocx/enum/__init__.py:34`

The runtime-only same-object alias and sys.modules registration at line36 are unchanged. The exact public `from rdocx.enum.section import WD_SECTION, WD_ORIENT` strict-mypy import therefore retains the independently observed pass1 import-not-found failure. `crates/rdocx-py/tests/typing_smoke.py:562` still imports enum.text rather than qualifying that path. Green smoke/stubtest results do not resolve the missing typed submodule. The import surface remains in `.claude/plans/F-X238-design.md:66`, and line164 explicitly leaves section.py creation outside authorized implementation pending an affirmative user answer.

Keep D2 unresolved and withhold clean review/preparation until the typed-import contract is resolved and independently qualified without missing-import ignores or Any substitutions. No new file/module permission is inferred by this review. The existing blocked permission state is a genuine limit, not a reason to mark the remaining defect clean.

## Resolved findings

### D1, Established picture exception family restored

`crates/rdocx-py/src/document.rs:8627`

The width/height mismatch now uses the existing rdocx_to_pyerr mapper with native Error::Other, which maps to RdocxError at `crates/rdocx-py/src/lib.rs:130`. The actionable dimension pointer remains. The new parity assertion at `crates/rdocx-py/tests/test_python_docx_parity.py:1425` requires that actual exception family, unchanged bytes and the held paragraph. The predecessor artifact fails this assertion with NotImplementedError in `/private/tmp/F-X238-installed-1/D1-red.log`, while the successor-4 installed artifact passes the focused gate. The original core regression at `crates/rdocx-py/tests/test_core.py:3411` is unchanged.

Independently authenticated the successor-4 wheel SHA256 and all12 recorded wheel members, plus all8 installed package/native/stub members. Installed dist-info RECORD is pip-rewritten and was not incorrectly equated to archived RECORD. All6 logs named by focused-receipt-5.json match their recorded hashes and exit0. Word focused result is9 passed/806 deselected, PPT focused result is1 passed/860 deselected, with both-family strict smoke and stubtest successful. Those checks support D1 closure, not D2 closure or a final complete-suite pass.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- Correctness: 0 additional findings in the repaired mapper path or retained native dimension admission.
- Contract: 0 additional findings. The existing exception family and no-mutation refusal are restored. D2 remains the explicit typed-import gap.
- Panics: 0 findings. The repair adds no unchecked indexing, narrowing or untrusted invariant.
- OOXML: 0 findings. The repair returns before native publication and introduces no XML change.
- Tests: 0 additional findings. The D1 regression distinguishes old/new exception behavior and checks source/held authority. D2's exact import remains unqualified by the green smoke test as recorded above. Running complete-suite/scoped checks are not claimed passed here.
- Structure: 0 findings. The repair reuses the existing mapper and adds no feature, dependency, trait, generic, module or product file.

Review pass ended. Return the tracked write lease. F-X238 remains blocked on D2 and is not ready for preparation or integration.
