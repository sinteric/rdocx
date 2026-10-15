# F-X220 public Python review pass 2

**Reviewed:** Frozen D1 correction on `work/f-x220-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Only two production carriers and their two existing test carriers change since pass1. The twelve-carrier public patch now contains1,169 insertions and zero deletions. Seventeen native and eight other public carriers remain exact. This is a separate reader-remediation review, not full feature completion.

**Verdict:** Zero defects, zero smells, zero nitpicks. Pass1 D1 is resolved.

## Defects

Zero findings.

## Resolved finding

The Word binary-reader loop at `crates/rdocx-py/src/document.rs:8594` and matching PPT loop at `crates/rpptx-py/src/presentation.rs:3249` retain the reader's callable and request bounded chunks until an actual empty bytes result. Short nonempty results continue reading. Each result must be bytes and remains borrowed until its cumulative bound is checked. `crates/rdocx-py/src/document.rs:8608` and `crates/rpptx-py/src/presentation.rs:3263` refuse any result larger than the remaining64MiB before copying.

Every accepted append maintains data.len at or below the limit. Therefore remaining subtraction, remaining+1 and needed addition are bounded. At exactly the limit the loop requests one byte to distinguish EOF from over-limit input. Geometric capacity growth is capped at the existing limit and uses fallible reserve before append. A reader that returns more than requested is still checked against actual cumulative bytes. Read exceptions propagate through the call, including failures after earlier valid chunks. Nonbytes later chunks refuse explicitly. Native authoring and publication remain after complete input preparation.

The concrete controls at `crates/rdocx-py/tests/test_core.py:5716` and `crates/rpptx-py/tests/test_documented_examples.py:9319` exercise valid short chunks, later nonbytes output, later OSError and cumulative over-limit output. They assert complete font bytes for success and unchanged package/held owners for failure. PPT also exercises its authored EmbeddedFont constructor through the same reader path. Previously reviewed license, kind, native graph, snapshot, no-op and checked publication mechanisms remain unchanged.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings in EOF detection, cumulative preparation, capacity calculation or exception propagation.
- Contract: zero findings. The change restores the approved bounded binary-reader input without changing native formats, face kinds or explicit license policy.
- Panics: zero findings. Cumulative length is checked before addition/copy, capacity growth is saturating and bounded, and reserve errors become MemoryError. No new unchecked input indexing is introduced.
- OOXML: zero findings in this correction. Native font graphs and settings are byte-identical to the previous qualified phase. Python input preparation creates no XML route.
- Tests: zero findings. Genuine old-runtime failures distinguish all eight new reader cases. Actual new twelve-node and constructor successors assert complete input or atomic refusal, with allocation and alias regressions retained separately.
- Structure: zero findings. The correction stays in the two concrete existing input functions and two existing test entrypoints. No new file, dependency, trait, generic or revision framework is introduced.

## Actual evidence authenticated

Checkpoint3 SHA-256 is `cce11c3dde73c12337f16d1cd5defd41a069ea6323dab015255243a311147d9a`. Exact public patch SHA-256 is `15fe34cc15b791d648e3501bf48bd975962c6d32d47924e38b882ab0ba04b78f`. Authenticated all checkpoint evidence and raw command logs, complete current tracked sources and the exact four-carrier successor delta. This reviewer ran no product, Cargo, package import, CP or Office operation.

RED receipt `4fe4a3f51ee43fe0437fd9e471023819d317fc05d4686d15e1678ec5466186f2` retains actual exit1 and eight real installed runtime failures against the old native binaries. It is unqualified, not a passing result or missing-method failure. The corrected final pipeline receipt `15cd96d5ad8c6fcfcf60888a1e0b5916a3d2de387ee04921311fd466740c5d42` records nine qualified zero rows: third-wheel install, exact12-node collection and execution, both strict mypy consumers, both complete stubtests, six real bytes-allocation refusal successors and three alias successors. Actual runtime summary is12passed0failed. Source and owned installed prefix remain exact before/after acceptance commands, with safe owned cleanup. Initial Clippy correction checkpoint remains distinct from its passing successor.

Independently checked all third-wheel members, RECORD digests/lengths, current Python/stub bytes and native target hashes. Word wheel SHA-256 is `d6b02faaf051d1ad16b74cfd72de75434b3301010796c245b9f533f7668fb50c`, twelve members. PPT wheel SHA-256 is `f93b87345d81aeefe4938eb5c26d750cf6ce879dc675f28d19d45abcd6dbd4b7`, fifteen members. This is isolated installed acceptance, not managed CP or native Office credit.

The actual six allocator cases remain genuine CPython bytes-allocation refusals, with MemoryError before candidate publication, unchanged complete package and held owner, and successful unarmed successors. Result SHA-256 remains `8358a67207702cc1efdf04dfa6189b240c64b3b16bd984bce3865c517e8ddf5c`. The three current alias cases are independently retained at public-reader-D1-alias-results.json, SHA-256 `36a05874da795d9c03979cf9ea69295dd7208344739faea850752ee72e8bf2b5`, with matching actual producer receipt/log.

The original alias result body previously authenticated at pass1 with SHA-256 `4db9d503a1d65053320abb854522471cc5e917e66a7cc001dcf857ce801f4230` was accidentally overwritten and is permanently unavailable. No authentic copy was retained by this reviewer. The original qualified receipt, raw producer log and pass1 authentication remain retained, but this review does not reconstruct or claim access to the lost body. Current successor results have a distinct path and qualified producer, and retention manifest SHA-256 `b89f302df665a4e786d24b8866c353090840680b2637aa486a8935c96da1964c` records the loss explicitly. Historical body loss does not invalidate the truthful separately qualified current successors.

## Remaining boundary

CLI, final HLD/recipes, current verified archives, required producer/corpus/native Word and PowerPoint face-use evidence, root managed CP and full feature/risk/handoff gates remain pending. Office activation remains a separate permission boundary. No feature completion, sprint closure or release credit is granted here. Existing mechanical bibliography metrics propagation remains unchanged and is not bibliography feature work.

## Review integrity

Read canonical workflow/review rules, approved contract, progress and pass1. All3,205 current tracked hashes, HEAD, branch, complete binary diff, exact public patch and physical/logical index remain exact before and after report. Full diff SHA-256 remains `3af4422e48d1fbc57aa000c7df7305a00adef84774bd0591decb8e14d53081ac`. Physical index remains `0de54845b89f71bb5f690d1247df7eb853a1fae5090625b670c03bea1d5c482d`, logical index `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`.

Only this report was written. No source, tests, plan, progress, HLD, ledger, runtime, Cargo, Git mutation, CP, UI or Office changes occurred. This review ends and returns control and the writer lease to the orchestrator.
