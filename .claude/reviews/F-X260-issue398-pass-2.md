# F-X260, issue398, pass 2

**Reviewed**: fresh independent `/microscope F-X260 --working`, aspect issue398, branch `sprint/s95.3`, against HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. Eleven scoped files, including run.rs, contain 1315 insertions and 115 deletions, 1430 changed lines. Raw totals include the excluded Issue400 hunks in shared files.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. D1 remains open on a different imported control input. A10 is not satisfied.
**Reviewer**: Codex in this audit session, without agents or implementation changes.

This is a focused feature microscope, not another sprint review. No feature, issue or sprint closes.

## Authority and exclusions

Read AGENTS.md, CLAUDE.md, complete canonical WORKFLOW and microscope command, full active F-X260 plan and progress, actual Issue398 pass1 and previous clean binding-cost pass3. Confirmed F-X260 is in-progress in CURRENT_SPRINT and BACKLOG. Read the cited publication, handle-lifetime and binding-test HLD sections and applicable risk-routing, design closure, verification provenance and differential-testing reference material.

The current approved plan, including its D1 publisher closure, is the contract. Its two doc-hidden native projections are approved additive pre-1.0 methods with existing binding consumers. PR399 algorithms and fixtures remain candidate input. Neither candidate code nor self-generated input establishes compatibility by itself. No new Python API, trait, document wrapper or second owner registry is authorized by this increment.

The original `/tmp/s95.3-issue398-review-source.json` records pass1 source. Five of its ten entries differ now: plan, binding document, Python tests, HLD03 and HLD10. They are not falsely reported as matches. The pass2 addendum and `/tmp/s95.3-issue398-pass2-source-manifest.json` identify the frozen repair, added run.rs and current extension. All 25 entries in that manifest, including excluded dirty files and the extension, matched before source audit/probes and again immediately before this report. Source/build association relies on the successful install record and the user's freeze, not embedded compiler source attestation.

Issue400 selector/range correctness, its five Python cases and TOC paragraph in HLD12 are excluded. Other dirty F-X259 plan, process/adapters, field, regression and rendering-spec changes are outside this verdict. TOC and other field callers were inspected only for publication of F-X260 facts. Unchanged native table and OOXML accessors were inspected to explain actual mutation coordinates, not to conduct a separate native-feature review. The earlier clean binding-cost verdict remains historical at its own source.

## Defects

### D1, run refresh can update the selected cache while another live paragraph actually changes

`crates/rdocx-py/src/run.rs:176`

The first repair rereads the candidate using the original facade location, then updates only the captured run owner's paragraph (`crates/rdocx-py/src/document.rs:6214`). It does not establish that this is the physical paragraph the mutation changed. Scope and parent-kind validation prove current identity, not read/write coordinate equivalence.

Concrete accepted input: one body table with one cell containing a block content control whose paragraph is `controlled`, followed by a direct paragraph `after`. Both paragraphs have one ordinary run. Equivalent XML for the cell payload:

```xml
<w:sdt><w:sdtContent><w:p><w:r><w:t>controlled</w:t></w:r></w:p></w:sdtContent></w:sdt>
<w:p><w:r><w:t>after</w:t></w:r></w:p>
```

Import this in a table with one grid column, one row and one cell through `Document.from_bytes`. The bounded public reproduction after import is:

```python
cell = document.tables[0].cell(0, 0)
controlled, after = cell.paragraphs
run = controlled.runs[0]
run.add_tab()
run.add_break()

print(after.text)                       # actual: "after"
print(cell.paragraphs[1].text)          # actual: "after"
print(after.runs[0].text)               # actual: "after\t\n"
print(cell.text)                        # actual: "controlled\nafter\t\n"
```

The held `after` paragraph XML also contains both `w:tab` and `w:br`. `save(BytesIO())` leaves held and fresh paragraph text at `after`. Reopening those bytes produces paragraph texts `["controlled", "after\t\n"]`, while current live reads still produce `["controlled", "after"]`. Expected for the new cache: every live paragraph read agrees with its current native paragraph, XML and saved/reopened content. Actual: the changed second paragraph remains stale. No scope exception or dependency error occurs.

The coordinate cause is inspectable. Cell `ParagraphLocation::edit` uses `cell.paragraph_mut(paragraph)` (`crates/rdocx-py/src/paragraph.rs:273`). Native immutable cell paragraph access includes controlled paragraphs, while mutable cell paragraph access excludes content controls (`crates/rdocx-oxml/src/table.rs:2154`, `crates/rdocx-oxml/src/table.rs:2161`, consumed by `crates/rdocx/src/table.rs:2291`). Thus facade paragraph zero reads `controlled` but mutates direct paragraph `after`. The new candidate reread still sees unchanged `controlled`, and the publisher refreshes that owner's cache. The second owner's fact is never refreshed, and its getter returns the old string (`crates/rdocx-py/src/paragraph.rs:931`).

