# F-X197, all aspects, pass 2

**Reviewed**: Complete frozen R2 working implementation against Base and HEAD `b96c7888b7d99bf1ab6c720ea5ba318c7103d611` on `work/f-x197-codex`. There are 22 existing changed files, 1,546 added and 199 removed lines. Full binary diff is 105,886 bytes, SHA-256 `62337618a743f28d94747da64dd04aee47ea68d3357c070f9378b3176422bfb5`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and scope

Confirmed branch, HEAD, git status and canonical in-progress status in both sprint ledgers. Reused the complete contract, workflow, command and HLD reading from pass1, and read current progress and the R2 receipt. The reviewed plan remains the approved shared extractor, live font colour format, enum, finite brightness, source-preservation and installed binding test contract. No native parser or serializer change is selected.

Authenticated `/private/tmp/S91-F-X197-evidence/implementation-freeze-r2.json`, SHA-256 `43e2538aa9adb82def49edc1cf4336b0f9832e4e7d8f0d33135d7be09e3bade1`, all 22 file sizes and hashes, complete current binary diff, and the 15,767-byte remediation delta SHA-256 `7af4d511e019922d74e68280ab28cc6b973aba8a4cdc45b10c96ff7ba4926ba9`. Reversed every remediation hunk in memory and independently recovered the exact R1 hashes of all three changed carriers: shared support lib.rs, Word test_formatting_tables.py and PPT test_documented_examples.py. The other 19 reviewed implementation files are byte-identical to R1. Rechecked the current guards, diff and HEAD immediately before writing.

This pass covers all six aspects across the full implementation, concentrating additional scrutiny on the three remediations and their regression risks. The original complete pass1 findings and retained evidence remain distinct.

## Remediation assessment

D1 is resolved in source. At `crates/oxml-py-support/src/lib.rs:15`, the explicit malformed-string closure creates the affected parameter's ASCII-hex ValueError. String conversion failures at :20 and ASCII validation failures at :23 share it. Validated slicing and channel parsing remain unchanged, and extraction still precedes caller mutation. The support unit control constructs a genuine lone surrogate through Python chr at :230 and checks the category, full parameter and ASCII message at :238. Installed Word and PPT owner matrices include the six-character surrogate string at `crates/rdocx-py/tests/test_formatting_tables.py:1192` and `crates/rpptx-py/tests/test_documented_examples.py:5055`. They retain source and getter atomicity checks. UnicodeEncodeError's existing ValueError ancestry was not treated as an incorrect broad category in pass1. The repaired behavior supplies the missing explicit message.

D2 is resolved in the test source. `crates/rpptx-py/tests/test_documented_examples.py:5155` defines both unknown DrawingML futureTransform and qualified foreign metadata containing whitespace, an entity, nested child, text and comment. Neither is a modeled ColorTransform. The matrix retains positive exactly-once raw-byte checks before mutation at :5198, applies brightness at :5200, and verifies preservation after save/reopen at :5204. Same-kind replacements retain both raw children and the modeled alpha, tint, gamma and luminance transforms at :5222. Kind changes separately require their disposal at :5233, preserving the approved shadow-alpha exception. The existing native ordered raw-child carrier and production cloning paths are unchanged. These are meaningful source-built acceptance controls, with their actual runtime results still pending.

D3 is resolved in the test source. The Word gate at `crates/rdocx-py/tests/test_formatting_tables.py:1266` now uses the existing to_pdfa_deterministic operation, whose binding calls the native deterministic route at `crates/rdocx-py/src/document.rs:2297`. Rasterization pins Poppler 26.01.0 and 72 DPI at test :1227 and :1232. Bundled Liberation Sans is requested. Source-built theme accents select pure red and blue while the retained literal cache is green. Literal form equivalence, visible red/blue pixels and theme-versus-literal image equality at :1269 to :1274 discriminate theme resolution from a cached-literal fallback.

The additional PPT test at `crates/rpptx-py/tests/test_documented_examples.py:5282` similarly authors controlled red/blue theme accents, verifies reopened source properties, and tests existing deterministic PNG and PDF routes at :5315 and :5316. The binding's to_pdf route delegates to native to_pdf_deterministic at `crates/rpptx-py/src/presentation.rs:223`. Theme and literal equality controls cover both raster backends. Actual solid red interior pixels are selected before asserting positive and negative brightness effects at test :5336 to :5344, so background or antialiasing changes alone cannot satisfy the discriminator. The original literal RGB controls remain.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Shared malformed conversion now follows its explicit error contract. Valid triples, overflow handling, theme mappings and finite brightness logic retain the reviewed implementation. |
| contract | 0 findings. All three pass1 gaps are addressed. Live owner-backed formats, scalar migration, None/auto policies, supported enum vocabulary, cached Word modifiers and intentional bool/NaN divergence remain unchanged. |
| panics | 0 findings. The production conversion change adds no unchecked indexing or expect. ASCII-hex slices retain their exact-length guard. New unwraps are assertions inside the existing initialized Python unit test. |
| ooxml | 0 findings. Native parsing and writing are unchanged. New opaque controls distinguish modeled transforms from true raw subtrees and retain same-kind versus kind-change policies. Theme fixture rewriting uses existing qualified OPC parts. |
| tests | 0 findings. New controls address the concrete missing cases, preserve the previous atomic/source and lifetime tests, and use deterministic render paths with an explicit external version precondition. No actual execution result is inferred. |
| structure | 0 findings. Only three existing carriers differ from R1. The small local malformed-error closure removes duplicated validation construction. No new module, file, trait, generic, dependency, feature flag or public API is introduced by remediation. |

## Runtime qualifications

The R2 receipt records AST checks on 13 Python/stub files, direct support rustfmt and diff whitespace checks. Those are source checks, not Rust compilation, binding import, installed runtime or pixel evidence. No R2 tests have been collected or run. Exact Poppler availability, actual theme rendering, brightness pixels, PyO3 compilation and source preservation remain deciding runtime obligations.

The retained scoped Base equivalence and pinned python-pptx oracle remain qualified as in pass1. The local finite bool/NaN refusal remains an intentional approved divergence. Installed wheels, strict mypy, recursive stubtest, shared-support tests, WASM, hash, packages, HLD completion and final scoped gates are pending. This clean source review does not claim feature completion or a sprint gate.

Only this report was written. No Cargo, binding import, runtime, source, progress, formatting or scratch edit was performed. The pass ends here and returns control for a separate implementation and verification phase.
