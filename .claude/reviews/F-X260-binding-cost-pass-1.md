# F-X260, binding cost, pass 1

**Reviewed**: fresh independent `/microscope F-X260 --working` on `sprint/s95.3`, HEAD `6f206c834bd81266877887c09479dae0ef59a2e1`. Complete eight-file working diff, 589 insertions and 159 deletions, 748 changed lines. Five binding implementation/test files plus the approved plan and HLD 03/10.
**Verdict**: 3 defects, 0 smells, 0 nitpicks. Changes required.
**Reviewer**: Codex in this audit session, without implementation or delegated agents.

Read AGENTS guidance, `CLAUDE.md`, canonical `.claude/WORKFLOW.md` and microscope command, the approved F-X260 plan including both takeover sections, progress notes, HLD 03 owned publication, HLD 10 lifetime and checked XML contracts, and relevant HLD 12 binding qualification. Confirmed the branch, HEAD, working status and both in-progress delivery rows. Read takeover passes 1/2 and the latest earlier feature review as context. This review is distinct from those A7 passes and is not another sprint review.

## Defects

### D1, the new capture parameter fails the required lint gate

`crates/rdocx-py/src/document.rs:4196`
`.claude/commands/verify.md:97`

Adding `removal_scope` makes `word_owner_capture` take eight arguments. The supplied `/tmp/s95.3-codex-binding-cost-clippy.log` finished during review with `clippy::too_many_arguments (8/7)`, rejected by `-D warnings`, for both the library and library-test compilation. The required scoped lint therefore fails on this exact source. This is an observed gate failure, not a proposed style preference. Resolve it and obtain a successful affected lint observation before acceptance.

### D2, append and conservative plain removal still mutate before fallible preparation

`crates/rpptx-py/src/shape.rs:1778`
`crates/rpptx-py/src/shape.rs:1784`
`crates/rpptx-py/src/shape.rs:1793`
`crates/rpptx-py/src/presentation.rs:3731`
`crates/rdocx-py/src/document.rs:11525`
`crates/rdocx-py/src/document.rs:6213`
`crates/rdocx-py/src/document.rs:6253`

Concrete PowerPoint input: a valid root-slide `add_textbox(0, 0, 100, 100)` when the revision counter cannot allocate the next ten generations, or when a checked allocation in scope preparation fails. `reserve_textbox_owners` reserves the inventory and maps, then `add` mutates `presentation.inner`. Only afterward does `publish_appended_textbox` call `prepare_scopes`, which can return overflow or allocation refusal. No rollback restores the native append. The call returns an error with the new shape already present but without its three current owners. Capture and the returned `PyShape` allocation are also fallible after that mutation. Reserving the maps does not reserve these operations.

Concrete Word input: two plain body paragraphs, `remove_content(0)`, and a counter at its serial/generation limit, or a checked scope-allocation failure. The tail proof returns `None`, native removal runs, and `publish_plain_body_removal` takes the live owner vector out of `self` before preparing the revision. A preparation error leaves the paragraph removed and the owner inventory empty, with the old indexes still installed. The newly prepared tail path avoids this ordering, but the non-tail fallback does not.

These late-preparation sequences were already present in the predecessor shortcuts. They remain executable consumers of the revised publication machinery and are not resolved by the new reservations or tail proof. The review request explicitly includes fallback and overflow/allocation refusal. They violate A7 and `docs/hld/03-architecture.md:73` plus `docs/hld/10-bindings-spec.md:96`, which require preparation and returned-object allocation before publication and unchanged state on refusal. Prepare the complete operation before mutating live native state, or retain an exact bounded rollback/staged boundary. The allocation/overflow effects here are source-derived. No allocator fault injection or counter exhaustion was executed.

### D3, replacement publication rebuilds allocating indexes after committing scopes

