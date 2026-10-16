# F-X178, Clearable direct run formatting setters

**Status**: completed
**Sprint**: S92
**Size**: S
**Depends on**: F-X200

## Problem

Issue364 identifies missing nullable reset forms for Run setters. The remaining
native gaps are width scale and baseline position in `crates/rdocx/src/run.rs:991`
and `crates/rdocx/src/run.rs:1002`. Both currently allocate direct properties and
assign Some. Earlier formatting work already supplies the other reset forms.
The Python Font surface does not currently expose these two properties.

## Spec reference

- `docs/hld/10-bindings-spec.md`, "Python Word formatting and checked list publication", absent-source clearing and native setter delegation.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", unit and round-trip gates.
- `docs/hld/14-development-backlog.md`, F-X178.

## Approach

Add `Run::set_width_scale_value(&mut self, percent: Option<u32>)` and
`Run::set_position_value(&mut self, half_points: Option<i32>)` beside the
existing setters. Each returns without allocating properties when both the
argument and properties are absent. Otherwise assign only the selected field.
Retain the existing builders and setters, delegating setters to Some.
Do not add validation or reinterpret existing units or legal values.
This is additive native API. Preserve the established Python surface because
there is no corresponding exposed width-scale or position property to repair.

## Rejected alternatives

- Clearing with 100 percent or zero position leaves an explicit override and does not restore inheritance.
- Adding Python properties expands Issue364 beyond its existing Rust setter contract.
- A generic setter helper adds indirection for two concrete assignments.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| unit | `clearing_width_scale_and_position_does_not_create_run_properties` | Both None setters on a property-free run leave properties absent |
| unit | `clearing_width_scale_and_position_preserves_other_direct_properties` | Some values, including zero and signed position, persist in the typed fields, selected clears leave the sibling, bold false and opaque XML intact |
| round-trip | `run_width_scale_and_position_can_restore_inheritance_after_reopen` | Existing setters and new Some forms serialize correct w:w and w:position, clearing one leaves the other, clearing both retains unrelated properties and opaque XML through save/reopen |

**Test gate**: unit and round trip. Each property writes its explicit value,
removes only its own element on None, and adds no w:rPr when absent.
Retain explicit-false controls for boolean setters. Values survive save and
reopen, with all existing deterministic hashes unchanged.

Use the existing unit module in `crates/rdocx/src/run.rs` and existing
`crates/rdocx/tests/integration_test.rs`. No new module or source file.
Run cargo check -p rdocx --all-targets while iterating, then scoped verification
including cargo fmt, rdocx all-target/all-feature clippy, cargo test -p rdocx,
hash harness and repository policy checks.

## HLD impact

- `docs/hld/10-bindings-spec.md`, native nullable setter contract beside the formatting section.

## Risk routing

- Public API of a published crate: additive methods with existing signatures preserved. Run patched-local workspace cargo publish --dry-run --allow-dirty under the scoped gate and assert every archive is under 10 MiB. Follow the exact patch list in .claude/commands/verify.md.

No unit conversion, parser, serializer, binding, dependency or baseline change.
The round-trip regression still proves preservation of unrelated raw XML.

## Hash harness

Unchanged. Existing callers retain identical assignment and serialization.
New clear operations are exercised only in source-built tests.

## Implementation checklist

- [x] Add failing unit and save/reopen tests in existing test entrypoints.
- [x] Implement the two nullable methods and delegate existing setters.
- [x] Prove absent-property, independent clears, explicit false, zero, negative position and opaque payload preservation.
- [x] Pass scoped verification and additive API packaging riders.
- [x] Obtain independent zero-defect, zero-smell microscope review.

## Open questions

None. Keep this repair on the existing native Run surface.

## Gate maintenance clarification

The approved source additions change the packaged rdocx member inventory size.
The orchestrator authorized mechanical measurement maintenance in README.md and
scripts/readme_doctests.py on2026-10-15. The exact locally generated archive
SHA256 is0fccd1fcf3cce06efbc0a4ebb50e0937a2dc83d8889b7e0097cc8b3ea4b447c9.
Using the existing archive_measurement policy, its compressed size is3703343,
normalized member bytes51782842 and member count37. The64-byte compressed
tolerance and10MiB bound are unchanged. Receipt is retained at
/private/tmp/S92-F-X178-archive-receipt.json. The mandatory policy suite generated
the artifact through cargo package --locked --allow-dirty --no-verify with every
local patch. The separate full workspace dry run still verifies every archive.

The worker also carries the parent-owned scaffold64d1b50f one-line assertion
correction in scripts/test_sprint_workflow.py, changing the planned release range
from92..96 to94..98. This is inherited sprint policy maintenance, not an F-X178
product change. These additional evidence carriers require independent pass2.


## Scoped verification evidence

All required gates completed on2026-10-15 against the reviewed worker source.
The implementing parent observed missing-method RED cargo check exit101 before
adding the two methods. The same named tests now pass in the complete crate run.

- `cargo fmt --all --check`, pass.
- `cargo clippy -p rdocx --all-targets --all-features -- -D warnings`, pass.
- `cargo test -p rdocx`, pass:626 unit,380 integration,2396 regression and2 doctests.
  Existing ignored counts are12,8 and7 respectively. Pinned Poppler26.01.0 and
  LibreOffice26.2.5.2 equipped the actual successful run outside the sandbox.
  All Poppler tool hashes matched the retained installation receipt. Default
  newer tool and sandbox-converter failures were retained rather than credited.
- `python3 scripts/hash_harness.py --check`, pass,49 entries unchanged.
- `python3 scripts/prose_check.py`, pass,0 violations.
- `python3 scripts/sync_agent_skills.py --check`, pass,26 adapters.
- `python3 -m unittest scripts.test_sprint_workflow`, pass,180 tests and2 skipped.
  Actual registry access and the default archive target path were available.
- `cargo publish --workspace --dry-run --allow-dirty` with all22 canonical local
  patches, pass, actual exit0. Verification builds used an isolated target while
  Cargo retained archives in the worker target/package. A separate assertion
  examined all44 generated archives,22 top-level packages, and required each
  below10485760 bytes. Maximum observed size4900985 bytes. The first assertion
  refused the guessed build-target archive path, then the corrected assertion
  checked the actual generated artifacts. No size bound or check was waived.
- Independent microscope passes1 and2 report0 defects and0 smells.

Full workspace tests, no-default-features, WASM, workspace documentation and
supply-chain checks are deferred to the integrated full gate because this
scoped plan does not trigger their riders. Durable execution logs and receipts
are named in the worker progress record. Combined-source integration must
remeasure the archive row rather than retain this worker's isolated-source row.

## Canonical batch integration evidence

Canonical product crates and Cargo inputs are byte-identical to reviewed worker
source d2d7445cf4c5adf8fe6539950b899150025268b6. Its scoped native suite and
verified packaging rider remain valid at this feature scope. Actual canonical
archive metadata was rederived with the existing helper:3703340 compressed,
51782842 normalized member bytes and37 members. The three-byte compressed
difference from the recorded3703343 is within the unchanged64-byte allowance.
No measured constant or count was relaxed. Receipt:
`/private/tmp/S92-integration-preflight/F-X178-canonical-archive.json`.

Current canonical focused unit tests passed2 and the independent save/reopen
integration test passed1. Logs are F-X178-native-unit-isolated.log and
F-X178-native-roundtrip-isolated.log in that evidence directory. The initial
slow root-target compile was interrupted without credit, followed by the
successful coherent isolated-cache run. No semantic conflict was present.
Final integrated full verification, hash gate and sprint review remain due.
