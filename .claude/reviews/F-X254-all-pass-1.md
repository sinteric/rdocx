# F-X254, all, pass 1

**Reviewed**: working diff against810af7306e409ad167d512dc370ca4f61df76e4d on work/f-x254-codex,10changed files,1407insertions and18deletions.
**Verdict**:1defect,0smells,0nitpicks.

## Defects

### D1, ordinal-only exception routes can transfer opaque source to another shape identity

`crates/rpptx/src/lib.rs:1733` constructs provenance routes from ancestor expanded names and child ordinals. `crates/rpptx/src/lib.rs:1924` then admits an old exception using only that route, exact island bytes and resolved namespace/MC meaning. Neither fact binds the enclosing physical shape's nonvisual identity.

An imported slide can have a valid first shape with id1 and one original non-Ignorable unknown direct child. Replace the complete slide XML with the same source except that shape's cNvPr id becomes a fresh unused id3, retaining the unknown child verbatim at the same ordinal. Both raw and staged scans find exactly the old island route, raw bytes and meaning, so the exception passes. Whole-part assignment at `crates/rpptx/src/lib.rs:6185` accepts the replacement. The subsequent model validation at `crates/rpptx/src/lib.rs:6220` checks only nonzero/unique ids, so it does not reject this reassignment. The direct shape setter's identity check at `crates/rpptx/src/lib.rs:6078` does not protect whole slide/layout/master replacement.

This gives another physical shape owner the original owner's opaque exception, contrary to the approved same-physical-owner provenance rule. The same omission can permit copying/moving an island while swapping enclosing shape identities so the island remains at its previous positional route. Bind exception ancestry to the actual supported physical owner identity as well as structural occurrence, without freezing legitimate typed scalar edits or expanding into a global opaque inventory. Add a whole-part identity substitution and owner-swap negative control, paired with a same-owner typed edit positive. Refusals must retain bytes and held owners. No runtime reproducer was executed in this review. The finding follows the complete raw/staged/assignment/model-check source path.

## Smells

0smells.

## Nitpicks

0nitpicks.

## Not found

No additional findings in selected value boundaries, frame flip no-op/atomic refusal, actual style graph lookup, pairwise CLI publication checks, explicit diagnostic mapping, four finite grammar particles, iterative storage, namespace URI decoding, inherited Ignorable scope and unsupported new complex MC refusal. Existing invalid_xml_child diagnostics remain report-only for ordinary save and unrelated SmartArt edits, while old diagnostic policies remain blocking. Source updates are limited to the approved existing files and HLD impact, without new modules, dependencies or public struct fields. The additive diagnostic enum arm is explicitly approved and documented.

Reviewed current HLD intent alongside the contract, including direct source-before-projection checks and the distinction between local evidence and final native/oracle acceptance. Final original reporter, complete installed, typing/stub, native Office, independent schema/oracle, hash/pixel and packaging obligations remain pending as declared. Pending final gates alone are not a local defect.

## Evidence and custody

Frozen manifest SHA256 d2fc1a666ce3c2675ef8c5d53137b3f58eb7e70447a978508e0625108f0f3c7c. All12manifest entries, including approved plan and local progress, match before and after the audit. Actual working binary patch SHA256 dec36d7c6a1ea0430c5c1a7fb7deb574c612c12f53fedfe1b383903205edd617 matches readiness. Readiness SHA256759c3099bae6652e00f0c9e7a5efe9e0e4d78db2f01f6bd548f3117ce5cd90a3 and all eight referenced local command receipt hashes authenticate. Those receipts record successful focused checks with unchanged source. They are author evidence, not reviewer-executed checks.

No product/tests/docs/progress edits, test/build/native execution or remediation occurred. Only this review file was written. Return control and the worktree writer lease to the orchestrator for a distinct remediation phase.
