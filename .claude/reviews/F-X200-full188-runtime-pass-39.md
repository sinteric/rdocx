# F-X200, full188 runtime, pass 39

**Reviewed**: Independent complete R26 handover successor against clean R25 runtime pass37 and static handover pass38. Complete runner delta, graph, metadata checks, templates and current source guards. All96 command rows and42 native rows retain their exact R25 identities. No actual producer admission or execution supplied.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, compiled assets outside src are omitted from actual producer differences

`/private/tmp/S91-F-X200-full188-runtime-r26-prepared/run.py:75`

The input filter compares Rust files, files under `/src/` and Cargo manifests. It excludes actual compiled inputs such as `crates/oxml-layout/fonts/Carlito-Regular.ttf`, embedded by `crates/oxml-layout/src/bundled_fonts.rs:33`, `crates/oxml-pdf/assets/sRGB2014.icc`, embedded by `crates/oxml-pdf/src/conformance.rs:11`, and `crates/rpptx/assets/default.pptx`, embedded by `crates/rpptx/src/lib.rs:157`. Existing compiled include_str inputs outside crate directories also need ownership, for example `crates/rpptx-layout/src/context.rs:7305` includes the tracked corpus manifest under scripts.

When the admitted producer has positive compilation evidence for the affected crate, but its embedded asset bytes differ from the current consumer while Rust and manifests agree, the asset contributes no differing seed. The corresponding root and reverse consumers may retain foreign embedded bytes. The unknown-target fallback masks this only while that target is unproved. Proved compilation does not prove equality of omitted input bytes.

Independent metadata-only checking changed only the captured producer SHA for the real tracked Carlito-Regular asset in memory. The complete derived transition and difference inventory remained identical. An analogous Rust input delta is detected. No font or product file was changed or executed by this check.

Include complete tracked compiled input ownership in the actual comparison. A conservative comparison of all tracked files inside selected crate directories covers the local assets. Resolve and guard existing include_bytes/include_str inputs outside those directories, including cross-crate and scripts inputs, and invalidate their consuming roots and selected reverse consumers when their captured bytes differ. Do not substitute a successful compilation observation or source-map hash for this input comparison.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Retained and corrected safeguards

All sealed references authenticate, including receipt795d1bbc7dc553b7ff503a2feaa390dbaaa48ee88585176e5fc25002c4b1020f, helper2ed47edd898351555f1127b9b08682086502ae1df5ff6c99ed454255f884494f, queueff53d31cef71a2b3aedbcfcbcac3d27af9b3b9a614aa083ef4c1f85014b7ee4f and graphd963ad1dcf4cc1425842cc5aad8c9bb1228a441877982a64115b133976bc16b7. All2714 current bytes, sizes and modes match682540f9d394248583ab7ff9dd572aeaf2c31bb16c0100ca6093ec8db0abce49. HEAD and staged index match. All55 recursive referenced files authenticate.

The runner now derives actual Rust differences from the admitted producer map rather than trusting the old17 selectors. Captured manifest texts and root build inputs must agree exactly or require a renewed proposal. Exact raw receipt/row/log/source identities, no active group, safe cleanup and the latest no-intervening boundary are mandatory. Observed eligible local Compiling lines establish only those observed crates. Check-only or cached rows without them do not establish native artifact provenance. Unknown targets enter conservative own-root and reverse invalidation, and the seal's unknown list must equal the derived result.

Independent extracted metadata checks reproduce15 native/build roots and22 required crates. Empty evidence gives22 unknown roots and22 selectors, so stale17 is rejected. A changed actual Rust snapshot is detected and a changed manifest refused. These improvements resolve the analogous unchecked-unknown and stale Rust-map patterns, subject to D1's omitted assets.

All prior runtime functions other than the changed transition have identical ASTs to R25. Original96 command rows,188 installed collection/outcome membership,42 native rows,67 provider/decorator qualifications, four constants, fresh paired wheels, native CLI artifact lineage, typing, publication and policy remain unchanged. Owned-group cleanup, source guards and attempted/terminal provenance remain intact. No actual collection or pass is added by this review.

## Not found

Panics, OOXML and structure produced no findings. Correctness, contract and test-admission coverage have D1 above. No additional defect was found in preserved memberships, cleanup or raw producer identity checks.

## Runtime qualification

Future producer receipt, source, schema review, transition receipt and admission remain null or false. Neither completed source snapshots nor metadata checks are runtime acceptance. No current producer,22-root transition, wheel build, installed188 result, native acceptance or feature completion is claimed.

Only this report was written. No source, packet, environment, Cargo, product import, runtime, cache, timestamp or UI action occurred. This pass ends with D1 recorded and remediation left to a separate phase.
