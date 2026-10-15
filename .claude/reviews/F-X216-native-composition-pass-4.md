# F-X216, native composition, pass 4

**Reviewed**: Frozen successor against native composition pass 3 and claim Base `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. The seal retains 19 current/Base carriers and the cumulative 20,864-line working patch. The new production delta snapshots preexisting raw occurrences after member mapping and consumes each occurrence once. A defining same-boundary multiplicity control is added. The final nested-if collapse has separately authenticated syntax-equivalence evidence.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D4, same-boundary matching can still reverse the source opaque sequence

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3446`

One-use snapshot matching resolves the pass-3 multiplicity defect. However, existing occurrences are searched independently without preserving their relative order. Consumed indices prevent reuse but do not prevent crossing matches. The successful merged boundary can therefore lose the source's original opaque sequence.

Use equal physical namespace contexts, including `xmlns:z="urn:meta"` and matching `mc:Ignorable="z"`. The local tree ends in `<z:b/><z:a/>`. The source master has its decoration followed by `<z:a/><z:b/>`. Appending the source decoration shifts the retained local children to the same final target boundary used by both source children. Source A matches local index 1 and source B then matches local index 0. Both source occurrences are skipped. Output keeps B,A and no longer contains the source A,B sequence. Distinct comments provide the same schema-valid ordering witness without foreign-extension admission.

There is also a non-crossing version: local A and source B,A at that boundary. Newly unmatched B is appended after local A. A then matches the old local occurrence and is skipped, leaving A,B instead of retaining source B,A. Thus only constraining matched indices to increase would not protect the append-then-old-match case.

Retain both original per-boundary sequences through ordered one-to-one equivalence, additional source occurrences where necessary, or a named atomic refusal. A raw source occurrence cannot be discharged by an earlier local occurrence if the resulting merged sequence violates source order. This is the remaining within-boundary ordering part of D4. It does not expand schema or namespace admission. The six current tree controls do not exercise these crossed or mixed matched/unmatched sequences.

## Prior resolutions

The snapshot at `crates/rpptx-oxml/src/shape_tree.rs:3426` prevents newly inserted source occurrences from satisfying later source occurrences. The consumed `(target,index)` set at `crates/rpptx-oxml/src/shape_tree.rs:3451` ensures each preexisting occurrence is reused at most once. Genuine source multiplicity and different-boundary retention now have defining successor evidence. Target computation and nonrepresentable member-boundary order admission remain before deduplication. D1 source owner metadata/flag/group admission, D2 complete context protection, D3 expanded attribute identity and distinct schema-final extension refusal remain unchanged from the inspected pass-2 corrections.

## Authentication and qualification boundaries

Manifest SHA `f424f15fd738258ddc40a3ba8c529b901100d4afad73e2eef49408092498a30e` matches all 19 immutable current/Base copies and live changed carriers. Source-map SHA `49df56f25066d786ea967582d6a17bc81be468715869b6b069448255e03c8131` matches all 3,296 mapped live bodies, sizes and modes. HEAD matches the claim Base and logical staged-entry SHA matches `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07`. Cumulative patch SHA is `d73735155d554f5ad7726f77cf64304d19634c480495467a6e4fd62def1de87a`.

Current successor-7 receipt SHA `0ce200a9e1e9bf1028bff8ca642a8a2e7ad26c2bf47a6f2124685de6b40cf6a6` matches. Its two raw logs match their hashes. The six distinct named tree controls pass with 0 failures and 0 ignored tests. Strict four-crate all-target/all-feature Clippy passes. Both rows have qualified exit 0 and safe process cleanup facts.

Semantic predecessor successor-6 receipt SHA `9eb6e5b25829ce5b9ea21eb182c8be18ca7f2355dfe687fc10fbb18bec367fdf` matches all four raw logs. The all-target check, six tree controls and exactly the sealed 26 native import names pass. The strict-Clippy row genuinely exits 101 for the collapsible-if lint and remains a failure. No all-green predecessor batch is claimed.

The syntax-only inverse proof SHA `cc06b8132fde32f5bde5aefa54cdf5663256d6e5febd4f61fb425c719d08311a` identifies explicitly reconstructed predecessor bytes, not an original retained body. Those bytes independently hash to the actual guarded predecessor `e701c14689579c7554ad85174dd67cd430e881889eb86f4d776376cdeb5c1cd6`. Comparison to current shape-tree SHA `ede08e2cf2c18942bd0f935c6843be8e8e5988033046272279088f60042cd8b3` shows only the same-condition nested-if to let-chain collapse and corresponding braces. Evaluation order and body are preserved. The 26 import/check evidence is reused through this precise bridge, rather than claimed as newly rerun on source 7. The retained integration binary SHA `15b9ed2152488c9393d6b0e40ced7340d498651f4473cacca38b5a9aaceea9bb` belongs to semantic predecessor source 6, as the manifest explicitly states. Runtime remains the documented 8 MiB default debug test environment.

Genuine multiplicity RED receipt SHA `886b63b51cbc4fe24ad999b47556f433c961cb07293d9c67518ed1f673936c41` and its raw log hash match. The fully qualified named control runs one test and fails with one marker versus expected two, result 0 passed, 1 failed, 0 ignored and actual exit 101. Safe cleanup is authenticated. Earlier REDs, the wrong-key postprocess failure and the withdrawn invalid common-extension fixture remain historical failures and limitations. They are not relabeled successful.

## Smells

0 found in this bounded pass.

## Nitpicks

0 found.

## Acceptance limits

No source, runtime, build, UI, package or Git mutations were performed by this review. The current six tree controls and source-bridged 26 import/check controls qualify their exact paths. Office save/reopen, deterministic corpus comparison, fresh installed Python/types, broader applicable gates and formal handoff remain pending. No full-story acceptance is claimed. The remaining D4 prevents a zero-defect composition verdict. This review ends with the report and no remediation.
