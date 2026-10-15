# F-X195, Honor caller fonts and variable styles across render paths

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: none

## Problem

Word CLI conversion consumes `font_dir` only for PDF at
`crates/rdocx-cli/src/commands.rs:714`. Raster conversion constructs an
unrelated deterministic layout at `:745`. Word Python PNG, SVG and page
rendering lack caller-font arguments at
`crates/rdocx-py/src/document.rs:2302`, `:2319`, `:2340` and `:2354`.
PowerPoint Python PDF, slide and notes PNG and text-layout methods lack them
at `crates/rpptx-py/src/presentation.rs:220` through `:265`.

The font manager records requested style flags without selecting variation
coordinates at `crates/oxml-layout/src/font.rs:1324`. Metrics and shaping use
default coordinates at `:1276`, `:1427` and `:1734`. PDF ordinary subsetting
at `crates/oxml-pdf/src/font.rs:390` and PNG default-coordinate outlines at
`crates/oxml-pdf/src/raster.rs:955` therefore disagree with advertised variable
Bold names. This plan resolves Issues 296 and 297 as one selection contract.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X195, Honor caller fonts and variable styles across render paths".
- `docs/hld/08-rendering-spec.md`, "The PDF backend", "The rasteriser", "The SVG backend", "Slide text layout as data" and "Performance".
- `docs/hld/10-bindings-spec.md`, "Python API shape", "Native Word facade stability", "Native PowerPoint text layout", "WASM" and "CLIs".
- `docs/hld/03-architecture.md`, "The dependency rule".
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering", "Packaging" and "Dependency policy".

## Approach

Preserve original behavior when caller inputs are absent. Word normal APIs
retain normal font discovery and bundled fallback. Deterministic Word CLI
and PowerPoint APIs retain bundled fallback. Existing caller-only native
layout methods remain caller-only.

Issue313 explicitly adds ordered Python font_dirs inputs. Keep singular
font_dir compatible and add keyword-only font_dirs=None to every listed
render and measurement method in both families. Supplying non-None font_dir
and font_dirs together is a named ValueError before output. Reject a scalar
string for font_dirs rather than iterating its characters. Validate each path
without dropping unreadable entries. Process directories in caller sequence
order and each directory in lexical path order. Later same-rank directory
faces win, then explicit fonts inputs win last. Empty sequences and valid
empty directories retain the ordinary fallback policy.

Directory inputs load in lexical path order before explicit `fonts` inputs.
Explicit inputs load last and override same-family directory faces. Caller
faces override document-embedded and fallback faces. Validate missing
directories and paths that are not directories consistently before output
creation. Use the font's embedded family identity, retaining existing
filename aliases. Empty valid directories are accepted and retain the
selected entrypoint's ordinary fallback policy.

Add a normal-base Word caller-font layout operation:

```rust
pub fn layout_with_additional_fonts_and_options(
    &self,
    font_files: &[(&str, &[u8])],
    options: RenderOptions,
) -> Result<WordLayoutResult>;
```

Use this operation for normal Word PDF, PNG, SVG and raster rendering with
caller inputs. Deterministic Word CLI rendering uses the existing caller-font
bundled-fallback layout operation. Each renderer consumes the exact resulting
owned layout bundle.

Add caller-font native output variants for single PNG, SVG, selected raster
pages and all PNG pages. The variants retain existing render parameters and
add `font_files: &[(&str, &[u8])]` and `RenderOptions` where applicable. Add
keyword-only `fonts=None`, `font_dir=None` and `font_dirs=None` to Word Python PDF, PNG, SVG,
selected and all-page rendering, `layout` and `layout_page`. Preserve existing
positional parameters, revision selection and return shapes. Copy Python font
bytes before releasing the GIL. Perform directory reads and layout and render
work while detached. Keep measurement and previews on the same explicit-input
face selection as PDF. Correct the existing Python PDF typing prose that
incorrectly describes caller-only fallback.