`crates/rpptx-py/src/presentation.rs:3604`
`crates/rpptx-py/src/presentation.rs:2905`
`crates/rpptx-py/src/presentation.rs:2915`
`crates/rdocx-py/src/document.rs:6597`
`crates/rdocx-py/src/document.rs:5998`

Concrete input: a normal staged owner-growing edit, such as adding a slide or adding enough Word paragraphs to exceed the existing derived-map capacity. Successful owner preparation is followed by scope commit and inventory replacement, then `refresh_owner_indexes`. Its `HashMap::insert` calls may allocate, without prior fallible capacity preparation. The PowerPoint helper also clones path keys and layout/master part identities during this post-commit phase. A path longer than the inline SmallVec capacity needs a fresh allocation even if the map itself has spare capacity.

If one of these new allocations cannot be satisfied, the default allocator can terminate the process rather than produce the required prepublication allocation refusal. The old scopes have already retired. The Word candidate assignment and the PowerPoint collection-scope assignments have not necessarily finished. The simple textbox reservation covers only its append variant, not `OwnerChanges::Replace` or Word replacement publication.

This is introduced by the derived-index rebuilds and violates the infallible-final-publication contract in `docs/hld/03-architecture.md:73` and `docs/hld/10-bindings-spec.md:96`. Prepare the replacement indexes, including owned keys, alongside the candidate inventory before scope commit, then install them without allocation. No low-memory stress run was performed. This finding follows the actual allocation sites and commit order, not an observed out-of-memory crash.

## Smells

None.

## Nitpicks

None.

## Assessment and explicit zero counts

**Current indexes and inventory order: zero additional findings.** Traced initial construction, whole-inventory replacement, property relocation, Word plain XML generation replacement, Word tail drain, PowerPoint local descendant generation replacement and textbox append. Indexes contain current owners, not owner history. Word replacement removes old run-generation entries before installing successors. The tail drain removes generations and adjusts only the proven section suffix. PowerPoint path lookup uses `entry(...).or_insert(index)`, preserving the old iteration's first match. All constructed text-body addresses have `layout_index: None`, so the new explicit key does not change the old body lookup's result. Shape multiplicity is keyed by physical part identity and counts every indexed shape identity rather than collapsing duplicate IDs.

Appending newborn owners at the vector end changes private ordering but leaves existing indexes and parent-before-descendant order intact. Traced survival, footer and changed-body consumers, including loops zipped with proofs from `unchanged_owners`. They do not require globally contiguous slide ownership. Whole replacements restore traversal order and rebuild indexes. The bounded reordered-publication control below passed. This is not a claim that passing tests alone establish every invariant.

**Held, fresh, stale and foreign authority: zero additional findings.** Word physical captures include the removal epoch, while actual runs also retain document authority. Snapshot captures retain snapshot/document retirement. Checked paragraph replacement validates captured scopes before indexed resolution and verifies the selected owner's exact state. Furniture and content/property/grid views continue through physical captures or inherited paths. The tail proof requires the last body slot, contiguous direct descendants and a section-only remaining inventory suffix. Header, nested-cell, hyperlink or other later owners close that shortcut and use the existing fallback. Earlier fresh captures use the new epoch. Removed numeric slots do not revive old captures. First-cause selection still chooses the earliest retirement, independently of later scope reuse. PowerPoint checked paths validate origin and stale scopes before indexed generation lookup. No foreign-generation shortcut was found.

**XML and scope semantics: zero additional ordinary-input findings.** Word native local admission still requires a plain current and incoming paragraph, identical accepted run paths, unchanged paragraph section properties, checked namespaces, relationships and identities. Other structures stage. Local Word XML prepares content/property and contiguous run successors before mutation and commits only after a real change. Exact no-op and native rejection discard preparation. The new PowerPoint local preparation applies to ordinary shapes and is committed only when the before/after identity, kind, table flag and paragraph/run counts agree. It retires descendant physical/text/XML scopes while retaining the selected shape. Topology changes, groups, tables and other Element kinds retain existing survivor/publication paths, subject to D2/D3's preparation boundary.

