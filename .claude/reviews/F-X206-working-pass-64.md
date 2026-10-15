# F-X206, replacement owner proof, pass 64

**Reviewed**: current three-carrier remediation of whole working pass63 D1, its complete typed native equality and publication consumers, and the strengthened existing installed regression entrypoint.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and contract

Worker is work/f-x206-codex at HEAD `7b1e5ef29464e93ae30e68df1523ce09143578ff`. All2714 current byte, size and mode records reproduce the supplied sorted indent2 JSON with final LF digest `697391d1e1548b4d90968cfb86eeda6abee9988567447249fbe9dcb68ac082b7`. Relative to the preceding actual HLD/source union `3ba5a6c7fe47334e0b31f85764935e99bccfa2b9586094bf7ae85b2d03d7777f`, only these three carriers changed:

- rpptx/src/lib.rs, SHA `cc97a4855f74dfe82b79c7af929587434eee412bf65dd5144070f49685f50e09`.
- rpptx-py/src/presentation.rs, SHA `c005a0a2e232743c43f392b4601ad84b3d5dd90f13ea9f65454e80e87076771b`.
- rpptx-py/tests/test_documented_examples.py, SHA `7864bbfeee6860a474e229139cdd72fb76f707d51718248075c67f50a5b070c5`.

All2711 other records match that preceding union. HEAD and raw staged-index digest `c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c` remain unchanged. The native and installed test deltas also match their authenticated prior source carriers from R16 apart from the declared additions.

Read approved F206 plan, current progress, whole working pass63 and the actual consumers. The contract requires retaining unchanged physical ancestors and unrelated owners while retiring affected text descendants. Displayed-string equality is not sufficient no-op evidence. Earlier component and whole-feature review authority remains retained. This pass qualifies the separately authored D1 remediation and does not recertify historical code authored by this reviewer.

## Defects

0 defects. Pass63 D1 is resolved in source.

`crates/rpptx/src/lib.rs:8826` derives PartialEq for TextFrameRef over the actual borrowed CT_TextBody. The existing CT_TextBody equality at `crates/oxml-drawing/src/text/mod.rs:47` compares body properties, list style, complete paragraphs and raw child ownership. It therefore distinguishes A/B formatted runs from AB/empty runs without relying on flattened text. The unused lexical serialization bookkeeping excluded by that existing equality is unchanged by native literal replacement and is not a substitute for accepted paragraph equality.

`crates/rpptx/src/lib.rs:8211` supplies a concrete notes-model comparison, including actual notes presence, typed notes content and its retained raw data. No independent counter or allocated text projection was introduced.

The shape, cell and notes branches at `crates/rpptx-py/src/presentation.rs:1331` compare corresponding original and candidate native bodies before publication. `prepare_owners` uses the resulting text_changed proof to retire only the affected Text scope at `crates/rpptx-py/src/presentation.rs:1055`. Both presentation replacement at `crates/rpptx-py/src/presentation.rs:1202` and slide replacement at `crates/rpptx-py/src/slide.rs:284` use that proof. Unchanged owner topology and body equality retain their generations. Count-mismatch errors return before proof/publication, and zero matches do not publish.

The strengthened existing installed test at `crates/rpptx-py/tests/test_documented_examples.py:5532` sets up differently formatted A/B runs, recaptures valid final descendants after setup, and exercises both public routes. It requires same-string replacement to produce AB/empty while preserving bold/italic run facts, retiring prior paragraph/run/font descendants with the exact public first cause, retaining slide/shape/frame ancestors and an unrelated run, and preserving saved/reopened facts. Count mismatch and no-hit controls keep bytes and held descendants unchanged. Source inspection establishes that these assertions discriminate the original flattened-text helper. They have not been executed in this review.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: 0 findings. Typed accepted-body equality covers regular/field/break distribution, properties and retained raw children. Original-to-candidate comparison precedes publication.
- Contract: 0 findings. Same-string redistribution now retires affected text ownership while unchanged bodies and unrelated owners retain their scope identities. Additive native notes comparison remains subject to the existing published native API and archive riders.
- Panics: 0 findings. New production comparisons use Option-aware borrowed lookups and existing checked owner paths. No new unchecked indexing, allocation, arithmetic or unwrap was introduced.
- OOXML: 0 findings. Equality does not serialize or rewrite source. Literal replacement continues to use the existing staged native operations, preserving source relationships and opaque payload under those operations.
- Tests: 0 findings. The strengthened existing entrypoint retains previous coverage and adds genuine final-run-distribution, first-cause, owner-survival, no-hit, refusal and save/reopen assertions. Actual installed execution remains required.
- Structure: 0 findings. Existing concrete native references and the existing PartialEq trait suffice. No new trait definition, generic, file, module, dependency or historical registry was introduced. The notes method performs actual typed comparison for its current consumer.

## Limits and handback

This is source-only remediation review. No Cargo, product imports, installed runtime, formatter, target, source, environment, baseline, cache or native UI action ran. No production GREEN or full feature acceptance is granted. The actual fresh installed regression, current native checks and all original Issue309/323, geometry, symbol/caps, recovery, font, publication, WASM, typing, hash and scoped completion obligations remain required. Only this canonical review file was written. Review pass ends and returns the lease to ROOT.
