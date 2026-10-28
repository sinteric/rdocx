# F-X268, coherent Word repair union, pass 23

**Reviewed**: Working diff against c62e0a718431710cd3cde7f7452b2cef22671012. Fourteen tracked implementation, test, plan and HLD files, 818 insertions and 66 deletions. The additional existing local progress file was read and authenticated. Combined freeze f5f788b95098a7086cc93d2d5ce4acfd68eb12b20ec3419586407781a5e763cc and working diff 9af4c52598952a877d6e5b8f47540903bf21f3764a84701cd4afad418a13fa35 match the actual tree.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and boundary

Codex flat_cost_design performed this read-only implementation review. This reviewer authored earlier F-X268 optimization work and prepared the c62 runtime, A6 oracle correction and A7 semantic evidence. Those prior involvements are disclosed. Shared and SourcePieces authored this repair union. This reviewer did not implement its F-288 writer repair, ABI acceptance amendment or current-source producer shortcut. Only this review file was written. No source fix, formatting, compiler, test, renderer, native application or measurement was run in this pass.

F-X268 is in progress. F-288 retains its earlier approved local provider checkpoint while its final native repair is reviewed here within the active integrated union. This review does not alter completion metadata or turn that prefix into final acceptance. The canonical workflow, microscope and risk routing requirements were read. F-288, F-289, F-290 and F-X268 contract amendments and the merged HLD03/04/10/12 text were checked against the actual diff.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

### Correctness and source ownership

The new cache insertion at `crates/rdocx-oxml/src/text.rs:13204` precedes the source end-marker run rather than placing display text inside that run. Both Start/End and Empty marker event paths call it. The original run retains non-namespace owner attributes. The result receives its namespace declarations and formatting. Source offsets come from the same monotonically written run, the content slice is checked, and insertion occurs once before the outer end. No new untrusted arithmetic, unchecked input index or reachable panic was identified.

The own-bookmark admission at `crates/rdocx-oxml/src/text.rs:6221` remains authoritative. Its begin/end checks require empty typed run content, exactly the marker raw child after root-attribute exclusions and no alternate drawings. Extra ordinary run payload and opaque direct element sidecars therefore do not become an admitted pure end merely because the insertion helper copies a prefix. Existing nested ordinary-child guards and property XML retention were inspected. The repair neither broadens Q25/Q27 admission nor treats a following equal Boolean as proof that the original owner survived.

The ordered legacy descriptor comparison at `crates/rdocx/src/field.rs:975` covers source part, ordinal, name, kind and cardinality before publishing. Body and related-story paths share that check. Existing requested kind and identity checks run before propagating the captured pre-edit inventory error. Ordinary child counts are correctly excluded because literal parent assignment can remove a supported ordinary child. Reopen, current-value comparison and final staged commit retain atomic refusal. No unsupported publisher or changed unrelated field was identified from this diff.

### Current-source and writer authority

`crates/rdocx/src/document.rs:25014` uses the existing complete actual source/model comparator. Its exhaustive document destructuring covers root attributes, namespace declarations, both background carriers, section properties, content cardinality and every current body child. A mismatch does not prevent a later missing source proof from returning None.

Read equality is not promoted to writer authority. The existing fallible context and frame preflight remains before either branch. The independent current writer check at `crates/rdocx/src/document.rs:25046` requires the actual normal part, current staged children, lexical/private admission and matching frame. Only Some(true) admits the new route. Some(false) and None retain the original full child-normalization path. Source blocks stay staged=None, and the new route does not manufacture ScopedChildCorrespondence or a persistent mutable-model cache.

The caller route through source_blocks retains block_proofs_enabled, current part identity and the authoritative whole fallback. Source child reads, shell parses and actual writer bytes remain separately counted. The captured buffers are dropped before later allocations. No guard removal, deferred required validation, work relabelling or new public API was identified. Five owned publisher coverage and source/frame/resource controls remain in the scoped matrix.

### ABI compatibility and behavior coverage

`crates/rdocx-py/tests/test_core.py:9571` separates seven native class metadata contracts from the two existing behavioral tests. Python3.9 now checks the explicitly approved unavailable class signature, while newer Python checks complete names, order, parameter kinds and exact default types and values. Class identity and public exports remain checked. Constructor calls, wrong primitive shapes, frozen properties, defaults, explicit None and the form/mail-merge operations remain in the original behavioral bodies. No constructor implementation, wrapper, stub or supported runtime was removed.

This is an explicit acceptance amendment documented in F-289, F-290, HLD10 and CHANGELOG. The historical failed full Python3.9 run remains failed under its original expectation. It is not relabelled by the amendment. The targeted current-test-source execution records nine actual passes on each interpreter using the unchanged authenticated c62 normal module. No new normal-module execution is inferred from those metadata tests.

### Tests and structure

The OXML matrix distinguishes canonical, aliased and default scopes, both marker spellings, both initial Boolean states and repeated assignments. It checks the actual whole-document reopen, retained defaults, all three bookmark axes, formatting, one original owner attribute and a canonical byte no-op. Facade cases add physical cell and related header ownership, neighboring checkbox/dropdown identity and same/opposite assignments. These assertions reject the observed lost-projection and shifted-owner defects rather than accepting only matching values.

Producer tests distinguish actual source equality from writer equality, writer Some(false) from None, late unproved children after earlier mismatch, complete frame/model fields, zero and duplicate blocks, namespace/private/field exclusions and force-off fallback. The prototype old/new/OFF public comparison is evidence about its exact prototype, not a qualification of the future normal union.

No new crate, module, source file, trait, generic parameter, feature flag, stable binding export or retained cache root was introduced. HLD changes describe current intent and preserve the shared F-288 additions. No structural or documentation contradiction was found.

## Evidence consumed and limits

The supplied freeze and every one of its 15 source/progress hashes were reauthenticated before this report was written. The tracked diff also matches working.diff byte-for-byte. Shared's native evidence records 38 unique OXML controls, nine facade library controls and 57 integration controls, including the new fixture combinations. F-X268 records 24 selected native controls and current check/clippy/fmt/prose results. The selected test assertions and source routes were inspected rather than treating aggregate counts as native parity.

The 24 canonical F-X268 native outcomes precede only the recorded nested-if to short-circuit-and-let lint delta. The before-lint source and actual correction were inspected. Evaluation order and the guarded body are equivalent, and current all-target clippy typechecks the final source. No post-lint native rerun is claimed.

The ABI receipt at /private/tmp/s954-abi39-amendment-c62-targeted/receipt.json binds the current test hash to actual c62 producer executions on3.9and3.12. The source and artifact distinctions in these receipts remain explicit.

This is a clean source review, not a final sprint gate or performance verdict. The current c62 normal picture timing failures and native setter failures remain preserved. The reviewed union still needs its single Word-only successor normal/debug/count artifact refresh, actual corrected native checkbox setters, applicable runtime checks, unchanged79flat/fivelinear/Original407 gates, final hash and memory riders. In particular, changed document.rs invalidates the old memory port's byte-identical-owner assumption and requires its named source/lifetime reassessment. Unchanged reviewed CJK and PowerPoint work was not reopened as a confirmation pass. No final native, timing, memory, release or publication success is asserted.

The review pass ends here and releases the review-only writer lease to Root.
