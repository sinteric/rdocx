# F-X195, working, pass 2

**Reviewed**: the complete current working change against `2a9bf14072231e46fcb34df70f88cf7aa79fbd79` on `work/f-x195-codex`, 54 tracked files, 14,300 added lines and 963 removed lines. This includes prerequisite contributions and approved plan amendments.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. This is a source review. It does not claim feature completion or a new runtime pass.

## Authentication and independence

Read the approved F-X195 design, cited architecture, rendering, bindings and toolchain contracts, risk routing, current sprint and backlog status, progress, whole working pass 1 and incremental proposal pass 89. F-X195 remains in progress.

Independently authenticated all current byte digests, lengths and platform modes in both adoption manifests under `/private/tmp/S91-F-X195-default-axis-svg-remediation-r2-prepared/root-production-adoption-1`:

- `actualSource-after.json`, 317 entries, SHA-256 `2e79ea12c425eb2d56fcaab541040072201b0eaf335f4af085abf98bbffb7f82`.
- `actualTracked-after.json`, 2,679 entries, SHA-256 `d9f0377fc87a4cf8b0eb132c501e9c15010b7ee0a6e7ae58b48ec4cc65010eff`.

No mismatch was found at the end of inspection. The complete working diff SHA-256 is `170cca7261664b1aa44b7a50d3e377af706746330f37894de13352c2bb1065bb`.

Compared every modified code, test and manifest carrier against accepted canonical prefix `40ee5410ed15b1eedab58743b2eddb01bbf93c19`. Twenty-one carriers are byte-identical prerequisite contributions. Thirty-two differ, with the feature plan counted separately. This comparison uses actual Git object bytes rather than filename exclusions. Intertwined Word visibility, table and colour prerequisites were retained in context when inspecting the current font consumers.

The reviewer is independent of the fonts-authored default-axis SVG correction and current font/provider production implementation. Historical participation includes external runtime adapters, oracle preparation and review, and test-helper remediation preparation. Retained test segments previously authored by this reviewer do not acquire new independent authorship credit through this pass. Their prior independent reviews and actual oracle qualifications remain separate evidence.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Previous D1 and the current correction

`crates/rdocx/src/svg.rs:928` derives registered CSS axis defaults from the exact physical face and face index. It freezes `wght`, `wdth`, `ital`, `slnt` and `opsz` defaults before overlaying selected nondefault coordinates. An existing coordinate replaces its default entry rather than producing competing declarations. Other explicitly selected axes remain represented. The layout's original font bytes and nondefault coordinate vector are not mutated.

Both embedded font rules and actual text use the resulting settings. Default weight 100 consequently overrides CSS normal weight 400 without changing the font manager's intentionally Thin selection. Default 400 and selected 700 retain their respective instances. Static faces with no selected variations retain their old declaration and text style. Synthetic painting still disables browser synthesis and uses its explicit stroke and baseline shear over the selected physical outlines.

The existing inline discriminator at `crates/rdocx/src/svg.rs:1473` establishes physical fvar defaults for actual SC and Thai source bytes, checks emitted text attributes and declarations, resolves the CSS weight override explicitly, and requires distinct default-100, default-400 and selected-700 results. It also retains exact static declaration behavior, source immutability and the synthetic default-100 stroke/shear controls. This addresses the source defect and the missing deciding control recorded in working pass 1. It is not a browser or installed paint acceptance claim.

The equivalent lint correction at `crates/oxml-layout/src/font.rs:1164` uses `size_of_val(staged)` for the same slice allocation term. The earlier checked byte calculation establishes that the length-times-element-size term fits before reaching this subtraction. Event ordering, owned request-string accounting, budgets and capture behavior are unchanged.

## Six-aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Physical face identity and selected coordinates remain consistent through metrics, shaping, subset preparation, raster, SVG and synthesis. |
| contract | 0 findings. Normal Word fallback, deterministic bundled fallback and existing caller-only construction remain distinct. Ordered caller directories, explicit last precedence, strict path errors, all requested native and Python routes, and CLI publication boundaries remain represented. |
| panics | 0 findings. Reviewed new selected-instance preparation, checked source ranges, font lookup, trace budget arithmetic, TTC extraction and the justified post-preparation PDF expectation. |
| ooxml | 0 findings. Rendering consumes staged or immutable owners, keeps source mutation separate, and retains the accepted visibility and colour prerequisites. No newly established raw XML loss or sequence violation was found. |
| tests | 0 findings. Deciding default-axis controls, physical-byte and face-index controls, cache request/order controls, independent font and Q/C references, selected-owner placement, negative geometry cases, repeat output and source preservation remain strict. |
| structure | 0 findings. No new module, crate, trait, generic parameter, forwarding framework or feature flag was introduced. Existing concrete trace and preparation carriers have actual Word and PowerPoint consumers. |

## Complete implementation scope checked

The core inspection covered strict lexical font intake, embedded-family and filename alias behavior, matching static face priority, usable weight and italic axes, fallback synthesis, selected-instance ownership, shared TTC bytes, HarfRust and ttf-parser coordinates, font widths and metrics, contributing-font-only checked PDF preparation, legacy no-partial-output behavior, stable PDF allocation order, scoped PDF synthesis, raster transforms and private ink bounds.

The provider inspection covered bounded Resolution and Contribution events, final requested-family ownership, legacy resolution-only compatibility, whole-carrier replay validation before mutation, nested captures, Word paragraph/table/header-footer caches, PowerPoint shaping caches, language and allocation accounting, final multilingual contributions and ordered current-layout physical identity. Warm caches do not gain an alternative font selector.

The consumer inspection covered Word normal and deterministic layout bundles, revisions and selected pages, checked PDF and PDF-A, PNG, SVG and raster output, PowerPoint slides, notes, handouts, timeline, animation and fit measurement, both CLI font-directory routes and staging, detached Python filesystem/render work, exact keyword signatures and stubs. The new dependency edges use existing pinned workspace packages. The shared-to-format layering remains intact, and bundled font assets and legal-file inventory are unchanged. The public FontData literal source break remains explicitly declared by the approved plan rather than treated as source compatible.

## Runtime and completion limits

No Cargo, compiler, installed product, renderer or native application ran in this review. ROOT reported the genuine default-axis Base failure and started the exact corrected GREEN stage. That live stage is not credited as a terminal pass here. Historical failed wrappers, their narrowly authenticated successful child results, and subsequent continuations retain their original statuses and scope.

The correction changes SVG production, so retained earlier wheel and paint results alone do not establish fresh current-source acceptance. Required actual full104 installed cases, all eight native SVG owners with independent quadratic and cubic references, caller19, affected scoped gates and their current-source qualifications must be completed or renewed through the reviewed runtime path. The mandatory ignored SVG test is not credited by an ordinary library ignore.

Remaining clippy and other riders, MSRV, no-default-features, both WASM graphs including rendering, deterministic49 attribution, verified published archives and size/source checks, read-only registry policy, listed HLD updates and workflow completion remain required. These pending obligations are not newly manufactured source defects and are not waived by the clean review.

The review phase ends with the authenticated source and diff identities above. No implementing remediation occurs in this pass.
