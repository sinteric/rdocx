# F-288, native ordinary children, pass 2

**Reviewer**: Independent Codex reviewer `/root/flat_cost_design`.
**Reviewed**: Frozen Q25/Q27 eight-file diff against `85bd31e8f99b4dbabbe01e3333b8c2dd13f88b7e`, 1217 insertions and 70 deletions after excluding Root's separate Q26 HLD12 subsection. Product behavior changes remain confined to `crates/rdocx-oxml/src/text.rs`. Pass 1 D1 remediation changes only that file since the previous eight-file freeze. Canonical microscope, current approved plan, progress, prior review and applicable HLD contracts were read. No implementation or execution was performed by this review.
**Verdict**: Zero defects, zero smells, zero nitpicks. Pass 1 D1 is closed. This is a bounded implementation review, not final native A6, normal-release or sprint acceptance.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## D1 closure and complete run-storage audit

The actual-source admission guard at `crates/rdocx-oxml/src/text.rs:6247` now rejects nonempty `alt_drawings`, classified legacy horizontal rules and disallowed typed content before own-bookmark projection collapses the source span. It reparses every physical source run across the outer field, so the same guard covers both ordinary-child regions and content inside or outside the child.

The entire `CT_R` storage declaration at `:3075` was audited. `content` holds ordinary drawings, references, symbols and other typed run children and remains subject to the existing whitelist. `alt_drawings` is the separate supported AlternateContent drawing projection. The parser sets the accompanying raw drawing flag together with that vector at `:3745`. The guard uses the actual freshly parsed vector, not a guessed local-name heuristic.

`extra_xml_positions` has five classification flags at `:3090`. Legacy horizontal rules represent a separate observable run item and are now explicitly refused. Their classification uses resolved Word/VML/Office namespaces and the existing enabled-rule predicate. `RunItemRef` consumes the same classification at `crates/rdocx/src/run.rs:365`. The alternate drawing flag backs `alt_drawings`, while the comment-reference flag backs the already checked typed content. Root-attribute and local-binding flags retain namespace metadata. `properties` holds formatting. No additional typed owner storage was found omitted from this bounded admission guard.

Unknown raw content, unsupported AlternateContent and producer hints remain raw preservation rather than being promoted to typed drawing owners. The repair does not blanket-reject `extra_xml` or change the underlying parser's namespace/classification policy.

## Discriminating controls and consumed outcomes

The regression at `crates/rdocx-oxml/src/text.rs:20821` covers two child regions, two extra-owner placements and two typed sidecars. It checks that the outer stays untyped, the actual drawing/text-box remains discoverable with text `boxed`, the VML rule classification remains discoverable, and serialized wrapper plus both physical bookmarks occur exactly once. Two compatible opaque/unsupported positives per region remain admitted and byte-preserved at `:20888`.

The original supported AlternateContent fixture was factored into a concrete test helper at `:17940` and reused without changing its bytes. This keeps the known layout-visible representation and its original parser test alongside the new refusal control.

Actual retained RED is `/private/tmp/s954-q27-sidecar-red.log`: the old guard compiled, then the focused test failed with `typed owner collapsed: instruction=false, child=false`, child exit 101. It is a result-region counterexample, not an asserted execution of every negative combination. Current focused GREEN passes 1, native selection passes 17 and AlternateContent selection passes 3 in the logs named by `/private/tmp/s954-q27-sidecar-receipts.json`. The focused case is also in the 17-test selection, so these counts are not added as distinct tests. Scoped OXML all-target/all-feature clippy records exit 0, with the compiler log reporting 10.32 seconds. Format, diff and zero-prose-violation outcomes are retained in progress. None was rerun by this reviewer.

## Broader Q25/Q27 invariants examined

- Exactly one total complex ordinary TIME, DATE or formula leaf remains required at `crates/rdocx-oxml/src/text.rs:6168`. Instruction ownership requires the sole direct Nested argument. Extra operands, both regions, deeper nesting, simple children and legacy/private owners retain fallback. Scalar switches remain compatible.
- The actual-source simple-child check remains ahead of projection at `:6244`. The sidecar repair closes a different representation omission without weakening it.
- Pure outer boundaries, unique matching physical bookmark pair, actual source namespace frame and direct/accepted/tracked-axis remapping remain unchanged. Reserved-prefix conflicts still refuse admission at `:6309`.
- Outer saved current and explicit reset default remain separate authorities. Instruction child source configuration survives literal parent assignment. Existing ordinary evaluation, locks and explicit/unresolved context behavior are not replaced by a new evaluator or clock.
- Canonical reconstruction still refuses a retained own-name bookmark after source/instruction identity stops qualifying at `:11170`. The source-preserving writer routes remain earlier. Checked literal publication compares original and candidate bytes before mutation at `:2051`, preserving no-op and atomic failure.
- Five HLD changes preserve these bounded behaviors and retain final qualification debt. Public API, binding signatures, module/dependency structure and F-X268 field/private fallback policy do not change in this repair.

No additional correctness, contract, panic/arithmetic, namespace/schema, opaque preservation, test-discrimination, public API or structural finding was identified in these paths.

## Authentication and applicability limits

All eight actual file hashes match `/private/tmp/s954-q27-sidecar-frozen-hashes.json`. Product `text.rs` is `0447a4d5619b761678444120eefad61bb15eb68f653a9faa90215254f38fb5f7`. The other seven scoped hashes are identical to the previous freeze, including the native facade and Python test sources. Root's Q26 plan, PowerPoint test changes and native-reference HLD subsection were excluded.

All six native input hashes in `/private/tmp/s954-q27-sidecar-native-input-applicability.json` were independently authenticated. Read-only namespace-aware XML inspection confirms no AlternateContent elements or enabled Office-hr VML rectangles in their Word XML parts. Thus none contains the two newly refused sidecar conditions. This is static applicability evidence, not a fresh run of a rebuilt binding.

The earlier isolated debug module `b12c12ee24df64bece9328932728bacf3bc0b1258c0e0236b2c1cfafa243e139` remains the recorded artifact for 37 Python cases and the six-file ten-field inventory proof. Those earlier executions were consumed with pass 1's limits. This review does not relabel them as execution of the new product hash. Original seven-of-ten failures, result-only proof and current debug inventory repair remain distinct evidence.

The next normal-release artifact, current release/native consumption, evaluated-output and remaining native A6 comparisons, integrated hash harness and applicable full sprint/release riders remain pending. Source cleanliness in this review supplies none of those outcomes.

## Handback

F-288's consumed local provider checkpoint remains separate from this pending final-native remediation. Q26, Q27 PowerPoint design, other dirty HLD hunks, F-X268 performance and publication are outside this pass. No builds, tests, renders, native UI, RSS or timing processes were started. Only this review file was written. The pass ends and immediately releases the sole-writer lease to Root for orchestration.
