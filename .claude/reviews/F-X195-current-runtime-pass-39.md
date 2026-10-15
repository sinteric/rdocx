# F-X195, current runtime, pass39

**Reviewed**: Fonts-authored prospective R2 runtime packet, complete428-line runner, queue, admission template and R1 to R2 deltas. Current R29 source producers and mandatory eight-owner native SVG consumer were checked against both concrete artifact schemas. Independent review only.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Do not execute this packet.

## Authentication

Packet: `/private/tmp/S91-F-X195-current-r29-runtime-r2-prepared`.
Receipt SHA-256:
`dab944489aa989ed7080c499a7878b6218f175dbe909b1d9149d5c03aaa1344f`.
Runner SHA-256:
`74ef4fe677fd6552be808d8ce23ac96ff0ead12714256054af2a6bd8feaf876e`.
Queue SHA-256:
`ee4dedab66f50719c124f93a38935db52d016aa08d913438d1aff66e046d68aa`.

All receipt artifacts authenticated by actual hash, size and mode. All2679 current tracked byte/size/mode guards, HEAD and semantic index match. All317 compilation/oracle inputs match their tracked rows. Immutable input and tool hashes match. Runner AST parses without executing it. Current worker HEAD is2a9bf14072231e46fcb34df70f88cf7aa79fbd79. Source-map authorities remain dc070fa626092335410ffe92bb3b3fbe2666ca052c0ef62cccf0abb8cc620173 and b0a031f30ee5dabb5896a11200a72afa94b93934128f417d1b829383fa2f90cf.

## Defects

### D1, style handoff still requires an artifact field that four producers leave null

`/private/tmp/S91-F-X195-current-r29-runtime-r2-prepared/run.py:272`
`/private/tmp/S91-F-X195-current-r29-runtime-r2-prepared/run.py:279`
`crates/rdocx-py/tests/test_rendering_threads.py:1581`
`crates/rdocx-py/tests/test_rendering_threads.py:1583`

The corrected adapter no longer requires expectedSVG for ordinary styles, but still requires a nonempty oracleFont and oracleFontSHA for every owner. The actual style producer assigns static_path only when case['axis'] is nonempty. It consequently emits oracleFont=None and oracleFontSHA=None for genuine-italic, synthetic-bold, synthetic-italic and synthetic-combined. Even if all104 installed cases pass and emit the required eight receipts, artifact_inventory fails its assertion for these four owners before creating the mandatory native admission.

This is a remaining concrete schema mismatch from pass38 D2, not a geometric gate failure. The native consumer already distinguishes present independent static oracle bytes from the absent-field original embedded source route at crates/rdocx/src/svg.rs:3711. Reconcile the actual physical-source provenance for both kinds explicitly. Keep actual SVG, both Q/C references, all eight owners, source/font identity checks and0.99 acceptance. Do not invent paths for absent artifacts or bypass the native gate.

## Smells

None.

## Nitpicks

None.

## Prior findings and aspects

- correctness: One defect, D1. Pass38 D3 is addressed by exact consumed transition admission/head/index/source/helper/queue lineage and actual current byte, inode, size, mode and mtime checks at run.py:309-332. Every command guard repeats them. No additional arithmetic or membership defect found.
- contract: D1 prevents the actual full104 to native SVG handoff. Word56 and PPT48 remain exact collected/executed membership requirements. Both quadratic and cubic references remain mandatory for every native owner. No other contract finding.
- panics and failure handling: Zero additional findings. Pass38 D1 is addressed. Popen is inside the guaranteed cleanup boundary at run.py:204-205. Receipt saving, child waiting and observation failures enter bounded TERM/KILL, group disappearance and leader reaping. Original exceptions remain attached, and unknown cleanup records explicit doNotReleaseLease. Normal successful leader exit with surviving descendants refuses qualification. Post-cleanup metadata failures do not bypass the already-completed ownership cleanup.
- ooxml: Zero findings. The runtime packet changes no product or XML source. Native five-Thai/default-axis and exact eight-owner checks remain intact.
- tests: D1 is the one source-backed artifact-schema defect. Fresh paired wheels, installed native/member provenance, strict typing/stub checks, full104 collected PASSED identity equality and mandatory exact native SVG1 passed/0 failed/0 ignored remain scheduled. No additional test weakening found.
- structure: Zero findings. No product module, dependency, wrapper or separate Cargo target is introduced. The runner retains the existing shared target and bounded stage receipts.

## Execution limits and handback

The latest-producer adapter at run.py:38-68 still accepts only the exact F200 rdocx all-features/all-targets check schema. This is explicitly pending in the preparation and is not execution admission. ROOT reports that R21 subsequently completed two ODT tests. Their real terminal and source/cleanup lineage must be included by a separately renewed adapter before execution. The earlier check cannot be relabelled as that later test terminal, and the template correctly retains authorization false and null latestProducer. No future receipt was fabricated in this review.

R19 retained failed queue provenance is separate from the most recent execution. Its30 qualified commands and terminal semantic101 remain honest historical cache evidence. No prior wheel or retained diagnostic earns current acceptance. Successful staged receipts must bind the actual queue/helper/source identities and fresh wheel prerequisites.

No runner, product import, Cargo, compiler, painter, wheel build/install, cache or timestamp change, UI operation or source adoption occurred. No full104/native acceptance or F195 completion is claimed. Only this review report was written. This pass ends and returns the canonical review writer lease for a distinct remediation phase.
