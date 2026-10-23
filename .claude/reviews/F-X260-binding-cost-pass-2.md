# F-X260, binding cost, pass 2

**Reviewed**: fresh independent `/microscope F-X260 --working` on `sprint/s95.3`, HEAD `6f206c834bd81266877887c09479dae0ef59a2e1`. Entire eight-file working diff, 736 insertions and 318 deletions, 1054 changed lines.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Changes required. D1 and D3 are resolved. D2 remains open on another executable publication boundary.
**Reviewer**: Codex in this audit session, without implementation or delegated agents.

Read AGENTS, `CLAUDE.md`, canonical `.claude/WORKFLOW.md` and microscope command, the approved F-X260 plan including its cost and pass-1 amendments, progress notes, HLD 03/10 publication and checked XML contracts, and relevant HLD 12/14 sections. Read binding-cost pass 1 fully and the latest takeover and earlier feature reviews as context. Confirmed branch, HEAD, working status and both in-progress delivery rows. Prior clean reviews are historical context, not proof of this verdict. This is not a sprint review.

The eight scoped files are the plan, Word document and paragraph bindings, the existing Word core test file, PowerPoint presentation and shape bindings, and HLD 03/10. Native and process files have no working diff. Unchanged providers and callers were inspected to establish behavior.

## Defects

### D2, topology-changing XML still mutates native state before a rejecting owner preparation

`crates/rpptx-py/src/presentation.rs:2408`
`crates/rpptx-py/src/presentation.rs:2443`
`crates/rpptx-py/src/presentation.rs:2460`
`crates/rpptx-py/src/presentation.rs:2471`
`crates/rpptx-py/src/presentation.rs:3450`

Concrete input: a readable imported deck with two duplicate shape IDs on slide 0 and a normal textbox containing `target` on slide 1. Replace the latter's shape XML with the same XML plus `<a:p><a:r><a:t>new</a:t></a:r></a:p>` immediately before `</p:txBody>`.

The native replacement succeeds and changes the live presentation. The prepared simple-shape revision describes the old topology. Because paragraph counts changed, the binding discards that preparation and enters general survivor/inventory preparation against the already changed live presentation. Its structural proof finds the unrelated duplicate IDs and rejects. There is no restoration on this exit.

An authenticated, in-memory public Python probe observed exactly:

```text
ValueError: structural owner proof has an ambiguous shape id
bytes_unchanged False
target_text 'target\nnew'
held_run target
fresh_new_paragraph IndexError: physical XML owner is absent
```

The imported fixture was constructed by serializing two ordinary slides, changing the second textbox ID on slide 0 to the first textbox ID inside the in-memory ZIP, and reopening through `Presentation.from_bytes`. Slide 1 stayed unique and valid. No allocator fault, private counter manipulation, file output or unsupported XML was needed. The corresponding unique-ID topology-change control succeeded and published a usable new paragraph.

The required result is either successful publication with coherent owners or refusal with exact original bytes and live tokens. The binding instead returns refusal with changed native state, old live descendants and no current owner for the new paragraph. This violates A7, `docs/hld/03-architecture.md:73`, `docs/hld/10-bindings-spec.md:96`, and the explicit refusal/byte-preservation contract at `docs/hld/10-bindings-spec.md:4323`.

Retain D2 because this is the same mutation-before-fallible-preparation invariant on a different consumer. The original non-tail Word and root textbox examples are repaired. This XML fallback was inherited from the predecessor and remains executable in the complete current diff. It is not attributed as a newly introduced native regression. Preserve the existing structural ambiguity refusal, but perform its proof and complete preparation before publishing native state, or retain an exact staged/rollback boundary through every later refusal.

The same ordering leaves checked allocation and revision-budget refusals late on these general XML paths. `publish_stable_element_xml` at `crates/rpptx-py/src/presentation.rs:3966` also prepares after native mutation for non-simple stable shapes. Scalar shape and fill publishers at `crates/rpptx-py/src/shape.rs:406` and `crates/rpptx-py/src/dml.rs:225` still call fallible `publish_local_property` after native changes. These are source-derived members of the same ordering problem, not additional executed failures or separate finding counts. Post-native `read_xml` and structure reconstruction also remain fallible. The review cannot certify that all fallible Python-side work precedes native mutation.

## Smells

None, 0.

## Nitpicks

None, 0.

## Pass-1 repair assessment

**D1 resolved.** `word_owner_capture` groups document and removal epochs in one argument at `crates/rdocx-py/src/document.rs:4238`. It no longer has eight parameters or a new lint suppression. The supplied affected clippy log ends with successful completion for both binding crates. This does not erase the historical failed lint log.

