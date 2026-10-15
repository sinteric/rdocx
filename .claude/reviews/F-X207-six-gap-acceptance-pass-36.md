# F-X207, six-gap acceptance proposal, pass 36

**Reviewer**: Independent bindings worker, ROOT adopts its completed read-only report after writer lease return.
**Scope**: External two-carrier test-only.diff df495d64a76ee603809adbef84e936a01a361047e556714c285958d23755ecfe. All 218 added lines are append-only. No worker source was changed or runtime executed.
**Verdict**: 1 defect, 0 smells, 0 nits.

## Defects

### D1, structural oracle does not compare the persisted Rust result

Candidate crates/rpptx/tests/integration.rs:28901 and :28907 in /private/tmp/S91-F-X207-six-gap-acceptance-r1-prepared/candidate-integration.rs.

The oracle asserts its wrap, run sizes, families and text at lines28892 to28898. The Rust side only asserts returned point_size12 at28883, then noAutofit and byte-stable reopen at28907 to28908. Persisted Rust wrap, run/default/end sizes, families and text are never compared against those oracle facts. Returning12 and writing noAutofit while retaining original20-point runs or wrap-none would pass. Compare the selected Rust-owned body facts with the same oracle facts, including applicable end properties, before claiming uniform-fit structural agreement.

## Smells

Zero findings.

## Nits

Zero findings.

## Other categories

Correctness, panics, OOXML and structure have zero further findings in this finite external proposal. Tests has D1. Contract has no separate defect, because the proposal preserves every remaining acceptance barrier and does not claim its unrun selectors complete the feature.

## Authentication and limits

Receipt feae209e51cc1a494374628e04a5d60cb87d031c8339712523ac41adbb1d5177 and all14 manifest artifacts authenticate. Actual2943 source records match 7e94c6b9ff5acac667965ddb37677f9d2be825524a5e0ca3f45ae09dcda07160 in bytes, sizes and modes. HEAD8f57b6ac5e6c9e2d8ac40cce1224201744a964d1 and staged index552c7c50dc88b07aa98293b026b73fb4d3e0238d949e55ea60c9f661e1b9e2be match. Independently reconstructed diff equals the packet and all original lines remain in order. Generated packages preserve ZIP member order and every other part exactly.

Q-only width, genuinely distinct shared-master field consumers and wrapped/rich/vertical operations are meaningful proposed controls. Raw SFNT confirms the Q-only trial preconditions. No runtime is credited. The contour adapter reads original fonts and uses the exact quadratic-to-cubic conversion, but audited owner origins/HB operands and actual paint remain due. Provenance labels do not establish origin authority.

The9823EMU native Shape height discrepancy remains unwaived. Full native affine owners, variable700/HB/source-font ownership, top/center/bottom and rich/vertical/bullet independent paint remain deciding barriers. The ignored structural oracle must run exact with1passed, 0failed and0ignored after remediation. Retained29 native,15 installed and typing evidence remains limited to its unchanged authenticated sources. No compilation, import, paint, native UI or gate ran during this review.

## Boundary

Review pass ends here. D1 remediation is a separate phase. No source adoption is authorized by this report.
