# F-X195, acceptance tests, pass 11

**Reviewed**: Native acceptance R2 test-only delta on work/f-x195-codex,
HEAD2a9bf14072231e46fcb34df70f88cf7aa79fbd79. Five existing carriers,
36112-byte delta SHA-256
7d5a7e0bd2d7c660a306b3fcb3b419281cb3479627883610e63463706261a1dd.
The native author is independent of this reviewer. Earlier core/provider
changes and the previously reviewed native production retain their own passes.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. No Cargo, external renderer,
font-tool command, source fix, formatting or runtime test was executed.
These defects block the acceptance queue until separate remediation and review.

## Identity and contract

Read CLAUDE, WORKFLOW, microscope, the full approved195 plan, cited rendering,
native API, CLI, deterministic font, dependency and packaging sections,
differential-testing, progress and native-pass10. Sprint and backlog identify
the feature as in-progress. The source-only receipt has SHA-256
cc82726fc2aa7e6ba7c55369ec2549dd9ec953a65747c49159a8884f2f5f1e4d.

Authenticated all five before/current file hashes, sizes and modes. Independently
reconstructed the exact test-only delta from those retained before files and
current sources. All three production prefixes, every other native R1 guard,
all twelve held Python inputs and baseline9ecae73d remain unchanged. Native
R1 receipt and production review10 retain their original identities.
There is no new dependency, product API, module, test binary or tracked font asset.
The source SVG inventory is exactly25 tests, not25 executed tests.

The independent static oracle receipt has SHA-256
7df18fec40cca6d4a1c2878cb79886ef8a91d7826bd5ebd54c993f040c740668.
All six static400/700 controls, original Thai/Arabic/Devanagari bytes,
LICENSE-Noto and retained hb-subset/hb-shape/hb-view binary hashes match.
The receipt records HarfBuzz14.4.0, exact axis/subset commands and bounded
outline rounding limits. This review authenticates retained evidence, not a
new tool-version invocation or historical13.2.1 parity claim.

## Defects

### D1, both new SVG acceptance tests call the wrong constructor arity

`crates/rdocx/src/svg.rs:1989`
`crates/rdocx/src/svg.rs:2071`
`crates/oxml-layout/src/output.rs:576`

Both LayoutResult::new calls supply only pages and fonts. The actual current
constructor also requires metadata and outlines. The test target therefore
fails to compile before either variable or synthetic acceptance control runs.
Use the existing four-argument constructor in a separate test-only correction.
A compiler failure must not become a font-rendering Base or acceptance result.

### D2, independent SVG references omit the mandatory white page background

`crates/rdocx/src/svg.rs:2002`
`crates/rdocx/src/svg.rs:2092`
`crates/rdocx/src/svg.rs:53`
`crates/rdocx/src/svg.rs:2827`

Production render_page always paints a full white rectangle before text.
Both independent expected SVG strings contain only black text or the black
glyph rectangle, leaving the remaining page transparent. rasterise_with_resvg
starts with a transparent pixmap and performs no common-background composite.
At an untouched corner the generated image is opaque white and the reference
is transparent black. After D1 is fixed, the synthetic max-channel difference
at :2104 is255 instead of at most1, and the variable luminance SSIM at :2008
compares different page backgrounds rather than matching font painting.

Match the explicitly declared page-background semantics on both independently
authored sides, or perform a documented common-background composition before
comparison. Preserve independent static-face/analytic geometry and the700 versus
400 negative discriminator. The existing luminance_ssim helper at :2787 ignores
alpha, so transparent black glyph masks must not be passed to it as though it
measures coverage. A matched opaque background permits meaningful luminance
comparison without a product renderer change.

## Smells

0.

## Nitpicks

0.

## Other aspects checked

Correctness and contract, apart from D1/D2: the variable reference loads the
hash-bound independent static face rather than rerendering production output
as its own expectation. The synthetic reference at svg.rs:2082 uses independently
specified rectangle corners,1/24-em round stroke and baseline-preserving12-degree
shear. It compares actual pixels, checks selected synthesis flags and unchanged
advances. These are meaningful intended controls once their compile/background
preconditions are repaired, not runtime passes in this review.

Native Word selected-format control at document.rs:33977 covers the caller
bundle, two pages, accepted/tracked option propagation, selected PNG/JPEG/TIFF,
PNG/SVG page variants, invalid selected indexes, deterministic PDF and PDF/A,
repeated output and unchanged source. This document has no tracked-change
content, so it proves option propagation rather than a new revision-rendering
oracle. PowerPoint additions in the existing caller-bundle test at lib.rs:12370
cover out-of-range PNG, invalid DPI and caller-font PDF/A, retaining earlier
slide, notes, handout, fit, timeline and animation assertions. Existing consumer
error/source controls remain unchanged. PDF/A byte comparison proves forwarding
the same prepared layout and profile, not independent archival conformance.

Word CLI matrix at rdocx-cli/tests/integration.rs:4981 checks a distinct embedded
caller family and exact selected font bytes/index, different omission layout,
layout JSON, PDF/PNG/JPEG/TIFF convert, PNG/JPEG/TIFF render, unsupported HTML
refusal, repeat layout and unchanged input. PowerPoint matrix at
rpptx-cli/tests/integration.rs:2202 checks the selected caller bytes/index,
different omission layout, four convert formats, three render formats and the
actual320-pixel thumbnail DPI formula. CLI output is compared with the native
selected bundle, which is an appropriate routing assertion. Independent font
geometry is supplied by the separate core/SVG controls, not this byte comparison.

The corrected Word negative command at integration.rs:4896 now supplies
--output-dir for render and --output for convert, matching the actual clap
fields. All other supplied flags are declared. Its stderr assertion at :4908
requires a font-related failure, and it retains sentinel output and source
bytes. The actual render implementation loads font inputs before output staging
at commands.rs:2004. This removes the prior unrelated --output parser failure.
PowerPoint's corresponding font-related assertion is at integration.rs:2129.
These controls have not been executed in this pass.

Panics: new indexing and unwraps construct controlled fixtures or assert test
preconditions. No new untrusted production panic surface was added. OOXML:
production serialization is unchanged, native/CLI tests retain source bytes.
Structure: concrete inline helpers and existing carriers are used, with no
speculative trait, generic, module or dependency. Zero additional findings.

## Remaining qualification

End this pass at the review record. Separate remediation must fix D1/D2 and
freeze the resulting test-only delta before independent review and actual
named runtime checks. No native R2 acceptance, Python wiring, HLD, archive,
hash-baseline movement or feature completion is granted here. The full approved
caller-font, variable/synthetic, entrypoint and scoped rider obligations remain.
