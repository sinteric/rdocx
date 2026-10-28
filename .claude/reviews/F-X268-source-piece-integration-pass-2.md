# F-X268, source piece integration, pass 2

**Reviewed**: Increment after pass 1 in `work/f-x268-codex`, parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding four-file diff contains 6692 added and 214 removed lines. The new scope is the source-child read proof, its carry across reopen, the two related inline regressions and page-border prepublication validation. Source SHA-256 is `201b83f8a022d2cf2cb112347ad93a3dd76b5a8a712bd42687d397dbeb93be47` for `crates/rdocx/src/document.rs` and `ff2aac81347f906cafa53de82684d9b2c305b48ff0a1c93e7d4fe9d38c7e4edf` for `crates/rdocx-py/src/document.rs`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement these changes, run builds or tests, or modify source.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this scope.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML preservation, tests and structure produced no finding in the inspected increment.

`read_source_child` actually parses a closed one-child document under the supplied current head and tail. It checks that removing that child leaves the same normalized empty shell. It records the parsed item separately from writer eligibility. `staged` is assigned only after an exact child-writer byte comparison. A second call cannot turn the prior parsed fact into a writer proof. Non-round-tripping field, tab, bookmark and entity sources decline before the candidate is published. Existing whole-path errors remain authoritative on a declined proof. Inspected paths are `crates/rdocx/src/document.rs:9295` and `crates/rdocx/src/document.rs:23188`.

The closed-body guard excludes a cross-body source. The body parser computes each direct child's namespace prefixes from the parent rather than carrying a sibling's local declaration forward. Paragraph, table and control readers consume their own complete child. The first-section distinction cannot be silently composed as an ordinary source child because the one-child and unchanged-shell checks reject it. Changed child reconstruction still receives the complete-model resource check and the exact final partition check before publication. This review does not infer XML validation from normalization alone.

`install_carried_blocks` binds the exact current part bytes to the carried part, reconstructs an actual parsed model from the record and compares its normalized serialization with the reopened model. It carries source-read facts even when source bytes normalize on write. It does not assign a missing `staged` value or otherwise grant normalized source bytes serializer reuse. Existing writer value alignment and private-fact exclusions remain active. Inspected path is `crates/rdocx/src/document.rs:23231`.

Page-border validation still prepares the candidate before revision publication. The replacement uses the existing save-equivalent method against the same candidate. That method validates and prepares the first save before comparing the second, retains its trusted-baseline reread conditions and keeps ZIP fallback for unqualified packages. The unchanged false/true comparison result is immaterial to this validation call. A preparation error still returns before owner revision or native assignment. Inspected paths are `crates/rdocx-py/src/document.rs:1782` and `crates/rdocx/src/document.rs:22709`.

## Evidence examined

`source_read_proof_does_not_grant_writer_reuse` compares the actual parsed source with a whole parse, repeats the source read, requires `staged` to remain absent, and requires the next writer to serialize the normalized child. Its private tab-stop control requires a whole-path fallback for non-round-tripping source. `source_piece_note_references_keep_neighbor_reads_bounded` uses two changed warmups and two changed measured edits at 50 and 200 paragraphs, checks note creation and all six work slots, and requires the first four slots to stay zero. The shared source-splice and malformed-source tests remain in the affected batch.

The initial native note RED is preserved in `/private/tmp/fx268-codex-note-source-red.log`. Its measured row is `[13524, 10143, 3381, 0, 1, 52]`. The first attempted repair in `/private/tmp/fx268-codex-note-source-green.log` still failed, with 52 versus 202 child reads, and separately exposed the changed entity-source eligibility. It is not a passing result. `/private/tmp/fx268-codex-note-source-green3.log` records 6 passed tests in 2.18 seconds. `/private/tmp/fx268-codex-note-source-proof-batch2.log` records 7 passed tests in 2.26 seconds, including writer-proof and private-fact controls. The earlier proof-batch compilation failure is not semantic RED evidence.

The prior original Python batch on artifact `79ba306a0bf0f58689014283a273dcd1a7e3e913cafe8abdd445c80b506a9a86` completed with 7 passed and 3 failed in 197.50 seconds. That result replaces the in-flight status reported at pass 1. The remaining failures were footnote, fragment and page-border work bounds. It does not qualify this increment. The new Python build log records a successful dev build in 17.04 seconds. Focused installed Python work and page-border semantic/refusal results remain pending at this review boundary.

## Scope and pending work

This bounded result covers the inspected source-read and validation increment. It is not mechanism 7 completion, a whole-feature microscope or final acceptance. The fragment producer-frame branch remains unfinished. Mechanism 8, A3 and A5 composition, release equality and error-order execution, HLD completion, performance, memory, native Word, hash and full integrated verification remain pending. Pass 1's unchanged proof and memo observations are retained rather than relabelled as new executions.
