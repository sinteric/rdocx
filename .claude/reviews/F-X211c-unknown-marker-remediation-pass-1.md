# F-X211c unknown-marker remediation, pass 1

## Scope and verdict

Independent read-only review of frozen source `04625d8353057372d29a8e6db1320e19dfb5c612289900585ad6363c083f99d0`. All3,366 worker bodies, sizes and modes match. Exactly `crates/oxml-layout/src/font.rs` and `crates/rpptx-render/src/text.rs` differ from failed a726. Patch authority is `40c5f0e70ed005fb7b10b4a333de0a17df10ce7092af5286b36e2a19c5aecb33`.

Zero defects in the proposed PPT remediation. One distinct remaining Word contract defect found by the requested cross-consumer scope check. Zero smells and zero nitpicks. No tests, runtime probes, installs, UI actions or source changes occurred in this review. Concurrent native checks are not runtime credit in this report.

## PPT remediation evidence

At `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1726`, the old original-coverage fast path is replaced by a read-only query with the same selected-index, authenticated physical-family and uncovered-character predicates at line1773. Font resolution, mappings, loaded resources and source ownership remain unchanged. The new query does not mutate the provider.

At `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-render/src/text.rs:878`, warning collection is limited to a marker retained for visible body text. It follows successful final contribution admission at line885, and applies only to character bullets at line886. The helper at line904 names canonical Symbol or Wingdings requests, compares original and displayed scalars, excludes known mapped replacements, whitespace/control and unrelated code ranges, and suppresses warnings for authenticated original-code physical coverage. Line933 says unknown semantic mapping, not falsely missing glyph coverage. No new guessed mapping is introduced. Its original-family/scalar messages are deduplicated in the local accepted text-body diagnostic vector, with at most1024 possible new canonical pairs per such vector. The font trace, cache budget, rollback stores and provider historical diagnostics are unchanged.

The controls at `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:5333` distinguish caller labels from authenticated physical names and actual glyph coverage. Renderer tests at `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-render/src/text.rs:3679`,3737 and3763 exercise actual retained F0FF and missing glyph0, known mappings and unrelated PUA, repeated provider usage, empty noncontribution, an existing Latin glyph under an unverified Wingdings label, and repeated bounded original-scalar dedup. These are real source-level controls, not evidence that the pending installed runtime passed. The repeated stack_text test warms the font provider, not a persistent cross-call shaping cache. The diagnostics use local accepted-layout output, so they do not require new font-trace event replay.

## Defect D1, Word unknown explicit symbol diagnostics remain absent

The same approved F-X210 unknown-code contract covers source-preserved explicit symbols, not only PPT bullets. See `/private/tmp/rdocx-f-x211c-codex/.claude/plans/F-X210-design.md:316` to320 and the same-pass diagnostics contract at line323.

`/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:9735` resolves `RunContent::Symbol` through the shared resolver and immediately shapes at line9743. That resolver still retains unmapped F0FF without an unknown-code diagnostic at `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1750` to1763. The Word numbering-marker path at `/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:8838` has the same silent behavior. Neither Word call adds a symbol warning.

This is not covered by existing multilingual fallback. A plain F0FF symbol does not meet the Word multilingual trigger at `/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:13259`. Its ordinary accepted contribution at line10009 reaches the provider approximation function, whose `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1686` accepts only Aptos or Aptos Display. When surrounding content selects the multilingual path, engine line13379 calls `shape_multilingual_paragraph`, and font.rs line2350 discards that helper's auxiliary diagnostic vector. That vector contains small-caps fallback facts, not the absent original-family symbol warning. It therefore cannot establish existing Word unknown-symbol coverage.

The new PPT helper is reached only from the PPT marker consumer. Thus fixing the actual PPT failure does not fulfill the remaining Word explicit-symbol and numbering-marker diagnostic contract. Add bounded deciding Word controls in the existing entrypoints before completion, including F0FF source/save-reopen immutability and same-pass actionable family/code facts, genuine original-face coverage suppression, known mappings, ordinary non-symbol text and unrelated PUA, and repeated/provisional layout isolation. Keep ownership and lifecycle routing explicit when selecting the implementation owner. This review does not prescribe a shared diagnostic registry or new framework.

## Boundary and disposition

The original installed15-pass/1-fail candidate, earlier facade failure and immutable100 denominator remain retained. No frozen assertion is waived or changed. Finish this review before a separately owned remediation. Fresh installed sixteen controls, typing, source-specific whole100 and original native/render hard gates remain unqualified here. The canonical review-file lease is released when this report is delivered.
