# F-X211c indexed column remediation, pass 1

**Reviewed**: Exact f3 to f4 one-line source remedy in `crates/rpptx-py/src/table.rs`. Read-only static review with all3366 current and captured source bodies, sizes and modes authenticated.
**Verdict**: Zero defects, zero smells and zero nitpicks.

## Authenticated scope

The sealed f4 source map SHA256 is `543e95102fb7f9c2c95bd82fa15e24913ea08382b7d27d20411d2a0e1fa7907b` and capture receipt is `47454499c719c42b4b593cd7a38afc2953716368b07ec76a1a94622935382c6e`. Comparison against the immutable f3 source6079 finds exactly one changed tracked carrier. Replacing this one new capture expression with its predecessor restores the complete old table.rs body. Tests, collection ancestor captures, owner publication, source preservation and other carriers are unchanged.

## Contract and authority

At `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:421`, ColumnCollection.item now captures Some(EpochKind::Grid) for the returned indexed Column. This directly closes the diagnosed missing scope. At `crates/rpptx-py/src/presentation.rs:3104`, that explicit scope captures the physical table grid token, while row paths already infer Grid through their Row segment. Thus indexed columns no longer rely on the cell classification that did not recognize a column-only Cell segment.

Integer indexing and slicing both use item at `crates/rpptx-py/src/table.rs:442` and `:451`. Iterator members use the same item at `:569`. Returned newly added columns already explicitly capture the prepared Grid token at `:493`, and that path remains unchanged. Indexed column reads and writes validate their captured authority at `:583` and `:595`. Removal validates a supplied column at `:516` after the existing foreign-presentation check at `:513`, before any publication. The remedy therefore applies uniformly to indexed, sliced and iterated children without changing mutation or recovery policy.

Collection owner captures remain None at `crates/rpptx-py/src/table.rs:344` and `:363`. Their physical owner authority remains captured by the existing capture_owner_path logic, so parent removal still retires them, while a successful local grid change does not retire the collection ancestor. The unchanged transactional restructure/publication logic still decides actual grid invalidation and first cause. This capture-only change publishes nothing, does not introduce an epoch bump, and changes no invalid, no-op, foreign-owner or failed-operation behavior. The deciding two-axis controls and all saved/source/byte-atomicity assertions remain unchanged.

## Findings and limits

Defects: zero. Smells: zero. Nitpicks: zero. Correctness, child versus ancestor authority, first-cause routing, source custody and smallest-diff structure produced no findings.

This is source review only. The actual f3 installer remains failed and its18 passes and two axis failures retain their original evidence. Current installed native artifacts are not f4 acceptance. No tests, builds, CP probes, specimen replay or score updates were executed here. Fresh source-qualified execution remains required. This pass ends with this report and releases the canonical review-write lease.
