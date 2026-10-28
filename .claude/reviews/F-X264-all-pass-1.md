# F-X264, all, pass 1

**Reviewed**: Working diff on `work/f-x264-grok`, Base and HEAD `8084d2598786b2a41f1fcce798f5330bca49a679`. 20 tracked files, 3379 insertions and 391 deletions. Frozen binary diff SHA-256 `dbe10fa438cb802e846f6e10bb3b12eecd11521a27d56f8ecc5d386b7f71ec29`. The untracked Python extension is an authenticated test artifact and is outside the source diff.
**Reviewer**: Codex root, independently of the Grok implementation. Grok transcribes this report mechanically.
**Contract**: Canonical approved plan revision `b098410cd9af026b049b9a8254b086a5718d5672`, with the acknowledged local implementation and evidence mapping.
**Verdict**: 0 defects, 2 smells, 0 nitpicks

## Defects

None found.

## Smells

### S1, The new entry-name promise exceeds the retained writer rule

`crates/oxml-opc/src/package.rs:298`
`crates/oxml-opc/src/package.rs:326`
`.claude/plans/F-X264-design.md:107`

The method and plan promise names without a leading slash, but the shared writer mapping removes at most one slash. A fresh package can accept `set_part("//word/a.xml", b"<a/>".to_vec())`: `set_part` retains the first authored key at `crates/oxml-opc/src/package.rs:364`, and serialization checks identity collisions rather than canonicalizing those keys at `crates/oxml-opc/src/package.rs:534`. The mapping consequently yields `/word/a.xml`.

This is a source-traced contract counterexample, not an observed runtime RED. Existing writer naming and authored spelling must not be silently changed to make the new statement true. Reconcile the additive contract with the existing save-equivalent rule, then cover ordinary canonical names and retained caller-authored aliases through the same writer. A bounded clarification that accurately describes the existing writer needs no invented new normalization policy. Keep any actual preservation control distinct from semantic RED.

### S2, A8 does not assert the update and layout observations

`crates/rdocx/tests/regression_test.rs:407882`
`crates/rdocx/tests/regression_test.rs:407895`
`crates/rdocx/tests/regression_test.rs:407898`

The self-closing story regression accepts any one evaluation row and then checks only that the two update calls return `Ok`. It ignores the evaluation outcome, update reports, saved field cache and actual layout observation. A row that is refused or left stored, followed by successful calls that do no applicable work, satisfies these assertions. That does not prove A8's evaluation, update and layout consumer agreement or establish the declared endnote PAGE output change.

Extend this existing case with the actual supported expected outcomes and source positions or layout values. Preserve the header and footer cache policy as a control. Include the updated and reopened observations needed by the contract. Do not call a successful method return a measured output delta. No new Word oracle is required by this local correction.

## Nitpicks

None.

## Not found

- Correctness: no additional runtime defect found in exact-grid conversion, Python bounds and one-conversion publication, tolerant comment projection, physically ordered marker validation or related REF admission.
- Panics: no additional unchecked path found in the new marker position translation. Checked subtraction and addition guard source offsets. The retained flow lookup is populated from the same paragraph inventory.
- OOXML: no new child-order or unmodelled-source loss found. Namespace closure only inserts declarations for the marker coordinate translation. Empty paragraph handling uses qualified Word identity and inserts declarations before the empty-tag close.
- Structure: no new trait, module, crate, feature flag, generic abstraction or forwarding-only public layer found. The owned paragraph projection keeps the private source type private. Concrete tuple lint exceptions stay local.
- Contract: the public strict comment snapshot signature remains, the tolerant sibling is additive, Comment remains frozen with seven-field metadata equality, and Python move still takes StoryRunRange. S1 is the naming-contract issue found.
- Tests: the preserved original crossing composition remains an atomic refusal control, while the corrected well-nested positive has its own actual claim-Base RED. Reversed, crossing, nested text-box and namespace-offset controls discriminate the physical stack. BodyKeep, BadRelated and duplicate SAME distinguish body retention from uniqueness filtering. S2 is the remaining observation gap.

## Evidence consumed

The review inspected source and assertion membership. It did not run tests or change product files.

- Native scoped receipt: `/private/tmp/s95.4-fx264-scoped-prep-ready.txt`. Formatting and changed four-crate all-target/all-feature clippy pass. Native batch passes 30 selected tests, including the real native cast control.
- Final Python log: `/tmp/fx264-py-q10-final.txt`, exit 0, 20 selected parameterized cases pass. The old 19-of-20 failure remains at `/tmp/fx264-py-focused-green.txt`, with its historical filename explicitly identified as a failure.
- Local binding receipt: `/private/tmp/s95.4-fx264-local-ready.txt`. Fresh extension SHA-256 `ff8374cd7d0921b5fb6fd15fae437b8dc30d862c2d11fdb0695554f99fffab30`. The Python crate was relinted after its one-conversion repair. Three CLI tests and mypy 2.3.0 on the one existing typed-use file pass.
- Actual corrected-positive Base failure: `/private/tmp/s95.4-fx264-valid-base-red-receipt.txt`, test exit 101 at the recorded claim Base. The original invalid fixture is preserved.
- The duplicate-name and repeated Python conversion failures were separately observed before their repairs. Their logs remain attributed to those intermediate sources. Native saturation controls and already-correct tab publication are preservation controls, not newly failing original-source cases.

## Acceptance boundary and handoff

A1 through A7 and A9 have the recorded local observations. A8 still needs S2's stronger observation. A10 remains an explicit non-fix. The hash harness, whole installed typing and stubtest, WASM, archive checks, native Word forms and the integrated full gate remain pending. This review grants no delivery, release or native-parity credit.

Keep body-table accepted text and the self-closing-story endnote PAGE behavior in separate labelled commits with their expected deltas reviewed. No code commit or integration has occurred.

Return to the orchestrator. Resolve S1 through the design contract and preservation evidence, strengthen S2 in the existing test, refresh affected local checks, then obtain pass 2. This pass ends with this report and performs no remediation.
