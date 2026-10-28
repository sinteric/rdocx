# F-X268, exact canonical readback, pass 25

**Reviewed**: Working diff against eec45dc4e8788735132fc28419dd2060f1d74da1. Seven tracked files, 462 insertions and 28 deletions. Frozen diff SHA256 is 980034dd7f0bbcbe8261a18fa84db28d67e73cc600ab47ffd4adb06d781c69fa. Native document.rs SHA256 is 5761931b3b808404ddf170c0ff44d6347042a039e5fea2de0ffa9b7d10838392. All seven current path hashes match the before and final manifests.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and scope

Codex flat_cost_design performed this independent implementation review. This reviewer implemented earlier F-X268 work, prepared qualification adapters and independently assessed the current design before implementation. Shared authored the current temporary prototype and stripped proposal. Root authored the canonical port, tests and documentation. This reviewer did not implement this reviewed batch. Design pre-audit observations are not substituted for the actual canonical diff review.

The canonical workflow, microscope command, risk routing, approved plan, five listed HLD impacts, active sprint status, latest progress and pass24 were read. F-X268 remains in progress. This review covers relationship readback, drawing no-op reconstruction and final retained-source readback, including their unchanged fallbacks and publication consumers. Earlier clean CJK, PowerPoint and form implementations are outside this bounded pass. Only this review file was written. No compiler, test, renderer, timing, profile, RSS process or source repair was run by this reviewer.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Source and contract audit

The approved three-site contract is at `.claude/plans/F-X268-design.md:618`. Its HLD impact matches the five actual documentation paths. No public signature, cache field, source file, module, dependency, generic, process switch or resource budget was introduced.

`cached_document_read_context` at `crates/rdocx/src/document.rs:9217` requires enabled proofs, literal current head identity, exact part/body/tail qualification, an empty parsed frame, complete namespace context, matching child count and reusable current owners. It reads existing successful child locks without filling them. Child private behavior, lexical fields, failed or missing proofs and changed root/body bindings decline before a speculative private parse. The explicit head-prefix check completes the partition requirement beyond the existing context helper. Existing immutable publisher invalidation is retained.

`current_source_model_matches` at `crates/rdocx/src/document.rs:9256` retains complete actual source/model equality and the exact original model resource check. Drawing admission at `crates/rdocx/src/document.rs:23476` compares every replacement against its actual checked source range after original discovery and ID reservation. Numeric identity alone does not qualify. The original edit vector and complete reconstruction remain when qualification fails. Registry updates, retained-source handling, related stories, image canonicalization and signature authority continue.

`qualify_document_children` at `crates/rdocx/src/document.rs:9302` extracts the original mandatory first phase without changing its order or failure boundary. Missing source children are read on the same edited record. Failed first-phase qualification returns directly to the original whole fallback. Successful qualification followed by a private or unavailable reconciliation executes the original second phase once. It does not call the first phase again. The original `parsed_document` composition is retained only for tests after its two production consumers are explicitly split.

`reconcile_readback` at `crates/rdocx/src/document.rs:9267` qualifies all cached reads before replacement. It prepares actual unequal child copies and the complete source frame without mutating the candidate. Equal children remain owned in place. Original empty-namespace-owner certificate completion occurs before the transaction, preserving that phase-two work rather than shifting it downstream. Actual source differences are published, not ignored through weaker equality.

`apply_readback_replacements` at `crates/rdocx/src/document.rs:9029` checks bounds, unique ordered indices and the empty source frame before mutation. Exhaustive destructuring includes section properties, namespace declarations, both background collections and root attributes. Between the swaps and their outcome, only the exact pure CT_Document resource validator executes. Refusal restores every child and frame field before returning decline. Indexing remains covered by the preflight and unchanged vector length.

Relationship publication at `crates/rdocx/src/document.rs:23570` offers reconciliation only for the actual empty edit vector. It installs the same qualified edited record with part=None, then continues the existing live relationship registry update. The original whole-source path remains authoritative when the fast attempt is unavailable.

