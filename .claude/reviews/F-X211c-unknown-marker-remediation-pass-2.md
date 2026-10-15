# F-X211c unknown-marker remediation, pass 2

## Verdict and source

Zero defects, zero smells and zero nitpicks. Authenticated all3,366 worker bodies, sizes and modes against source map SHA256 `8cb3efa3c40b760a057a32820377f9e02ab0c6bdfec890bc9cde39f8cc16cd59`. Exactly font.rs, PPT renderer text.rs, Word engine.rs and existing Word test_rendering_threads.py differ from failed a726. No source changes, runtime probes, test reruns, installs, UI actions or corpus modifications occurred in this review.

## Shared coverage and diagnostic semantics

The shared original-code query at `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1768` retains the exact former physical-family plus original-glyph coverage predicates. It does not trust caller labels alone. The owned message query at line1783 inspects accepted selected-face IDs without changing provider history, resolution, traces, resources or cache storage. Known mapped replacements, unrelated families/ranges, empty/control/whitespace and genuine original physical-family coverage suppress warnings. Unknown semantic mapping names the original family and code while retaining source text, without claiming that an existing fallback glyph is missing. Each returned family vector admits at most512 original low-byte/PUA scalar facts. Consumers retain first-use message dedup, with the two canonical families giving at most1024 distinct symbol facts per local vector.

The modified test-only name-table helper at `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:4295` appends variable-length encoded names, updates its table record/checksum and head checksum adjustment. Its deciding physical-name test at line5407 compares every other original table and the actual glyph index. This authenticates the positive physical-name branch while preserving the fake caller-label inverse. It does not add a font asset or alter production font bytes.

## Both consumers and lifecycle boundary

PPT retains the reviewed accepted-visible-character-marker call after contribution admission at `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-render/src/text.rs:885`. The local helper at line904 consumes the shared pure query and deduplicates into the current text-body diagnostic vector. There is no provider historical warning or trace event to leak from a discarded probe.

Word now calls the same query after successful shaping at `/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:8858` for numbering markers and line9752 for explicit w:sym. Hidden runs leave before that branch at line9110. Both paths preserve original modeled XML and retained source codes, and use the existing local message dedup function at line12675.

The cache boundary was inspected rather than inferred from repeated rendering. Numbered paragraphs are excluded by engine line5621 and Symbol run content by line5657. Table, header/footer and restart cache eligibility reuse those predicates. Consequently the new facts cannot be lost by a prior global duplicate suppressing an eligible paragraph's diagnostic suffix at line5172. Warm explicit-symbol/numbering layouts execute the actual consumers again. The ordinary eligible cache implementation and its budgets remain unchanged.

Caller-width measurement creates its own fresh diagnostic vector at `/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:3512`, and final layout creates its vector at line3897. The query does not record mutable provider diagnostics. A failed layout returns no successful diagnostics, and a later layout cannot obtain these warnings from loaded-font history. Actual cold/warm, measurement-then-empty, hidden/unrelated and source-preservation controls cover these relevant boundaries. No new global diagnostic registry or replay architecture was introduced.

## Actual scoped evidence

Original scoped handle99268 is terminal. Independently authenticated receipt `/private/tmp/S91-F-X211c-unknown-marker-remediation-2/native-scoped-receipt.json`, SHA256 `3e1fa76d644461776910cfd000ede1ea08fcb493bce1d3db00073f31aa99623d`. All12 rows have exit0, qualified status, safe cleanup and matching raw-log hashes. They cover affected three-crate all-target/all-feature check and strict Clippy, native compilation, actual font1/PPT3/Word2 deciding controls, no-default font compilation/control and two existing seven-character font-helper consumers. These actual scoped native facts are distinct from pending installed binding acceptance.

The existing Word Python two-owner control at `/private/tmp/rdocx-f-x211c-codex/crates/rdocx-py/tests/test_rendering_threads.py:1979` checks same-pass tuple facts, source bytes, repeat and saved/reopened explicit-symbol/numbering owners. Its predecessor RED remains retained. This review does not claim that the fresh paired wheel has executed it.

## Completion boundary

The earlier Word gap and actual PPT failure are addressed at source level with scoped native evidence. Failed predecessor sources and installed15-pass/1-fail receipt remain retained. No immutable corpus script, expectation or denominator changed. Fresh paired artifacts, installed sixteen PPT and two Word controls, typing and source-specific whole100/native/render qualification remain pending. This pass ends with this report and releases the canonical review-file writer lease.
