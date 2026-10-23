# F-X260, Codex takeover, pass 1

**Reviewed**: fresh independent working-tree microscope on `sprint/s95.3`, HEAD `15d55d4c4b46df56362abc6944103f496fd78cd6`. Four scoped files, 57 insertions and 4 deletions against HEAD, 61 changed lines.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.
**Reviewer**: Codex in this audit session. This session did not implement the takeover repairs.

This is a local feature review of F-X260 A7 and the superseded F-X262 cell-hyperlink test. It is not a fourth sprint review, final qualification or issue closure. Existing Word and custom-store changes are excluded.

Read `AGENTS.md`, `CLAUDE.md`, `.claude/WORKFLOW.md`, the canonical `.claude/commands/microscope.md`, both approved design plans including F-X260's takeover section, and `.claude/scratch/F-X260-progress.md`. Checked branch, dirty status and both in-progress backlog/sprint rows. Read the latest F-X260 and F-X262 feature reviews as historical context, independently tracing current code. Consulted HLD 03 owned publication and checked XML, HLD 10 lifetime and Element contracts, and HLD 12 binding and checked XML qualification. No primary-source compatibility claim was newly introduced or externally qualified by this review.

## Defects

### D1, graphic-frame rollback backup makes local scalar XML edits copy unrelated slides

`crates/rpptx-py/src/presentation.rs:2382`
`crates/rpptx-py/src/presentation.rs:2385`
`crates/rpptx/src/lib.rs:1110`
`crates/rpptx/src/lib.rs:1135`

Trigger: call `table_shape.element.replace_xml(...)` with only a changed `p:cNvPr/@name`, keeping the frame identity, table presence, relationship facts and text structure unchanged, on an unsigned deck. The same issue applies to other graphic frames. The new backup unconditionally calls `owner.inner.clone()` for every graphic-frame shape before native admission, including exact no-ops and malformed inputs.

`Presentation` derives `Clone` and owns `slides: Vec<SlideRecord>`, together with layouts, masters and package state. Consequently the binding copies unrelated slide models even when native replacement can edit only the selected shape and `publish_stable_element_xml` can retain the owner signature. This newly adds package-sized work and temporary storage to a local scalar replacement. A bounded authenticated probe confirmed that a graphic-frame name edit is supported, so this is an executable consumer, not a hypothetical unsupported transition.

This violates the affected-owner cost requirement in `.claude/plans/F-X260-design.md:36` and the explicit local shape-XML rule in `docs/hld/03-architecture.md:95`, which says these edits do not read unrelated slides. Preserve exact rollback for actual table-presence transitions without eagerly cloning the full presentation for every graphic-frame edit. A discriminating allocation or visit control should cover an unchanged-payload graphic-frame scalar replacement with unrelated slides present.

This finding is a source-derived work-bound violation. It does not assert a measured ratio above 3, invalidate the separately investigated text-box benchmark by inference, or relabel any native timing as Python-binding performance.

## Smells

None.

## Nitpicks

None.

## Consumer and refusal assessment

**Raw XML admission.** Traced all ten native checked replacement entry points through `replace_checked_owner_xml`, including part, shape, table, cell, body, paragraph and regular-run owners on slides, layouts and masters. The new check at `crates/rpptx/src/lib.rs:6099` examines submitted bytes before equivalence or typed assignment. Invalid or unproven fragments return to the existing whole-part path. That path splices into the physical part, checks raw frontiers with actual ancestry, then checks the projected candidate before commit. A fragment failure is not itself treated as proof that a valid ancestor grant is absent. Inspected both Start and Empty event handling, qualified namespace input validation, frontier provenance and MC policy matching, relationship-change fallback, signed-package fallback and local child restoration on assignment or validation failure. No additional correctness, panic or OOXML defect found in this change.

Inspected the fx254 assertions for raw foreign `opaque` and foreign `avLst`, the four frontiers, imported diagnostics, root/group Ignorable authority, blank/additive grants, declaration rebinding, qualified attributes, understood namespaces, once-decoded URI values and physical shape identity. These reject projection-away acceptance, detached-context over-refusal and borrowing an opaque exception from another owner. Whole-part fallbacks can still clone the package by design. Their safety does not establish a local cost bound for those exceptional inputs.

**Graphic-frame refusal.** At `crates/rpptx-py/src/presentation.rs:2428`, changed table presence restores the saved native presentation before returning the named refusal. No owner scopes have been committed at that point. This restores relationships, package state and dirty/signature bookkeeping rather than performing a second XML edit. Inspected native errors, post-edit reads and structure comparison, stable publication, survivor preparation and final commit. The supplied table-to-opaque regression checks exact package bytes and held table/cell handles. An independent reverse opaque-to-table probe checked the named refusal, complete byte equality and the held Element's XML. Both transition directions retain their refusal behavior. D1 concerns the unconditional backup cost, not the observed restoration result. Allocation-failure qualification was not executed here.

