# F-X256, correctness, pass 2

**Reviewed**: Bounded new-material contract and panics review of the public section admission helper in the same frozen eight-file working diff. Base d75f407b98128f611961c91617ce4cc3da45e905. Complete diff remains eight files, 439 insertions and 62 deletions. No remediation occurred between passes.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Review boundary and source identity

The clean pass1 report was actually written before root supplied the worker's concrete conversion-callback concern. Its original SHA256 is d1996a483f9af0b136f8847b76ec2543bd0b8abcd473ae3ce122397c386fa167. The reviewer initially amended that file with this finding, producing SHA25640f81e0b75e98e418a19871d3128e0a283b90f113b581a21e7913cf0a2e79dde. That amendment crossed the report boundary. At root's explicit instruction, pass1 has been restored to its exact original bytes and verified against its original SHA256. This distinct pass2 preserves the substantive finding unchanged and records the new material without rewriting the completed first pass.

All eight frozen source/test/HLD files and the approved plan remain byte-identical to frozen-source.json, SHA25662239f288d17dcb99dbf4c33c0d82b4d6c1ea4565a787474b6151aa6cdeac35c. The source-only assessment traces the extraction to the existing shared helper and PyO3 integer conversion implementation. No same-document mutation or runtime execution is claimed.

## Defects

### D1, public section type admission invokes unsupported conversion before rejecting it

`crates/rdocx-py/src/document.rs:6450`
`crates/rdocx-py/src/document.rs:6452`

The new public helper calls checked_section_length before checking that the supplied object is an actual PyInt. The unchanged shared helper extracts i64 at document.rs:6438. PyO3's integer extraction uses Python index conversion, so an unsupported object with __index__ is invoked before the new type refusal. For example, a non-int whose __index__ raises RuntimeError escapes with that arbitrary error instead of the public TypeError naming Length/zero and Twips/Emu. An unsupported object's callback also runs even when it simply returns an integer and is subsequently refused. The approved finite public contract rejects unsupported types rather than admitting their conversion protocol. Classify the public input before delegating extraction, leaving the shared helper's existing consumers unchanged. Add a counter/raising __index__ negative control that proves the hook is never called, the supported-route TypeError occurs, and source and held owners remain unchanged. This is source-confirmed diagnostic/type-admission behavior, not a claim that the same document can be mutated reentrantly.

## Smells

None.

## Nitpicks

None.

## Not found

No new unchecked arithmetic, indexing or Rust panic was found in this bounded path. The defect concerns unsupported Python conversion and diagnostic/error propagation. Other first-pass aspects are not re-audited by this new-material pass. The shared helper itself is an existing compatibility route and should retain its contract for other consumers.

## Verification boundary

No tests, builds, product imports, cache mutations, source/HLD/stub edits or remediation were performed by the reviewer. Scoped checks and fresh installed qualification remain worker-owned and were not claimed complete. Only the two review records were written during this explicit record correction. Return the worktree write lease to root after this report.