The native mutable-coordinate discrepancy predates this increment. It is not counted as a second finding. The incremental defect is the new stale cache on the paragraph that actually changed. Before caching, that paragraph's native immutable read would expose the change. This last statement is a source counterfactual, not a newly executed old-source result. D1's original plain body/cell/nested/header cases are repaired, but its freshness invariant is not closed. Repair must prove the actual affected physical owner and keep every changed fact coherent while preserving documented handle lifetime and native refusal semantics. The twenty supplied run-context cases lack this controlled-cell composition (`crates/rdocx-py/tests/test_core.py:10131`).

## Smells

None, 0.

## Nitpicks

None, 0.

## Publication and projection audit

Searched every native candidate assignment and direct mutation route across the Word Python source, including publishers not using the repaired helper. Traced native providers where needed for text, counts, coordinates or fingerprints.

| Boundary | Assessment |
|---|---|
| Initial construction | `from_document` constructs admitted addresses, accepted-text/count facts and complete indexes before exposing handles. Body/cell batch preparation is eager. First handle lookup does not populate a lazy package inventory. |
| Full replacement | `prepare_word_revision` derives new facts and indexes from the candidate before scope commit. Commit moves prepared state, followed by candidate assignment. Paragraph/run text and XML, append, table/cell XML, grid/row changes, merge/split, range edits, imports, templates, control/form operations, revisions, comparison, related-story operations, signing and field/TOC updates reach this boundary or its common publication helpers. |
| Property relocation | Table, row and cell property callbacks stage. `adopt_property_edit` obtains fresh candidate facts, proves relocation and prepares indexes before assignment. Failed correspondence uses ordinary publication. Fresh facts come from the new address, not the previous owner's cache. |
| Local plain XML | Parsed next accepted text and selected successor scopes precede native admission. Original submitted XML still reaches native validation. Successful publication installs the prepared fact. Exact no-op, refusal and staged fallback do not install speculative facts. |
| Tail removal | The shortcut requires the final plain direct-body paragraph, contiguous descendants and section-only owner suffix. It removes local generation/route/parent entries, pops the body paragraph projection and shifts surviving section index entries. The removal epoch prevents resurrection and retains the first cause after numeric-slot reuse. |
| Run tab/break | Candidate edit and candidate reread precede publication. Authority validation and string preparation are before assignment, with no late native read or scope retirement. Correct on supplied plain and paired contexts. D1 shows why selected-parent validation is insufficient in an accepted imported cell. |
| Other direct mutations | Font/run/paragraph properties, table style, settings, core/application/custom properties, defaults/styles/numbering, bookmark markers, hyperlink URL and image bytes/size were examined. These do not introduce a separate accepted-text/count/route-changing cache bypass in this increment. Native mutation refresh remains responsible for current fingerprints. |
| Save | Stream save and `to_bytes` stage output without replacing the live owner inventory. They do not repair D1. No filesystem save was executed. |

Route hashing and `same_route` omit fingerprints consistently. Hash buckets still require exact route comparison, in original first-match order. Generation lookup checks `same_state`, not only a generation number. Matching text, hashes or numeric coordinates do not independently authorize capture. Current indexed locations retain admitted owner routes, and native mutations retain fingerprint refresh. Cached facts are not serialization inputs.

Inspected both batch methods against the original single lookups. Body matching uses actual boxed paragraph addresses, direct-body slots and direct control-paragraph ordinals, preserving missing nested routes. Cell matching uses modeled cell pointers, enclosing direct containers, physical cell ordinals, early unroutable-owner stopping, source-order paragraph sorting and count correspondence. The original single cell reader's extra snapshot read is supplied by binding paragraph-source admission before a cached route can capture an owner. No additional batch/single route mismatch was established. No native bridge was built or new native parity test executed.

Table count facts use namespace-complete snapshots and optional checked standalone parsing. Unsupported parsing retains native count reads. Cell joins use physical row/cell axes and require paragraph-count agreement before using cached paragraph routes. Nested table enumeration follows the original story-item selection policy. Unsupported facade routes keep their native/snapshot fallback and refusal. These checks do not cure D1's distinct read/write coordinate mismatch.

## Independent bounded public probes

