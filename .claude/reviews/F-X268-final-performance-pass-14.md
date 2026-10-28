# F-X268, final performance, pass 14

**Reviewed**: Frozen working diff against `c71560d29d1969ce8079ee3b15dfd2303dddf1ae` on `sprint/s95.4`. Two existing files, 162 insertions and 54 deletions. Focused continuation under the canonical workflow and approved F-X268 contract, covering pass 13 D4, marker membership, counters, fallback ordering and the corresponding HLD update.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Required runtime verification remains pending.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D4 resolution

The marker reader at `crates/rdocx/src/document.rs:26871` now counts each actual XML read attempt and consumed-byte interval before either normal processing or propagation of the original read error. The candidate hooks count actual examined bounds, order pairs, membership candidates, hidden ancestors and marker attributes, including rejected or failed values. They do not substitute a constant entry count or count only matches.

The qualification at `crates/rdocx/src/document.rs:26826` requires valid ranges within the source and nondecreasing disjoint order. The forward cursor at `crates/rdocx/src/document.rs:26853` is then correct for the parser's monotonically increasing source offsets. It skips a paragraph when the offset equals its exclusive end, accepts an offset equal to its inclusive start, and rejects gaps before the next paragraph. Skipped ranges are visited once, while a current paragraph is checked once per eligible marker. Empty ranges cannot match and advance when reached. Empty inventories return false, and single or many touching disjoint ranges obey the same half-open containment rule.

Unordered, duplicate nonempty or overlapping ranges decline that qualification and use the original `Range::contains` search at `crates/rdocx/src/document.rs:26844`, with actual candidates counted. This retains the original behavior for nested or duplicated paragraph projections instead of assuming their ranges are disjoint.

Membership now runs after the unchanged pure hidden-owner, namespace and marker-name guards at `crates/rdocx/src/document.rs:26914`. The old membership predicate had no observable effects or errors, so avoiding it for ineligible events does not change which attributes are parsed or which error occurs first. Hidden-state computation and stack updates remain in their original relative order. Marker ordinal, identity and reference selection remain unchanged.

D4 is closed by source inspection. The batch must still demonstrate the resulting measured bound and affected correctness tests.

## Other aspects checked

The batched paragraph-discovery changes and matching comment-companion fixture remain as reviewed in pass 13. Optional qualification failures return to the exact old scanner loops. The rebase fallback retains its incremental ambiguity check, preserving early ambiguity versus later scan-error ordering. Actual source paths and offsets retain their direct and controlled axes. No new alias, shadowed-prefix, opaque-control, inline-owner or nested-owner defect was established.

No additional findings in correctness, contract scope, panics, OOXML preservation, error ordering or structure. No public API, cache field, source file or module was added. The HLD update accurately describes the new qualified cursor, original fallback and counted work without claiming a runtime pass.

## Evidence limits and source identity

No builds or tests were run by this reviewer. The implementer is holding the new verification batch until this review returns. No native count pass, affected range-test pass, clippy pass or hash result is inferred for this frozen source. The earlier valid-companion count failure remains historical RED evidence. Installed v6 results remain 74/79 flat gates, 2/5 linear gates and 12/12 original issue 407 gates, with the failed flat and linear gates unresolved by this source review.

Exact SHA-256 values:

- `crates/rdocx/src/document.rs`: `4ff0f27741fde6b7a567dda377fc9f938c76d16b9a68193caa2cf7182cb6f6f1`.
- `docs/hld/12-testing-strategy.md`: `3322052c061476cb32cff047276599d9856ef0517cf4a507e2cd46b83f3cfdfa`.

Separate debug controls, completed count qualification, installed-package timings, memory, native oracles and final integrated gates remain required. The reviewer changed no source.
