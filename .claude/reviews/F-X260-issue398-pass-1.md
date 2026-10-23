# F-X260, issue398, pass 1

**Reviewed**: fresh independent `/microscope F-X260 --working`, aspect issue398, on `sprint/s95.3` against HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. The ten requested files contain 1196 insertions and 113 deletions, 1309 changed lines. These raw totals include the expressly excluded Issue400 hunks in the two shared files.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. A10 is not satisfied.
**Reviewer**: Codex in this audit session, without delegated agents or implementation changes.

This is the focused Issue398 review, not another sprint review. No feature, issue or sprint closes.

## Authority and exclusions

Read AGENTS.md, CLAUDE.md, canonical WORKFLOW and microscope, the full active F-X260 design and progress record, and the previous clean binding-cost pass 3. Confirmed F-X260 remains in-progress in CURRENT_SPRINT and BACKLOG. Read the cited HLD publication, lifetime and testing sections, plus applicable canonical risk-routing, design contract-closure, verification provenance and differential-testing guidance.

The active plan is the contract. The two doc-hidden native read projections are explicitly approved additive methods using existing types. PR399 is candidate input, not approved evidence or authority. No new Python API, trait, document wrapper or second owner registry is authorized. This audit checks current source against the existing native single readers and current publication protocol. Constructed packages are local controls, not native Word evidence.

Issue400 selector and range correctness is outside this verdict. Excluded its five Python parameter cases and TOC paragraph in HLD12. Also excluded the other dirty F-X259 plan, field, native regression and rendering-spec changes. Inspected `rebuild_toc` only as a candidate publisher that must refresh F-X260 facts. Inspected unchanged callers, including run and formatting code, to assess the new cache contract. The prior binding-cost verdict is historical at its recorded source, not blanket approval of this increment.

## Defects

### D1, direct run tab and break mutations leave accepted paragraph text stale

`crates/rdocx-py/src/paragraph.rs:931`

The new text getter returns the owner inventory's cached string after scope validation. `cached_paragraph_text` checks the owner token but has no content-freshness check (`crates/rdocx-py/src/document.rs:6180`). Existing `PyRun::edit` installs a changed native candidate directly without refreshing that inventory (`crates/rdocx-py/src/run.rs:167`, assignment at line 176). Both `Run.add_tab` and `Run.add_break` use this publisher (`crates/rdocx-py/src/run.rs:347`, `crates/rdocx-py/src/run.rs:387`). They change accepted text while intentionally preserving live handles and revision scopes.

Concrete public reproduction, executed against the authenticated installed extension:

```python
import rdocx

doc = rdocx.Document()
paragraph = doc.add_paragraph("A")
run = paragraph.runs[0]
run.add_tab()

assert run.text == "A\t"                 # actual, passes
assert paragraph.text == "A\t"           # actual is "A"
assert doc.paragraphs[0].text == "A\t"   # actual is "A"
```

Expected: both held and freshly obtained paragraph handles read `"A\t"`. Actual: both read `"A"`, while native run text and paragraph XML include the appended tab. Save/reopen reads `"A\t"`, so the live cached projection disagrees with both native content and saved content. Replacing `add_tab()` with default `add_break()` produces the same defect with expected `"A\n"` and actual `"A"`.

Independently reproduced both operations in body, table cell, nested table cell and header paragraphs. All eight combinations returned old text from held and fresh paragraph handles. An in-memory `save(BytesIO())` did not repair those values. The run getter returned the updated text, and paragraph XML contained the new `w:tab` or `w:br`. The first body probe also checked reopened text directly. This is a current product failure, not a missing dependency, wrong test expectation or release-timing failure.

The cause is the new cache fast path combined with a pre-existing direct content publisher. Native reads formerly recomputed accepted text. Owner identity alone cannot certify unchanged content across these calls. Repair must make these direct accepted-text mutations update or invalidate the corresponding read facts while preserving their documented live-handle behavior. A lifecycle regression must exercise these callers, rather than only `Run.text` or `Paragraph.text` publishers. The nine supplied lifecycle variants do not cover them (`crates/rdocx-py/tests/test_core.py:10014`).