Read the native checked-owner admission provider and prior takeover controls for raw frontiers, qualified attributes, inherited MC authority, signed packages and relationship changes. The cost diff does not remove their validation or introduce a new XML serializer. Group/table, layout/master, imported duplicate/missing identity and AlternateContent paths were traced in source. No new raw-XML preservation, schema-order, normal indexing panic or identity-policy defect was established. These source checks do not qualify external native parity.

**Structure: zero findings.** No new crate, module, trait, generic parameter, product file, public API or historical ownership registry. The concrete append/replace representation describes two actual publication cases.

## Work-bound and acceptance evidence

The new counters observe actual index rebuild entries, generation/path lookups and selected descendant loops. The Word test edits a middle paragraph and removes the tail for 50/800 inputs. The PowerPoint test appends to a separate edited slide with zero or 40 unrelated slides of 40 boxes, then exercises scalar properties and changed simple-shape XML. Its equal bounded work vectors discriminate an unrelated inventory traversal on those paths.

These counters are not total allocation or execution meters. They exclude hash-table internal reserve/rehash work, native parsing and serialization, Python allocation, the target-slide duplicate-ID scan and several general inventory/survival scans. Fresh Word capture and general current-owner resolution still have linear paths. Complex XML and non-tail removal intentionally retain broader fallbacks. The controls do not prove constant cost for those fallbacks, allocation refusal, every lifecycle combination or the public release ratio. Native visit counters and binding traversal counters must remain distinct from timing.

Consumed supplied logs without running suites:

- Word and PowerPoint work-bound logs each report their named local test passed.
- `binding-unit-tests.log` reports seven Word library tests and one PowerPoint library test passed.
- `word-cost-regressions.log` remains 47 passed and 1 failed. The failure was the new control expecting `ValueError` where the actual contractual result is `StaleElementError`. The corrected `word-tail-control.log` reports 1 passed. The other 47 tests were not rerun or relabelled.
- `pptx-cost-regressions.log` completed with 160 passed and 858 deselected.
- `binding-cost-clippy.log` failed as recorded in D1. Successful tests do not supersede that failed check.

The historical release log `/tmp/s95.3-codex-issue389-release-qualified.log` measured the actual Python consumer on pre-repair source identified by `/tmp/s95.3-codex-release-manifest.json`, HEAD `6f206c83`. Strict ratios were PowerPoint textbox append 5.3553, alt text 0.6912, left 1.2683, fill 2.6813 and XML 3.5226, plus Word XML 7.5459 and removal 67.1116. Four rows failed Q10. Native-only observations did not qualify these rows. The two earlier `/tmp` realpath setup failures produced no measurements and are not samples.

That manifest records Word wheel SHA-256 `c79c66719b5e30e4b6653fe5ada772bdf6efdbfc94bb773805b3b110ccaa5290` and PowerPoint wheel SHA-256 `1466775645233a9fc72a79ef745ebfc14a9e4e5aa873a555a76f32c6563a1348`. The historical loaded release extensions were `/private/tmp/s95.3-release-probe/rdocx/_rdocx.abi3.so`, hash `7d861e498c0184a60b8d17f0b5e1869605985beba9c0cceeef048e93498091c5`, and the corresponding `rpptx/_rpptx.abi3.so`, hash `74245099c9c988b056abe87e095d80c0747014b5fe159f4b057815272fd88380`. Those source hashes differ from this repair. No fresh release GREEN is claimed. Q10 remains strictly ratio below 3 for all seven rows, without a timing floor or threshold reduction.

## Independent bounded probes

Only in-memory, bytecode-disabled probes ran. No extension build, benchmark, broad suite or external oracle ran.

