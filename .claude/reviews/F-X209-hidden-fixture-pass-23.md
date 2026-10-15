# F-X209, hidden fixture correction, pass 23

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable hidden-fixture-source70-proposal-r2, one existing test-carrier before/candidate pair and 2,209-byte incremental diff. No production changes.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Pass22 D1 URI corruption is resolved statically, but the added rebound negative has an incorrect expected error.

## Defects

### D1, rebound-owner negative expects an unreachable error

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/hidden-fixture-source70-proposal-r2/candidate.rs:1707`
`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/hidden-fixture-source70-proposal-r2/candidate.rs:1493`
`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/hidden-fixture-source70-proposal-r2/candidate.rs:1502`

The new source replaces the sole qualified cNvPr with a foreign q:cNvPr while retaining its following qualified cNvPicPr and nvPr. The existing parser preserves the foreign child without advancing its schema boundary. It then encounters cNvPicPr at boundary0 and returns InvalidValue with the exact p:nvPicPr child-order diagnostic. It cannot reach the asserted MissingElement("cNvPr"). Additionally, the later required owner check at758 names p:cNvPr, not cNvPr. Keep this genuine foreign-owner refusal negative and match its existing schema-order error, or construct a separate valid missing-owner setup for the intended named error. Do not alter production error behavior merely to satisfy this assertion.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0 production findings. Contract: 0 production findings. Panics: 0 production findings. OOXML: D1 concerns expected existing schema refusal. Tests: D1. Structure: 0 findings.

## Pass22 disposition and authentication

The replacement now touches only opening and closing p: element QName tokens. Namespace URI values and attributes remain exact. The fixture asserts the root's expanded PresentationML ownership before hidden-flag checks. Numeric entity forms &#49; and &#116;rue are added with true expectations. Existing missing/false/true/plain1, foreign attribute, opaque &#65; byte retention, repeat serialization and foreign-before-qualified-owner controls remain intact.

Receipt SHA256 c90db2ddad73e692a61ed58f413f0d5b9629482b8cd46c34d9fc5c30b4a42f11. All three records authenticated by size and SHA256. The before carrier is byte-identical to the frozen source70 picture.rs, SHA256 a576b3c42d8d1f40b821d3eebf2dea2592ccc63a874320540bfe6362cd50ae78. The complete carrier prefix before the test and suffix after it remain byte-identical. Thus no production or other test body changed.

The proposal is unadopted and unrun. Source70's reader failure and resolver pass remain the actual retained outcomes from pass22. No successor runtime, installed saved-deck comparison, native output or feature completion credit is granted. No source or runtime changes were made. This review ends at this report.
