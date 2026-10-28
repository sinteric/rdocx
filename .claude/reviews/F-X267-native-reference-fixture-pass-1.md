# F-X267, native reference fixture, pass 1

**Reviewed**: Focused Q26 pre-capture source inspection at base `8133233513e75c951d9d2ce1f3cc415dda3e7b14`. Three relevant files, 281 insertions and 18 deletions: `crates/rpptx/tests/integration.rs`, the Q26 addition in `.claude/plans/F-X267-design.md`, and only the new Native PowerPoint reference admission subsection of `docs/hld/12-testing-strategy.md`. The source patch SHA-256 is `f184cb37fd33c7f6c72b17df48a427f6ebf4dc3b31ce84f2d0b86466598afe68`.
**Reviewer**: Codex flat_cost_design, independently of Root's fixture implementation.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. The authored-target correction itself has no identified source defect. This is a source inspection of a pending final rider, not full feature completion or native acceptance.

## Defects

### D1, the exit paint control can pass when the exit has no effect

`crates/rpptx/tests/integration.rs:23207`
`crates/rpptx/tests/integration.rs:23236`

The exit check requires only an existing Hero state and inequality against the completely static slide. Earlier nodes already apply held opacity and transform to that same Hero. Removing the exit effect or retargeting node 108 to the empty Title placeholder therefore still leaves a Hero state and a Hero raster different from the static slide. The assertion labelled "exit must alter visible Hero paint" does not discriminate that wrong implementation.

The source makes the counterexample concrete. The prior opacity node has `fill="hold"` at `crates/rpptx/tests/integration.rs:22764`, the parallel transform nodes also hold at `:22766`, and the exit is a separate node at `:22770`. Its intended outgoing fade changes the selected owner's opacity and visibility, as implemented at `crates/rpptx-layout/src/timeline.rs:1185`. No executed mutation or RED result is claimed by this source analysis.

The approved Q26 contract requires visible owner-specific exit evidence at `.claude/plans/F-X267-design.md:318` and `docs/hld/12-testing-strategy.md:4648`. Add a discriminating assertion of the exit phase on the selected Hero and compare visible paint with the same timeline at the same elapsed time with only that exit disabled or deliberately retargeted. This isolates exit from the already held effects and does not require a new permissive threshold or a native baseline. Batch it with the pending affected checks.

## Smells

None found.

## Nitpicks

None found.

## Not found

- **Correctness of target selection**: `authored_timeline_rectangle` selects one immediate exact-name owner and returns that owner's own ID. It rejects ambiguity, non-shape children, missing or zero IDs, duplicate sibling IDs, placeholders, custom or non-rect geometry, absent transforms, nonpositive extents, negative offsets, overflowing/out-of-page extents and unequal solid fill. The two real generator call sites select the red Hero and yellow marker after actual package serialization. Neither retains a positional fallback or hardcoded shape ID.
- **Contract and scope**: The plan explicitly corrects Root's earlier unchanged-generator premise within approved Q26. Six slides, nine existing cases, local and encoded times, thresholds, existing historical pins and transition payloads remain unchanged in this diff. No product API, dependency, module or production renderer changes occur.
- **Negative and ordering controls**: The new fixture test covers missing name, duplicate name, duplicate ID, missing ID, wrong kind, wrong paint, placeholder, empty extent, negative offset and reversed child order. The selected-ID state checks would reject the historical first-placeholder bindings. These are source-inspected assertions, not executed passes.
- **Outgoing fade and marker binding**: The click target now names the actual authored yellow marker. The existing click boundary checks retain `fill=remove` retirement after 500 ms. The terminal outgoing request remains `u64::MAX`, `u32::MAX`, no outgoing slide, and zero-progress fade still compares with that exact terminal page at `crates/rpptx/tests/integration.rs:23281`. The source retains both blue Hero and yellow marker. Actual terminal paint and native fade parity remain pending, not established by source inspection.
- **Panics and arithmetic**: Helper arithmetic uses checked addition. Its Option-based rejection does not panic on missing fixture facts. Caller expect/unwrap/index operations are bounded to this controlled six-slide fixture and tests, not a new public input path.
- **OOXML and structure**: The helper uses the existing parsed shape properties and existing timing model. It adds no alternate parser, writer, trait, generic, tracked source file or forwarding-only wrapper. Timing XML payload structure and serialization paths remain unchanged apart from the chosen IDs.
- **Reference independence**: New native captures and trusted pin adoption remain pending. No changed manifest currently certifies itself or repurposes the old source-bound movie as the corrected fixture.

## Authenticated source and historical evidence

All four paths in `/private/tmp/s954-q26-fixture-source-freeze/manifest.json` match their recorded SHA-256 values. In particular, `crates/rpptx/tests/integration.rs` is `d58d39ff838d03a62e96a0868879920109c11ce1cd864bf3dfe7a06ee0eacb93`. The F-X268 plan is authenticated only as a frozen cross-reference. Its separate story implementation is outside this review.

The preserved old source is still `1f1a6da66e646ca4b2312fcc0f239e1dce023fdb54554c63b67e843a98bb5d0f`. Independent read-only ZIP inspection confirms slide 1 Title=2, Subtitle=3, Hero=4 and all seven old timing targets=2. Slide 2 has Hero=4, marker=5 and old click target=3. This substantiates the fixture correction without inventing a renderer defect.

The historical movie SHA-256 `69e78fded4f6f66cf7e416673a5632db04cf931dc41c6188f99b419ec4c33545` and all nine raw PNG hashes match `/private/tmp/s954-q26-powerpoint-requalification/timeline-frame-extraction.json`. The old source-generation, native-export, extraction and comparison receipts remain present. Their recorded failures remain historical failures, not observations of correctly targeted Hero animation. A first read-only frame-hash probe used the encoded-time argument instead of the output-path argument and stopped with FileNotFoundError. The corrected read-only probe authenticated every actual output path. No historical file was written.

## Evidence limits and handback

The plan remains approved. Shared trackers mark F-X267 done for the already completed local provider checkpoint and explicitly retain final rider debt. This narrowly authorized review does not reopen or recertify that prefix and is not a full `/complete-feature` microscope.

The new tests and helper are uncompiled. No cargo check, test, build, render, native application action or RSS process was run in this review. Actual local GREEN, corrected source generation and receipt, fresh native movie and nine exact-time extractions, unchanged threshold comparisons, trusted versioned pin adoption, hash harness and applicable final-union checks remain pending. The active memory measurement was not disturbed.

F288 changes, Q27, other dirty HLD hunks, Word qualification, product performance and prior completed packaging implementation are excluded. Only this review file was written. This pass ends and releases the sole-writer lease to Root for separate remediation and orchestration.
