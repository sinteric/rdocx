# F-X211c, report and CI, pass 1

**Reviewed**: frozen report-schema checkpoint ff861999c861f22bc957a4b900a811a0606e85a649d89c4625de377be8320c5b, three carriers against2087212c. Patch58730157f1d500f6eb2fccfe9609be1786e87bbdeeca6669b8ed39463d730f6a.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Boolean observation payloads can earn correct-file and native acceptance credit

`scripts/sprint_workflow.py:1622` and `scripts/sprint_workflow.py:1648`

Qualified case and hard-gate checks require a dictionary containing any truthy observation value. A list containing {"observation": true} satisfies both checks. With retained hash-bound evidence and admitted packet, these Boolean values therefore count toward correct_file and the full acceptance percentage despite providing no observed source/model, saved/reopened or native fact. The approved contract and this scoped review expressly exclude Boolean credit. Require concrete observation payloads rather than truthiness and add negatives for Boolean observation values in both case proof fields and hard-gate observations. The existing Boolean test only sets the outer source_model field to True at scripts/test_sprint_workflow.py:8340, so it misses this admitted nested form.

### D2, Gate membership does not require qualification of each declared native obligation

`scripts/sprint_workflow.py:1633` through `scripts/sprint_workflow.py:1653`

The gate inventory reduces every nonempty additional_native_obligations entry to its case ID. Qualification then accepts any nonempty observations list for that case. There is no binding between the frozen obligation texts and the individual qualified observations, and the admission contains only facts/source/attempt hashes plus status. For example, W-LIFE-07 declares both overflow preflight and weak-state reclamation, while P-AREA-10 declares removal/pruning/shared-reference riders plus an independently authenticated playable sample. A packet can mark each case-level gate qualified with one unrelated generic observation and omit those other defining obligations, yet gates_ready is true. This does not enforce the required ALL immutable manifest obligations and movie acceptance. Enumerate the actual required obligation identities from the existing frozen manifest and require each one's concrete qualified facts/evidence, retaining independent admission. No new corpus or widened obligation is needed. Add deciding omitted-second-obligation and omitted-playable-sample negatives in the existing test class.

## Not found

No additional defect found in exact100 distinct attempt allocation, immutable manifest/body digest binding,49 Word/51 PPT and area counts, original17/7 documentation subset, first-attempt refusal, deliberate W-XML-02/P-AREA-17 exclusion from success while retaining denominator100, changed evidence/provider refusal, or render versus check refusal for absent qualification. No panic, OOXML mutation, new runtime dependency, generic framework or extra delivery tracker is introduced by this scoped diff. Smells:0. Nitpicks:0.

## Actual evidence and boundary

The retained scoped receipt d954e7f7975ba129898cd58e8e141ded0d7d6bdd63bfb515704bba136db6b0c3 records four actual qualified safe rows, including seven defining unit controls, prose, adapters and diff checks. Root independently authenticated their raw logs and source identities. These are schema controls only, not native acceptance or immutable100 execution. The CI addition runs that unit test class at .github/workflows/ci.yml:283 and cannot itself establish correct-file metrics. Final corpus execution and its admitted native/render facts remain pending.

The audit ran no tests, corpus, build, interpreter install, application or Git mutation. Only this canonical review file was written. All three frozen current carrier hashes matched the checkpoint before and after review. Review ended and returned control for a distinct remediation phase.
