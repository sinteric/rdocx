# F-X216, public and CLI import surfaces, pass 4

**Reviewed**: The sealed five production carriers and two defining test carriers in public-cli-controls-frozen-4, manifestfe1d5f1c41acab285f6ed5175d0304d834637962cd00d9d88f3dcbfd814ba8b5. The seal's699-line Base diff has485 additions and80 deletions. This pass follows public passes1 through3 and closes the path-aware CLI output defect. Native consumer evolution and fresh installed Python acceptance are outside this bounded review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found. Pass3 D1 is resolved by the exact single-call change at `/private/tmp/rdocx-f-x216-codex/crates/rpptx-cli/src/commands.rs:1563`, from to_bytes to to_bytes_for_path(output). Complete predecessor commands hashffae18c2601edeac23a9bced168883357e4ed897161433db85b9449424c2ab93 becomes740668a73e8765b44ef3de321399413a0854efcde0f8a2213467431200559f4b with no other production difference in that carrier. The other four production hashes remain exactly those reviewed in pass3.

## Smells

None found in the reviewed bounded diff.

## Nitpicks

None recorded.

## Aspects checked

**Correctness**: No finding. The serializer now applies output-extension package-class conversion and existing VBA refusal before staged output publication at commands.rs:1565. The template control at crates/rpptx-cli/tests/integration.rs:3465 proves the original input is a Template, imports a real slide, reopens a two-slide Presentation output and compares both original input bytes. The refusal control at :3507 constructs the actual main-part VBA relationship and content type, requires exit1 with the VBA diagnostic, checks no output, checks both original byte sequences, and requires only the two input files in the task directory. It qualifies opaque VBA-bearing package refusal, not executable macro behavior.

**Contract**: No finding. The unchanged frozen Python import path preserves the old single Slide return, keyword-only source-format option, explicit diagnostic companion and ordered batch tuple contracts. Foreign destination-layout identity still precedes stale collection validation at slide.rs:1145, :1295 and :1335. The sealed Python control at tests/test_documented_examples.py:9984 preserves the distinct legacy add and import error messages, checks all five routes, then checks the local stale collection's original cause and byte identity. Fresh execution of that Python control is not credited here.

**Panics and atomic allocation**: No finding. The previously reviewed complete Python result and diagnostic allocation precedes publish_owners at slide.rs:1264. The output-path serializer fails before stage_and_publish for invalid class conversion. Source snapshots, empty-selection no-publication behavior, insertion survivor mapping and successful single-result invariants are unchanged from pass3. No actual Python allocator, overflow or broader native qualification is inferred from these observations.

**OOXML and output**: No finding within scope. The defect repair uses the existing package-class provider required by docs/hld/10-bindings-spec.md:1163. Both inputs and existing outputs remain protected at commands.rs:1536 through :1538. Selection components remain one-based, caller-ordered and duplicate-checked. No raw XML engine, graph transfer, source-theme, notes inheritance, comment, font or media mutation was added by this repair. Those evolving native surfaces remain outside this verdict.

**Tests and provenance**: Both deciding failures and successors were authenticated independently by exact receipt and raw log hashes. Predecessor receiptb39dbabd1c3055045757305a674348dd1bc196fa809ac0d2b3e67b516fcf6af6 records two actual exit101 failures. The template failure reports Template instead of Presentation. The VBA failure reports command exit0 instead of exit1. Neither is a setup or compilation failure. Successor receipt3e9b992bbe5a5fb5d1fe3e0abed68a2cd7f45bb5c4d0ddd708ff55d05111532b records the same exact selectors at exit0, each1 passed,0 failed,0 ignored. All four rows record natural completion, reaped leaders, groupGone and safe lease release. The documented8MiB native test environment and debug profile are explicit. No ordinary2MiB qualification is asserted.

The copied actual debug CLI in the seal matches SHA6423a798f850620046647685097769221267d5b170c1788c7739b646522a995c and70,594,408 bytes. This authenticates the retained artifact, not a new independent replay or whole CLI/native suite. The Base diff and both complete commands preimages match their seal hashes. Fresh current wheel, installed nodes, strict typing, stubtest, allocation refusal, complete native graph controls and Office acceptance remain pending.

**Structure**: No finding. One existing provider call fixes the issue, with no new wrapper, module, file, dependency, trait or generic. The production API, concrete request/result records, stubs and CLI flags remain those already reviewed. The controls use the existing integration entrypoint and actual existing providers.

## Authentication and boundary

All seven sealed carriers, both receipts, all four raw logs, copied CLI and predecessor/candidate commands bytes were independently authenticated. The five live production carriers matched the immutable seal before and after the audit. Evolving native and test work is not represented as a current full-source freeze or completed whole F216 feature.

Only this canonical review file was written. No source fix, build, product runtime, CP, target, Git, network, Office or external mutation occurred during this review. This pass ends here and returns the review lease.
