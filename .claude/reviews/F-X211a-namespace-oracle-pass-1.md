# F-X211a, Word namespace oracle authority, pass 1

**Reviewed**: Four genuine failing assertions in the frozen source113 Word114 regression result. Actual source113 text and regression bodies and both native binaries were preserved before source115 edits. This is an oracle-authority audit, not source115 implementation review or whole-child acceptance.

**Verdict**: Two oracle defect classes covering four failing assertions, zero production defects established by these four failures, zero additional smells and zero nitpicks. Bounded test amendments are justified only with the distinguishing successor proofs below. The original regression run remains failed.

## Defects

### D1, three oracles require loss of an actual Word namespace binding

`crates/rdocx/tests/regression_test.rs:60821`
`crates/rdocx/tests/regression_test.rs:61227`
`crates/rdocx/tests/regression_test.rs:28941`

These fixtures explicitly bind ancestor q or x to the WordprocessingML namespace. Their txbxContent is therefore an existing qualified Word text-box owner, not a foreign lookalike promoted by a later guessed prefix. The original tests require binding loss by typed serialization: comment-anchor refusal, scoped replacement refusal, or revision attribution to Body. Correct source-scope preservation now retains that same TextBox owner before and after staging.

The source scanner at `crates/rdocx/src/document.rs:10698` creates a candidate only for an actual Word namespace and nonopaque event. `owner_kind` at line10324 maps actual txbxContent to TextBox. `stories` at line18597 uses those qualified owners. This is complete source-qualified authority, not prefix-spelling or local-name fallback.

The comment fixture at regression_test.rs60807 binds q to Word on the enclosing paragraph. Actual source113 returns the body anchor text AB, excluding the separate nested story's OTHER text. The namespace-restoration safeguard at `crates/rdocx/src/document.rs:19900` still compares actual physical owner boundaries and refuses any real changed owner. Since the preserved canonical source already has the same qualified TextBox, no ownership reinterpretation is introduced by the successful read. The foreign q override in the other fixture branch remains an independent negative and must remain covered.

The scoped fixture at regression_test.rs61221 likewise contains a real nested Word TextBox. Its selected target is the outer body paragraph. The implementation at `crates/rdocx/src/document.rs:32158` still compares canonical and actual owner kinds and indices before publication. Successful namespace-faithful replacement is valid when those actual owners agree. Requiring failure solely because an earlier serializer lost the declaration no longer tests the atomic refusal contract.

The revision fixture at regression_test.rs28922 through28950 explicitly describes the earlier binding loss on body, paragraph or run. Actual source113 reports revision31 as TextBox rather than Body. `Document::story_revisions` at document.rs23913 scans the actual staged parts and pairs them with reported story owners at line23944. The source-faithful TextBox result is consistent with the original purpose: listing and accept/reject resolution must scan the same real revision and report the existing story that owns it. The mixed fixture at regression_test.rs28956 also requires correction of both qualified text-box indices, not merely changing the first row's kind while leaving both indices at0.

Required meaningful successor proofs:

- Independently establish the original fixture's resolved Word namespace and physical text-box payload before consulting product-generated expectations. Assert exact body/text-box inventory, source part and ordered owner indices before mutation and after save/reopen. Explicit foreign, locally rebound and unbound lookalikes must not become Word owners.
- For comment anchors, assert Body AB and distinct TextBox OTHER, comment range/reference identity and unchanged package/stories on reads. Save/reopen must preserve both texts and namespace authority. Retain the current foreign-object case and meaningful unsafe-owner or namespace-replay atomic refusals.
- For scoped replacement, assert exactly one outer body replacement, unchanged nested TextBox Version, unchanged second paragraph Version and unselected package parts. Save/reopen must retain the same actual owners and values. Retain the foreign-w object case and genuine unsupported/rebound namespace or physical-owner mismatch refusal with complete byte rollback. Simply replacing is_err with success supplies insufficient evidence.
- For revisions, retain the original list/accept/reject count equality, reported-story membership and default-namespace case. Cover all three ancestor declaration positions, an unrelated body edit followed by save/reopen, and two qualified text boxes with distinct ordered indices. Verify revision IDs, text and actual accept/reject effects, with foreign/unbound cases that cannot acquire TextBox authority.

