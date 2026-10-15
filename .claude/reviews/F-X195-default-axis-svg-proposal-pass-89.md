# F-X195, default-axis SVG proposal, pass 89

**Reviewed**: Independent audit of the final sealed R2 proposal for whole-feature working review 1 D1. One existing carrier, `crates/rdocx/src/svg.rs`, complete diff 9,679 bytes. Receipt `5cf5637c5d04ac1742d56ef282be6c250e17c22565d2dcf44062394682afcf21`, complete diff `03b29229ff81a989e946960ed5b7831c6731077010fdb5c12da14a6a141c85e3`, candidate `c7160ce8fe63144ee3ebdf2b155552c866144a61635839d767d7afed7c4e3905` under `/private/tmp/S91-F-X195-default-axis-svg-remediation-r2-prepared`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: The existing bounded selected-face parser supplies physical defaults for all five CSS-sensitive registered axes, wght, wdth, ital, slnt and opsz. Explicit selected coordinates replace those defaults once. Other explicit selected axes remain present. Both font-definition rules and searchable text now emit the same effective vector, including when the manager's nondefault vector is empty. This addresses CSS normal selecting 400 instead of the physical default 100 without changing manager selection.
- Contract: Read the approved default-retention, selected-axis and synthesis contract and whole-feature D1. The proposal preserves selected source bytes, face index, FontData coordinates, metric and outline ownership. Static faces gain no declarations. No provider, PDF, raster, Python, manifest, dependency, baseline or asset change is proposed.
- Panics: The production change reuses validated_font_face and bounded table parsing. It adds no unchecked untrusted offset or face-index path. New unwraps are confined to fixed inline test preconditions. Existing malformed-face diagnostics remain untouched.
- OOXML: This is SVG CSS emission only. No document namespaces, schema order or unknown XML preservation path changes. Searchable logical text and existing geometry emission are retained.
- Tests: Independently decoded the existing physical fvar records. SC source `b06144fa7b2d5212fe21344261449c9350f603e3e2ae625e76306022d024fbe5` has wght default 100. Thai source `5a1c559bb539583c8a1fd99d1c5b9491e5e14478c9cd2bd0970d5c3096cc9ef8` has wght default 400 and wdth default 100. The added inline control begins the effective weight at the CSS normal or bold value, then applies actual emitted declarations. It requires default 100, default 400 and selected 700, unique settings, matching definitions and unchanged source and stored coordinates. The current default-100 branch lacks the required style attribute, giving a meaningful predicted deciding failure, not an executed RED. R2 adds exact static font-definition byte comparison, no static text style, and explicit synthetic normal/normal CSS with default 100, stroke and shear checks. Genuine selected 700 remains distinct. All original tests, eight-owner Q/C controls, all five Thai owners and synthesis negatives remain byte-identical.
- Structure: R2 production is byte-identical to R1. The finite existing helper change and one inline test introduce no public API, module, feature flag or additional dependency. Removing the added test exactly recovers the production-only candidate, and removing it from the test-only candidate exactly recovers the original source.

## Authentication and limits

All artifact hashes and sizes matched. Actual current 317-source and 2679-tracked bytes, sizes and modes matched their pinned maps. The worker source remains the original preimage. R1 remains immutable and R2 is the final reviewed proposal.

No Cargo, compiler, renderer, browser, installed product or native UI ran in this pass. Genuine Base qualification, separate adoption, fresh affected full104 and mandatory native eight-owner Q/C acceptance are still required. Existing usvg limitations are not promoted to browser proof. Default-preserving SVG declarations intentionally change output and require measured hash attribution and the normal labelled behavioral review before baseline changes. This report does not grant runtime or whole-feature completion credit.
