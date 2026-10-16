# F-X238, all, pass 1

**Reviewed**: Working tree in /private/tmp/rdocx-s94-x238, branch work/f-x238-codex, against claimed Base a644dd9443346223e0003066bbfd33dc7937e3c7. The frozen diff contains21 files,933 insertions and112 deletions. This includes the root-authored later-plan prose correction from f65774b466997f326a2e705cc8f58618d1ce47a9 and the approved anchor-refusal clarification. F-X238 is in-progress. Reviewed the approved plan, progress record, cited HLD contracts, complete implementation/stub/test/documentation diff and available installed evidence. No source edits, builds, Cargo/cache mutation, commits or remediation.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Named picture refusal breaks the established exception family

`crates/rdocx-py/src/document.rs:8627`

The new width-only or height-only guard raises builtin NotImplementedError. The pre-existing Document.add_picture rejection is a named RdocxError, and callers catching that established package exception now miss the refusal. The unchanged regression at `crates/rdocx-py/tests/test_core.py:3411` explicitly catches rdocx.RdocxError for width-only insertion after an existing picture. Source inspection confirms the new guard intercepts that call before the existing native error mapping. The worker's concurrent complete installed suite also reports this failure. Preserve the established RdocxError family while retaining the actionable width-and-height pointer and atomic refusal. The plan requires retaining existing behavior and native admission at `.claude/plans/F-X238-design.md:71`. Do not change the pre-existing core test to accept the regression.

### D2, Runtime section-module alias does not provide the typed import path

`crates/rdocx-py/python/rdocx/enum/__init__.py:34`

The same-object runtime alias registered at line36 makes `from rdocx.enum.section import WD_SECTION, WD_ORIENT` work when Python executes imports, but static consumers cannot resolve that module. Independently checked the actual installed candidate using its Python with `-B -m mypy --strict --no-incremental --cache-dir=/dev/null -c` and that exact direct import. Mypy exits1 with `Cannot find implementation or library stub for module named "rdocx.enum.section" [import-not-found]`. No cache was written. The current green smoke test at `crates/rdocx-py/tests/typing_smoke.py:562` imports only enum.text and therefore does not qualify the newly supported section import.

The approved import surface is at `.claude/plans/F-X238-design.md:66`, with changed-surface strict typing and installed qualification at line125. Resolve or explicitly reconcile this typed-consumer contract before completion, and qualify the exact installed import without missing-import ignores or replacing typed values with Any. The separate section.py file exception remains unapproved under line164. This finding does not grant permission to create that file or module.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- Correctness: no additional defect in heading/style resolution, direct-body insertion, page-break staging, stream byte/type and short-write checks, run font delegation, highlight mapping or frozen-snapshot behavior. Native to_bytes serializes a staging clone, so stream-write refusal does not publish a partially serialized document.
- Contract: no additional scope expansion. Native template, image sizing, unit and rendering semantics remain unchanged. Unsupported section, list, table-column and anchor requests retain explicit alternatives. The three original divergence tokens remain remove_content, set_style and slides.remove. The section alias respects the existing-file permission boundary, subject to D2's typed-import gap.
- Panics: 0 findings. New color indexing is checked, heading levels are bounded, and the existing inserted-paragraph invariant remains guarded by successful native insertion.
- OOXML: 0 findings. New bindings reuse existing native style/run/table/picture providers and prepared publication. No new parser, schema reordering or raw-XML stripping route was introduced.
- Tests: no additional finding beyond D1's existing compatibility gate and D2's missing exact typed-import control. The added controls inspect saved expanded names, pinned structural output, source/owner atomicity, tuple snapshots and independent refusal classification. Immutable raw corpus outcomes and the historical passing-merge divergence remain separate from requirement dispositions. Running or externally blocked checks are not counted as successful verification by this review.
- Structure: 0 findings. No product file, module object, dependency, feature, trait or generic was added. The private tuple subtype owns string-index refusal, and the table-style resolver has two current callers. Stub/private export boundaries otherwise match the intended surface.

The review pass ends here. Return the tracked write lease for separate remediation and a numbered follow-up review.
