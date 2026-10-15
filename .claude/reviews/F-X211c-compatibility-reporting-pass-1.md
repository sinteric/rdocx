# F-X211c, compatibility reporting, pass 1

**Reviewed**: Two existing script carriers in the complete working report diff, 596 added lines, plus the approved current dependency contract and sealed named-disposition amendment. Source checkpoint48866166, current source mapdda1f9aa. Native and binding implementation is outside this reporting pass.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Final reporting omits the mandatory corrective Word child

`scripts/sprint_workflow.py:1662` requires a, b and the older provider set but omits F-X211d. Line1664 rejects any additional dependency, so adding the actual d completion evidence also fails validation. The current approved c plan requires d at `.claude/plans/F-X211c-design.md:6` and explicitly requires completed d and exact composition at line106. The report therefore accepts the superseded dependency boundary and refuses the current one. Require the current child proof, retaining all earlier required providers, and include a distinguishing missing-d control. This does not authorize measurement before d's formal completion.

### D2, A pending full-qualification admission still credits correct-file counts

`scripts/sprint_workflow.py:1682` authenticates the full-qualification admission file, then lines1683 through1687 check only its three digests before adding cases to qualified at line1709. Line1758 uses that set for total, per-area and documentation correct_file counts. A hash-correct pending admission with qualified facts can therefore report those successful-file counts before authority is admitted, contrary to the boundary at line1670.

Correction to the initial review wording: line1752 does require admitted status before assigning the full metric, so a pending admission does not produce a passing full metric or threshold. Missing status raises KeyError there. The remaining defect is the earlier unadmitted correct-file credit. Require admission before counting qualified files, or refuse the pending packet early, with a concrete pending-admission control. Retain the existing full-metric status guard and original90-percent and independent hard-gate requirements. A missing-status negative documents the existing refusal rather than claiming an old passing result.

## Smells

None within this bounded report implementation.

## Nitpicks

None.

## Not found

Correctness and contract: no additional findings in the exact three named failure identities, source and attempts bindings, retained log matching, unchanged frozen script checks, no-success-credit classification,100 denominator or90/89 threshold distinction. The original P-AREA-14 AttributeError at line11 cannot match the later line17 selector disposition. All fourteen XML and named lifetime and native obligations still require separate evidence.

Panics and OOXML: no native mutation or schema serialization is introduced by the reporter. Invalid input is refused rather than executed. No additional findings within this scope.

Tests: actual13 report controls are green and the genuine predecessor named-control failure is retained. Their files and receipt hashes were independently authenticated. They do not currently test D1 or D2. These synthetic controls are report validator evidence, never native document or corpus acceptance.

Structure: no new product file, module, runtime dependency, trait or generic was introduced. The existing report helpers have multiple concrete report consumers. No additional findings.

This review writes only this report and ends. It does not change source, tests, installed bindings, the immutable corpus, native outputs, archives or tracker state. Remediation is a separate normal phase.
