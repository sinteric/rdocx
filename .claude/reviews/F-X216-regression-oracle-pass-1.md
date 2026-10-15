# F-X216, regression oracle, pass 1

**Reviewed**: Three requested integration failures in the safely ended changed-crate gate, against exact claim Base bodies, approved F-X216 design and frozen native composition seal 5. This is a read-only test-purpose audit. Duplicate-slide/source-snapshot production regressions and other failure families are outside this pass.

**Verdict**: 2 test-authority defects, 0 smells, 0 nitpicks. No new production defect is established by these three failures.

## Defects

### D1, legacy import refusal expectations contradict the approved transfer contract

`/private/tmp/rdocx-f-x216-codex/crates/rpptx/tests/integration.rs:17017`

`import_slide_refuses_what_it_does_not_carry_without_mutation` still requires charts and slide/notes cross-slide jumps to fail. The unchanged Base test was written for the earlier bounded importer. The approved current plan at `.claude/plans/F-X216-design.md:33` requires actual chart graphs to be carried, and at line 37 requires slide jumps to map to selected/existing destinations or release their owned action with a diagnostic. Current `preflight_import_scope` at `crates/rpptx/src/lib.rs:17017` accordingly admits those relationship roles. The actual failure is the test's panic after import returns success, rather than a reported malformed-graph acceptance failure.

Revise only the obsolete chart/jump refusal purposes. Keep renamed-layout name-match failure, invalid explicit layout, invalid insertion position, destination rollback and explicit-layout/SmartArt positives. For carried-chart success use an actual valid authored chart graph or existing accepted provider fixture, then assert copied chart/workbook/relationship ownership, unchanged source and destination unrelated parts, package validation and reopen. The old `<c:chartSpace/>` fixture has no namespace binding or XML chart owner reference and cannot itself become the valid chart acceptance oracle.

For slide and notes jumps, qualify the real action bytes and relationship IDs before invocation. Use the diagnostic companion for missing targets, assert precise dropped-action/unused-relationship behavior, retained unrelated click payload, unchanged source and correct notes back-link, then save/reopen. Add or retain real unsupported internal relationship and malformed reachable graph negatives with exact owner/type error and destination byte rollback. Do not replace every former refusal with unqualified `is_ok`, or silently drop atomicity coverage.

### D2, two checked-owner fixtures violate the existing save invariant before the operation

`/private/tmp/rdocx-f-x216-codex/crates/rpptx/tests/integration.rs:31688`

`/private/tmp/rdocx-f-x216-codex/crates/rpptx/tests/integration.rs:32440`

`fx211b_native_relationship_actual_target_and_mode_refusal` and `fx211b_native_review_image_targets_refuse_truncation_and_mime_mismatch` insert relationships into the OPC package but give them no reference in the original source XML. They then call `deck.to_bytes()` to obtain the before sentinel, before invoking the checked run replacement. `validate_relationship_scope` at `crates/rpptx/src/lib.rs:18765` classifies required-but-unreferenced XML relationships as dangling. Its actual roles at line 18789 include IMAGE and HYPERLINK. `to_bytes` at `crates/rpptx/src/lib.rs:2339` debug-asserts the existing save invariant. These tests therefore abort at fixture capture instead of exercising their ownership or image-decoder matrices.

The three reviewed test bodies are byte-identical to their claim Base bodies. The Base `to_bytes` already contains the same save assertion at retained Base `crates/rpptx/src/lib.rs:2311`. This source comparison establishes a fixture/invariant conflict, not an executed Base pass or a new F216 save assertion regression.

Preserve all relationship kind, actual target, external-mode, complete PNG/JPEG, truncation, corrupt CRC, trailing payload, unsupported SVG, short signature and MIME mismatch cases. Make the source setup independently valid at the ordinary save boundary. One narrow existing-carrier setup route is to retain an explicit namespace-qualified relationship reservation in an opaque extension outside the selected run. Its real `r:id` belongs to the actual slide relationship scope and remains untouched while the incoming owned `a:hlinkClick` or `a:blip` attempts the checked role. Use one actual schema-valid extension-list owner, adding an extension to any existing list rather than duplicating it. The foreign reservation cannot itself be used as proof that the incoming owned role is valid.

Assert source validation is empty before serializing the before sentinel, and preserve the opaque reservation/source bytes. The subsequent incoming checked edit must still prove the same allowed/forbidden relationship matrix and actual media decoding. Every refusal must retain the complete serialized before bytes, and positives must validate/save/reopen with the actual expected relationship and bytes. This is an unexecuted concrete fixture correction proposal, not qualification of such a fixture. If source validity cannot be represented for a particular negative, retain that invalid-source case separately with explicit validation/selected-owner observations rather than calling a save API that deliberately asserts. Do not disable the assertion, weaken invalid-target expectations, ignore these controls or rely only on release builds where the debug assertion disappears.

## Evidence

The actual changed-crate integration log matches SHA `63d41ccf0833921ec07b0ef11dcbf7b5ce65695d63afeca3aa551cbb2950cc7a`. It reports 369 passed, 14 failed and 24 ignored. All three requested names are actual failures. The legacy test fails at integration line 17029 expecting the removed chart refusal. The two checked-owner tests panic at the byte-save debug assertion with dangling `rIdXml` and `rIdCheckedImage`, before their target mutation calls. This report does not relabel that full row successful.

Exact Base/current reviewed function hashes are `8bb9124030c9383e05c1a2a56bfcab2037c710a5fb6b5f3f587057dac7f9857d` for the legacy import test, `897599f0929f698631983322fd3f3691df4bd90ee38de069e3ef35efc244617b` for the relationship matrix and `a2a7a653b5bc9e69ca0c3613347e49343672f043a55af3ce630832c507469cb9` for the image matrix. Their full carrier hashes match the current/Base copies in native composition seal 5, manifest SHA `94b423dcc30f2b30a16b5d61c2d5aa39a8811d57a58f5aceca6b882f44140ad6`. Frozen native lib SHA is `da8d576934764446611d66502cc045590db56cf3ee885caba58251f785cfd463` and integration SHA is `4cf8db7a7586989b674f0552828d0e4e8f9746d0759deb1e72267d9533c6e960`.

## Smells

0 found in this bounded oracle audit.

## Nitpicks

0 found.

## Limits

No tests, builds, runtime, source edits or Git mutations were performed. Other missing-oracle and sandbox failures are not qualified or adjudicated here. Actual duplicate/source-model snapshot failures remain separate production investigations, not oracle edits authorized by this report. Current successful successor execution is still required after any test repair. This review grants no full native, installed, Office, corpus or whole-feature acceptance. The pass ends here and returns the writer lease.
