# F-X211 resource boundary pass 1

**Reviewed**: Frozen source13 and proposed weighted admission14, SHA `0282eb56e18e913be347dd35fae825e6203574ee8ad0c7d194abd416cba856cb`. This is a bounded resource-policy and owner-route review, not full feature acceptance.

**Verdict**: One defect, zero smells, zero nitpicks. The candidate arithmetic is supported by the retained measurements. Production admission remains blocked on D1 and the declared remaining acceptance.

## Defects

### D1, opaque inline shape content is admitted as recursive typed ownership

`crates/rdocx-oxml/src/document.rs:93`

The current preflight admits both inline and anchor drawing containers, then admits graphicData, wsp, txbx and qualified txbxContent without retaining the drawing-owner route or graphicData interpretation. Controls within that content therefore consume the256 budget. The actual inline parser at `crates/rdocx-oxml/src/drawing.rs:1435` has picture and chart projection but no typed shape text-body branch. The retained earlier inline measurement likewise proved opaque preservation rather than typed recursion. A qualified-looking deep control subtree inside that opaque inline shape can consequently be refused even though the native parser preserves it without constructing those controls.

Proposal14's actual-owner requirement at `.claude/scratch/F-X211-start/resource-boundary-proposal-14.json:39` must use exact modeled drawing ownership. Preserve opaque inline shapes and unsupported graphicData routes without counting their descendants. Keep the genuine supported anchor/VML ownership rules and inherited namespace scope. Add a distinguishing opaque inline-source control with more than256 qualified-looking descendants and exact preserved bytes, alongside real typed anchor refusals. Do not use local names alone or expand the native model to make the preflight match.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Supported proposal and remaining work

Inclusive path budget256, cost1 per qualified control and cost16 per actual typed text box has normal2MiB lifecycle evidence at16 boxes,240 controls plus1 box,128 controls plus8 boxes, and12 mixed boxes. Pure256 controls retains separate positive lifecycle evidence. Sibling closure must restore path cost, with checked arithmetic and no ambient admission authority. These measurements support implementing a documented combined bound while preserving the approved pure-control boundary. They do not establish every owner route or serializer entry.

The independent source-input admission must precede recursive construction. Borrowed-model validation must precede recursive serialization and facade staging clones. Related-story entrypoints, alternating controls/tables, actual default-stack CLI/Python, recoverable refusals, atomic source/relationship/generation preservation and unchanged frozen100 metrics remain mandatory. No new module, dependency, quota bypass or larger consumer stack is authorized by this review. An explicit design amendment must record the additional text-box policy before implementation.

## Evidence and integrity

Independently authenticated proposal14, source13 inventory and all10 terminal successful row logs against retained hashes and safe cleanup receipts. No runtime command was executed. Captured all3203 tracked file hashes/sizes/modes, HEAD, branch, logical index and tracked binary diff before and after this pass. Complete source aggregate is `102bd242e3cd96601950419825bafabd53b84b8d2fac683dd8ce90dcbd7a5b92`. Only this review file was written.

## Not found

No further findings in candidate arithmetic, pure256 compatibility, explicit namespace intent, sibling accounting, retained evidence honesty or structural scope. Full correctness, final mixed-owner safety and feature completion are outside this bounded proposal pass. This pass ends here and returns control to the orchestrator.
