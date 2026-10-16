# S95 sprint review, pass 1

**Reviewed**: sprint/s95 at465c9be1c3fedd4d3680f05c818d169f4f3eac7c against main merge base d44a69af2c6e583fad68bfe0b8d4690c830da78d. The integrated delta contains131 files,9065 additions and528 deletions. It touches26 crate directories: oxml-chart, oxml-cli-support, oxml-core, oxml-drawing, oxml-layout, oxml-media, oxml-opc, oxml-pdf, oxml-sml, rdocx, rdocx-cli, rdocx-html, rdocx-layout, rdocx-opc, rdocx-oxml, rdocx-pdf, rdocx-py, rdocx-wasm, rpptx, rpptx-chart, rpptx-cli, rpptx-layout, rpptx-oxml, rpptx-py, rpptx-render and rpptx-wasm.
**Verdict**: 0 blocking, 0 should-fix, 0 nice-to-have

## Blocking

None found.

## Should-fix

None found.

## Nice-to-have

None found.

## Milestone gate

The M24 end gate at `docs/hld/14-development-backlog.md:2668` requires that
"the approved capability matrix has no unexplained partial row" and that the
broad source-built document and pinned Word corpus pass their complete declared
validation, authoring, rendering, preservation and binding checks without repair.
That complete milestone gate is not claimed by S95. F-310 remains the later
stability boundary, and S99 remains its planned home. The applicable S95 boundary
is the nine contracts and definition of done at
`docs/sprints/CURRENT_SPRINT.md:21` and
`docs/sprints/CURRENT_SPRINT.md:49`.

The eight repairs and release preparation have completed feature gates, clean
independent feature reviews and the actual integrated acceptance described below.
Issue384 validation has CLI and both Python entrypoint observations. Typed SDT
values retain independently captured Word evidence and explicit native/schema
divergences. Edited paragraph XML uses the authenticated finite schema and MC
frontier, rather than a renderer whitelist. PowerPoint visibility has retained
native and LibreOffice truth-table evidence plus fresh final output-route
observations. Original Issue386 acceptance now includes the actual paired
19-attempt replay and independently observed16-group companion. Issue385 remains
partial for F-X250/F-X254 in S95.11, as required by
`docs/sprints/CURRENT_SPRINT.md:59`. No Python3.14, full native font/formula parity,
whole M24 completion, registry publication or release-tag acceptance is inferred.

## Acceptance evidence and source applicability

The full integrated union receipt is
`/private/tmp/S95-final-integrated-acceptance-1/receipt.json`, SHA256
`1862e6904d8cd72877b541ee06862edf3be93f470487a8b4416068308167631c`.
It binds actual producer6b7c97987f6edcec5d147116c2778865e289c5fa.
The seven actual receipt hashes and six independent root authentication hashes
in that union were independently reauthenticated during this review.

The final native receipt is
`/private/tmp/S95-final-native-6b7c9798-1/receipt.json`, SHA256
`f972c5a81d0b82e10459abbfb6c42aaa0024a8477bb51716bfbd8dad4de32fdb`.
All18 command exits and actual log hashes were checked. Raw test summaries
independently total6582 changed-default passes with71 ignored,6615 overlapping
all-feature workspace passes with73 ignored and149 no-default-font passes with
one ignored. These are separate executions, not a unique-case sum. Formatting,
workspace and binding clippy, policy, adapters, prose, both WASM graphs, Rust1.93,
documentation, README, supply-chain and publication commands pass. The publication
log contains exactly22 unique Verifying records. All22 retained archives were
rehashed and remain below10MiB. Native raw authentication SHA256 is
`3651f634632e4b8928b27cc399ebaa76ff4c48d61427f51f73710ff6b40c4c05`.

The final ordinary installed receipt records947 Word and961 PowerPoint tests,
1908 total with no failures or skips, plus strict typing and stub checks. Hosted
run38936352909 binds the same source and records27 successful deciding jobs,
four publication jobs skipped and26 unique payloads. Actual downloaded-wheel
priority execution totals172 calls across Python3.9/3.12, with strict3.12 typing
and stub checks. Local workers' earlier wheels and scoped observations retain
their original epochs and are not relabeled as these final executions.

The public supplement retains23 artifacts, including13 Word and10 PowerPoint
outputs, three typed observations and four actual output-mutation rejections.
Final PowerPoint PDF, PNG, CLI convert and render observations preserve the
native flag truth table and exact imported chart values58,52,4and69. Historical
negative mutants retain their original producer attribution and only the exact
reviewed source-applicability exceptions, without a claim of fresh compilation.
The paired Issue386 replay passes19 original attempts and16 separate semantic
groups. The pinned Issue384/385 reporter union records12 XPASS(strict) with
expected raw exit1 and12 passing unchanged bodies under --runxfail with raw
exit0. These independent denominators remain distinct. Its self-catching output
and strict expected-failure contradictions are not presented as raw green.