### D2, raw-child count includes private namespace-fact metadata

`crates/rdocx/tests/regression_test.rs:15825`

The imported ordinary run has no raw payload or metadata, and that original negative still passes. The pict run has one actual opaque VML child plus the required namespace-fact record preserving inherited v and o authority. The test counts every raw position as a payload child and therefore expects1 where the actual vector has2. This does not demonstrate an added opaque child or incorrect horizontal-rule classification.

The source113 parser appends retained namespace facts separately at `crates/rdocx-oxml/src/text.rs:3429`, marked with the existing root-attribute metadata classification. The real raw payload is still stored before that record. The metadata flag and position mask preserve the payload's legacy-horizontal-rule bit and position rather than creating a second pict.

A successor must retain the ordinary run's empty vectors, distinguish only proven root metadata from actual raw payload using the existing classification, and assert exactly one opaque pict with its exact original bytes, position and positive legacy flag. Assert required actual v/o bindings and one emitted pict after save/reopen. Keep a foreign VML lookalike negative with the same spelling and a false legacy flag. Do not delete the slot assertion, accept arbitrary additional raw payload, or treat every raw slot as ignorable metadata.

## Smells

Zero additional findings.

## Nitpicks

Zero findings.

## Actual retained evidence

Preserved record `657c417889e74930bc6a6a338e3aef4207903f70bb5a76085b52a8d929ab1e4c` authenticates actual source113 text SHA `3488afa8b27ba28c62a6a9f49ce39df5117ec2f29840b388952782da8ba2759d`, regression SHA `b3dd6a6df32e6e461a46168b952f90f279a17d28b3622342a53c42ca01459bd2`, native binary `4154451c6ee5534dfc391a715c7e6e87988f5e69fb0cad0f5ea4ea212a71ae32` and regression binary `6d67b75401152c97cee6e61eb1794fe71a3f0f36b6afee9365baa2d5f4cbfdca`. Each preserved body authenticates. Unchanged document/comments/drawing and regression carriers match the recorded source113 map. The evolving source115 text was not consulted for this conclusion.

Actual Word114 receipt `98db652d16cac8478699f6232cb55320e6c58e15c6f7d3d2096a8bd0d8152018` and all six raw logs authenticate, with safe naturally reaped groups. The actual native library result is616 passed, zero failed and13 ignored. The actual regression result is2392 passed, four failed and seven ignored, exit101. Regression raw log SHA is `0ca8dc03f2cc1c3435569d41c1b0512623b182633bdbd9c7be46b76e63854777`. No ignored case receives oracle credit, and no full regression success is inferred.

## Not found and boundary

Correctness and namespace authority: no production defect is established by these four old assertions. Each conclusion follows from the fixture's actual namespace and source-owner identity and the concrete surviving refusal guards. It does not waive the independently reported mixed-use lexical precedence defect from native pass4.

Panics and OOXML: no new finding is established in this bounded oracle audit. The observed unwrap_err and assertion panics are test expectation failures. Foreign lookalikes, unsupported namespace-valued content, mismatched owner publication and raw preservation still require their actual negative and successor controls.

Structure: zero additional findings. Only existing test carriers and classification helpers are necessary for bounded corrections. No new production type, module, file, dependency, owner framework or quota is justified.

Read canonical workflow/microscope, child plan, progress, native pass4 and the preserved original cases. This audit ran no provider, test, Cargo, CP, Office or target operation and changed no source, index, Git reference, ledger or baseline. Only this canonical report was written after root granted the writer gap. Amendments and deciding runtime qualification require a separate normal remediation phase. Review ends and returns control to the orchestrator.