## Smells

None, 0.

## Nitpicks

None, 0.

## Publication and projection audit

Traced inventory construction, all native candidate assignments in the Python source, and direct mutation callbacks by behavior, rather than only callers of the new helpers.

| Boundary | Assessment |
|---|---|
| Initial construction | `from_document` prepares admitted owner addresses, read facts and complete indexes before exposing the document. Batch projections are eager, not filled by the first handle call. |
| Full publication | `prepare_word_revision` builds fresh owner facts and indexes from the candidate. Commit moves prepared state, then installs the candidate. Paragraph/run text and XML, table/cell XML, append, structural row operations, range operations, imports, control/form operations, revisions, field updates, comparison, related-story edits and signing reach this preparation boundary. |
| Property relocation | `adopt_property_edit` derives new facts from the candidate, preserves proved scopes and prepares replacement indexes before assignment. Relocated owners copy the new address's facts, not the old cached text/counts. |
| Local plain paragraph XML | The submitted paragraph is parsed and its next accepted text is prepared before the native call. Only successful publication installs that text. No-op, local refusal and staged fallback do not install speculative facts. Native admission still receives the original XML. |
| Tail removal | The shortcut requires a final plain paragraph, contiguous descendants and section-only suffix. It removes route/generation/parent entries for the drained range, pops the last facade paragraph projection and adjusts surviving section indices. The removal epoch preserves the original stale cause across later numeric-slot reuse. |
| Direct mutation | Run tab and break content changes bypass cache refresh, D1. Run/font/paragraph properties, table style, document styles/defaults/numbering, bookmarks, hyperlink URL, image replacement/size, metadata and settings were also examined for changes to accepted text, counts or route axes. No additional cache-changing bypass was established. Their changed fingerprints must still be refreshed through native mutation providers. |
| Save | Native `to_bytes` and ordinary save stage output on a clone. They do not replace the live owner inventory or repair D1. In-memory save was exercised. No filesystem save was performed. |

Route hashes exclude fingerprints consistently with `same_route`. Hash matches still require exact route comparison, and bucket order preserves the original first match. Generation lookup checks `same_state`, not generation number alone. No authority from matching text or a hash alone was found. Prepared captures retain the checked native path. Read caches are not serialization inputs.

Body projection source inspection compared boxed paragraph pointers, direct-body slots, direct control-paragraph ordinals and missing nested routes with `paragraph_story_location`. Cell batch inspection compared modeled pointers, direct-container selection, per-container physical cell ordinals, stopping at an unroutable earlier owner, source-order sorting and count mismatch fallback with `cell_paragraph_story_location`. The single reader's extra snapshot readback is supplied by admission of paragraph source into the binding inventory. No additional batch/single mismatch was established. The review did not compile a native bridge or execute another native parity test.

Table counts are optional facts parsed from namespace-complete table snapshots. Unsupported standalone parsing keeps native count reads. Parent joins retain physical row/cell coordinates. Cached cell paragraph routes require agreement with the modeled paragraph count. Unsupported body/nested facade routes keep their original snapshot/native fallback, rather than receiving invented physical authority. Nested-table enumeration was compared with the original story-item policy, including controls that remain outside ordinary table items.

## Independent public probes

Only bounded in-memory probes ran, using `PYTHONDONTWRITEBYTECODE=1 /tmp/s95.3-rpptx-venv/bin/python -B`. No probe script, package or output file was written. No build, suite, benchmark or external oracle ran.

Actual imported extension path:

`/Users/atulsharma/Documents/projects/tensorbee/rdocx/crates/rdocx-py/python/rdocx/_rdocx.abi3.so`

SHA-256 before probes, across subsequent invocations, after completed batches and at final authentication:

`a03ef93737fc13756e32d9fee8d6efc48dc3d134b1c68d1114f73dc67880a934`

