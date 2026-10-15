# F-X211b, modern comment text admission, pass 1

**Reviewed**: Current shared source15 text-body root predicate and real modern-comment consumers. This bounded review is read only and adds no implementation.

**Verdict**: One defect, zero smells, zero nitpicks. Native source must be repaired before completion.

## Defects

### D1, valid modern comment and reply text bodies are rejected

`crates/oxml-drawing/src/text/mod.rs:126` rejects the official PowerPoint2018 namespace even when the selected root is txBody. Existing comment and reply providers first prove that exact expanded root at `crates/rpptx-oxml/src/comments.rs:691` and `crates/rpptx-oxml/src/comments.rs:848`, then pass their actual inherited bindings into this parser. The official namespace is declared at `crates/rpptx-oxml/src/comments.rs:14`. Rejecting it breaks real current callers rather than refusing a foreign input.

Actual root native successor2 rpptx-oxml integration executes151 passing and7 failing controls. All seven failures have the same foreign text body namespace error. Their fixture/assertion citations are integration.rs:5005,5049,5271,5609,5672,5863 and5928 in crates/rpptx-oxml/tests. The original receipt remains failed with exit101, no timeout, naturally gone group and reaped leader. Log SHA is7815d2683f9d6a602a6b4ed0ca80a37ff54bcf4604a01ddb5705ae81c1626e66. Earlier147 passing rpptx library controls remain distinct evidence and do not refute these integration failures.

The repair must admit only the existing official PowerPoint2018 plus txBody tuple, retain the exact DrawingML descendant checks and existing finite A/P, Chart txPr and DiagramML t behavior, and preserve comment/reply lexical sidecars and extension ordering. Actual modern-comment consumer parse, save and reparse, alias/default/inherited positives and wrong namespace/local-name negatives are required. Do not add unsupported rich roots or arbitrary namespace acceptance.

The public selected XML boundary remains separately strict. `crates/rpptx/src/lib.rs:1167` specifies the actual A/P namespace of its selected TextBody owner, and `crates/rpptx/src/lib.rs:5195` checks the exact namespace and local name before candidate assignment. A normal shape or cell must still reject a PowerPoint2018 comment root, with unchanged source and revision state.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Bounded aspects and evidence

Correctness and compatibility have the cited D1. Preservation, child order and namespace scope are required successor obligations rather than claimed fixed behavior. No new dependency, module, generic or structural mechanism is needed. No panics are introduced by the proposed finite predicate change. The actual failing assertions are existing consumer regressions, not missing future APIs or altered fixtures. The current shared carrier SHA isb69b4d6a989af4b853a29deffe547d9fe811f96432876ecc0bbdb9bdef4ac700. Whole-feature, installed, archive, policy, Office and integrated sprint acceptance are excluded.

Only this canonical review artifact is written. All four inspected source carriers remain byte exact before and after. No source fix, test, target, managed Python, UI, ledger, baseline, branch or commit action is performed in this review pass. Review ends and returns control for separate remediation.