Thread `font_files: &[FontFile]` through PowerPoint's existing preparation and
slide, notes and handout assembly functions. Load caller fonts before resolving
or measuring text. Add caller-font native variants for presentation PDF,
single and all-slide PNG, notes PDF, all-notes PNG and text layout. The variants
retain existing parameters and add `font_files: &[(&str, &[u8])]`. Common
preparation also carries the configuration into handout and timeline paths
without introducing a separate font resolver. Fit checks use the same manager
configuration as PDF and previews.

Add keyword-only caller arguments to every corresponding PowerPoint Python
method, including the same font_dirs ordered input, `to_pdf`, `to_notes_pdf`, `render_slide_to_png`,
`render_all_slides`, `render_all_notes` and `text_layout`. Update installed
typing signatures and method documentation in the same change.

Add `--font-dir` to every CLI text-layout command, including Word `layout`
and `render`, and PowerPoint `render` and `thumbnail`. Apply it to PDF, PNG,
JPEG and TIFF conversion. An output format that cannot consume font bytes
returns an error before producing output. Preserve existing output limits,
selection semantics and staging behavior.

Extend owned output identity:

```rust
pub struct FontData {
    // Existing fields retained.
    pub variations: Vec<([u8; 4], f32)>,
    pub synthetic_bold: bool,
    pub synthetic_italic: bool,
}
```

Retain equivalent private instance metadata in `LoadedFont`. These fields are
additive in purpose but source-breaking for external struct literals. Record
that pre-1.0 source break explicitly as the ownership change required by
Issue 297. Update workspace literal construction sites. Instance identity
belongs to each owned layout, not a global registry. The existing request
keys remain valid because each font ID identifies one exact loaded instance.
Changed font inputs invalidate resolution and shaping state through the
existing manager lifecycle.

Prefer a matching static face. For a variable face, bold selects `wght=700`
within declared axis limits. Nonbold preserves the current default coordinate,
including the intentionally Thin bundled CJK fonts. Prefer a genuine italic
face, otherwise select `ital=1`, otherwise a usable `slnt` coordinate, normally
`-12`. Retain exact selected coordinates and distinguish genuine style from
synthesis. If a restricted axis cannot provide the requested style, use the
verified synthesis path rather than claiming an unavailable instance.

Apply coordinates consistently:

- Call `ttf_parser::Face::set_variation` before extracting layout metrics, advances and raster outlines.
- Construct `harfrust::ShaperInstance::from_variations` and pass the instance into both existing shaping paths.
- Call existing `subsetter::subset_with_variations` when preparing PDF fonts. The current default-enabled feature instances TrueType outlines and converts CFF2 when needed.
- Apply the same coordinates when computing PDF font widths and metrics.
- Emit explicit matching variation settings in searchable SVG text and its embedded font declarations.

Where a requested style has no suitable face or usable axis, use verified
deterministic synthesis. Synthetic bold uses a centered fill-and-stroke width
of `1/24` em with round joins and caps. Synthetic italic uses a 12-degree
shear, with factor `tan(12 degrees) = 0.21255656`, around each run's baseline.
These constants are deliberate implementation choices validated by independent
controls, not claimed defaults of the existing toolkit. PDF, PNG and SVG
implement equivalent geometry. PDF contains scoped fill-and-stroke and text
matrix operations. PNG uses equivalent outline painting and affine geometry.
SVG uses explicit stroke and baseline-relative geometry rather than relying
on browser synthesis.

Synthetic painting preserves shaped advances. Private ink bounds include
stroke-radius expansion and the shear-transformed outline bounds. Independent
geometry controls verify that preservation is correct. If controls demonstrate
a necessary advance adjustment, document and review it before changing the
policy.

Keep embedded font names truthful. A regular source with synthesis is not
presented as an actual static Bold or Italic font. Genuine variable instances
carry their actual selected weight and slant identity. Preserve Unicode
mapping, shaping clusters, static fonts, coverage fallback and font licensing.