**D2 partially resolved.** Non-tail Word removal uses `clone_for_staging`, native candidate mutation, complete owner/index preparation, scope commit and native assignment at `crates/rdocx-py/src/document.rs:11435`. The unsafe live non-tail removal publisher and its shifting helpers are deleted. Tail preparation remains before native removal. Native plain admission and removal were inspected: the admitted tail removal has no error after removing the child. Absent indices leave state unchanged. Complex, nested, relationship-bearing and prefix-blocked cases retain the staged path.

Root-slide textbox append now reserves inventory/maps, prepares nine newborn owner/text/XML scopes plus the document successor, captures the future path and allocates the Python result before native append. Only successful append reaches ID binding and scope commit. Native failure drops the preparation without publishing scopes.

Traced the sole exact operation-label caller in `shape.rs:2170`, `ShapesMut::add_textbox`, native local and staged append, `textbox_member`, and `CT_Shape::new_textbox` through `new_with_shell`. The constructor creates one ordinary shape, one real text body, one empty paragraph and zero runs. The native append returns its new last root slot, keeps prior owners, and assigns the constructor ID. The binder's three physical variants match those fixed births. It reads only that root shape and copies scalar ID/kind values. It does not allocate, call Python or change topology. Group, layout and master additions use the general staged binding branch. Missing/duplicate identity preflight precedes the root shortcut's native mutation and still traverses the target slide and descendants.

The exclusive PyO3 mutable presentation borrow covers preparation, Python result allocation, native append, binding and commit. The append closure is a fixed Rust closure, not a Python callback. Reentrant Python access during result allocation cannot acquire a mutable borrow of the same wrapper. There is no Python callback between native success and commit. `RevisionCounter::commit_scopes` validates before retirement and does not allocate. Its origin/serial/generation assumptions hold under this private exclusive sequence. No additional infallible-bind panic or authority defect was established.

**D3 resolved for index installation.** Word replacement and property relocation prepare complete `WordOwnerIndexes` before publication. PowerPoint replacement prepares complete `OwnerIndexes`, including cloned owned path and layout/master identity keys, alongside its owner vector. Both replacement committers move prepared values rather than rebuilding maps after retirement.

Local Word run successors and PowerPoint simple-shape descendants reserve generation-key capacity before native mutation. Their committers remove old generations and install reserved successors without constructing new path keys. Root textbox append reserves three entries in each map. Its post-commit insertion clones only paths of length two or three and `PartIdentity::Slide(u32)`, so keys stay inline and identity copies do not allocate. The full replacement route, including deep group/table paths and owned layout/master names, constructs its keys before commit. This resolves the reported cache-rebuild-after-commit defect, while D2 separately prevents certification of the earlier native publication boundary on fallbacks.

## Full diff assessment and explicit zero counts

**Index coherence and lookup policy: 0 additional findings.** Traced initial construction, replacement, Word property relocation, local Word run-generation replacement, Word tail drain, PowerPoint append and local descendant replacement. Generation maps contain current inventory entries. Removed generations leave the map. Tail drain adjusts the proven section suffix positions and removes the direct-body slot. Replacement rebuilds maps from the replacement inventory.

PowerPoint path keys retain the old first-match policy with `entry(...).or_insert(index)`. Text-body keys distinguish the body alias from its shape or cell. Every inventory text-body address uses `layout_index: None`, so explicit key qualification preserves the old body lookup. Shape-ID counts use physical part identity and count duplicate occurrences, rather than collapsing equal IDs. Missing IDs stay absent. Layout and master identity strings are separate enum variants. No bypass of foreign or stale authority was found before indexed resolution.

**Reordered inventory and publishers: 0 additional findings beyond D2.** Appending births at the private vector end preserves prior indices. Traced unchanged, footer, removed-layout, moved/removed-shape, slide-edit, literal-text, cell-text, appended-text and text-changed survival producers. Their proofs follow each actual current owner or explicitly pair with the same vector. They do not require slide-contiguous inventory. Full replacements restore traversal order and derive all indexes afresh. Parent-before-descendant relationships and path matching remain valid. Group/table/grid and layout/master branches retain general publication, subject to D2's late native boundary for Element XML.

