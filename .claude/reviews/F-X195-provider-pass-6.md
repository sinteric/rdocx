# F-X195, provider, pass 6

**Reviewed**: The exact R1 to R2 provider remediation, 10,184 bytes with SHA-256 cabe89e60a0324302152cb64347f39ab403bc8c397879fa394a28a72a1aa3d8e. Two existing files change. The previous full provider review and seven unchanged core files retain their separate authority.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass-5 D1 and D2 are resolved at source level. Runtime acceptance remains pending.

## Authentication and assessment

Authenticated all eleven current source guards and all three frozen diff artifacts. Independently reversing every supplemental hunk in memory recovers the exact R1 engine and PowerPoint text hashes. Read the full remediation, approved provider contract, previous findings and actual cache hit/miss and retention consumers. No Cargo, source mutation, formatting or scratch write occurred in this review.

D1 is resolved at crates/rdocx-layout/src/engine.rs:3829, :3838 and :3847. Each existing retention predicate matches both ordered event variants and passes the selected FontId reference to the original membership test. Paragraph, table and header/footer eviction policy is retained without a new conversion abstraction.

D2 is resolved by the Word control at crates/rdocx-layout/src/engine.rs:21568 and PowerPoint control at crates/rpptx-render/src/text.rs:3773. Each forward and reverse order uses one engine or manager/cache for Aptos, Calibri and Aptos Display, then revisits each request warm. Actual native shaping supplies the filtered final Contribution projection. The controls require original request inequality while selected glyphs, face index, variation/synthesis identity, bytes and ordered inventory remain equal. Word explicitly asserts the real cache hit counter. PowerPoint asserts three retained entries on revisits. Its inspected hit branch validates and replays existing events, whereas its miss branch appends an entry, so unchanged entry count distinguishes cache reuse on this bounded six-call path. Legacy Resolution-only controls remain separate.

The existing rich Word cache refusal remains unchanged. These new Latin controls do not claim a multilingual Word paragraph-cache hit. All new controls remain uncompiled and unexecuted. Prior actual Base failures, core compile corrections and staged source reviews retain their exact scopes.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Both event variants preserve selected-ID membership and existing eviction. |
| contract | 0 findings. Shared-cache request attribution now has both required orders and warm revisits. |
| panics | 0 findings. New test indexing follows the explicit six captures and checked cache fixture construction. |
| ooxml | 0 findings. No parser, serializer, namespace or child-order behavior changes. |
| tests | 0 findings. Native event projection, same fallback identity and cache reuse have distinguishing assertions. Runtime remains due. |
| structure | 0 findings. Existing functions and test modules suffice, with no new trait, generic, module or dependency. |

This ends the review pass. It does not certify later facade, binding, rendering entrypoint, packaging, HLD or full feature completion work.