**Textbox append identities.** Traced `ShapeCollection.add_textbox` through collection authority, the new target-slide scan, native ID allocation/append and prepared owner birth. The scan uses only `slide(slide).shapes()` and descendants, including groups and selected AlternateContent fallback children. AlternateContent itself is skipped as an ID-bearing owner. Missing IDs and duplicates refuse before native mutation. Compared this with `finish_prepare_owners` structural uniqueness policy and its physical owner inventory. Inspected native connector/group fallbacks, nesting checks and append validation rollback. No additional defect found in the new identity check.

Independent probes confirmed duplicate IDs split between a nested group member and a root shape refuse with unchanged package bytes and a usable held child. A duplicate pair confined to slide 1 does not block `add_textbox` on slide 2. The first compound probe subsequently attempted `.text = "ok"` on that new box and hit the pre-existing general structural owner proof against the unrelated duplicate. The isolated append then passed. That separate setter path is not counted as a new takeover defect or as append failure.

The new identity traversal is bounded by the affected slide, not unrelated slides. Existing native connector/group transaction fallbacks and binding owner-vector work remain distinct from this assertion. No elapsed-time or Python A5 qualification is inferred.

**Renamed legacy hyperlink test.** `crates/rpptx-py/tests/test_documented_examples.py:4892` now follows approved F-X262 A7. It retains exact-byte absent-clear controls for both `None` and empty string, accepts an external address, refuses control characters without changing bytes, and observes the address after reopen. The companion Issue392 test at line 11671 separately covers cell and text-box slide relationship scope, replacement, clearing, unsupported cell `target_slide`, relationship targets and saved-file reopen. Traced the binding address setter to `set_cell_run_hyperlink` and shared staged `set_run_click`. No unsupported target policy was silently removed. No test-contract defect found.

**Structure.** No added trait, generic, wrapper, dependency, public API, module or product file. No structural finding.

## Evidence and qualification boundary

Reused supplied observations after inspecting membership and relevant assertions:

- `/tmp/s95.3-codex-rpptx-installed.log`: pre-repair first run, 59 failed, 958 passed, 1 skipped. This remains a failed historical run. Its concrete scoped failures distinguish duplicate-ID refusal, rollback bytes, foreign geometry projection and ancestor MC authority. Unavailable external font/viewer/raster evidence is not converted into success.
- `/tmp/s95.3-codex-rpptx-regressions.log`: the requested `fx206_duplicate_imported or fx211b_graphic_frame or fx254 or cell_hyperlink_assignment or issue392` union reports 57 passed and 961 deselected. This is selected local evidence, not a complete installed-suite pass.
- `/tmp/s95.3-codex-rpptx-native.log`: seven named fx254 native tests passed. Inspected their finite-frontier, MC, fallback and storage assertions. Native source tests do not qualify a Python timing claim.
- `/tmp/s95.3-codex-rpptx-clippy.log`: supplied scoped all-feature clippy completed successfully.
- `/tmp/s95.3-codex-rpptx-repair-build.log`: supplied optimized dev binding build and editable install completed. This is not a release benchmark.

Bounded probes used `/tmp/s95.3-rpptx-venv/bin/python`, with bytecode writing disabled and only in-memory packages. Imported `rpptx._rpptx` resolved to `crates/rpptx-py/python/rpptx/_rpptx.abi3.so`. Its SHA-256 was `e94663ee215025e7819b72ccd10ad95363305cebb08da29818569c1aa0c0945b`, equal to `/tmp/s95.3-rpptx-py-target/debug/lib_rpptx.dylib`. The editable-install metadata points to this repository's `crates/rpptx-py`. The extension hash was checked again after successful probes. This authenticates the executable used for these observations without claiming a clean release-wheel provenance packet.

A5 needs requalification after the append/XML changes. Historical native-only ratios are not Python-binding evidence. The actual Issue389 Python benchmark is separate work. B1 authentic native Word evidence remains unavailable. Full verification, integrated installed suites and applicable font/viewer/raster, hash, rendering, WASM and publication riders remain pending. No full suite, external oracle, sprint review, benchmark or baseline recording ran in this audit.

## Source identity before and after

The following complete-file SHA-256 identities were captured before source inspection and checked again after the probes, immediately before report creation. All matched. HEAD and the four-file diff counts also remained unchanged. Other existing dirty files were preserved.

| Source | Before and after SHA-256 |
|---|---|
| `crates/rpptx/src/lib.rs` | `c6ba546a8463721703da052b27f60af8aa4549a5c95ea8db7c6a5400cf559562` |
| `crates/rpptx-py/src/presentation.rs` | `fdbc8edf264c52ec71e5c4b2bc7fb8b7951f89dc183743566556beee9b5f952a` |
| `crates/rpptx-py/src/shape.rs` | `28f0aaf81053665ce9bfdbdacdaeb25a72ac757ca27afb0e036bce847a80e5b8` |
| `crates/rpptx-py/tests/test_documented_examples.py` | `594b3af3ea4a6b694596935f50d8659ed41e7ec30a04aad9faeafa31c9654c9a` |

Only `.claude/reviews/F-X260-codex-takeover-pass-1.md` was written. No product, plan, state, progress, historical review, ledger or baseline was modified. No commit or push. Review stops here without remediation.
