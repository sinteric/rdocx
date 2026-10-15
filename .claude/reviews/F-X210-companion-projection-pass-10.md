# F-X210, companion projection, pass 10

**Reviewed**: Incremental R2 correction against independently reviewed pass9. Two existing test carriers change in the 1768-byte delta. Nine other candidates and every production operation remain byte-identical.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Scope and provenance

Packet `/private/tmp/S91-F-X210-companion-projection-r2-prepared` has receipt SHA256 `c69e385726246eca821c3352de8729fe905af195918c5c1f5c533f4bf41530ca`, complete diff SHA256 `9b3adc3be449f64d2cba79419bf5955db12ed8fd2e327298c4ce74b72fa867a0` and incremental diff SHA256 `7b07aaf0536ee94edc3f6645f064d9bac9b723346bb7815d52ab2ac8602cf919`. All referenced artifacts and candidate hashes match. All 2909 actual worker byte, size and mode guards still match baseline `02546232c82687cf793ed28af7df790cd7a64535c8d238438833640b7d87b178`.

The workflow, approved design and prior pass9 remain the review contract. Pass9 SHA256 `6d6314d331e4f00931617b1328a8b2c04c94f1ce26d8d669e74b5bea447bd34a` retains the independent companion review and its authorship disclosure. This pass reviews only the new test correction, not an independent re-audit of self-authored historical provider work.

## Defect closure

Pass9 D1 is resolved. At candidate `crates/rdocx/src/document.rs:44068`, explicit `Some(&[])` now requires `crate::Error::Raster(oxml_pdf::RasterError::EmptyPageSelection)` and immediately verifies source bytes remain equal. At candidate `crates/rdocx-py/tests/test_rendering_threads.py:1971`, `pages=[]` requires `LayoutError` with the complete anchored message `raster export error: at least one raster page is required`, followed by source byte equality.

These assertions match actual unchanged production routing. Worker `crates/oxml-pdf/src/raster.rs:182` rejects empty selection. Its display text is defined at line83. Worker `crates/rdocx/src/error.rs:22` prefixes the raster message, and `crates/rdocx-py/src/lib.rs:122` maps raster errors to `LayoutError`. The test carrier already imports pytest at line16. None means all pages, explicit empty remains refusal, and the existing TIFF positive control remains intact. No production policy is broadened to satisfy the tests.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: 0 findings. The correction requires the exact native refusal and existing Python error route.
- **Contract**: 0 findings. Existing page selection semantics and source atomicity remain intact. All companion signatures, payloads and owned diagnostics are unchanged from pass9.
- **Panics**: 0 findings. The invalid success unwrap is replaced by a deciding error assertion in test code only.
- **OOXML**: 0 findings. Both new refusal controls verify unchanged serialized source. No parser or serializer changes.
- **Tests**: 0 findings. The corrected assertions distinguish explicit empty refusal from None and preserve TIFF and all positive payload controls. All other candidate test bodies remain exact.
- **Structure**: 0 findings. Only two localized existing test bodies change. No new API, file, module, dependency, trait or generic layer.

## Evidence limits

R2 remains unadopted, uncompiled and unexecuted. No Cargo, product import, environment change or worker source mutation occurred during review. This clean incremental verdict closes pass9 D1 without granting GREEN or completion credit. Remaining export routes, independent paint, fresh installed wheels, typing, scoped riders, packaging, hash and HLD obligations remain required.