**Scopes and lifetime: 0 additional findings.** Word physical capture includes the shared removal epoch. Runs additionally retain document authority. Snapshot captures retain document and snapshot retirement. Checked fast paragraph mutation validates captured scopes and exact selected owner state before indexed resolution. The tail proof requires the final direct-body slot, contiguous direct descendants and a section-only suffix. More complex inventories close the shortcut. Property, grid and furniture wrappers inherit physical authority. Retired captures cannot revive when a numeric slot is reused. PowerPoint retains selected shape authority while replacing unsafe descendant/text/property scopes. Fresh and held paths, foreign origin checks and earliest causal retirement were traced. The executed D2 input is the explicit exception where failed publication leaves these mechanisms incoherent.

**OOXML and panics: 0 additional findings.** Native plain Word XML admission still checks accepted run paths, paragraph section properties, source namespaces, relationships and identities before local assignment. Unsupported structures stage. PowerPoint keeps native frontier, namespace, relationship, signature and physical-owner validation. No serializer, schema order, raw-XML preservation rule, numeric conversion or public signature changed in this diff. Binder expectations follow the fixed constructor and validated private call sequence described above. No ordinary-input indexing panic was established.

**Structure and HLD: 0 additional findings.** No new crate, module, product file, trait, generic, feature flag, forwarding-only wrapper, public API or historical ownership registry. Prepared concrete append/replacement representations correspond to actual operations. HLD 03/10 describe derived indexes, the restricted Word tail path, removal epoch and simple textbox births. Existing HLD 12/14 obligations remain in the approved plan. Local bounds apply to proven simple/tail subsets, not general staged fallbacks.

## Evidence and limits

Inspected assertions and inputs for A1 through A7, the new tail lifetime regression, local work counters and the non-tail preparation fault injection. The fault-injection unit test forces `prepare` to reject using an already retired internal inventory token, then checks exact native bytes, owner/index cardinalities and serial. It is a passing GREEN with a source counterfactual for the old shortcut. It is not an executed historical RED or an allocator exhaustion test, and its inventory assertions compare cardinalities rather than every stored entry.

The counters observe selected inventory/index operations and descendant loops. They do not meter every allocation, hash-table reserve/rehash, parser/serializer, Python result, target-slide duplicate-ID scan or general proof traversal. Word fresh capture and several conservative lookups remain linear. General removals and complex XML keep broader fallbacks. These local tests cannot qualify the seven release ratios.

Consumed supplied logs without running suites or rebuilds:

| Artifact | Actual observation |
|---|---|
| `/tmp/s95.3-codex-binding-cost-order-clippy.log` | Successful affected clippy completion for both bindings. |
| `/tmp/s95.3-codex-binding-order-unit-tests.log` | Word 8 passed and PowerPoint 1 passed, including local work and Word preparation-refusal controls. |
| `/tmp/s95.3-codex-word-cost-order-build.log` | Editable dev build/install completed, unoptimized with debug information. |
| `/tmp/s95.3-codex-pptx-cost-order-build.log` | Editable dev build/install completed, optimized with debug information. |
| `/tmp/s95.3-codex-word-cost-order-regressions.log` | Finished during this audit, 48 passed, 366 deselected. |
| `/tmp/s95.3-codex-pptx-order-installed.log` | Finished during this audit, 963 passed, 55 deselected, 11 warnings. Excluded font qualification is not supplied by these passes. |
| `/tmp/s95.3-codex-workspace-tests.log` | Failed in `oxml-chart`, 98 passed and 3 failed. Each displayed failure could not run the `soffice` version check because the executable was absent. This is not a passing workspace gate. |
| `/tmp/s95.3-codex-rdocx-order-release-build.log` | Release wheel build completed. Build completion does not establish any timing outcome. |
| `/tmp/s95.3-codex-pptx-order-release-build.log` | Release build underway at inspection. No timing or final build gate inferred. |

Earlier product evidence remains separately attributed: the supplied prior Word core run had 414 passes. Prior PowerPoint evidence had 960 passes and three sandbox viewer aborts, followed by a separate normal-access run with those three passing. The 55 unavailable font cases remain excluded. These observations are not combined into a newly executed all-input full gate.

The historical actual release public Python log `/tmp/s95.3-codex-issue389-release-qualified.log` remains RED, four of seven rows failing, bound to its predecessor source and release manifest. Its highest problematic rows include Word XML 7.5459 and Word removal 67.1116. Earlier native-only ratios are not public binding acceptance. No fresh GREEN timing was executed by this review or established by the inspected build logs. Q10 still requires all seven strict ratios below 3, with unchanged workload and no timing floor.

## Bounded independent probes