Before probing, inspected `/tmp/s95.3-new-intake-dev-final.log`. Its final child records successful dev compilation, wheel creation and editable installation of rdocx. This ties probes to the actual imported artifact and observed successful installation. The association with frozen source relies on that install record and the user's source-freeze instruction, not embedded source attestation or release qualification.

Observed passing controls, individually:

- Cell paragraphs with controls first, between direct paragraphs, and nested controls read their explicit source text in order. Paragraph XML agreed for the supported routes in that initial three-context batch.
- Body direct controls, nested-control legacy reads, duplicate text and zero paragraphs returned expected values with unchanged serialized bytes. The nested unsupported paragraph XML route still refused.
- Imported controlled rows and controlled cells preserved the facade's direct physical row/cell axes. Grid-span and omitted-leading-grid cases kept physical cell counts. Cached paragraph text agreed with native `cell.text` in these four table contexts.
- Paragraph property and run formatting changes followed by `Run.text` refreshed text correctly. Exact paragraph XML no-op and invalid-child refusal preserved current text and bytes.
- Tail removal preserved fresh survivor text, retired held handles with `Document.remove_content` as first cause, and did not resurrect them after append. In-memory save/reopen retained expected text.
- Cell property relocation preserved a held paragraph and run. A subsequent held-run text edit refreshed the cell paragraph read.

Exploratory stops are not reported as passing batches. One compound imported-body probe stopped because it incorrectly requested XML authority for an unsupported nested-control facade paragraph. The corrected control explicitly observed that refusal. A later compound table probe stopped because its expected outer-cell text omitted text that the existing control traversal includes from a controlled nested table. A follow-up printed matching native cell text and paragraph values, then stopped on an incorrect assumption that ordinary `cell.tables` exposes that controlled table. Source inspection of the old story-item filter confirms this exclusion was already present. These observations do not establish another regression, and no product expectation was changed in the repository.

## Supplied evidence and measurement limits

Consumed the named logs without rerunning their commands:

| Log | Actual recorded result |
|---|---|
| `/tmp/s95.3-issue398-native-parity.log` | Native batch/single parity case, 1 passed. Inspected its direct, controlled, nested and multi-paragraph inputs and exact location comparisons. |
| `/tmp/s95.3-new-intake-binding-units.log` | Binding union, 10 passed. |
| `/tmp/s95.3-issue398-whole-walk-work.log` | Two targeted work/collision cases passed. |
| `/tmp/s95.3-issue400-python.log` | 14 passed, comprising nine Issue398 lifecycle variants and five excluded Issue400 variants. |
| `/tmp/s95.3-issue398-word-core.log` | Actual full core result: 414 passed, 7 skipped, 2 failed. Failures identify wrong Poppler 26.09 and missing python-docx. This remains a failed full run. |
| `/tmp/s95.3-resume-restored-dependency-tests.log` | Nine affected cases passed after the reported dependency restoration. This is not a newly executed full suite or a replacement aggregate result. |
| `/tmp/s95.3-new-intake-final-clippy.log` | rdocx and rdocx-py completed successfully. The user identifies all-target/all-feature scope. No linter ran during this review. |
| `/tmp/s95.3-issue398-dev-red.log` | All three reporter handle rows failed on the earlier dev implementation. |
| `/tmp/s95.3-issue398-dev-calibration.log` | Later dev reporter rows print passing observations. These are diagnostic, not release acceptance. |

The restored python-docx 1.2.0 and checksum-source-built Poppler 26.01 provenance is supplied execution context. This audit did not invoke either tool or independently rebuild dependencies. The initial nested-cell lifecycle expectation omitted the trailing empty paragraph left by nested-table insertion. Its expectation-only correction is not evidence of a product defect.

Inspected what the work test measures. The current walk reads 50/400 body paragraphs and 24/192 cell paragraphs. It compares bounded selected calls, asserts total work per read and exact eightfold growth for eightfold input. Charges include actual owner-index preparation traversal, route-bucket comparisons and selected generation lookups. Source-wide fallback entry points charge document content count. Disabling the body projection exercises that fallback and exceeds the bound. This is a discriminating internal negative control, not execution of historical old source.

