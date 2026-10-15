# F-X206, combined paint controls, pass 52

**Reviewed**: the external test-only Issue323 packet in `/private/tmp/S91-F-X206-issue323-combined-paint-controls-prepared`. One existing carrier, one parametrized function with two proposed installed cases. No production proposal is reviewed or adopted by this pass.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Tests are unrun. Native PowerPoint, LibreOffice and no-repair acceptance remain pending.

## Authority and authentication

Read canonical CLAUDE.md, WORKFLOW.md and microscope, the full approved F-X206 group/connector contract, current sprint/backlog status, the R12 prospective carrier and relevant retained fixture/public API helpers. The plan at `.claude/plans/F-X206-design.md:88` explicitly requires independent combined rotation/flip evidence, with the native application requirements retained at `:274`. F-X206 remains in progress.

Receipt SHA-256 is `989c20de746e2c8dacefbc5ea81f2f55b45f43c79b99339969da90313d651355`. Patch SHA-256 is `6156f92af8237b2321c6cec55c8063963488594c99f6d32137e5600dbb2fce42`. Independently authenticated every referenced packet digest. All 2,714 actual current worker byte, size and mode records match source map `a9de0b3018cca10e5ac9b4a951de8542a3546060b5946917016cf516ba1884a4`.

Prospective R12 carrier base is `af1cead7afa1caa6309be1cd6ed2d352708cdba6da8826f669f5c7e649df1686`. Complete proposed carrier is `153cee1d1aa566676722f8d09443ad5842b34c53233f0467dda7c3a1607c58e5`, 381,011 bytes. It retains the entire original literal prefix and every original top-level AST node exactly, then appends one function. Its decorator selects uniform2x and nonuniform2x/1.5x. AST parsing is source inspection, not product import or test execution.

## Six-aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Candidate `crates/rpptx-py/tests/test_documented_examples.py:7957` fixes group offset, extent, child extent, clockwise30 rotation and horizontal flip as independent source operands. The oracle at `:7995` scales child coordinates by2 horizontally and2 or1.5 vertically, subtracts the corresponding frame centre, flips horizontal displacement, rotates clockwise, then restores group position and converts EMU to72dpi pixels. The reverse branch rotates before flipping, providing the intended different affine operation. |
| contract | 0 findings. Candidate `:7973` invokes the reported public group add_connector operation, not a pre-imported connector. Both unequal child/outer coordinate frames are tested. This supplements the approved combined-transform obligation without replacing grouping/ungrouping, unsupported decomposition refusal, glue routing or native no-repair barriers. |
| panics | 0 findings. Fixed generated-package prerequisites assert exact shape inventory and replacement count. Pixel sampling bounds are checked before access at `:8007`. No untrusted parsing or runtime mechanism is added to production. |
| ooxml | 0 findings. Opaque extension bytes include entity spelling and whitespace, are placed last within shape properties, and are checked after rendering and save/reopen. Group source transform facts, child order and IDs are checked independently from pixel positions. Existing source-aware fixture helpers remain exact. |
| tests | 0 findings. Four interior connector samples require real near-black paint at independently calculated positions at `:8011`. Two separate interior rectangle samples require its actual colour at `:8013`. The reversed-order negative at `:8014` must miss black paint at at least one location. Independently calculated reversed-order distances from the intended connector segment range from11.18 to58.10 pixels across both cases, so this is not an algebraically identical or rounding-only negative. Render source equality, retained held owner, saved opaque bytes, reopened child identities and exact repeated PNG are explicit controls. |
| structure | 0 findings. One appended parametrized test reuses existing generated-package helpers and existing installed libraries. No new source file, asset, module, dependency, production API or executor is introduced. |

## Findings and acceptance limits

0 defects. 0 smells. 0 nitpicks.

The proposed cases are `test_fx206_issue323_added_group_connector_paints_combined_affine_and_keeps_owners[uniform-2x]` and `[nonuniform-2x-1p5x]`. Their oracle uses fixture constants and elementary affine math, not product-resolved coordinates or a second invocation of the product transform. It is independent geometry evidence for actual paint once executed. It is not independent PowerPoint or LibreOffice output.

The new controls verify successful-operation owner/source survival. They add no rejected-mutation scenario and therefore earn no new invalid-operation atomicity credit. The existing broader source/refusal controls and full accepted grouping, inherited rectangle, namespace, relationship, member recovery and live-handle contract remain required. Pending R12 production/candidate reconciliation and its compile-only renewals are separate adoption barriers. The actual current worker map is not misrepresented as the prospective carrier map.

No genuine Base, all installed cases, native application opening, repair absence, full-feature runtime, hash or completion result is inferred. Only this canonical review file was written. No packet, worker source, Cargo, environment, cache or UI changes occurred. The pass ends and returns its review writer lease.
