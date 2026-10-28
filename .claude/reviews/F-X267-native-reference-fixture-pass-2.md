# F-X267, native reference fixture, pass 2

**Reviewed**: Same focused Q26 pre-capture fixture scope as pass 1, including D1 remediation and the corrected negative-test setup. Three relevant files against base `8133233513e75c951d9d2ce1f3cc415dda3e7b14`, 318 insertions and 17 deletions. Only the Q26 subsection of the plan and the new Native PowerPoint reference admission subsection of HLD 12 are included. Current `crates/rpptx/tests/integration.rs` SHA-256 is `f7faa239e8753af0afb4408cf75dd623018a796b6dbf51ad43db75659803cb6c`.
**Reviewer**: Codex flat_cost_design, independently of Root's implementation.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded fixture source scope. Pass 1 D1 is closed. The actual 330 ms semantic failure remains unresolved and blocks qualification. This is not a whole-feature GREEN or native acceptance verdict.

## Defects

None found in the reviewed fixture correction and D1 remediation.

## Smells

None found.

## Nitpicks

None found.

## D1 closure

`crates/rpptx/tests/integration.rs:23208` now checks the selected Hero's exit visibility and opacity 0.39 at 2805 ms. The same-time control at `:23238` removes exactly the unique outgoing fade node, verifies node 108, retains the other timeline content, reopens the package and evaluates the same position. It asserts opacity 0.5 without exit, equal prior transform and clip, and different rendered paint at `:23265` through `:23277`.

This rejects the pass 1 counterexample where held earlier effects alone make the page differ from the static slide. It introduces no new threshold or native replacement reference.

The authenticated isolated wrapper actually executed the numerical and paint counterfactual and passed 1 test with exit 0. Its log records opacity 0.39 versus 0.5, equal prior transform/clip and unequal paint. The two actual PNG hashes differ and match the receipt. The primary round-trip body did not reach these assertions because it failed earlier at 330 ms. The wrapper does not independently repeat the primary body's explicit exit-visible assertion. Source inspection and the executed bounded counterfactual support D1 closure without claiming that the complete round-trip test passed.

## Not found

- **Correctness and contract**: The unique authored-name helper and both target bindings retain the pass 1 guards. The negative placeholder case now clones the actual existing template placeholder after asserting it exists, rather than calling an unsupported Default constructor. No product renderer or timing implementation was changed.
- **Meaningful discrimination**: The exact current negative-owner body passed. It retains missing/duplicate name and ID, wrong kind/paint, placeholder, extent and reorder controls. The exit counterfactual isolates a real changed owner at the same elapsed time and preserves prior state.
- **Outgoing marker and fade**: The exact existing terminal fade body passed. It retains the yellow marker source and terminal outgoing position `u64::MAX`, `u32::MAX`, with no outgoing slide, then compares its paint with incoming fade at progress zero. This proves that local composition assertion, not native fade parity or every click phase.
- **OOXML, arithmetic and structure**: The helper's checked bounds, parsed property checks and ordinary shape requirement remain intact. D1 removes one controlled complete XML element from this exact fixture and reparses it. No new production API, trait, generic, module, dependency or source file occurs.
- **Reference provenance and historical preservation**: Nine case coordinates, timing payloads, thresholds and historical pins remain unchanged. The corrected source has no adopted native movie or trusted reference pin yet. The old wrong-owner evidence is not relabelled.

## Actual source and artifact authentication

The current source hash matches Root's stated freeze and the v3 source receipt. All seven extracted fixture/helper/test functions are independently byte-for-byte equal to the current tracked source. The actual wrapper source hash matches its source receipt, and executable SHA-256 `7d504ec7a2e9a965618713b02a0f39bdc3d2a82da47d0a57ae30d83e787e688c` matches the build and execution receipts.

The v3 build receipt records compile exit 0 in 11.56 seconds. Its preserved v2 compile log records the unsupported `CT_Placeholder: Default` constructor failure. That failed compilation is not GREEN evidence. The v3 exact three-test batch records exit 101 with 2 passed and 1 failed. The negative-owner and terminal-fade bodies passed, while the complete round-trip body failed at the selected Hero visibility assertion.

The later `timeline-corrected-local-v3/independent-generation-and-D1.json` and matching log record the separate one-test wrapper exit 0. The earlier handoff's NOT RUN generation/D1 fields describe its earlier stopped phase. They do not override the later actual receipt, and the later receipt does not erase the earlier failure.

Corrected source `/private/tmp/s954-q26-powerpoint-requalification/timeline-corrected-source-v3/timeline-oracle-source-corrected.pptx` is `afbf77a7de08cf057a2e787cf89e4d39eeee729c12a870c8359282b13e3d1570`. Its length and hash, both exit/control PNG lengths and hashes, and the wrapper executable were authenticated read-only. Independent inflated-ZIP comparison against preserved old source `1f1a6da66e646ca4b2312fcc0f239e1dce023fdb54554c63b67e843a98bb5d0f` confirms identical entry sets and exactly two changed parts. Slide 1 differs only by seven `spid="2"` to `spid="4"` substitutions. Slide 2 differs only by one `spid="3"` to `spid="5"` substitution. Every other inflated part is byte-identical. Placeholders, shapes, timing payloads, clocks and transitions remain intact.

## Remaining semantic failure and acceptance boundary

`crates/rpptx/tests/integration.rs:23203` still fails because the actual selected Hero is invisible at 330 ms. The assertion is neither removed nor relaxed. Source inspection identifies an existing relevant path at `crates/rpptx-layout/src/timeline.rs:1161`: a future incoming effect sets the same owner's visibility false and opacity zero before its phase. This is a source-based explanation of the observed local failure, not a native PowerPoint compatibility conclusion.

The fixture itself now targets the intended visible authored owners, validates and round-trips deterministically in the isolated wrapper. Fresh independent native capture of that valid corrected source may investigate the 330 ms discrepancy. Requiring the product comparison to pass before obtaining the independent oracle would hide the behavior under investigation. Conversely, this zero-finding fixture review does not waive that discrepancy or authorize a passing qualification record.

Fresh corrected-source native capture, all nine exact-time comparisons, source/application/movie/frame provenance, trusted pin review, unchanged geometry/SSIM and cutoff/fade gates, full applicable integration checks and hash harness remain pending. Old wrong-owner source/movie/frame observations and their failures remain historical evidence. No new native capture, pin adoption, commit or release is approved by this review.

## Handback

This pass read the canonical workflow, microscope, applicable differential/risk guidance and prior review without changing their instructions. The plan and provider-prefix status retain the same bounded meaning as pass 1. F288, Q27, other dirty HLD hunks, Word, performance and memory acceptance are excluded.

Only this review file was written. No compiler, test, render, native UI, RSS or benchmark process was run. The pass ends and immediately releases the sole-writer lease to Root. No extra confirmation pass is required for the clean reviewed fixture scope.