The meter does not count every native parser, serializer, allocation, hash operation, ancestor-scope traversal or source byte. Its fallback charge is a coarse source-wide work indicator, not a count of each native loop iteration. The test establishes the asserted supported-route controls, not universal allocation bounds or release ratios. Initial eager projection construction is outside its resets and remains a separately reportable setup cost. There is no newly executed initial-preparation timing in this review.

The old dev artifact was neither retained nor hashed. Its log is historical diagnostic evidence with limited artifact provenance. Do not attribute its timings to the currently authenticated extension. Neither dev log supplies release acceptance, and a passing timing script cannot clear D1.

## Explicit zero categories and pending acceptance

**Correctness: 1 defect**, D1. **Contract: the same D1 violates A10**, not a second finding. **Panics: 0 findings.** Indexed expectations and fixed-kind assertions were examined against preparation and local removal invariants. **OOXML and raw compatibility: 0 additional findings.** This read acceleration does not replace native XML admission, change child order or serialize cached facts. Unsupported routes retain fallback. **Tests: 0 separate findings**, with the uncovered D1 caller cases and measured limits described above. **Structure and API: 0 findings.** No new trait, generic, wrapper, Python method or owner registry was introduced. The two approved native methods have concrete existing binding consumers. **Smells: 0. Nitpicks: 0.**

A8 remains pending for all three exact installed release ratios below 3, including cold calls, late table selection, raw samples and setup cost with source/build identity. A9 has focused native parity and constructed public controls, not universal native compatibility certification. A10 is blocked by D1. All seven Issue389 strict release ratios also remain pending at final source. Do not close any acceptance row from the counter results or dev timings.

Authentic native Word, the qualified font bundle and final integrated suites, unchanged hash harness, WASM, packaging and all matched risk riders remain pending at their approved final boundary. No external oracle was substituted, no release threshold was relaxed and no previous failure was erased.

## Source authentication and stop boundary

All ten complete file hashes matched `/tmp/s95.3-issue398-review-source.json` before inspection and again after probes, immediately before writing this report. HEAD and pre-existing dirty source paths remained unchanged. Full before/after values:

| File | SHA-256 before and after |
|---|---|
| `.claude/plans/F-X260-design.md` | `028ea93ae9e8892dcbe53ae1cd44423380699bfd99d02061b33674c9f1f4447c` |
| `crates/rdocx/src/document.rs` | `11690682040a8a4e8c961ba415ed88ff92b9c0f7245b0d9fbab7bc3259de1ebe` |
| `crates/rdocx/tests/integration_test.rs` | `8cb8887bed42abed47e979cd6b27790028bc060a0a1d8cb5088eacf6a13065cc` |
| `crates/rdocx-py/src/document.rs` | `4f659f03f83735107338d08549f4c7cd57716b3c24986a0f90f299e7c8658459` |
| `crates/rdocx-py/src/paragraph.rs` | `84144e98f1920ade2d3296d3eab5a823a14208be6a1d3bd8de293a159ced6fb8` |
| `crates/rdocx-py/src/table.rs` | `c41cdeba561b65056a2c13e57ab08006a4f610b7c06072a7e34f72b83f14e79c` |
| `crates/rdocx-py/tests/test_core.py` | `fa79ec59d9ea997a533b4d6ef151bef3dd6b79838f068df36b502602477e9ee1` |
| `docs/hld/03-architecture.md` | `08b5e4f2c3f435c0e9a1c971ee427e327583aa9fe63a56c5f434a7004ed5bd88` |
| `docs/hld/10-bindings-spec.md` | `f169080758ff00a4f1e829f2a42b276ff89d09b2f1bc29497b8713420e3a092c` |
| `docs/hld/12-testing-strategy.md` | `06aff54f197efbd30d799f201503c369111a522868036f69e0f8c12fbcb2307a` |

Only `.claude/reviews/F-X260-issue398-pass-1.md` was written. No source, plan, progress, state, ledger, other review or adapter changed. No commit, agent, build, suite, benchmark or external oracle was invoked. The pass ends here without remediation.
