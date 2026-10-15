# F-X216, native composition, pass 5

**Reviewed**: Frozen successor against native composition pass 4 and claim Base `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. The immutable seal retains 19 current/Base source, test and plan carriers with the cumulative 20,977-line working patch. This pass examines the remaining ordered raw/event composition correction and the four owned formatting-only carrier deltas. Earlier native and public composition findings retain their previously stated boundaries.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0 found in this bounded native composition pass.

## Prior findings resolved

**D4, ordering and multiplicity**: `crates/rpptx-oxml/src/shape_tree.rs:3426` snapshots only preexisting local occurrences after physical member mapping. `crates/rpptx-oxml/src/shape_tree.rs:3442` advances a separate monotone cursor per admitted target boundary. Once a source occurrence is appended, `crates/rpptx-oxml/src/shape_tree.rs:3482` prevents later source occurrences from matching older local content that would precede that appended occurrence. Thus matched source occurrences remain in order, each original local occurrence can satisfy at most one source occurrence, and the unmatched source suffix remains in order. Existing local raw order remains unchanged. Distinct final extension conflicts still refuse before publication. The candidate publishes only at `crates/rpptx-oxml/src/shape_tree.rs:3505`.

The same ordered snapshot/one-use/append-only rule now applies to both recorded header event boundaries at `crates/rpptx-oxml/src/shape_tree.rs:3491`. Its cursor cannot exceed the snapshot length because it advances only by a successful suffix-match offset plus one. The earlier contains-only cardinality/order loss is no longer present.

Current defining controls cover crossed local/source order, a new source event before an old local match, equal sequence reuse, repeated source occurrences with local overlap, distinct physical boundaries and source-before-member boundaries. Header-event controls separately cover crossed order, append-before-match, duplicate source events and equal reuse. They assert actual raw sequences and serialized/reopened fixed points. The earlier namespace, expanded-attribute and schema-final negatives still execute.

D1 source owner metadata, effective header/footer and group/non-visual authority admission remains unchanged from the inspected pass-2 correction. D2 complete context admission protects existing local opaque authority. D3 expanded root attribute identity coalesces only equivalent values and full context. Those corrections and the unchanged source theme/font/resource and prepared-publication consumers show no new established finding in this pass.

## Formatting delta

Manual comparison of retained pass-4 and pass-5 copies confirms that `crates/rpptx-cli/src/commands.rs`, `crates/rpptx-cli/src/main.rs`, `crates/rpptx-oxml/src/comments.rs` and `crates/rpptx-py/src/presentation.rs` change only layout, trailing punctuation permitted by the same Rust constructs, and formatting of existing expressions. Existing literals, method selection, conditions, operation labels and publication order are unchanged. This is a semantic formatting bridge, not a claim that token bytes are identical. No new lint suppression is added.

## Authentication

Manifest SHA `94b423dcc30f2b30a16b5d61c2d5aa39a8811d57a58f5aceca6b882f44140ad6` matches. All 19 immutable current/Base copies and live changed carriers match their declared hashes. Source-map SHA `43f5ddbd45b7f68ca0f1450fe3d1a34a5adb517c8d2ddfc732893b88bc11f857` authenticates all 3,296 mapped live bodies, sizes and modes. HEAD matches the claim Base. Logical staged-entry SHA matches `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07`. Patch SHA is `a962b7b2e7181ff60bc16e38ea86f917e44c222f12446092b3b49767d812286b`. Current shape-tree SHA is `833be73df85ac805e8879eaed8aa5c67996318813dcc275600511740aec876b7`.

Actual current receipt `actual-native-notes-composition-successor-9/receipt.json` matches SHA `7f269e4c068b67249885bc2c34e75c94f6df28911ccc45513121dd5a4040e7e3`. All four raw log hashes match. Each row has actual exit 0, qualified status, group gone, leader reaped and safe lease release facts. All-target/all-feature native check and strict four-crate Clippy pass. Native import membership is exactly the sealed 26 distinct names, with 26 passed, 0 failed and 0 ignored. Tree membership is exactly eight distinct named notes-tree controls, with 8 passed, 0 failed and 0 ignored. Retained current integration binary SHA `415e270c392a6de0f2f0eb31bdbca4ac93a70b737a74918c939555ec4bd5eec5` matches the seal. Runtime is the documented 8 MiB default debug native test environment.

The genuine crossed-order RED receipt matches SHA `66a102e090e0a3f50265fb806e87b031efd2245aecc3f0460f8d07e88d68bba1`. Its hashed raw log shows the exact named control running one test and failing with B,A instead of B,A,B, exit 101, 0 passed, 1 failed and 0 ignored. The separately retained header RED receipt matches SHA `bfe2f73ba7bd8ffb3c622f6d38b9e8d05cb82b4aefeb7e3d5191da06441127f5`. Its hashed log shows the exact header-order control running one test and failing with the corresponding missing final header event, exit 101, 0 passed, 1 failed and 0 ignored. Their safe cleanup facts match the records. Earlier failures, the wrong-key RED driver postprocess failure, the invalid common-extension predecessor limitation and the predecessor Clippy failure remain retained without success relabeling.

## Smells

0 found.

## Nitpicks

0 found.

## Not found and acceptance limits

No additional finding was established in staged input/candidate atomicity, namespace and opaque authority, schema boundaries, matched local property precedence, source resource ownership, conservative automatic script-font refusal, sparse line preparation, prepared Python allocation/publication or CLI path-aware serialization in the examined composition. These are static observations tied to the frozen source and exact retained tests.

The reviewer executes no builds, tests, binaries, package/runtime operations, UI or Git mutations. Current native Office save/reopen, deterministic two-deck comparison, fresh installed Python and types, broader applicable gates and formal handoff remain pending. The zero verdict closes this bounded composition review only. It is not a whole-feature delivery or native/installed/corpus acceptance claim. This pass ends with the report and returns the writer lease.
