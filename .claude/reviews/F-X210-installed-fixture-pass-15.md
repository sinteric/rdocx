# F-X210, installed companion fixture, pass 15

**Reviewed**: Current installed Word companion control and actual fresh paired-wheel installation under source967978.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, Word fixture calls an unavailable default argument

`crates/rdocx-py/tests/test_rendering_threads.py:1955`

The fixture calls Document.add_paragraph() without text. The current installed API at crates/rdocx-py/src/document.rs:3957 and typed stub at crates/rdocx-py/python/rdocx/_rdocx.pyi:903 require text. Actual pytest therefore fails with TypeError before any diagnostic export assertion. Supply the existing empty-string argument before adding the intended run. This is a fixture repair, not an F-X210 API-default change.

Fresh paired wheel installation and installed module-origin verification both exited0. The actual Word control exited1 with one failed test. Its raw log SHA is6779f0b64d1143ec8e4f401e3318847fbe597bba845c7e2caad7b014e07e1e7c and receipt SHA is703cd3ed5fd376dc2473747f17eacf2d7473e65015be505b22e5116606f4430e. All module origins are within the pinned installed environment. No later PPT or typing gate ran. The earlier installation attempt failed before installation because the default uv cache was sandbox-inaccessible, and remains separately retained. The successful successor used cache-free installation.

## Not found

No product defect is established by this fixture failure. No additional contract, panic, OOXML or structure finding, no smells and no nitpicks.

## Handback

Only this review record was written during review. Return to a distinct one-line fixture correction, independently review it, then execute the actual installed controls and four typing gates without rebuilding unchanged production wheels.
