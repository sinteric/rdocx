# F-X268, final performance, pass 20

**Reviewed**: Frozen working diff against `6542884725608c2baabf149dcc2496485d85b1d1` on `sprint/s95.4`. Eleven existing files, 1228 insertions and 320 deletions. Read the canonical workflow and microscope command, approved plan, progress, HLD 03/04/10/12, frozen manifest and diff. All eleven source hashes matched the manifest.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. Both defects concern acceptance instrumentation or discrimination. No product semantic defect established. This is not final feature acceptance.

## Defects

### D5, production whole-proof fallback output escapes the transaction counter

`crates/rdocx/src/document.rs:8677`
`crates/rdocx/src/document.rs:27920`
`crates/rdocx/src/document.rs:43630`
`crates/rdocx/src/document.rs:6766`

When the optional block proof declines, `PreparedMainRead::prove_scoped_main_source` executes the actual whole-source production proof. This is distinct from the successful fast path's paused debug reference comparison. The production proof calls `namespace_restored_main_source`, whose first operation serializes the entire current model. It also serializes that current model again for normalization at line 43636. Neither output calls `note_transaction_output`. The normalization closure and occurrence checks produce additional whole-document writer outputs, also without that counter. Namespace replay and the checked/retained sentinel probes create further full source buffers. These outputs have different provenance and must be classified accordingly, rather than all being labelled identical current-model output.

Consequently a force-off or private/unproved-child fallback can perform repeated genuine current-model serialization while the phase counter reports only the initial prepared view output. The hooks in `serialized_main_part` and the no-record branch of `prepared_main_read` do not cover these later calls. The current phase budget can therefore conceal production fallback costs. Existing staging parsed-byte counters do not repair the transaction output count/byte measurement.

Count actual production current-model outputs and separately identify restored, normalized and masked model outputs/copies. Keep only the audited debug reference comparison paused. Add a fallback sensitivity control that demonstrates those outputs remain visible. A costly correctness fallback need not satisfy the qualified one-output budget, but its work cannot disappear from the report. This finding is established from source, not from a reviewer-executed mutant.

### D6, pinned splice packet evidence permits the general publication fallback

`crates/rdocx-py/tests/test_core.py:14994`
`crates/rdocx-py/tests/test_core.py:15001`
`crates/rdocx-py/tests/test_core.py:15051`
`crates/rdocx-py/src/document.rs:6208`
`crates/rdocx-py/src/document.rs:6888`

The pinned original-407 test requires the three removed-work counters to be zero and classification visits to be no greater than the owner count. Zero classification visits also pass. It does not assert that a complete packet was prepared and consumed, or measure the full-inventory/index work of general publication. The all-six-consumer test likewise checks only the three removed-work counters, plus semantics and handles.

A regression that declines packet preparation for these 100/1000-paragraph six-run documents can use the existing semantic fallback and satisfy these assertions while doing a full publication rebuild. A decline after classification also meets the upper bound. The mandatory final index walk at line 6888 must remain visible and distinguishable from a fallback rebuild.

The existing `test_word_owner_pieces_follow_edits_splices_and_related_owners` at `crates/rdocx-py/tests/test_core.py:13756` does measure zero whole-body work across all six consumers on 20/80-paragraph review-shaped fixtures. It is useful positive pipeline coverage and prevents claiming a universal absence of fallback discrimination. It does not establish packet use or index-work accounting on the separate pinned cold/warm six-run path.

Require successful packet preparation/consumption and actual unchanged-index/full-publication work on those pinned inputs, with a deliberate fallback control. Preserve the broader existing six-consumer coverage. This is a source counterfactual about the assertions, not a claim that current execution selected the fallback.

## Smells

None.

## Nitpicks

None. No separately actionable needless-borrow finding was established.

## Product paths and aspects with no findings

Correctness: no additional finding. Traced all four owned native bridges and both shared finalizers. The source package and original invalidation flag remain the signature authority. The complete normal and bundled engine mutexes, including poison state, move through the finalizer. Borrowing APIs retain their clone boundary, zero replacement and mismatch behavior. Python retains the outer private candidate and publishes only a successful checked result.

