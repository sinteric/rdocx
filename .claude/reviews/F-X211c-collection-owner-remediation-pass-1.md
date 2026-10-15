# F-X211c, collection owner remediation, pass 1

**Reviewed**: Exact f2 to f3 three-carrier semantic diff, source6079cc5f80f5df5d4e7987ff8298fa50f4a2c2aa276af3e2b6db64fba8ede489 and patch01cb229abda1d30e4fdf264ffdea313dccbbb3b862a0a37aed3dea9fc6933205, against approved parent/c contracts and prior diagnoses1044cf09 and87972216.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Authentication and scope

Independently authenticated all3366 current worker source bodies, sizes and
modes against the sealed map. Exactly engine.rs, the PPT table binding and
its existing documented-examples test entrypoint differ from f2. Checkpoint
19f260d3d99ec698fad14086c2d6d158dc544b1616a3c8273c21d2639df702f0
accurately marks this as static source preparation without executed tests or
fresh wheels. No product file, dependency, asset, registry or public API is added.

## Not found

- Collection lifetime drift at worker `crates/rpptx-py/src/table.rs:344` and `table.rs:363`. Both lazy collections capture the current surviving table path with no extra Grid descendant scope. Physical owner/part authority remains captured. Indexed row, column and cell views still acquire Grid through `crates/rpptx-py/src/presentation.rs:3104`. Newly returned members retain explicit prepared Grid captures at `table.rs:493` and `table.rs:706`. Grid mutation preparation/publication and first causal retirement are unchanged.
- Unsafe legacy-test relaxation at worker `crates/rpptx-py/tests/test_documented_examples.py:2597` and `test_documented_examples.py:5554`. The old collection-stale expectation becomes a live collection length assertion, while indexed-cell retirement, one-bump semantics, precise RowCollection.add_row cause and byte-atomic failed mutation remain required.
- Vacuous new controls at worker `crates/rpptx-py/tests/test_documented_examples.py:10376`. Both first-axis parameters retain the same held row and column collections through each own/opposite-axis append and removal. Actual returned size, counts, retained cell text and exact unrelated owner XML are checked. Foreign members, invalid insertion and same-value row/column size assignments preserve package bytes and live owners. Old row, column and cell views must refuse with the exact first grid operation and no byte change. Each stage checks native save/reopen and actual XML grid/row counts at line10437. Removed table authority is tested separately at line10458, including both held collection mutations, and leaves the sibling intact.
- Cache-contract dilution at worker `crates/rdocx-layout/src/engine.rs:16043`. The fixture keeps its image, opacity, text and geometry but makes the retained raw Word pict namespace-self-contained. This avoids an extra inherited Run binding record while preserving namespace correctness. The function is within the cfg(test) module at line13574. Production cache admission and layout remain unchanged, and the existing six/twelve cache counts and deliberately unsafe variant expectations remain unchanged. This resolves the diagnosed fixture classification without admitting arbitrary raw metadata to the cache.
- Corpus or acceptance changes. Original scripts, assertions, denominator and failed companions remain untouched. The tests are meaningful source-level deciding controls for the approved held collection and unsafe descendant contracts. Their runtime success is not inferred from syntax or this review.

## Pending execution boundary

Fresh affected scoped compilation and deciding native/cache tests, installed
Python collection controls and current artifact/source typing acceptance
remain required. This pass gives no fresh runtime, wheel, Office, original100
or full metric credit. Existing native core/runtime bridges remain subject to
their exact source-specific admissions. The original failures remain failures.

Only this review file was written. No source/test changes, compilation, tests,
targets, CP, UI, ledger, HLD, network or Git mutation occurred. Review ends and
releases the canonical review-write lease before any separate remediation.
