# F-X216, native composition, pass 6

**Reviewed**: Limited new delta from the zero-finding native composition seal 5 to immutable seal 6, against approved plan `3eb5c671ecf74d67f1de64c2660e6bf7cd40596a5cb93f5abf0e4801fd409baf`. Changes restore ordinary duplicate model-source handling and numeric relationship remapping, correct three regression fixture/oracle purposes, preserve imported jump payload through qualified attribute-value edits, and add current native/CLI acceptance controls. The seal retains 19 current/Base carriers and its working patch has 13,202 lines. Prior bounded native/public reviews do not imply later Office, installed or corpus-render acceptance.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0 found in this limited new-delta review.

## Production review

`crates/rpptx/src/lib.rs:8423` and `crates/rpptx/src/lib.rs:8535` now obtain slide and notes XML from the actual modeled owner for ordinary `import=None` duplication, while genuine imported operations continue using the once-staged source package. This restores the Base path that preserves actual dirty model edits. It does not replace source-format staged XML with a stale typed projection. `crates/rpptx/src/lib.rs:17474` limits numeric-ID reassignment to the bounded ordinary-copy path and records the real old/new ID map. That map is passed to source-preserving relationship rebind at `crates/rpptx/src/lib.rs:17515`, rather than an empty map. Original relationship metadata and external mode remain retained.

The private jump helper at `crates/rpptx/src/lib.rs:16058` uses decoded namespace identities for DrawingML hyperlink elements and relationship-ID attributes. It rejects ambiguous target attributes, selects only IDs supplied by the actual unmapped-jump operation, and edits only their value bytes and the unqualified owned action value. A missing action gets the exact noaction attribute. It retains all other attribute spelling, payload, sound, tooltip, opaque descendants and unrelated/foreign nodes. Source span checks precede edits and reverse-offset application preserves remaining bytes. Its BOM coordinate correction, entity-equivalent namespace/ID recognition, no-op and fixed-point behavior have an exact byte oracle in the named unit control.

`crates/rpptx/src/lib.rs:16210` retains the existing ownership check after neutralization and refuses if a released ID still occurs outside the admitted hyperlink role. Removal remains limited to relationships no longer referenced, with source-preserving relationship rebind and candidate publication downstream. The mapped notes jump keeps its destination and the unmapped jump keeps the click payload while releasing only the actual action target. Actual notes back-link and sound ownership have a separate native control.

No new type, module, file, dependency, public API, quota or runtime-stack change is introduced by this delta. Previous D1-D4 notes materialization corrections are unchanged from zero-finding pass 5.

## Regression authority review

The oracle corrections implement the separate review `F-X216-regression-oracle-pass-1.md`, SHA `dbe7961881b74251b4d0ed07550170d9eb9c169769d677294a8358023f01fa0f`.

The renamed legacy test retains real missing-layout, invalid explicit layout/index, complete destination byte rollback, explicit-layout and SmartArt checks. Obsolete chart/jump refusal fixtures are replaced by real authored-chart transfer and selected/existing/self/unmapped jump controls. Unsupported internal types and missing reachable graph parts retain exact error/owner and byte rollback negatives. No invalid unqualified chart fixture is promoted into valid chart evidence.

The two checked XML-owner fixtures now create one controlled source extension-list reservation outside the selected run, explicitly assert the producer has no preexisting list, and require source validation to be empty before capturing the save sentinel. Both complete target/mode and image-decoder matrices remain intact, including allowed PNG/JPEG, truncated JPEG, CRC/trailing PNG, unsupported SVG, short signatures and MIME mismatch. Incoming owned XML validation remains independent of the foreign reservation. Refusals still assert complete byte rollback and positives still save/reopen. Save assertions are not disabled and the tests are not ignored.

The two fixed corpus merge inputs are used for count/order/input-byte/package validation controls. That evidence is package/native-library comparison only. It does not substitute for the still-required source-format render metric or PowerPoint save/reopen protocol.

## Authentication and actual results

