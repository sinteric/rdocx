# F-X268, final performance, pass 21

**Reviewed**: Frozen working diff against `6542884725608c2baabf149dcc2496485d85b1d1` on `sprint/s95.4`. Eleven existing files, 1431 insertions and 333 deletions. Read canonical workflow/microscope guidance, approved plan, HLD 03/04/10/12, progress, pass20, frozen full diff and remediation receipts. F-X268 remains in progress under codex. All eleven source hashes and the binding artifact matched the frozen manifest.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass20 D5 and D6 are resolved. This is local source and evidence review, not final feature or sprint acceptance.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D5 resolution, real fallback outputs

`crates/rdocx/src/document.rs:6769` records actual output count and bytes by phase and provenance. `note_transaction_output` retains the aggregate qualified current-output count while `note_transaction_buffer` separately exposes normalized, copied and rewritten buffers. These are debug observations, not new persistent document caches.

The current writer at `crates/rdocx/src/document.rs:27953`, namespace replay result at line 27998, second current writer at line 43672, normalization writers at line 43666, retained probe copy and masked rewrites at lines 43686 through 43690, and masked occurrence writers at line 43695 now report their actual successful outputs. Inspected both namespace replay branches, including its copy when no declarations need replay. Canonical-record copies, normalized-opening copies, ordinary assembled output and no-record span writer output also have separate labels.

The production whole-proof fallback remains inside the active immutable read phase. Only the successful optional proof's audited debug reference comparison enters phase zero at `crates/rdocx/src/document.rs:8690`, restoring the prior phase afterward. The scope still ends before source mutation, preparation and reopen at line 43586. No production fallback was moved outside the window or hidden by the debug-reference pause. Error and short-circuit order remain unchanged.

The actual old-module regression failed with two reported current outputs against the required minimum of three. The authenticated current private-tab receipt reports four current outputs and visible normalized/masked/copy categories. The native force-off control at `crates/rdocx/src/document.rs:50967` checks the same production categories and the qualified control proves the debug reference stays excluded. The Python private-tab control at `crates/rdocx-py/tests/test_core.py:15064` exercises a genuine public changed edit and checks reopened content. Costly safety fallbacks are visible and are explicitly outside the qualified one-output budget.

## D6 resolution, actual packet use and index work

Preparation is counted after successful reservations at `crates/rdocx-py/src/document.rs:6727`. Consumption is counted after actual splice publication and its invariant check at line 6955. Mandatory final owner indexing and general-publication indexing use distinct labels on the real shared owner loop at line 4581. The reference-only rebuilt-index comparison is guarded, preventing reference work from masquerading as production work. Mandatory production indexing remains present at line 6894.

The four pinned original-407 cases at `crates/rdocx-py/tests/test_core.py:14968` retain six verbose formatted runs, 100/1000 paragraphs, cold/two-insert warm state and public index-10 insertion. They now require positive bounded classification, exactly one prepared and consumed packet, exact post-edit owner cardinality in mandatory indexing, and zero general-publication indexing. Returning no packet would fail these assertions. The six-consumer test at line 15011 independently requires a prepared/consumed packet and positive mandatory indexing with no general-publication indexing.

The corrected natural decline control at `crates/rdocx-py/tests/test_core.py:15110` starts with an actually reopened empty document. Its single section owner is classified but cannot supply a body-story owner. Public insertion therefore records zero prepared/consumed packets, zero mandatory reindex visits and exactly three general-publication index visits, with returned and reopened text checked. This demonstrates the fallback counter on the actual publication path.

The prior claim that an add-run mutation necessarily declines the packet is retracted. That fixture actually selected the packet. The coherent run's one failure was the incorrect negative-fixture expectation, not a product failure. The corrected named repository test passed against the unchanged authenticated native module. Existing 20/80 review-shaped all-six whole-body-work coverage remains.

Append paragraph/table survivors retain their original contract while held ordinary runs become stale. The changed test assertions reflect the authenticated legacy behavior and do not change the product stale-handle policy.

## Product and structural aspects with no findings