Commit465c9be1 changes exactly five Markdown files above the actual6b producer:
the F-X255 plan, AS_BUILT, BACKLOG, CURRENT_SPRINT and SPRINT_TRACKER. The exact
diff was read. Independent comparison against the immutable6b tree confirms
all3564 other prior tracked files are byte-identical, including runtime code,
packaged Python, stubs, enum modules, manifests, lockfile, workflows, HLD,
README compile inputs, fixtures and baselines. Root's corresponding applicability
receipt SHA256 is
`6c680bb5e38609331ba578c27c8cd2224e2827ff90bfaa2b04856b990c252335`.
This is bounded completion-only applicability, not another producer execution.

The native full gate at the future committed review HEAD has not run yet.
Committing this report and running that final gate remain root-owned close
preconditions. This review does not assert they have already occurred, and it
adds no authority to merge main, tag a sprint or publish either release family.

## Not found

- **Interaction.** F-X248 namespace propagation remains effective through F-X249 typed SDT handling and F-X252 paragraph admission. Individual SDT error translation stays local, preserving bulk and native contracts. The direct-child XML frontier delegates nested typed metadata rather than freezing it. F-X256 public geometry admission preserves live section EMU forwarding. F-X257 delete-before-readd tab guidance agrees with F-X251 duplicate-position refusal. Relevant source is `crates/rdocx-py/src/document.rs:1268`, `crates/rdocx-py/src/document.rs:5777`, `crates/rdocx-py/src/document.rs:6466`, `crates/rdocx/src/document.rs:8137` and `crates/rdocx/src/document.rs:41715`. Existing installed and native controls remain deciding at the integrated source.
- **Duplication.** No competing owner graph, schema framework or duplicate generic helper was introduced. Local namespace carriers serve distinct parser, checked-edit and source-emission purposes. The shared spacing helper retains the two properties' different sign policies at `crates/rdocx-py/src/formatting.rs:13`.
- **Layering.** Static comparison covers all27 workspace manifests. No oxml crate gains a Word or PowerPoint dependency. Chart metadata remains format-neutral in `crates/oxml-chart/src/lib.rs:343`, with physical package authority supplied by `crates/rpptx/src/lib.rs:11999`.
- **Harness.** No tracked baseline or fixture changes occur in this sprint delta. Actual final logs record49 matching hashes and seven matching deterministic pixel buffers at150DPI. F-X253's master-visibility behavior is separately labeled and native-qualified. The source pass changes ordinary master gating at `crates/rpptx-layout/src/context.rs:283` while retaining the layout gate at `crates/rpptx-layout/src/context.rs:292`. Earlier S93 deltas remain historical, not new S95 changes.
- **Gate.** The actual final acceptance union and raw native logs support the nine-story boundary. Completed plan checklists and nine done sprint rows agree. Recounting actual feature rows, excluding25 repeated table headers, gives603 total,534 done,65 pending and four archived, with zero in-progress. The ledger summaries and45.00 stories/week calculation are consistent. Per-feature effort remains unrecorded, and the80percent throughput escalation is disclosed at `docs/sprints/SPRINT_TRACKER.md:718`.
- **Docs.** HLD04/06/07/08/09/10/12 describe finite source ownership, chart, visibility, numeric, enum and language contracts. HLD10/15 preserve source availability, close and separately approved publication as distinct stages. F-X255's completion entry at `docs/sprints/AS_BUILT.md:20340` preserves actual producer epochs. Issue385's remaining scope and M24's later end gate remain explicit rather than silently closed.
- **Deps.** Manifest semantics, feature/default selections, workspace membership and external lock records remain identical after normalizing only the approved internal version carriers. No new external dependency or feature requires a consumer justification. The separate unpublished PowerPoint WASM version remains0.12.1.
- **Surface.** Added narrow OXML/facade queries, Python diagnostics, mapping operations and registered-ID/language projections match the approved plans. Only the two explicitly approved enum product modules are new. Word and PowerPoint color getter migrations, optional chart type and existing string setters are accurately distinguished in runtime, stubs and release notes at `CHANGELOG.md:28` and `CHANGELOG.md:68`. No unrequested public provider or mutable snapshot graph was added.

## Review boundary

Read-only source, Git-tree, manifest, receipt, archive and raw-log analysis only.
Earlier private preparation informed source analysis but supplies no formal
acceptance or review credit. No product tests, builds, network, GUI, cache,
source, state or ledger mutations were performed during this pass. Only this
required review file was written. The pass ends here and returns control to
root. No confirmation review is required without new material.
