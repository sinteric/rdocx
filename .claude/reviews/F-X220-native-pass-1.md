# F-X220 native pass 1

**Reviewed:** Frozen native core, rendering and caller-font diff on `work/f-x220-codex`, HEAD `e918fa29195153a6d6cbda179a269366033fe6f5`. Four existing carriers contain 3,000 insertions and 28 deletions. The two model carriers retain their clean model pass 2 result. Newly reviewed carriers are `crates/rpptx/src/lib.rs` and `crates/oxml-layout/src/font.rs`.

**Verdict:** Two defects, zero smells, zero nitpicks. This is a bounded native checkpoint, not full-feature acceptance.

## Defects

### D1. Font removal publishes edits despite foreign or missing targets

`crates/rpptx/src/lib.rs:12230` converts every `checked_font_target` error into successful cleanup. `remove_embedded_font` removes the modeled face before calling this helper at `crates/rpptx/src/lib.rs:12141`, then commits at `crates/rpptx/src/lib.rs:12144`. Whole-entry removal likewise calls the helper and commits at `crates/rpptx/src/lib.rs:12164` and `crates/rpptx/src/lib.rs:12166`.

A modeled face whose relationship is missing, external, of a foreign type, points to a missing part or has a non-font content type therefore loses its face or whole font entry and returns success. Listing these cases as Unknown does not reject them. The approved transaction contract requires these unsafe targets to refuse before publication. Embedding already propagates the same target check, so the operations disagree.

Validate every face targeted by removal before committing, including targets which still have another preserved reference. Add distinguishing controls for both removal APIs using foreign and missing targets, asserting an error and exact unchanged document bytes. This is a source-derived finding, not a newly executed runtime result.

### D2. Timeline rendering discards the configured embedded font tier

`crates/rpptx/src/lib.rs:1661` requests the legacy font-manager branch for the public deterministic timeline path. That branch creates a fresh bundled-only manager at `crates/rpptx/src/lib.rs:8251` and loads only `assembly.input.fonts` at `crates/rpptx/src/lib.rs:8253`. Those inputs contain the caller slice, not embedded faces, at `crates/rpptx/src/lib.rs:19148`.

Although preparation builds an embedded-aware manager, the timeline path replaces it before resolving and laying out the requested frame. A slide using an embedded, uninstalled typeface therefore uses fallback metrics and glyphs for timeline output while ordinary prepared output uses the embedded face. The completed caller tier still works, which allows existing caller-only timeline controls to pass.

Retain the configured embedded tier in this rendering path. Add a genuine embedded-only timeline control with a distinguishable uninstalled family and compare actual selected font bytes or frame glyph provenance. Include caller override and missing-flag negatives. No timeline execution was run during this review.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Reviewed behavior without further findings

- The sfnt directory bounds, duplicate tags and named collection refusal are checked before table slicing. EOT envelopes distinguish unsupported compressed or unknown flags from decoded sfnt, and preserve original stored parts.
- Native listing decodes each physical part once into shared Arc bytes with a checked aggregate decoded-data limit. Repeated aliases retain one body. Model reference counts include preserved presentation source, and shared replacement uses copy-on-write.
- Tri-state native flags and genuine no-op checks retain presence-sensitive behavior. Source-preserving model changes remain at their prior clean checkpoint.
- Genuine caller exact and label-derived alias candidates are ranked together with CSS selection and constructor provenance before the embedded alias tier. Explicit byte-free aliases retain their existing exact-family-first contract. Embedded sources retain bounded alias/data admission, atomic refusal and unchanged-set caches.
- Normal prepared rendering, notes, handouts and the three authoring consumers use the shared configured manager. D2 identifies the separate timeline branch which still bypasses it.

Correctness and contract produced the two findings above. Panics, OOXML, structure and additional test issues produced zero findings within this bounded scope. No new trait, generic, crate, module, dependency or source file was introduced by the reviewed diff.

## Authenticated evidence

Checkpoint SHA-256 is `a00cd3579a5217e3e34e32de2da317391ee679157dd6b7a83e50bdbd9f0001a7`. The retained patch SHA-256 and current binary working diff are `048e4baa1c8ab3d136d807265344ffc3463ce4f914a4960c30b6b9ea7898637d`.

Authenticated all four frozen carrier hashes, seven retained receipt hashes, their source inventory files and every retained row log hash. Final retained functional results include 61 manager tests and eight focused native tests. The later existing all-output control passed once and the existing F207 selection passed eleven tests. Retained checks, Clippy and formatting rows exited zero with qualified results and safe owned-process cleanup.

The final all-output and freeze source inventories exactly match the current 3,199 tracked files. Earlier functional and check inventories qualify their recorded intermediate snapshots. The embedded-tier selection has the final manager source, while its native carrier precedes the added existing output-test coverage. Earlier results are not falsely described as fresh executions over the complete final source. The receipts assert their actual complete before/after source guards. Source inventory hashes and driver assertions were inspected rather than rerun.

Native tests use the explicitly recorded 8 MiB stack. The earlier default-stack limitation remains retained. Physical font-oracle and final changed-source 49-output qualification remain pending. Word, Python, CLI, native Office, affected archives, HLD and full-feature acceptance remain required outside this pass.

## Review integrity

Captured HEAD, branch, physical index, logical index, tracked binary diff and complete SHA-256 mapping of all 3,199 tracked files before review. Rechecked the exact capture immediately before and after writing this report. Complete tracked-source aggregate remained `747aea72fdbfd1674795691af4a87c5604956a57ca23c92364c004e68b1d4ff1`. Physical index remained `9fb296d122bd2a99880625492750520bdf6c3aaefd633dbe429344b7d2dee883`.

Only this review file was written. No source, plan, progress, HLD, test, formatting, Cargo, CP or Office operation occurred. F-X215 sources remain frozen. This pass stops here and returns to the orchestrator.
