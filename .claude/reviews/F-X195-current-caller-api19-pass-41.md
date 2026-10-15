# F-X195, current caller API19, pass41

**Reviewed**: Fonts-authored caller API19 external packet, comprising run.py (289 lines), queue.json (183 lines), the retained mechanism excerpt (198 lines) and admission template (13 lines). The eight selected existing test functions and their19 parameterized node identities were inspected. This is an independent review of the new caller gate. I authored the earlier R3 pipeline artifact-schema correction. Independently clean pass40 remains the authority for that earlier correction, not this author's self-review.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authentication

Packet: `/private/tmp/S91-F-X195-current-r29-caller-api19-prepared`.
Receipt SHA-256: `83d5440533d1578925e0789734579140541802cf53ee43603edc0a33ec5011db`.
Runner SHA-256: `273d8962c678b133930361e566d98c3d2c8b9db504bf5167ea7eb6dcf9873e86`.
Queue SHA-256: `9b646e46d03cf20000b3c7cee6eb77c7ddcafeb1a9d81a7e01b6ac4951c7f20f`.

All receipt-listed artifact hashes match. The pipeline queue, helper, receipt, provenance probe and historical R14 receipt match their declared identities. All2679 tracked and317 runtime input bytes, sizes and modes match the adopted current maps. Worker branch is work/f-x195-codex. HEAD is2a9bf14072231e46fcb34df70f88cf7aa79fbd79 and semantic index hash is c964c1c38e50c6b8a84c8ea0f8cf0f5f38bd230cd96a8d542ed4c30272c83db1.

AST source segments match all eight declared function hashes. The actual decorator values establish Word12 and PPT7 exact unique node identities. Their functions are disjoint from the four variable/style functions forming full104. The new runner AST parses. None of these checks executes the runner or imports a product.

## Defects

### D1, pipeline receipts are checked against the caller runner's identity

`/private/tmp/S91-F-X195-current-r29-caller-api19-prepared/run.py:74`
`/private/tmp/S91-F-X195-current-r29-caller-api19-prepared/run.py:225`

The copied prior_receipt validator compares each existing pipeline receipt's helperSHA with sha(__file__). In this new module, __file__ is the caller API19 runner, whose authenticated SHA is273d8962c678b133930361e566d98c3d2c8b9db504bf5167ea7eb6dcf9873e86. Genuine current pipeline receipts were produced by the separately authenticated R3 helper with SHA d285b99abec9ca878f64fc3f3287e30f905a99f3f89ddfcde1643fc50d55a298. These identities necessarily differ.

Consequently, a valid nativeSVG prerequisite fails at main's first recursive receipt validation before collecting or running any of the19 caller cases. Changing the receipt to the caller helper hash would falsify actual provenance and is not an admissible remedy. The prerequisite validator must bind the exact guarded pipelineHelper identity. The caller's own admission and ongoing self-guard must retain the separate caller runner identity.

## Smells

None.

## Nitpicks

None.

## Aspects

- correctness: One finding, D1. No additional finding.
- contract: D1 prevents the required fresh-current paired build/install/full104/native SVG lineage from admitting the caller gate. No additional finding. Current R14 remains historical evidence only.
- panics and failure handling: Zero findings. The retained run boundary encloses spawning, active receipt persistence and normal wait observations in try/finally at run.py:146-180. It checks the whole owned process group, bounded TERM/KILL waiting, ESRCH disappearance and bounded leader reaping. Unproved cleanup records doNotReleaseLease. Original exceptions and actual child exits remain distinct from qualification. The two installed provenance probes surround the selected caller tests at run.py:257-281.
- ooxml: Zero findings. This packet changes no product or serialized XML. The selected embedded-font and caller-font tests retain original document/presentation byte equality checks.
- tests: D1 blocks execution. No additional finding. Exact collection and terminal outcomes require all12 Word and7 PPT cases, with unique membership, no skips and before/after installed provenance. The tests retain caller-only physical faces, deterministic directory precedence, TTC selection, real embedded-versus-caller ordering, invalid intake errors, selected-page ordering and original no-input behavior. Full104 and both Q/C native SVG remain independent prerequisites, not replaced by these19 cases.
- structure: Zero findings. No product module, dependency, public API or additional Cargo project is introduced. Prior independently reviewed R3 closures remain retained rather than reclassified by this author.

## Limits and handback

This is static review of prepared orchestration. No admission, helper, compiler, Cargo command, product import, wheel operation, test, painter or UI action was executed. No source, cache or timestamp was changed. Only this review artifact was written. No current19, full104, native SVG or full-feature acceptance is claimed. The review ends here and returns the canonical reviews writer lease for a distinct remediation phase.