Contract and error ordering: no additional finding. Checked picture option validation, non-XML validation, related-source loading, selection, mismatch and reopen failure paths. Owned failure consumes only its private candidate. Public Python signatures and stubs are unchanged.

OOXML and proof authority: no additional finding. The temporary read view borrows the exact current document and owns its canonical bytes, spans and writer record. Producer-source matching remains independent from canonical/writer proof. Private-field, frame, namespace, resource, complete-child and force-off guards remain. The view's lifetime ends before package/model mutation and reopen. No source-only fact grants writer authority.

Panics: no additional finding in the changed production paths. Reviewed indexing and scope bounds in the splice preparation and commit paths.

Structure: no additional finding. No new source file, module, trait, cache field or public API was introduced. The private temporary view has an actual bounded lifetime. The common bridge/finalizer paths have multiple current consumers.

Splice semantics: no additional finding. Inspected all six callers, the single inventory classification walk, first-owner slot collision policy, borrowed story selection, scope-count reuse, prepared inserted-owner vector, fallible pre-edit reservation, returned-handle preallocation, birth/retirement, append versus conservative invalidation, and the retained mandatory index update. The three removed collections/walks are actually absent from these paths.

## Evidence consumed and limits

The frozen-source native log reports 65 passed, 0 failed and 1 unchanged external Word oracle ignored in 228.60 seconds. The authenticated debug binding module is `/private/tmp/s954-transaction-green-site/rdocx/_rdocx.abi3.so`, SHA-256 `1537cf3b8e7b97cb2bbab3f24e3fcfb50920f9740b6a0c3c5d0c74714067b92b`. The verification receipt records successful native build, binding build/install/authentication and native tests. Python tests, clippy and hash completion were not present in the receipt consumed by this review.

Inspected the actual old-artifact RED receipts, rather than inferring failure from a later pass. The mechanism RED records nine failures. The separate pinned original-407 RED records four failures across 100/1000 paragraphs and cold/warm states. Interrupted or fixture-error attempts are not counted as complete evidence. Existing native tests discriminate owned versus borrowed transactions, no-op and mismatch, signature authority, engine payload/poison preservation, reopen faults, duplicate view preparation and guarded source/private fallback cases.

No tests or builds were run by this reviewer. No final release timing, memory qualification, Word parity, full performance acceptance or sprint completion is claimed. D5 and D6 remain blocking acceptance-evidence defects even if the pending checks pass.

## Frozen source authentication

| File | SHA-256 |
| --- | --- |
| `.claude/plans/F-X268-design.md` | `48eaa8076f5195575b84373f4bf1f30e3809b295dccdf30f4e28abd97a3683d1` |
| `crates/rdocx-py/src/document.rs` | `44792d1a3bd05be78efcca0a6058ff64d33df11953be51305b3d01bb7f8e6eb3` |
| `crates/rdocx-py/src/lib.rs` | `d4c4bc4cae5b6d1cfd4bfc8f996257f36781057183c707b0638416bc802733c4` |
| `crates/rdocx-py/src/paragraph.rs` | `d5ff8477d55c87599275850e18bd0c3290e30badb90a0e21d9d66b3da03244de` |
| `crates/rdocx-py/src/table.rs` | `c81a8e8a8f0f4f9fa695bcd4d27638174571f2532709c1caf5243a590ca0dfc1` |
| `crates/rdocx-py/tests/test_core.py` | `c3b791aec4690eccc3ca5290e54d6a6bf2482ac1a0fbe33be886410ec670a17c` |
| `crates/rdocx/src/document.rs` | `769f251833f929605141616af88f4bcefd7aa6e409ed3f0030ee254b5f16ea69` |
| `docs/hld/03-architecture.md` | `0b281f3787fbd19b6f1443383bd2e2876ee324060f40e547aa18c52c2d705909` |
| `docs/hld/04-opc-and-packaging.md` | `b98316f33bb6c37a9bafea95faee1483e4f93b4dd41bd0e5c90c71118efacfb9` |
| `docs/hld/10-bindings-spec.md` | `266b398f6531e0a30673ce5bc4e0869db1c6fcacc8bd30a47c6ef4163ca56ddb` |
| `docs/hld/12-testing-strategy.md` | `84765a534e2a126f6d3bef05d35f2ff20f736b54ed6c0b48ccf1ca1913b2990f` |

