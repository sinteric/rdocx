# F-X216, native composition, pass 3

**Reviewed**: Immutable successor against native composition pass 2 and claim Base `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. The seal retains 19 current/Base carriers and the cumulative 20,820-line working patch. The new production delta computes the admitted target boundary before deduplication and restricts lookup to that boundary. It adds one defining tree control. Other native/public composition carriers remain unchanged from pass 2.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D4, same-boundary deduplication still collapses source-owned multiplicity

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3439`

The different-boundary witness from pass 2 is fixed. However, the lookup still searches the evolving candidate, including raw occurrences inserted earlier from this same source master. It neither distinguishes preexisting local occurrences from new source occurrences nor accounts for how many source occurrences an existing local occurrence has already satisfied.

Use identical root bindings with `xmlns:z="urn:meta"` and, for an ignorable foreign-extension witness, matching `mc:Ignorable="z"`. The local tree has no raw children. The source master has two identical `<z:marker/>` children immediately before its source decoration. Both are retained in the same source boundary. The first marker is added at the mapped boundary. The second finds that newly inserted marker and takes the skip branch, leaving one marker instead of the source's two. Two identical comments at that boundary provide a schema-valid opaque-event witness without foreign-extension admission. If the local tree already contains one matching marker, that single occurrence can likewise discharge both source occurrences.

Preserve source-owned multiplicity while allowing only proven one-to-one equivalence with preexisting local occurrences at the admitted boundary. Newly inserted source occurrences cannot represent later distinct original source occurrences. Otherwise repeated opaque content is silently dropped despite identical bytes and namespace scope. This is the remaining cardinality part of D4, not a request to change accepted namespaces or schema limits. The new passing control covers distinct boundaries and does not cover repeated equal children within one source boundary.

## Prior resolutions

The requested pass-2 physical boundary witness now passes. `crates/rpptx-oxml/src/shape_tree.rs:3428` computes the target and checks nonrepresentable order before considering equality. The lookup at `crates/rpptx-oxml/src/shape_tree.rs:3442` uses that exact boundary. D1 source metadata/flag/group admission, D2 full namespace context admission, D3 expanded attribute identity and the distinct schema-final extension refusal remain unchanged from their inspected pass-2 corrections. No additional defect was established in those unchanged paths.

## Authentication

Manifest SHA `5e2493522e182f407f1883bd327b0bf77522c72f6553a78cc18745b44e15e174` matches. All 19 retained current/Base bodies and live changed carriers match their pinned hashes. Source-map SHA `980279003c396da254ad01290898187cb74183895b890dc19da61b3d79783add` matches all 3,296 live bodies, sizes and modes. HEAD matches the claim Base. Logical staged-entry SHA matches `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07`. Cumulative patch SHA is `82d1171769eab0cf5c33ddf902dfd98ddcd810fa608f83136582134bd7a927ab`.

Actual successor receipt `actual-native-notes-composition-successor-5/receipt.json` matches SHA `feacc56063e29432e3cc97d4327503435f1786aa7702e007a77e65a8dd457a34`. All four recorded log hashes match. Each row exits 0 with qualified and safe cleanup facts. Native import membership is exactly the sealed 26 distinct names, with 26 passed, 0 failed and 0 ignored. Tree membership is five distinct named notes-tree controls, with 5 passed, 0 failed and 0 ignored. All-target/all-feature check and strict four-crate Clippy pass. Retained integration binary SHA is `00c36a587ad344467a41d8ec97b9a2b962d99493b73549f62216a21bbf7ea413`. This remains the documented 8 MiB default debug native test environment.

The original boundary RED receipt matches SHA `d8deeb90d18343a58a950af2bf8cf5acdfeb4953ec8d2b8d1ad0c8dc8e21ccc7`. Its raw log SHA `36f5912799d06a8140f0737bdc1015c92d84fa1de08fb0a88f219a90e8ac6a3c` matches and independently shows the fully qualified named control, running 1 test, one failed assertion with actual 1 marker versus expected 2, and result 0 passed, 1 failed, 0 ignored. The row's actual exit is 101 and its process group is gone and reaped safely. Its original receipt status remains a running label because the driver postprocess accessed the wrong `exitCode` key. The separate reconciliation records the correct `exit` field and actual failure without changing that original receipt. This review credits the authenticated failed row, not a fictional terminal-success driver result. Earlier invalid common-extension fixture credit remains withdrawn as stated in pass 2.

## Smells

0 found in this bounded pass.

## Nitpicks

0 found.

## Acceptance limits

No builds, tests, binaries, package/runtime operations, UI or Git mutations were executed by this review. Current Office save/reopen, installed Python/types, deterministic corpus comparison, broader applicable gates and formal handoff remain pending. The authenticated 26 plus 5 native controls qualify their exact tested paths, not whole-story acceptance. The remaining D4 prevents a zero-defect composition verdict. The pass ends with this report and no remediation.
