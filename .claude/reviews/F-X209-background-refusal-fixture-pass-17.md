# F-X209, background refusal fixture, pass 17

**Reviewed**: Finite unadopted F115 integration fixture proposal r2, one existing test carrier and 1851-byte diff. No product source change.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Evidence

Receipt SHA256 `3bc55f83f5f8e99f65d68276c6e0ed2d94a8cd4e47edc9ef1c0846ef8049a656` and all six declared artifacts/references authenticate. The complete diff reconstructs exactly from before/candidate carriers with its declared headers. Before source matches actual source64 map. The original full OXML integration log records 155 passes, one failure and zero ignored, at the original refusal assertion. That failure remains retained.

Approved design `.claude/plans/F-X209-design.md:168` requires materializing effective background for explicit follow=false. The selected modeled reference can therefore be replaced when outside opaque source is preserved. The old fixture places its opaque container attribute and sibling outside the selected reference. Its unconditional refusal expectation no longer represents this approved operation.

## Static admission

The candidate at `crates/rpptx-oxml/tests/integration.rs:1641` first uses the original source for a genuine modeled positive. It checks the requested typed fill, exact outside-reference prefix/suffix, repeated-write equality and reopen equality. It then adds one opaque attribute directly to the selected reference, proves the setup changes source and captures that source before the original refusal and clear/raw-equality assertions. Those original assertions and the direct-background controls remain unchanged.

This is a bounded fixture migration which preserves the opaque-owner refusal contract and adds positive coverage. Static adoption is admissible after the current build source guard ends. Actual successor compilation/test outcome is still required. No passing whole integration suite, installed wheel, native paint or full feature acceptance is awarded here.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no finding in this finite proposal. No product behavior, module, dependency, baseline or public API is changed.

Review ends here and returns to sprint orchestration.