Ran only in-memory public probes with bytecode disabled using `/tmp/s95.3-rpptx-venv/bin/python -B`. Helper function definitions were read from the existing test source and evaluated in memory, without importing or executing its test suite. No scripts, packages, probe logs or documents were written.

Actual imported extension:

`/Users/atulsharma/Documents/projects/tensorbee/rdocx/crates/rdocx-py/python/rdocx/_rdocx.abi3.so`

SHA-256 on all invocations and after completed batches:

`7b7c0d09753939973384ffdc7fc95661b44c08f50df5e74dc231fffeefd48a67`

Before probes, `/tmp/s95.3-issue398-pass2-dev-build.log` recorded successful dev compilation, wheel construction and editable rdocx installation. This is the successfully installed artifact, not the preserved pre-repair release wheel.

Observed passing controls:

- Drawing-only, VML-only and paired text boxes each expose one logical paragraph. Two independently obtained paragraph aliases and the held run agree after tab, font-bold edit, column break and another tab. Held font remains usable. Physical XML has the expected tab/break multiplicity, including both paired copies. Stream save and reopen agree. Paired physical copies do not imply two independent public paragraph facts in this fixture.
- A body block control followed by a direct paragraph preserves the correct selected text through the same sequential tab/line edit. This positive control distinguishes the native body accessor from the failing cell accessor.
- Exact paragraph XML no-op retains a held run. Invalid-child XML refusal preserves bytes and text. Local plain XML changes held/fresh text. Tail removal preserves fresh survivor text and retires the removed handle with its original `Document.remove_content` committed-revision cause after append. Cell property relocation followed by a held-run tab edit keeps held/fresh/native text equal.
- The controlled-cell reproduction above independently establishes D1 through native run text, paragraph XML, cell text, held/fresh cache reads and save/reopen.

Exploratory stops remain explicit. The initial compound text-box/cell probe completed all three text-box controls, then failed a cell expectation and led to D1. An initial combined body/cell reproduction incorrectly expected the body accessor to share the cell discrepancy and stopped on the body assertion. The corrected control establishes body success and cell failure separately. A tail probe initially compared the entire stale-error string across append, overlooking the legitimately changed current-revision number. The corrected probe checks the unchanged first-cause clause. None of these stopped invocations is described as a wholly passing batch, and no repository expectation was edited.

## Supplied evidence and its limits

Consumed completed actual results without rerunning any command:

| Artifact | Actual observation |
|---|---|
| `/tmp/s95.3-issue398-native-parity.log` | 1 native batch/single parity test passed. Inspected exact location comparisons on direct, controlled and nested input. |
| `/tmp/s95.3-new-intake-binding-units.log` | 10 binding unit tests passed. |
| `/tmp/s95.3-issue398-whole-walk-work.log` | 2 selected work/collision tests passed. |
| `/tmp/s95.3-issue400-python.log` | 14 passed, comprising 9 Issue398 lifecycle cases and 5 excluded TOC cases. |
| `/tmp/s95.3-issue398-run-refresh-red.log` | Actual pre-repair matrix: 15 failed, 1 passed, including the passing header-page refusal. |
| `/tmp/s95.3-issue398-run-refresh-green.log` | Initial repaired union: 31 passed. |
| `/tmp/s95.3-issue398-run-all-contexts-green.log` | Expanded run-context matrix: 20 passed. |
| `/tmp/s95.3-new-intake-final-focused.log` | Completed final focused result: 34 passed, 414 deselected. This includes the 5 excluded TOC cases. |
| `/tmp/s95.3-issue398-word-core.log` | Actual full run: 414 passed, 7 skipped, 2 failed. Failures report missing python-docx and Poppler26.09 instead of26.01. It remains a failed full run. |
| `/tmp/s95.3-resume-restored-dependency-tests.log` | 9 affected cases passed after the reported restoration of python-docx1.2.0 and checksum-source-built pinned Poppler26.01. No new full-suite aggregate is inferred. |
| `/tmp/s95.3-new-intake-final-clippy.log` | rdocx and rdocx-py completed successfully. All-target/all-feature scope is supplied execution context. |
| `/tmp/s95.3-issue398-dev-red.log` | Earlier dev reporter script failed all 3 handle rows. |
| `/tmp/s95.3-issue398-dev-calibration.log` | Later dev script prints passing diagnostic rows. These are not release acceptance. |

The original pre-index dev artifact was not retained or hashed. Its timing provenance remains limited. The later pre-run-repair extension and release wheel were preserved according to the progress record, but neither is the current measured or qualified release artifact. No old diagnostic result is attributed to the current extension. The earlier lifecycle assertion omitted the trailing empty outer-cell paragraph after nested-table insertion. Its expectation-only correction was not a product failure.

