# F-X201, qualified selection correction, pass 18

**Reviewed**: Immutable qualified-selection-correction-r1, exactly three existing carriers against the pass17 source and lint successor. This is a bounded remediation review of pass17 D1, not another whole-feature review. Reviewer: fonts_resume, independent of this correction.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Evidence

Capsule receipt SHA-256: `d6d0cdd3390c75741624798a396b938621afd12423328fb8a1ffe185e5c62bea`. Exact incremental diff SHA-256: `bc14f1d3279fdc3c02aabefa5549cc9f2a9f91c7bc4714099bb8fe38a155b63f`. Full 3,076-input source map SHA-256: `d963a278003e6badf0934ef8b4618b7a668ed9b8d84d704d9d9f01ba54cc12e0`. All before and after carrier hashes authenticated. Comparing full maps confirms only drawing.rs, header_footer.rs and the existing regression entrypoint changed. The other 3,073 records are unchanged.

Drawing source changes from `e160bf05bc3443e63b8eb7a024f3ed8e1d1a3a3633f4abb15d191a4a14de38c2` to `d9de37183bb96b51cfb2cd82e14744153d6d04efd798ad21151ad39e76fcd6ca`. Header source changes from `e49a52472822fe73a1af503cba075f821f2d486904f2ba4c5acee7442d677cb1` to `cea95c1ce528a0ecda7f0096a778147549beb650454e01fda458f63c07685823`.

All six recorded authority hashes authenticated. Actual five-row receipt SHA-256: `513944b3dec3ab738ca7fc256a8ce988a8aac40954f00ad3cd4a6d9b930445e5`. All five raw logs match recorded hashes. The native/binding four-crate check exits 0, DrawingML module tests pass 24 with 0 ignored, header/footer module tests pass 25 with 0 ignored, the exact effective-watermark paint control passes 1 with 0 ignored, and four-crate Clippy exits 0. Every owned group records safe natural cleanup and ESRCH. Earlier failed attempts remain retained.

## Defects

None. Pass17 D1 is resolved in this exact correction.

At `crates/rdocx-oxml/src/drawing.rs:354`, the existing selector closes inherited namespace bindings only for projection and resolves expanded element names. It admits the qualified MC root, direct qualified MC Choice, direct Word drawing and exactly one qualified WP anchor/inline owner. A foreign/rebound Choice or WP owner cannot become the DrawingML projection that caused the previous disagreement. Scope collection retains actual declared bindings, and the existing reader still receives the physical drawing fragment. Original complete MC source remains the separately retained write-back authority.

At `crates/rdocx-oxml/src/header_footer.rs:1053`, VML suppression now calls that same selector. If no DrawingML owner is selected, it reads only the direct qualified MC fallback with inherited bindings closed for projection. The duplicate admission policy is removed rather than maintained in parallel.

At `crates/rdocx-oxml/src/header_footer.rs:1352`, the existing control now checks simultaneous DrawingML and VML inventories for the positive and rebound cases, including empty DrawingML inventory when fallback is selected. It preserves raw alternate-content bytes on save. At `crates/rdocx/tests/regression_test.rs:399838`, the real styled watermark cases rebind Choice or WP, require no DrawingML projection, inspect exactly one actual Text/MultilingualText paint and half-opacity SVG, and compare the saved header bytes exactly with the modified input. These controls exercise the actual shared selection seam instead of checking the VML inventory alone.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation/schema: 0. Tests: 0. Structure: 0. This correction adds no module, trait, dependency or separate public selection abstraction.

## Limits

The approved half-only authoring boundary, primary WPS/native WordArt fallback generation and earlier raw-owner checks are unchanged and retain pass17 authority. The original native Word full-range failures remain unresolved and receive no credit. Current canonical half-opacity Word/LibreOffice evidence remains its own qualified input authority. Earlier full native, installed and typing receipts predate this selector correction and are retained honestly, not relabelled as fresh execution. Current wheel renewal and eventual integration still require their own qualification. No source, runtime or environment was changed in this review, and no whole-feature completion is claimed.
