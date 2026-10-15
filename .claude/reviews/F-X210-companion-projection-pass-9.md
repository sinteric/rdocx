# F-X210, companion projection, pass 9

**Reviewed**: External unadopted companion proposal, 11 existing carriers, 604 added and 14 removed lines. Packet `/private/tmp/S91-F-X210-companion-projection-r1-prepared`, receipt SHA256 `65cef2c56959c496d760e607dbebd52e7eb208b5f2300c8e031cbdb20ee56de5`, complete diff SHA256 `8dd14116a068e9bfa2103696e24d93a86fcfe2f938f46fd32c9be5ffbe3837d4`.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Scope and provenance

Read the workflow, microscope contract, approved F-X210 design including the same-pass companion contract, progress and relevant rendering and binding specifications. The worker is `work/f-x210-codex` and the story remains in progress. All 2909 actual baseline byte, size and mode records match the ROOT R2 adoption map SHA256 `02546232c82687cf793ed28af7df790cd7a64535c8d238438833640b7d87b178`. All 11 candidate hashes match the sealed receipt, the prospective map changes exactly those 11 records, and the patch passes a read-only application check. Both Python test carriers retain every prior module AST node and append one function each.

This pass independently audits the new companion, projection, CLI and test delta. Historical F195 provider work and initial F210 provider work include reviewer authorship. Their prior independent review authority remains applicable, including provider review8. This report does not claim an independent whole-provider re-audit.

## Defects

### D1, explicit empty page selection is asserted to succeed

`/private/tmp/S91-F-X210-companion-projection-r1-prepared/candidate/crates/rdocx/src/document.rs:44068`

`/private/tmp/S91-F-X210-companion-projection-r1-prepared/candidate/crates/rdocx-py/tests/test_rendering_threads.py:1968`

The native test calls the new raster companion with `Some(&[])`, unwraps its result at line44070 and expects an empty page vector. The installed test similarly calls `pages=[]` and expects `[]` at line1969. The existing raster contract explicitly rejects an empty selection with `RasterError::EmptyPageSelection` at `/private/tmp/rdocx-f-x210-codex/crates/oxml-pdf/src/raster.rs:182`. The companion correctly delegates the supplied selection unchanged to that renderer at candidate `crates/rdocx/src/document.rs:28976`. Consequently both new tests fail on the intended implementation. Assert the existing refusal and retained source atomicity instead. Preserve the distinct `None` all-pages and TIFF positive controls. This is a test expectation defect, not a reason to broaden production selection semantics.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: 0 additional findings. Each companion derives diagnostics from the same owned layout used for its encode or paint. Word retains normal fallback policy and PPT retains deterministic caller and bundled policy. CLI routes emit ordered messages from the successfully published bundle without another render.
- **Contract**: 0 additional findings. Native arguments retain caller font slices and options. Python companions preserve the existing parameters, F195 keyword inputs and payload shapes. The public frame field's pre-1.0 struct literal impact is explicit. Notes, handout, timeline, direct-slide, archival, fit and additional CLI routes remain stated pending work in packet `proposal.md`, not completed acceptance.
- **Panics**: 0 additional findings. Production additions introduce no untrusted indexing or unwrap. Existing optional PNG and raster refusal propagation remain intact.
- **OOXML**: 0 findings. These additions consume layout snapshots and leave source XML, namespaces, schema order and unmodelled content unchanged. Deciding controls retain source byte equality.
- **Tests**: D1 above, 0 additional findings. Word observes the existing actual layout counter. PPT resets the diagnostic sink before each shape measurement and copies the result into the returned frame, distinguishing Aptos from subsequent Calibri and checking repeated frame equality. Native and installed payload comparisons use the existing byte APIs. No unexecuted test receives runtime credit.
- **Structure**: 0 findings. All changes use existing carriers and concrete types. No new module, file, dependency, trait, feature flag or generic forwarding layer appears.

## Evidence limits

No Cargo command, product import, runtime, environment change or source adoption ran in this pass. The proposed tests remain uncompiled and unexecuted. Existing producer Base evidence is retained as prior evidence, not a new pass. Full font paint, export-route, installed wheel, typing, scoped rider, hash and packaging obligations remain required before completion.
