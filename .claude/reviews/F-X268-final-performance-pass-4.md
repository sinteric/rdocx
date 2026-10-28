# F-X268 final performance remediation, pass 4

**Reviewer:** Independent Codex agent `/root/review_source_pieces`.
**Scope:** Frozen canonical diff against `8293c67a`. Five files contain 387 added and 25 removed lines: `crates/rdocx/src/document.rs`, the approved F-X268 plan, and HLD 03, 04 and 12. Scope is canonical-source-first namespace replay and temporary masked-query composition. Prior clean reviews supply the unchanged source/opening/provenance assessment.
**Method:** Read-only source, contract, assertions and existing execution-evidence inspection. No implementation, builds, tests or delegation.
**Verdict:** Zero defects, zero smells and zero nitpicks in this bounded batch. The implementation is ready for affected release qualification. No timing, memory or final acceptance result is inferred from native counted-work controls.

## Source identity

| File | SHA-256 |
|---|---|
| `crates/rdocx/src/document.rs` | `9e8bebfd4787492d13efd49e3d311d11b95dc9801608e2c7afc2c70e44e496dc` |
| `.claude/plans/F-X268-design.md` | `6d1497931b736976d19effd7c488afe1242794bdc90ca7572d77f64dfce727ed` |
| `docs/hld/03-architecture.md` | `e0812f3f2c020257b1797fd60b7b54f4a4b596967981ea0345eaba22a8b2d7a8` |
| `docs/hld/04-opc-and-packaging.md` | `5cb536ffe33cc26cd1c3fff56bf1b00909d65bd00f1d82b4908f7c33a43e20e0` |
| `docs/hld/12-testing-strategy.md` | `3fe7e26da065a7b474c2315eb9fbc18c4e65df82dbedd6357fa549a96db3acd8` |

## Writer and replay error behavior

The serialization closure now consumes the canonical writer result for the current or masked typed document before authoritative namespace replay (`crates/rdocx/src/document.rs:21563`). It uses exact owner-span proofs when available and otherwise leaves replay's authoritative span scan in place. A failed optional writer path invokes the original full writer, preserving its actual error rather than publishing an optional-proof failure. Successful full fallback counts the body children actually written.

The canonical and full paths use the same document writer admission and byte contract established in earlier reviews. The replay algorithm, owner ambiguity checks and diagnostics remain unchanged. If replay changes the canonical bytes, `relationship_xml_from_replayed` still drops the record. The subsequent relationship query retains its authoritative whole scanner. No normalized or replayed result is promoted into writer or source-parse proof merely because its semantics appear equivalent.

## Temporary masked-query certificate

`masked_query_frames` is limited to writer-produced records passed by this private query path (`crates/rdocx/src/document.rs:9477`). It requires exact current and original byte partitions, identical heads, equal block counts, checked empty frames, and exact root/body declaration and binding vectors. Every child must have a staged writer value and must not be raw, field-containing or private-fact-ineligible. Both frame span records remain tied to their own actual head and tail.

Namespace composition uses an original child's own relative proof only when the current child at the same indexed occurrence has identical bytes (`crates/rdocx/src/document.rs:9511`). Changed children are qualified against the masked frame. Every child span is checked for containment before rebasing with checked arithmetic. The new tail's spans come from the new frame and receive separate containment checks. Exact partitions establish the resulting body endpoint. The complete new span list is then supplied to the existing replay matcher.

This does not relabel old cached proofs with a new tail. The old child proof is still checked under its original frame. Identical head and root/body environments, closed child containment and independently checked new frame establish the temporary query composition. The generic frame invalidation, `namespace_context_matches`, actual parsed ASTs, writer eligibility and decoder locks are unchanged. No new retained cache field or per-edit history is introduced.

Relationship composition follows the same certificate, taking new-frame head/tail facts, original facts for byte-identical same-occurrence children and fresh facts for changed children (`crates/rdocx/src/document.rs:9568`). It deduplicates while preserving first physical occurrence order. Its caller falls back to the existing relationship query whenever composition declines (`crates/rdocx/src/document.rs:21697`). These methods answer the plain-binding opaque inventory query. They do not broaden unrelated inherited-binding query eligibility.

## Opaque occurrences, sentinels and controls

The complete current occurrence set still determines collision-free masking sentinels before any query composition. Modeled relationships are replaced in the masked typed document, while opaque occurrences remain untouched. Final filtering retains IDs that remain in masked output and appeared in current output, so a shared modeled/opaque ID is not globally subtracted.

The new transport regression compares both composed queries with their authoritative whole scanners and exercises a header ID also present in opaque XML (`crates/rdocx/src/document.rs:48746`). It compares the actual opaque-inventory result with forced-off behavior and requires the shared ID to survive. Changed head bindings, malformed tail, a child-crossing span and a field-containing block all decline. These are meaningful negative admissions, not empty-success substitutions.

The W15-owner plus modeled-header regression exercises the formerly growing masked-frame path through repeated distinct actual scoped replacements (`crates/rdocx/src/document.rs:48862`). It compares complete package parts and typed serialization with the whole reference. It retains the same strong assertion comparing both the staging vector and namespace qualification work at 25 and 100 paragraphs, rather than relaxing the growing-work assertion to accept linear scans.

## Plan and HLD

The plan clarification states the exact temporary query certificate and exclusions without changing user choices, public API, storage policy or budgets (`.claude/plans/F-X268-design.md:92`). HLD 03 and 04 describe the same source-versus-query distinction and authoritative opaque masking (`docs/hld/03-architecture.md:65`, `docs/hld/04-opc-and-packaging.md:1947`). HLD 12 records the actual positive and fallback discriminators, counted full-writer fallback and retained final riders (`docs/hld/12-testing-strategy.md:4439`). No constant-time whole-call or release-performance pass is asserted.

## Execution evidence and limits

- `/private/tmp/s954-namespace-masking-whole-writer-red.log`: genuine initial failure reports 109 serialized children at 25 paragraphs, exposing previously uncounted duplicate writer work.
- `/private/tmp/s954-namespace-masking-local-batch.log`: the intermediate candidate still failed the stronger namespace-work equality, 234,228 versus 894,828. This remains a failure, not a passing repair result.
- `/private/tmp/s954-masked-query-composition-native.log`: all nine focused final-gate tests passed in 17.58 seconds, including the unchanged stronger work assertion and new query transport controls.
- `/private/tmp/s954-masked-query-affected-native.log`: four affected namespace replay, positive-owner-span, retained-picture and scoped-proof regressions passed in 15.04 seconds.
- `/private/tmp/s954-masked-query-clippy.log`: applicable scoped clippy succeeded in 28.63 seconds. Other formatting, prose and adapter check results are not inferred from that clippy completion line.

No additional correctness, namespace, panic, resource, structural-smell or nitpick finding was established. The last actual release summary still contains six scope/picture/hyperlink misses plus the narrow `Cell.add_table` miss. Those remain failed until actual current-source release remeasurement. No release-v3 result is claimed. Memory qualification, native Word permission/execution, missing fixed PowerPoint resources and all remaining final riders stay pending. No final feature, sprint or release acceptance is claimed.