Inspected the work instrumentation and assertions. The walk covers 50/400 body paragraphs and 24/192 cell paragraphs. It asserts equal bounded per-handle work, total work bounded by reads and exact eightfold work for eightfold input. Actual charges cover owner/index preparation loops, selected generation lookups and route-bucket comparisons. Source-wide fallback entry points charge `content_count`. Disabling a body projection exercises that fallback and exceeds the bound. This is a useful internal negative control, not historical old-source execution.

The meter does not count every native parser, serializer, allocation, byte, hash, rehash or ancestor-scope traversal. Its fallback charge is coarse. Initial eager preparation is outside the measured resets and remains a separately reportable setup cost. This audit executed no preparation timing or benchmark. Neither instrumentation nor dev-script output qualifies release ratios or eliminates D1.

## Explicit zero categories and pending acceptance

**Correctness/publication: 1 defect**, D1. **Contract: the same D1 violates A10**, not another finding. **Panics: 0 findings.** Private fixed-kind/index expectations were checked against preparation and removal invariants. **OOXML/raw compatibility: 0 additional findings.** Original XML still reaches native admission, cached facts do not serialize, and no new schema order, namespace authority or opaque-source loss was established. **Tests: 0 separate findings**, with the missing controlled-cell composition and work-meter limitations recorded above. **Structure/API: 0 findings.** No new Python API, trait, generic, wrapper or second registry. **Smells: 0. Nitpicks: 0.**

A8 retains all three strict installed-release ratios below3 on the exact workloads, cold and repeated calls, late-table selection, raw samples, setup cost and source/build identity. A9 has focused native parity and constructed public controls, not universal compatibility qualification. A10 remains blocked by D1. All seven Issue389 release ratios also remain pending at final source. No release ratio is qualified by this pass.

Authentic native Word, the qualified font bundle, final integrated product suites, unchanged hash harness, WASM, packaging and all matched risk riders remain pending at their approved final boundary. No external oracle ran, no threshold changed and no failed result was erased. Separate F-X259 workflow refinements do not establish product correctness.

## Full-file authentication and stop boundary

All pass2 manifest entries matched before and after inspection/probes. HEAD remained unchanged. The eleven scoped complete-file SHA-256 values follow, identical at both authentication points:

| File | Before and after SHA-256 |
|---|---|
| `.claude/plans/F-X260-design.md` | `f1a3c1eab3cd3f38b81092cdbdad000ae519dfd503a1e52f490ee44693ca36a5` |
| `crates/rdocx/src/document.rs` | `11690682040a8a4e8c961ba415ed88ff92b9c0f7245b0d9fbab7bc3259de1ebe` |
| `crates/rdocx/tests/integration_test.rs` | `8cb8887bed42abed47e979cd6b27790028bc060a0a1d8cb5088eacf6a13065cc` |
| `crates/rdocx-py/src/document.rs` | `fe9786babe82b4b576cf577f081eaa077720007f2478ce4eb2977ab5302b11ee` |
| `crates/rdocx-py/src/paragraph.rs` | `84144e98f1920ade2d3296d3eab5a823a14208be6a1d3bd8de293a159ced6fb8` |
| `crates/rdocx-py/src/run.rs` | `ce6f38a7bacf76d8d0cea580ea56ce531338b8c1bcb3c45531bdde672b5ebdbf` |
| `crates/rdocx-py/src/table.rs` | `c41cdeba561b65056a2c13e57ab08006a4f610b7c06072a7e34f72b83f14e79c` |
| `crates/rdocx-py/tests/test_core.py` | `fa555753478aae7d990c1fe39e7ebbea1441e238980911f2768e9fb94352b337` |
| `docs/hld/03-architecture.md` | `5df1d0bbe193b6199b780f21d200ae5817b317a9e57449d6db1fe527204ec29b` |
| `docs/hld/10-bindings-spec.md` | `19c6828f7123f10e220223c565cfa41320993574ed6ea04c219ba7153430813e` |
| `docs/hld/12-testing-strategy.md` | `06aff54f197efbd30d799f201503c369111a522868036f69e0f8c12fbcb2307a` |

The actual probe extension hash also remained identical before/after, as recorded above. Original pass1 hashes remain historical and were not overwritten.

Only `.claude/reviews/F-X260-issue398-pass-2.md` was written. No source fixes, progress/state/ledger changes, other review edits, commits, agents, builds, suites, benchmarks or external oracles were performed. The pass stops here without remediation.
