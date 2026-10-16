# F-X236, Measure public setter inlining before changing policy

**Status**: completed
**Sprint**: S93
**Size**: S
**Depends on**: none

## Problem

[Issue367](https://github.com/tensorbee/rdocx/issues/367) proposes public setter
inline annotations to improve downstream compilation without LTO. The workspace
release profile enables fat LTO (`Cargo.toml:147`), so a workspace release timing
alone cannot answer that question. Run setters delegate to optional setters and
initialize properties (`crates/rdocx/src/run.rs:540`,545), while paragraph-mark
setters directly assign fields (`crates/rdocx/src/paragraph.rs:698`,703).

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X236, Measure public setter inlining before changing policy".
- `docs/hld/10-bindings-spec.md`, "Two supporting decisions", consuming builders and non-consuming setters.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", regression and Issue67 performance provenance paragraphs.
- `docs/hld/15-build-and-toolchain.md`, "Toolchain pinning" and "Packaging".

## Approach

Build a real temporary downstream crate outside the workspace, depending on the
local public facade, with explicit opt-level3, lto=false and codegen-units1.
Use the pinned toolchain and locked dependency graph. Record source manifest,
compiler, target, CPU, exact commands and generated assembly before measurements.
Keep temporary benchmark sources, commands and raw samples together in ignored
scratch evidence, with their digests and durable conclusions in AS_BUILT.

Measure independent run formatting and paragraph-mark formatting callers with
pre-existing properties and first-use property construction. Include a realistic
multi-setter authoring loop and a consumed serialized result. Use black_box on
inputs and consumed results, not an opaque operation around every setter. Inspect
assembly for actual cross-crate call elimination and redundant default stores.
Keep document construction and serialization outside setter-only timing and
report the end-to-end control separately. Warm up, alternate baseline/candidate
order across at least seven paired samples and retain every outcome and code size.

Test a narrow annotation candidate in existing run.rs and paragraph.rs carriers
only after establishing the baseline. Retain annotations only where compiler
evidence and repeatable timings show a benefit larger than measurement noise,
without a material end-to-end or code-size regression. Otherwise record no change.
No inline(always), blanket policy, public signature, dependency or product file
is proposed. Existing tests carry semantic regressions for true/false/None,
fresh and already initialized properties, and unrelated formatting retention.

## Rejected alternatives

- Workspace default release timings, because fat LTO hides the proposed boundary.
- A copied local setter microbenchmark, because it is not the library consumer.
- Blanket annotations or a fixed desired speedup unsupported by actual evidence.

## Test plan

The **test gate** is **regression**.

| Category | Test | Asserts |
|---|---|---|
| regression | existing run/paragraph tests and independently inspected external semantic consumer | True, false, None, genuinely fresh absent properties and pre-existing unrelated colour remain exact through save/reopen |
| integration | temporary external public consumer | Actual separate crate, no LTO, consumed output and inspected baseline/candidate assembly |
| regression | paired setter measurement | All samples, medians, variation, code size and realistic control support selective annotation or explicit no-change disposition |

Run changed-crate cargo check, relevant existing tests and clippy. Do not add a
test that merely checks annotation spelling. Run scoped verification and an
independent zero-finding microscope before completion.

## HLD impact

- `docs/hld/15-build-and-toolchain.md`
- `docs/hld/12-testing-strategy.md`

## Risk routing

- Published public setters: signatures and semantics stay unchanged. If source
  annotations change, run verified publication dry-run and actual archive size
  assertions for affected crates, preserving the existing10MiB ceiling and
  archive measurement policy. Required measured numeric updates may touch the
  existing README.md footprint row and scripts/readme_doctests.py catalog, with
  an exclusive metadata lease and unchanged64byte tolerance. Evidence-only
  disposition does not add an API.
- External compiler comparison: pin the actual compiler, consumer source,
  flags and CPU. No external oracle becomes a production dependency.

## Hash harness

Expected unchanged, all49 entries. An unexplained delta blocks completion.

## Implementation checklist

- [x] Approve the complete sprint designs and claim an isolated worktree.
- [x] Authenticate baseline external consumer, compiler evidence and timings.
- [x] Measure a selective candidate and decide from the complete evidence.
- [x] Prove semantics, scoped verification, packaging if applicable and microscope.
- [x] Prepare handoff without writing sprint totals or pushing a worker.

## Open questions

No product ambiguity. Selective annotation versus no change is the measured
outcome, not an approval question. Temporary evidence uses scratch carriers.