`canonical_retained` at `crates/rdocx/src/document.rs:9394` uses the exact final remapped source. Its one production caller preserves None versus Some(None), original slot rejection before parsing and unchanged package/main-part record publication. Final output bytes are prepared before model mutation. Both unavailable and rejected outcomes leave the candidate unchanged. Reconciliation failure restores it before the original qualified phase-two parse, whose failure still selects the original whole retained-source fallback. Opening AST ownership is unchanged.

## Evidence examined

The final canonical freeze is `/private/tmp/s954-readback-canonical-scoped/freeze.json`, SHA256 3c228f6e7a96331ce13c01795e846a5ff883fa5152f6cc5505c5f4fc0adb99dd. Its source hashes and full diff match the actual tree. The scoped receipt SHA256 is 0a6a9461d8ba77fb9bfd4be0cfc729f5bbef7f16c78530f129c65f7b3d9df98a. Every recorded log hash was authenticated. The actual checked native artifact was freshly produced by Cargo. All-target/all-feature check, native build, 26 unique selected native cases, clippy, fmt, prose and diff checks completed with exit0.

The new `canonical_readback_guard_matrix` at `crates/rdocx/src/document.rs:51793` contains ten assertion scenarios. Positive reconciliation directly asserts admission. Complete source equality has both true and false controls. Late missing and failed reads, stale source, actual frame invalidation and private rejection preserve the candidate. The private case asserts zero guard child reads followed by one original phase-two read. A genuinely overdepth typed replacement forces the actual validator to refuse, restoring two children and every frame field. This is a transaction test, not fabricated successful malformed-source provenance. Retained unavailable, rejected and successful outcomes are separately asserted.

The other selected native cases cover inventory and alternate-content occurrences, physical picture identity, namespace/frame invalidation, unrelated decoding/error order, private/source-writer distinctions, resource refusal, reopen failure, engine mutexes, signed package authority and scoped splice behavior. Their actual assertions and selected-case logs were inspected. No full-suite or external-oracle result is inferred from this scoped batch.

Temporary normal-cfg mechanism evidence is `/private/tmp/s954-eec45-warm-readback-v3/RESULT.json`, SHA256 c7dbde515a0145895bfeacfe54328375cd2480dd61efa23e576e3cf8498a2cb1. It identifies diagnostic module d9f62fbd15c68dfdc32c51f191f5d27da5fa235dff9823a8c0f785ab72a33dd5, debug assertions false and test false. Twelve ON and twelve OFF public actions compare exact saved/reopened parts, media, relationships and returned physical picture facts. Cross-version comparison also includes v2 OFF and the original unsplit v1 OFF, without XML normalization. Twenty same-build guard observations and eight focused Python controls pass.

Both relationship and retained reconciliation commit in all twelve diagnostic actions, copying seven unequal children covering1192source bytes at both25 and100 paragraphs. Eight warm drawing windows take the no-op. Actual parser and namespace/canonical scan work and unrelated opening copies remain visible and unchanged against OFF. OFF restores the original owned copies. These counters discriminate removed reconstruction rather than admission alone. The actual mismatches are three paragraphs and four controls, not the earlier unproved drawing.raw_xml explanation. Canonical differences from the prototype are the stronger head-prefix qualification, equivalent exhaustive fallback structure, test-only counters and direct equality/private-read controls. Temporary environment, TLS, cfg and export scaffolding is absent from the shipping diff.

## Remaining qualification

The earlier normal eec45 picture timing failures remain failures. This pass does not establish a new normal module, either picture timing bound, the complete79-flat/252-call/five-linear/Original407 gates, final A9 vectors, memory/history qualification, hash harness or full-union acceptance. These remain with Root's integrated qualification. No threshold, sample selection, native reference or performance claim was relaxed. The pass ends here and the review writer lease is released.