- Word: remove a tail section-break paragraph, observe the section count change from two to one, update the fresh remaining section, append another paragraph and reopen bytes. The resulting section count and paragraphs agreed. This challenges a tail containing section properties, rather than only the supplied plain-text example.
- PowerPoint: append owners to an earlier slide after owners on a later slide exist, make a stable scalar shape-XML edit, move shapes and slides, then read/edit held shapes. Fresh descendant lookup and repeated run append preserved the earlier live run. Removing a shape and appending a replacement retained the removed handle's original `ShapeCollection.remove` cause. Equal XML kept the held run usable. Malformed XML refused with unchanged selected XML and a live run. Changed shape XML retired the prior run with `Element.replace_xml` provenance.

Two exploratory compound PowerPoint invocations stopped on probe API mistakes, `move_shape` instead of public `move`, and passing BytesIO to path-only `save`. They are not product failures or passing compound controls. The corrected lifecycle probe passed. PowerPoint saved-byte/reopen evidence was not produced by this review.

The loaded Word extension resolved to `crates/rdocx-py/python/rdocx/_rdocx.abi3.so`, SHA-256 `db32015f74ea5b3921f742e9a375745e1714ead1798fccd61fe69ff9d141c5a5`. PowerPoint resolved to `crates/rpptx-py/python/rpptx/_rpptx.abi3.so`, SHA-256 `1b8fb5b8078cec812cdbaa844461bda443b5a43a019ef3827002408a15a1f977`. Each matched its respective `/tmp/s95.3-rdocx-py-target/debug/lib_rdocx.dylib` or `/tmp/s95.3-rpptx-py-target/debug/lib_rpptx.dylib`. The successful cost-build logs identify editable dev builds, Word unoptimized and PowerPoint optimized with debug info. These identities were checked before and after probes. They authenticate the binaries used here, not a new release-wheel timing qualification or a complete build-source manifest.

## Source identity and remaining gates

Full-file SHA-256 values captured before inspection and checked again after the probes all matched. HEAD, status and diff counts also remained unchanged before report creation.

| File | Before and after SHA-256 |
|---|---|
| `crates/rdocx-py/src/document.rs` | `4e5f6f6065bace5b000de912e26e0aed0184748dfcf28a1fb515d374b43adaaf` |
| `crates/rdocx-py/src/paragraph.rs` | `c7ec2469d5c8bd07c663841f36e1748036bf3cc3855cb3a09ab86a33d34f3766` |
| `crates/rdocx-py/tests/test_core.py` | `eb6749f0416fd38152a18704e768f2391e1e5efca49e30d4c2b58600b807fc44` |
| `crates/rpptx-py/src/presentation.rs` | `f7cb9c5ffb0c79e9b8bbff3723f6dca5cea2f69e1e315154fe4b99e90fa4b9a5` |
| `crates/rpptx-py/src/shape.rs` | `97611a5b5f2cbdaeac9ff049ca79388aaba95c26a28f51ca55c1499619e75e9a` |
| `.claude/plans/F-X260-design.md` | `19402538ac266aaf6bca921b983ec4ccf398616c1cb50419057b9813bc521096` |
| `docs/hld/03-architecture.md` | `41d2a1f4e175fa52364c978c38629e8b18d2d3979cd7270a1f0d690e4629f45e` |
| `docs/hld/10-bindings-spec.md` | `5ba8dfe258a0b6af05d0025a38e3a334d5b3c964888ee8870f7b9d773cb274b1` |

A1 through A4 and A6 retain their existing acceptance obligations. This scoped audit and the selected supplied checks do not replace full installed-product qualification. A5 requires the fresh release measurement after repair. A7 retains refusal and ownership obligations, including D2/D3. Authentic native Word and font-oracle evidence remain unavailable, as confirmed by the user. Applicable final hash, rendering, WASM, packaging and other integrated riders remain pending.

Only this report was written. No source, plan, HLD, progress, state, ledger, historical review or baseline modification. No agents, commits, sprint review or sprint closure. Review stops here and returns control for a separate remediation phase.
