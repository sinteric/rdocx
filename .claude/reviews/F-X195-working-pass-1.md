# F-X195, working, pass 1

**Reviewed**: the complete working change against `2a9bf14072231e46fcb34df70f88cf7aa79fbd79` on `work/f-x195-codex`, 54 tracked files, 14,147 added lines and 963 removed lines. The full diff includes completed prerequisite contributions and the approved plan amendments.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. This is a source review, not feature completion or a new runtime result.

## Authentication and scope

The feature is in progress in CURRENT_SPRINT and BACKLOG. Read the approved F-X195 design, its rendering, architecture, native API, Python, CLI and toolchain references, risk routing, progress and prior scoped reviews. The review covered selected faces and axes, shaping, PDF preparation, raster and searchable SVG painting, private ink bounds, ordered request traces, Word and PowerPoint cache consumers, native caller-font entrypoints, CLI intake, Python bindings, stubs and meaningful controls.

Independently checked every size, byte digest and mode in both current manifests:

- `source317-after-root.json`, SHA-256 `d82f8186c217d85b5414ed43901afcd1ad44a65d67ad5432a49410543550484c`.
- `tracked2679-after-root.json`, SHA-256 `a574afb06d342938fb67df6a9f9c9320c1b5d9786a19c9630b4870395b17329c`.

Both are retained under `/private/tmp/S91-F-X195-logical-raster-causal-test-r51-prepared/root-adoption-1`. No mismatch was found. The full working diff SHA-256 was `06cc605d49f1f0b9ed74ab017c2b91e474c160d3334ccc989e608b9b19a07aff`.

Compared current code and tests with the accepted canonical prefix `40ee5410ed15b1eedab58743b2eddb01bbf93c19`, rather than excluding files by name. Twenty-one changed carriers are byte-identical prerequisite contributions. Thirty-two code, test and manifest carriers differ from that prefix, with 9,599 added lines and 354 removed lines by line comparison. Intertwined carriers were examined in their complete current context. The R7 before-source manifest and full-working diff supply additional retained lineage. A separate receipt for the previously described 27-carrier union was not located. This limits that historical record, not the inspection of current behavior.

## Defects

### D1, searchable SVG changes a regular variable font's default weight

`crates/rdocx/src/svg.rs:169`, `crates/rdocx/src/svg.rs:298`, `crates/rdocx/src/svg.rs:306`

The SVG emitter supplies `font-weight="normal"` for a regular run and only supplies `font-variation-settings` when the stored nondefault coordinate vector is nonempty. The embedded face declaration also advertises normal weight at `crates/rdocx/src/svg.rs:165`. The manager deliberately omits coordinates equal to the font's own defaults at `crates/oxml-layout/src/font.rs:558`, and the output contract explicitly describes the vector as nondefault coordinates at `crates/oxml-layout/src/output.rs:512`.

This is observable for valid existing bundled inputs. Independently decoded physical fvar tables show a default `wght=100`, with range 100 to 900, in NotoSansSC-FX058-subset.ttf, NotoSansJP-F266a-subset.ttf, NotoSansKR-F266a-subset.ttf and NotoSansHebrew-F266a-subset.ttf. Their regular selected instances therefore have an empty weight adjustment vector. Layout, PDF and raster retain weight 100. Searchable SVG advertises normal weight without an explicit weight coordinate.

CSS defines normal weight as 400 and applies the font-weight property to the wght axis before explicit font-variation-settings. A conforming browser therefore selects weight 400 for this SVG rather than the selected default 100. This conclusion follows from emitted source and the normative CSS mapping, not from an unperformed browser rendering experiment. [CSS Fonts, font-weight](https://www.w3.org/TR/css-fonts-4/#font-weight-prop), [variation resolution](https://www.w3.org/TR/css-fonts-4/#font-feature-variation-resolution).

The approved contract requires default retention and matching SVG coordinates at `.claude/plans/F-X195-design.md:124` and `:138`, and explicitly rejects forcing regular CJK fonts to 400 at `:209`. A caller font with another non-400 default has the same failure. Preserve the selected instance when authoring CSS, including effective defaults for axes that CSS style properties would otherwise override. Do not change the manager's default selection to compensate.

The mandatory SVG test does not detect this case. Its static resolver adapter acknowledges that usvg ignores variable CSS at `crates/rdocx/src/svg.rs:3606`, and its effective-coordinate proof authenticates Thai defaults of 400 at `:3632`. Starting that proof from fvar defaults misses the CSS property override for a default-100 face. Add a genuine default-100 searchable SVG control whose effective CSS instance is established under the CSS axis-resolution rules, retaining the existing default-400, selected-700, outline, owner and Q/C controls.

Physical source hashes used for the fvar check:

- SC: `b06144fa7b2d5212fe21344261449c9350f603e3e2ae625e76306022d024fbe5`, 87,480 bytes.
- JP: `03baa76ccf6c13a66f54814e912a90156f8d8ba32ef7b443f7d59031ece65be7`, 14,152 bytes.
- KR: `2bcb03d663d97d3b7db7e0ff3c7a67773130063b13f5c68331dbee9cc8bc5a2c`, 9,608 bytes.
- Hebrew: `b54ddced1bec92db800c88b0aac20988118f92d89fc2113f64c08fa8023ccf04`, 6,176 bytes.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | D1, SVG effective default instance differs from selected instance |
| contract | D1, default retention and matching searchable SVG coordinates |
| panics | 0 findings |
| ooxml | 0 findings |
| tests | D1, default-100 CSS axis mapping is not covered by the default-400 adapter |
| structure | 0 findings |

No additional findings were established in strict lexical font intake, static-face priority, TTC shared source ownership, finite selected coordinates, genuine versus synthetic style metadata, contributing-font-only checked PDF preparation, font allocation order, fail-before-output behavior, scoped whole-painted-run synthesis, raster alpha bounds, or private conservative ink bounds.

No additional findings were established in bounded ordered Resolution and Contribution events, legacy Resolution-only trace compatibility, whole-carrier replay prevalidation, nested capture and final multilingual contribution admission, Word rich-cache refusal, warm cache identity, or current native and Python caller-font dispatch. The source review also checked single-bundle selected page painting, detached filesystem and rendering work, public stub signatures, CLI failures before publication, and retained ordinary versus deterministic defaults.

## Evidence limits and remaining gates

No compiler, Cargo, installed product, renderer or native application was executed in this review. Earlier oracle and runtime-admission preparations and reviews retain their individual scope. They do not substitute for this inspection of the current production behavior.

Retained runtime records include the reported 531, 86, 73, 35 and 2,331 test scopes, two compile-only Word doctests, full installed 104 acceptance, eight SVG owners with both Q/C references, and the separate 19 caller API cases. Historical failed R58 ordinary integration and the subsequent ODT and R62 Word qualifications remain distinct. The live PowerPoint continuation at review assignment is not treated as terminal evidence here. The remaining 24 scoped gates, integrated output attribution, HLD completion, hash review, packaging and workflow completion obligations are still due. Their absence is not relabelled as a completed feature or a runtime pass.

The reviewed production source is blocked on D1. The implementing phase must remediate separately, then obtain a new source review and the actual affected gates.
