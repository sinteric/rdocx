# F-X205, paint and installed corrections, pass 8

**Reviewed**: Frozen source32-to38 correction, two existing Rust carriers, 332 additions and 14 deletions. Also the three frozen source40 fixture, typing and stub carriers. Bounded correction review only.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Identity and scope

The immutable paint capsule is `paint-correction-source38-capsule` under `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1`, manifest SHA-256 `402cc1798415534755ba55d1ddbdb9a6df807e833ec2921730a7fe2f2635cb2a`. Every manifest artifact hash and size matches. The reviewed incremental diff SHA-256 is `6d59beed30922e5aa80ef610a620de301a1738b69cf9b0e493ad84a1aa638d04`. The historical HEAD-union source-diff.patch was not used as the correction scope.

The installed correction capsule is `installed-fixture-stub-source40-capsule`, manifest SHA-256 `9ac91233fe5bd509664ae0d50b2fa678a28edc7e0fcef6aa541c302e2cea72dd`. All artifact identities match. Comparing source38 and source40 maps yields exactly its three Python carriers. Reversing the one fixture return, nine __all__ additions and from_odp parameter spelling reproduces the respective source38 hashes exactly. Removing the appended typing function reproduces the source38 typing hash exactly.

This pass excludes later pattern raster phase repair, later API implementation, full native paint acceptance, fresh source40 stubtest execution and whole-feature completion. The reviewer did not author these F205 corrections. Retained independent provider authority remains distinct from the incremental conclusions below.

## Correction dispositions

Pass7 D1 is resolved in these exact bytes. `crates/rpptx-render/src/text.rs:86` now charges Grid colour-vector capacity using saturating multiplication and addition. The deciding cache control at `crates/rpptx-render/src/text.rs:4091` requires both successful below-budget caching and rejection of additional over-budget owners, then independently totals retained colour allocations and vector overhead against the existing limit. It preserves the existing cache publication and font-event replay paths.

Mixed ordinary text and fields both use the corrected substantive segmentation at `crates/rpptx-render/src/text.rs:388`. Runs split on actual selected family changes, preserve their exact source-string concatenation, and keep preceding whitespace, ASCII punctuation and combining marks with their shaping context. Equal physical-family slots retain a single segment. The control at `crates/rpptx-render/src/text.rs:2187` verifies source-font coverage, distinct physical Latin/Arabic/CJK faces, nonzero glyphs and actual lowered paint for both text and field inputs. The genuine Base log records the Latin sample incorrectly using Noto Sans Arabic, followed by a one-test semantic failure. The corrected control passes.

The PPT-only resolver implements channel-directed gamma at `crates/rpptx-layout/src/context.rs:4357` and conditions its application at `crates/rpptx-layout/src/context.rs:4375` and `crates/rpptx-layout/src/context.rs:4400`. It applies the special rule to two endpoint stops or matching-first/last three-stop lists. Shifted endpoint and other stop lists remain linear. Alpha remains linear. Original stop positions and colours remain in the sampled linear consumer list. Shared Word Paint interpolation and the independently observed native pattern palette are unchanged.

The conditional rule matches the primary Microsoft Open Specifications response, including directional channel weighting and its bounded stop-list conditions. [Microsoft's gradient interpolation explanation](https://learn.microsoft.com/en-us/answers/questions/2248059/non-preset-a-tilerect-behaves-strange-in-case-of-g). No universal linear-light policy or fitted gamma is introduced.

The fixture at `crates/rpptx-py/tests/test_documented_examples.py:8636` re-fetches the same paragraph after add_run publishes structurally. The source operands, returned run and deciding assertions remain unchanged. The stub export list at `crates/rpptx-py/python/rpptx/_rpptx.pyi:53` now includes the nine already-existing native classes. Its from_odp signature at `crates/rpptx-py/python/rpptx/_rpptx.pyi:247` names the existing runtime parameter file. These are pure fixture/annotation corrections, with no runtime class or API change.

## Six aspects

- **Correctness**: Zero findings. Cache accounting closes D1, physical-slot splitting retains source text, and the conditional interpolation follows the authenticated primary rule.
- **Contract**: Zero findings in the bounded corrections. Pattern raster parity remains pending. Gradient diagnostic improvements do not constitute a newly inferred acceptance threshold.
- **Panics**: Zero findings. Family splits use char_indices byte boundaries, and cache arithmetic remains saturating. The correction introduces no unchecked untrusted indexing beyond guarded fixed stop counts.
- **OOXML**: Zero findings. These corrections consume resolved paint/text facts and do not rewrite source XML or discard opaque owners.
- **Tests**: Zero findings. The mixed-face Base is a genuine executed semantic failure. Budget tests require actual admission and rejection. Gamma controls distinguish opposite channel directions, linear alpha, shifted endpoints and mirrored stops. Fixture/stub failures are preserved rather than relabeled passed.
- **Structure**: Zero findings. No new module, trait, dependency or public forwarding wrapper is introduced.

## Actual evidence and limits

The genuine mixed-face Base receipt SHA-256 is `08343806ead58ce9d4267b7ed1e0ebbaeb59190cc40ae4eec83ec628f12a8d10`. Raw output reports zero passed, one failed and zero ignored after successful compilation, with the physical-family discrepancy. Its failed wrapper status is retained.

Current cache, mixed-face and gradient receipts are respectively `10dc54da54a87b2b62716a47323104f1439b616c32476437bda477b7a873db68`, `ca96421ed067705908e40f9167c2954984fea713b5223c4a1de53deb8b287ab7` and `b453d4f8736ecb55674f44e7f8c1c22235e1424248bdce8ecef4e1f9bff4fb72`. Each selected test reports one passed, zero failed and zero ignored. Source-after guards and natural owned-group cleanup are true. These are retained runtime observations, not executions by this review.

The native/product metric artifact SHA-256 is `7971add9396c7f547e5399b56b394b55b2a62d8980f142e9e93f4ee3cec22860`. It records gradient interior RGB errors of approximately 1.03 to 1.60, and explicitly retains large unresolved pattern differences. It states diagnostic comparison only. This report grants no native crop parity, tolerance waiver or full pattern acceptance.

Installed attempt2 remains failed with 35 passes and five fixture failures. Attempt3 records the five successors passing and strict mypy succeeding, followed by the two genuine stubtest inconsistencies. Its failed status remains failed. The frozen source40 annotations address those exact inconsistencies, but no fresh packaged source40 stubtest PASS is claimed here.

Zero smells. Zero nitpicks. Review ended without source or runtime mutation.