Use existing source and test modules. The existing rdocx crate adds only the
already pinned workspace ttf-parser dependency for private SVG selected-face
extraction and instance-aware ink bounds. No package, version or checksum
change is permitted. Cargo.lock may record only this existing dependency edge
in rdocx. The two installed Python binding consumers also use the existing
pinned oxml-layout workspace dependency for strict FontFile::from_dir
loading. Promote the existing Word dev edge to a normal dependency and add
the PowerPoint normal edge. Cargo.lock may record only those existing binding
package dependency-list edges, with no package, version or checksum change.
Word PyDocument.render_pages also uses the existing pinned oxml-pdf workspace
edge to paint one configured native layout bundle, with no new native API.
Cargo.lock may record only this existing rdocx-py dependency-list edge.
Add no other dependency, crate, feature flag or tracked binary fixture. Existing `harfrust` 0.12, `ttf-parser` 0.25 and
`subsetter` 0.2.6 provide the required APIs. The latter already enables
`variable-fonts` by default. Neither WASM wrapper gains a host-directory API.

### Checked PDF preparation

Normal Word and PowerPoint Result-returning output paths use an additive
`oxml_pdf::render_to_pdf_checked` operation. In existing PDF font.rs, prepare
every contributing font once into the concrete prepared-font map, returning
the existing MissingEmbeddedFont error for a missing or failed selected
instance. The checked public operation performs that preparation before the
existing writer allocates output. Archival preflight retains its additional
profile restrictions and passes the same prepared map to the writer. Normal
PDF output does not inherit archival action or paint restrictions.

Keep the existing `render_to_pdf` byte-vector signature. On font preparation
failure it returns an empty vector before allocating output, rather than a
partial PDF or a default-instance substitute. Document that legacy failure
behavior. Native Word and PowerPoint Result consumers use the checked route
and propagate the actual error before file publication. Valid existing output
stays unchanged. Reuse the existing error variant with wording applicable to
normal and archival PDF, without a new enum or dependency.

This extends the core phase lease to existing oxml-pdf lib.rs and
conformance.rs, with inline controls for missing fonts, failed selected
instances, mixed valid and invalid contributing fonts, unused loaded fonts,
unchanged source and no partial output. Native consumer wiring and file
publication controls remain in the later facade phase. No additional file,
module, trait, generic parameter or feature flag is introduced.

## Rejected alternatives

- Changing only font names or style flags leaves metrics and actual glyphs unchanged.
- Instancing only PDF output leaves PNG previews and fit checks inconsistent.
- A process-wide style registry breaks the owned-layout contract and repeated-render isolation.
- Forcing every regular variable font to weight 400 breaks the existing Thin CJK contract.
- New font dependencies are unnecessary because existing versions already provide the required APIs.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | `caller_fonts_are_honored_by_every_cli_layout_and_render_command` | Word layout, convert and render and PowerPoint convert, render and thumbnail use a caller-only family |
| regression | `installed_python_render_and_layout_methods_share_caller_font_selection` | Every affected Word and PowerPoint method and typing signature accepts the same inputs |
| regression | `explicit_font_bytes_override_lexically_loaded_directory_faces` | Directory order is deterministic and explicit inputs win |
| regression | `invalid_font_directories_and_unsupported_formats_fail_before_output` | Missing paths, non-directory paths and unused format options produce consistent errors |
| regression | `variable_bold_changes_actual_outlines_and_matches_measured_advances` | PDF subset outlines, PNG output and layout agree with independent static controls |
| regression | `italic_faces_axes_and_synthetic_styles_draw_the_selected_geometry` | Static italic, ital and slnt axes and synthesis produce distinct verified geometry |
| regression | `changing_same_family_font_bytes_does_not_reuse_a_previous_instance` | Repeated layout and shaping cannot retain stale instance identity |
| regression | `omitted_font_inputs_preserve_existing_static_and_bundled_behavior` | Original no-input behavior and default CJK coordinates remain unchanged |
| integration | `notes_previews_and_fit_checks_use_the_same_caller_faces_as_pdf` | Slide and notes output and measurement share preparation |
| golden | `synthetic_painting_preserves_advances_and_expands_only_ink_bounds` | PDF, PNG and SVG match mathematical stroke and shear controls |

