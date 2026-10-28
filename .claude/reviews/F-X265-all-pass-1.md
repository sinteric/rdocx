# F-X265, all, pass 1

**Reviewed**: Frozen working diff on `work/f-x265-grok`, Base and HEAD `d5b41edf4191a1f1b37c9f699b2f3dd2ab06b319`. 10 tracked files, 854 insertions and 8 deletions. Binary diff SHA-256 `565a692e418a095d6d998b5b436ea8e1a9a9fd79009bb939d933bf0d3c41042a`.
**Reviewer**: Codex root, independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None found in the runtime repair.

## Smells

### S1, The native control pins a retargeted picture as its expected result

`crates/rdocx/tests/integration_test.rs:29154`
`crates/rdocx/tests/integration_test.rs:29156`
`.claude/plans/F-X265-design.md:96`
`.claude/plans/F-X265-design.md:106`

The native control inserts a red source drawing beside a blue destination drawing, then expects two blue pictures. Its comment explicitly acknowledges that the inserted drawing uses the destination relationship. That does not demonstrate the test's stated picture-byte identity, and it pins the known retargeting as a required result.

The plan also needs a factual reconciliation. A6 says the publishers share the relationship and style rules, while the approach and C5 explicitly retain native typed insertion as a different, unaffected body route. The source at `crates/rdocx/src/document.rs:35220` clones body content and merges styles and numbering. It does not use the fragment media relationship importer. The old aggregate test failed in the template section before reaching this native assertion, so there is no observed isolated Base result for the blue/blue expectation.

Retain a valid unchanged native control whose source drawing already has the intended destination media relationship graph, with exact correct picture bytes after reopen. Clarify A6's shared-provider claim so the fragment publishers receive the new rule and the native route remains an unaffected control. Do not add a native media-import repair to this story or silently promise that behavior. Do not turn an unsupported distinct-media case into a passing correctness claim. This is a contract and fixture correction, not a new runtime defect attributed to the pending-id repair.

## Nitpicks

None.

## Not found

- Correctness: no additional runtime defect found in pending-id registration, registry intersection, source-first style deduplication or clearing the default flag on renamed copies.
- Contract: no public API, file, module or test binary was added. Q17 remains explicit. The reconstructed available picture case is not claimed equivalent to the unavailable original.
- Panics: no new unchecked untrusted indexing or arithmetic found in the runtime diff. Existing infallible image and hyperlink publication contracts remain.
- OOXML: no new child-order or unmodelled-target-source loss found. Target duplicate style elements are compared as complete elements. Source deduplication precedes dependency closure.
- Tests: the seven old-source failures are actual assertion failures at the claim Base with only test changes. First-definition, shadowed-dependency, same-id default and different-default-id controls discriminate the style rules. Picture tests compare resolved target bytes through save and reopen. S1 is the remaining native control problem.
- Structure: the added state uses existing concrete maps and methods. No new abstraction or public pending-id registry is introduced.

## Evidence consumed

Root inspected source, assertions, caller paths and retained outcomes. It did not run tests or modify product files.

- `/private/tmp/s95.4-fx265-local-ready.txt` records seven actual Base failures in `/tmp/fx265-red-semantic-3.log`. Earlier compiler and harness failures remain separate.
- `/tmp/fx265-green.log` records the first six native gates passing and the first post-fix native control failing. `/tmp/fx265-green-template.log` passes after the disputed expectation was changed. That last pass does not close S1.
- Native artifact SHA-256 `4bee75b53108ab002efadb6156fc3a7e08cf51c62dab81fcdc1922baf2e8cbce`.
- Python evidence is a direct invocation of the single new test function using a temporary pytest import shim, not a pytest suite or installed-package qualification. Root checked that the selected function has no fixture or pytest dependency and explicitly loops over both policies and both picture orders. Its real assertions execute against fresh extension SHA-256 `61bea238f1aa339a6360726c8cb55ffb9252fc7ed1c9f77f186fee20a0d9a86d`. The source path, build command, failed links and successful build are retained. No broader test collection, wheel or typing credit is granted.
- Formatting and rdocx all-target/all-feature clippy exit 0.

## Acceptance boundary

A1 through A5 have applicable local evidence. A6 needs S1's fixture and contract reconciliation before preparation. The original Issue 404 input remains unavailable and unverified under Q17, without becoming a closure blocker. No reporter comment was posted. Final full verification, hash, installed typing and stubtest, WASM, archive, native Word and oracle riders remain pending.

This review ends here and returns control to the orchestrator. It performs no remediation.