Only bytecode-disabled public API probes ran, using in-memory packages. No broad suite, benchmark, oracle, extension rebuild or persistent probe artifact was created.

- The D2 duplicate-ID/other-slide topology probe failed atomically as detailed above: refusal occurred, but byte equality failed and the new owner was absent. This is an actual current-candidate product failure, not an inferred historical RED.
- The corresponding ordinary unique-ID topology edit succeeded. Fresh new paragraph/run lookup worked. Exact XML no-op retained a held run. Malformed XML refused with exact bytes and the held run intact. Stable changed XML retired the previous run. Appending on an earlier slide after a later slide already had owners, then moving slides, preserved both held shape identities. Removing a shape and appending another retained the removed handle's original `ShapeCollection.remove` cause. In-memory serialization/reopen preserved the unaffected later textbox.
- Word exact XML no-op and malformed XML refusal retained the earlier run. Absent removal returned false without retiring it. Tail removal retired the earlier held paragraph. A fresh paragraph could be edited, a replacement appended and non-tail removal performed, with the expected reopened text.

One exploratory PowerPoint control invocation passed an integer to `ShapeCollection.remove`, which requires a Shape, and stopped with a probe `TypeError`. It is not a product failure or a passing compound control. The corrected complete invocation passed. The no-op shape move in that control does not establish a nontrivial shape permutation. Nontrivial shape remapping was inspected in source, and slide movement was executed.

Imported extension identities were printed before each probe and checked afterward:

| Extension | SHA-256 |
|---|---|
| `crates/rdocx-py/python/rdocx/_rdocx.abi3.so` | `3af6fc2ffd2f71f4edd80c2491af9f5191b3efd9d30e1dce39576944fbc44eb4` |
| `crates/rpptx-py/python/rpptx/_rpptx.abi3.so` | `45b2eb27627f8be6d2f6fff2c9757ef4dfdc0f6d87a008eb1555d8e4506f8f60` |

Each matched its respective `/tmp/s95.3-rdocx-py-target/debug/lib_rdocx.dylib` or `/tmp/s95.3-rpptx-py-target/debug/lib_rpptx.dylib`. Interpreters were `/tmp/s95.3-rdocx-venv/bin/python` and `/tmp/s95.3-rpptx-venv/bin/python`. These authenticate the actual probe binaries and their supplied dev-build provenance. They are not release timing artifacts or a complete reproducible build manifest.

## Source identity and pending acceptance

Full-file hashes captured before detailed diff inspection and checked again after probes matched for all eight scoped files. HEAD, dirty paths and diff counts were also unchanged before report creation.

| File | Before and after SHA-256 |
|---|---|
| `.claude/plans/F-X260-design.md` | `2f50bbd6c341dfce7e7b2c5c899b7e771ee63228c8ca539f941efe2246d7001f` |
| `crates/rdocx-py/src/document.rs` | `951a68c60e072f9a18475fc70d6cb70952ff0076e08e5ee929270c0869753487` |
| `crates/rdocx-py/src/paragraph.rs` | `c7ec2469d5c8bd07c663841f36e1748036bf3cc3855cb3a09ab86a33d34f3766` |
| `crates/rdocx-py/tests/test_core.py` | `eb6749f0416fd38152a18704e768f2391e1e5efca49e30d4c2b58600b807fc44` |
| `crates/rpptx-py/src/presentation.rs` | `80877f0f15693b09469ee3a75f8ad05e0dbe32146cb24d1984c49d7d0742e66b` |
| `crates/rpptx-py/src/shape.rs` | `295d8fcc786e8c0360b4096fdb21b4390c7544ae64e35a13566d9de7b7a0ff6c` |
| `docs/hld/03-architecture.md` | `41d2a1f4e175fa52364c978c38629e8b18d2d3979cd7270a1f0d690e4629f45e` |
| `docs/hld/10-bindings-spec.md` | `5ba8dfe258a0b6af05d0025a38e3a334d5b3c964888ee8870f7b9d773cb274b1` |

A1 through A4 and A6 retain their final acceptance obligations. A5 has no current release timing GREEN. A7 has the reproduced D2 refusal defect. Authentic native Word fixtures and the qualified font oracle bundle are unavailable, as confirmed by the user. B1 remains pending. Applicable final hash, render, WASM, packaging and integrated qualification riders remain pending. No issue, feature or sprint closes from this report.

Only this report was written. No other source, test, plan, HLD, progress, state, review history, ledger or baseline mutation. No agents, commits, pushes, fourth sprint review, closure or implementation. Review stops here.