Implement the regression cases in existing crate test modules and existing
CLI and Python test entrypoints. Installed Python wheel tests cover both
families and the exact runtime keyword and typing surfaces. Each listed
entrypoint must have acceptance evidence, including Word selected and all
pages, SVG, layout and layout_page, and PowerPoint slide and notes PDF and PNG
plus fit checks. Cover TTC indices, filename aliases, embedded-versus-caller
precedence, empty directories, selected page ranges, invalid inputs and
repeated rendering. Use a caller-only renamed family that is absent from
system discovery, not an ambient installed font.

Use existing licensed Noto variable fonts for weight controls. Thai, Arabic
and Devanagari have default 400 and weight range 100 to 900. Generate
independent static 400 and 700 controls with exact `hb-subset` 14.4.0 and record
source hashes and tool identity. Compare actual outlines and advances, not
names or production variation metadata alone. Do not claim historical
13.2.1 oracle parity.

Use bundled static four-style families for static-face controls and
regular-only synthesis controls. Generate minimal valid ital and slnt SFNT
controls in existing test modules with independently specified outlines and
metrics. A pinned licensed Lora corpus reference may supplement acceptance
evidence in ignored storage. No new tracked binary asset is needed. The
existing `LICENSE-Noto` and static-family licenses remain unchanged.

Focused checks cover `oxml-layout`, `oxml-pdf`, both native facades, both
CLIs and both Python binding compilations. Installed Python tests provide
runtime evidence instead of linking PyO3 extension-module Rust test binaries.
Run the scoped feature gate and the applicable riders below. Full workspace
verification remains the integrated sprint gate.

### Qualified independent paint oracles

Keep the full104 installed matrix and0.99 positive threshold. PNG and
searchable SVG style controls use independently decoded original/static
physical glyph contours painted by the reviewed standalone resvg0.48.1,
usvg0.48.1 and tiny-skia0.12.0 reference. Preserve quadratic outlines and
independently equivalent cubic controls, source-derived absolute placement,
ROI, notes preview scale,12degree baseline shear and1/24em round stroke.
No product outline, shaper, subsetter, image registration or pixel-fit input
is an oracle. Retain all style, origin, wrong-geometry and shear negatives
below0.99. Both quadratic and cubic paint must meet0.99 independently.

PDF retains independent original/static font-text paint under the same pinned
Poppler26.01.0 at0.99. Do not substitute shared-backend PNG paint for this PDF
proof. Reference resvg and product share a raster backend, so this pipeline
comparison cannot detect a common backend defect. Original mixed failures
and the finite24-owner diagnostic remain retained. They do not satisfy fresh
full104 acceptance or the separate mandatory eight-owner native SVG gate.

Use existing Python test entrypoints and an embedded reviewable standalone
renderer source. Runtime takes explicit hash-bound compiler qualification,
renderer binary and pinned Poppler paths from the admission environment.
No tracked private temporary path, new dependency, module or test file is
required. ROOT authenticates compiler, isolated transitive artifact closure,
source and binary before use. Tests never compile or invoke Cargo. Fresh
paired debug wheels, installed-source provenance and exact current-source
collection are required before the new full104 installed stage. All current variable-font default, selected-instance source, axes, owner,
advance, exact geometry and exact repeat controls remain. PNG, SVG and layout
comparisons retain their existing semantics. Variable PDF versus independently
instanced original/static font paint uses source-derived per-owner ROI at0.99,
with wrong-weight and wrong-origin negatives below0.99. Every embedded used
subset glyph must match independently decoded original/static source contours,
bounds and advances exactly. PDF drawing commands and all pixels outside the
source-derived ROIs remain exact. This qualifies equivalent quadratic point
encodings that the pinned rasterizer paints differently, as established by
bidirectional raw-glyph counterfactuals. No original font substitution,
canonicalization, fitted pixel tolerance or threshold change is permitted.