Correctness: zero findings. Rechecked the full owned transaction/read/splice batch and the remediation delta. The four consuming bridges share the checked mutation bodies and finalizer. Actual original package and invalidation state remain signature authority. Entire normal and bundled engine mutexes, including poison state, transfer through the common finalizer. No-op, mismatch, option validation, failure and outer Python publication behavior remain as reviewed in pass20.

Contract and error ordering: zero findings. Temporary `PreparedMainRead` still borrows the current immutable document. Related-source load order, selection, complete admission, private/model/frame/resource guards, producer-source versus writer authority and force-off behavior remain intact. Extracting the current serialized buffer for instrumentation preserves its prior evaluation and failure order.

OOXML: zero findings. No namespace authority, schema ordering, retained XML preservation, private-field admission or source/writer equivalence condition was broadened by remediation.

Panics: zero findings in the changed production paths. Counter indices are fixed internal call-site values. Indexing, scope sizing and prepared inserted-owner slices retain their prior checked preparation and commit invariants.

Structure: zero findings. No new source file, module, trait or persistent cache field. The redundant borrows were removed and the displaced complexity annotation restored. The additional counter argument labels actual shared index work rather than bypassing it. All six public splice callers still retain their fallible preparation, eager scope/index publication and existing handle policies.

## Evidence and limits

Consumed actual logs and receipts, without rerunning builds or tests:

- Previous full batch: 65 native passed and one unchanged external Word oracle ignored.
- Remediation native batch: 9 passed, 0 failed in 1.51 seconds.
- Remediation Python batch: 18 passed and one incorrect negative-fixture failure. Corrected named repository test: 1 passed, 626 deselected in 1.73 seconds.
- Scoped clippy exited zero. Hash harness matched all 49 entries.
- Actual prior-module D5 RED failed on the current-output assertion. Earlier mechanism and pinned six-run RED evidence remains as recorded in pass20.

Current module: `/private/tmp/s954-pass20-green-site/rdocx/_rdocx.abi3.so`, SHA-256 `6e000125acf5dc64ee32e52f44ca28c933f2b5125a54cd15b292b1a97e03f22b`. The final correction changed Python tests and prose only, with no product rebuild required. The frozen manifest authenticates the current product source used for the module.

No new final timing, full A9, memory, native Word, release qualification or feature completion is claimed. The remaining integrated qualifications remain pending. No source, tests, HLD or progress files were modified by this reviewer.

## Frozen source authentication

| File | SHA-256 |
| --- | --- |
| `.claude/plans/F-X268-design.md` | `f9c1f5a5787c940f8d6e4d898414e417a82b430f79e69710dbf1a810c21831d7` |
| `crates/rdocx-py/src/document.rs` | `0db110ae37de1674beb034dc1785889fbdd7996812d17d3c0ae95a277bd11419` |
| `crates/rdocx-py/src/lib.rs` | `f88ec41242ec80245602131b2dc087b5d4e028b62f71dd2a0c7f3c9675da8cbc` |
| `crates/rdocx-py/src/paragraph.rs` | `d5ff8477d55c87599275850e18bd0c3290e30badb90a0e21d9d66b3da03244de` |
| `crates/rdocx-py/src/table.rs` | `c81a8e8a8f0f4f9fa695bcd4d27638174571f2532709c1caf5243a590ca0dfc1` |
| `crates/rdocx-py/tests/test_core.py` | `ac7c9c5a3a263a4079ea35766503c2eb722ad5db805b885623f105d6c2ce0350` |
| `crates/rdocx/src/document.rs` | `d83d859b76dccb0b272702037aeeb3cab91906718dc41668336efea9363f4737` |
| `docs/hld/03-architecture.md` | `addf8e5f10bdccb06ec19257bb817539c256ba1f820cbee81144ff3b033fe831` |
| `docs/hld/04-opc-and-packaging.md` | `96d0d7b4df43519db6bfebf87164231e1254a3e7fc339ab83a01ea1f1039717e` |
| `docs/hld/10-bindings-spec.md` | `706e28518fe7ba616b4d6bf4b4ec2c33f873844ada9ae0e3bb1fbffb9d198ae8` |
| `docs/hld/12-testing-strategy.md` | `94e032bff49e0e247273bea8482832a55ea2d2bc2302df9057e6bea7cf2b12e2` |