Manifest SHA `acb0edf6733325d4505217d57fbd370730d81d43361998c292bc4be5ab8f878d` matches all 19 immutable current/Base copies and live changed carriers. Source-map SHA `5343ec9764508de7798d76ea460c7b150085830a9ae7c1be9d2a32d680128429` matches all 3,296 live mapped bodies, sizes and modes. HEAD remains `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. Logical staged-entry SHA remains `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07`. Patch SHA is `fd08e0c4523a4c0f12fb7d6cd6ada7e64f5e0c0d6878ab64a95af0324cf95dfb`. Current lib and integration SHAs are `02ba10adfc7aaac39951b3873c38fda4da624645d9ca5cea11168bd2c4c1b1b4` and `41510e8638a99dd128af957eb65d1aaf781eb93f2c663d1b09431e3fbbe2dd3b`.

Current `actual-native-contract-acceptance-2/receipt.json` matches SHA `7d74ab711197b822baf5662a096bf3758d9b083f4bdee119841d5fee631d6595` and its source SHA matches this seal. All five row log hashes match, with exit 0, qualified status and safe cleanup. The actual native log contains exactly the sealed 30 distinct names, 30 passed, 0 failed and 0 ignored. Eight named tree controls pass. The fully qualified neutralization unit name is present, running 1 test, result 1 passed, 0 failed and 0 ignored. Its raw log SHA is `10a40c0f29d2f35b972505f9d8991197251c987a1b2ba3ea9e9ee08963b195e5`. All-target/all-feature check and strict four-crate Clippy pass. Retained integration binary SHA matches `90f3be54d7cabf7514ec5d1a6fb4698f7bb567f2a4fdfed1f26eff6e354ed920`. Runtime remains the documented 8 MiB default debug native test environment.

Authenticated supplemental receipts retain distinct source points, not a false all-current-source aggregate:

- `actual-duplicate-default-successor-1`, SHA `681cf87d2dd6147af4f3188a8e79241c2fac29ff5661a67abb2d9e304866f2cd`, executes each of the five actual failed duplicate/text/image/save regression selectors once. Every row reports exactly one named pass and safe exit 0.
- `actual-native-oracle-successor-1`, SHA `cb58d0fff3a297d78f3f144ebb459274a80dd25e37a19d14ee2f3f19dd542954`, executes the two complete checked-owner matrices and the revised layout refusal test, each exactly one named pass and safe exit 0.
- `actual-cli-contract-acceptance-1`, SHA `1e1fa9160c8be2eae685cc35dd8ae15f47d54cf239ce54fe5708a0a3e25a9c58`, executes three real CLI output-class/VBA/range/input-atomicity controls, each exactly one named pass and safe exit 0.
- `actual-notes-jump-contract-1`, SHA `b27463d83b820393b5c82d00b91cf05c35a853b7085468e8c44577c475c29531`, retains the genuine payload-loss failure, exit 101 and unqualified status. `actual-notes-jump-contract-6`, SHA `913e6a515ece55c0bc64308c2857fc88d4f09fb14f26a822458eaeec4fcbd682`, retains successful exact lexical/unit and notes-payload successors. The current full 30-name receipt independently includes that notes control.

All supplemental raw log hashes match their receipts. Earlier 369-pass, 14-fail, 24-ignore broad integration row remains failed. Missing fixed native oracles, sandbox UV-cache failures and intermediate compile failures are not relabeled passing. These scoped successors do not claim a new complete broad-suite run.

## Smells

0 found.

## Nitpicks

0 found.

## Acceptance limits

The reviewer executes no tests, builds, binaries, package/runtime operations, UI, CP or Git mutations. No additional finding was established in this limited source restoration, qualified lexical edit, fixture correction and focused-control delta. The zero verdict closes this review scope only. Current Office save/reopen, actual source-format deterministic comparison, fresh installed Python/types, remaining applicable gates and formal handoff remain separate obligations. This report claims no full F-X216 acceptance. The pass ends here and returns the writer lease.