### Mandatory installed SVG artifact gate

The installed style matrix must pass its actual104 named cases before the
separate native SVG artifact gate. Execute the existing inline
`svg::tests::fx195_installed_svg_styles_paint_authenticated_independent_contours`
test explicitly with `--exact --ignored --nocapture`. Require one actual
passed test and all eight authenticated SVG owners against their independent
paint controls at the unchanged0.99 threshold. The test is declared ignored
for ordinary library runs because its wheel and installed artifacts are
external prerequisites. An ordinary ignore is not acceptance or a waiver.
ROOT admits actual installed terminal, paired wheel/source equivalence,
whole current source bytes/sizes and platform modes, plus immutable artifact
hashes before this serial exclusive Cargo stage. No Python fallback launches
Cargo. Existing full native and publication riders remain required.

## HLD impact

- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/15-build-and-toolchain.md`

## Risk routing

- Layout and shaping, `.claude/skills/risk-routing.md`: deterministic font mode, focused render and metric controls, hash-harness comparison and independent review of every intentional delta.
- Public published API, `docs/hld/10-bindings-spec.md`: declare the `FontData` literal source break, verified `cargo publish --dry-run` checks and `.crate` size assertions for affected published crates, preserving the 10 MiB archive ceiling and source membership.
- Python and WASM graph, `docs/hld/10-bindings-spec.md`: installed runtime and typing tests and `cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm`, including the rpptx rendering profile. Workspace tests exclude both PyO3 binding crates.
- Font controls and assets, `docs/hld/15-build-and-toolchain.md`: verify licenses, source hashes and unchanged packaged font inventory.
- External oracle comparison, `.claude/skills/differential-testing.md`: pin hb-subset 14.4.0 and exact control inputs. Record limitations of geometry comparisons and do not use production output as its own oracle.
- Dependency graph, `docs/hld/03-architecture.md`: preserve format-neutral layering. Only the pinned rdocx to ttf-parser edge, the two installed binding to oxml-layout edges and the rdocx-py to oxml-pdf configured render_pages edge are permitted, including their existing lockfile dependency-list entries. Read the architecture dependency rule and verify unchanged package versions, checksums and inventory. Check MSRV with the pinned toolchain and existing supply-chain gates.
- No-default-features, `docs/hld/15-build-and-toolchain.md`: run `cargo test -p oxml-layout --no-default-features`.

No new trait, generic parameter, crate, module, file or feature flag is
introduced by implementation. The workflow plan itself is the existing
required design artifact.

## Hash harness

Original no-input static and regular bundled rendering should remain
unchanged. Existing variable-font bold runs can intentionally change their
outlines and advances when weight 700 replaces default-coordinate output.
Synthetic output can intentionally change unsupported-style runs. Enumerate
actual affected cases from the deterministic harness, review them
independently and move the exclusive baseline only in the designated sprint
story. Preserve the separate existing Thin CJK oracle contract. Every
intentional rendering delta belongs in this feature's labeled behavior commit.

## Implementation checklist

- [x] Centralize validation and deterministic directory loading in existing files.
- [x] Apply lexical directory-first and explicit-last precedence.
- [x] Thread caller fonts through Word and PowerPoint common layout preparation.
- [x] Extend every affected CLI and installed Python runtime and typing surface.
- [x] Carry owned variation and synthesis identity through font output.
- [x] Apply coordinates consistently to metrics, shaping, PDF, PNG and SVG.
- [x] Implement and independently verify synthetic geometry and ink extents.
- [x] Preserve static faces, fallback, licenses and original no-input behavior.
- [x] Run entrypoint, font-control, packaging, MSRV and WASM riders.
- [x] Review and declare actual harness deltas.
- [x] Update the listed HLD sections.

## Open questions

None. The sprint invocation authorizes the complete Issues 296 and 297
contract. The plan preserves original omission behavior, adopts lexical
directory-first and explicit-last loading, retains default CJK regular
coordinates, declares the required pre-1.0 output-identity source break and
permits only the scoped existing workspace edges stated in Approach. Synthetic constants are deliberate choices gated
on independent controls.

## Contributor intake, PR315

PR [315](https://github.com/tensorbee/rdocx/pull/315) entered the next rolling
refresh at immutable head `af388f255d3b178aaa0aef03313034b815c71eb2`. It closes
Issues296 and297 and belongs to existing F-X195, with Issue313 crediting that
same implementation. No new story is created. Read the complete changed
files, comments and reviews before claiming font work. Reuse accepted loader,
render-path and instance changes, then implement uncovered approved criteria.
Its stated omissions include synthetic italic, bundled variable instances and
caller-font handout/PDF-A paths. Its fallback on subset failure, directory
precedence and synthesis geometry must be reconciled against the approved
plan. Contributor gate claims are not local evidence. Archive measurements
will be generated from actual integrated source.

## PR315 assessment against the approved contract

The immutable head has38 changed files. Required patch and full metadata,
comments and reviews are retained in ignored canonical scratch. No contributor
gate is treated as locally passed. Reuse lexical directory loading, facade
font threading, batched page layout, one presentation font manager, explicit
variation metadata, HarfRust instances and static subsetting scaffolding.

Reconcile these differences before implementation starts:

- font.rs:585 forces regular wght400 instead of retaining default coordinates,
  and :1404 exempts every bundled face from requested bold instancing.
- font.rs:592 uses slnt -14, and output.rs:499 uses a1/30em stroke, instead of
  approved geometry. Genuine/synthetic italic and ink-bound expansion are
  absent. Preserve static style preference and verify clamped axes truthfully.
- oxml-pdf/font.rs:423 substitutes default glyphs after instance subset failure
  while writing instanced widths. Refuse or use verified matching geometry.
- PDF names and SVG CSS must reflect actual instances and synthesis. Prevent
  browser double synthesis and retain approved round caps/joins and shear.
- rdocx-py/document.rs:4006 and rpptx-py/presentation.rs:468 put directory
  inputs after explicit pairs. Approved explicit-last selection still needs
  an actual equal-family/style selected-byte assertion, not a vector claim.
- rpptx/lib.rs:1174 and :10299 pass empty fonts on timeline and handout paths.
  Complete all requested entrypoints including PDF-A and detached file reads.

Contributor default-coordinate and bundled-exemption assertions encode the
incomplete behavior and must not become acceptance. Use the retained licensed
HarfBuzz14.4.0 regular/bold controls with their measured numerical limits, plus
real italic/slant, subset-failure, TTC, collision/cache, PDF/raster/SVG and
notes/handout/timeline checks. Rederive actual local archive measurements.
The minimum subsetter0.2.6 requirement needs explicit manifest review, with
no new dependency or lock delta inferred. Source ownership stays exclusive
after the first wave and table work, with X207 and X210 following this story.

## Single contributing-resolution trace provider

F-X195 also owns the existing bounded paragraph trace/cache event identity
needed by X210 diagnostics. Extend actual contributing measurement/shaping
events with exact requested family, selected face/instance and selected actual
family, retaining current FontId validation. Replay those bounded events
inside the same layout lifecycle, not a historical FontId-to-request map.
Preserve the existing public `finish_paragraph_font_trace` result of
`Option<Vec<FontId>>` and `replay_layout_font_trace(&[FontId])` signatures.
Actual Word and PowerPoint cache carriers consume the concrete request-aware
finish/replay operations from the same bounded trace. The legacy projection
retains its selected-ID semantics, so same-face request inequality is tested
through the request-aware carrier, not the legacy ID projection.
Update actual cache carrier footprint/budget accounting when owned strings
are added. Preserve bounds and overflow bypass. No global diagnostic registry,
new source module or duplicated resolver. X210 owns substitution rules and
message policy, byte-output diagnostic companions and consumer projection.

Require warm/cold Aptos-then-Calibri and reverse ordering, Aptos/Aptos Display
sharing a fallback without attribution collapse, supplied genuine Aptos,
missing supplied glyphs selecting another actual face, probe-only/empty
requests and measurement versus PDF/PNG/SVG/raster exact selected instance
controls. Approximation facts arise only from contributing actual choices,
not a speculative attempted family lookup. These acceptance controls run
under this provider once. X207/X210 consume their reviewed result.

The diagnosed unmatched_requests defect belongs to unintegrated PR319.
Current canonical font.rs has no such map, so it is not a claimed live bug
or a reason to adopt contributor diagnostic code unchanged. Retain actual
request context and selected-instance identity through cache reuse.

## Parallel phase reservation

Start the existing core font-manager/output and PDF/raster/SVG test modules
as a disjoint phase alongside F-X194 and F-X212. No formal dependency is
added or removed. Initially reserve only existing oxml-layout font/output
files and oxml-pdf font/raster files and their inline tests. SVG wiring lives
in existing rdocx/src/svg.rs and remains in the later facade phase. Do not edit
CLI commands, Python/native facade files, shared integration carriers or
HLD/package measurements until root grants the corresponding handover.
Read the retained complete PR315 capture before adopting any slice.

Create meaningful test stubs first, freeze them and queue actual Base controls
under the global Cargo lease. Do not reuse contributor gates or previous
feature binaries as this source's acceptance. The start/hash and intentional
baseline phase remains exclusive and follows F-X194's unchanged gate and
F-X212's reviewed accessibility delta. Independently enumerate and review
font deltas before recording. Later facade wiring consumes completed owners
and is re-reviewed with the complete feature before handoff.

### Complete PR315 immutable source capture

The previously retained patch/metadata is now supplemented by all thirty-eight
complete immutable head blobs, authenticated against the untruncated Git tree,
Git blob identity, size and SHA256. Base versions reference authenticated local
733ce6f7 Git objects rather than duplicate files. All forty capture calls
actually exited0. Evidence is retained at
`/private/tmp/S91-PR315-af388f2-intake/receipt.json`. Full semantic
reconciliation precedes slice adoption. The six frozen core test stubs are
not executed Base or feature acceptance evidence.

## Disjoint contributing-request source reservation

Extend the provider phase to the existing private Word shaping/cache code in
`crates/rdocx-layout/src/engine.rs` and PowerPoint text shaping in
`crates/rpptx-render/src/text.rs`. These are disjoint from the active native
facade and Python binding owners. Preserve original requested family through
private item/language-slot metadata into the actual final successful shaping
event. A concrete request-aware FontManager paragraph operation has two
current consumers, these Word and PowerPoint paths. Keep existing shaping
signatures compatible and place the actual algorithm in one implementation.
Do not add a public TextSegment field, reconstruct a request from FontId, or
introduce a forwarding-only public wrapper.

Replay the bounded contributing trace with validated selected instances.
Account for owned request strings in every touched paragraph, table and
header/footer cache carrier and retain overflow bypass. Empty or attempted
resolver probes do not earn contribution credit. The warm/cold and exact
output controls in the provider section remain mandatory. Freeze and measure
these controls against Base before crediting implementation. This reservation
does not grant writes to native facade, CLI, Python, shared integration,
HLD or package carriers still held by other phases.
