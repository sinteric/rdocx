# F-X195, Python and SVG, pass 16

**Reviewed**: frozen R7 working implementation and controls. The full working
snapshot is 645970 bytes. The R6-to-R7 inventory changes 30 existing paths,
with 287 unchanged. The separately retained SVG remediation is 10813 bytes.
Focus is the novel thirteen Python methods, their installed signatures and
controls, and SVG remediation. The independently reviewed canonical
194/197/212 carrier union is preserved, not credited as new F-X195 work.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Authority

Worker work/f-x195-codex, HEAD
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. F-X195 remains in-progress in both
worker ledgers. Read CLAUDE, WORKFLOW, microscope, approved design, progress,
prior core, provider, native and test reviews, and relevant rendering,
bindings, architecture and dependency specifications.

Authenticated R7 receipt SHA256
56f3b9fe13fec461dfbc96be697198db60da772e7f873dc0b28e60d1dd978956,
source-input manifest SHA256
2575be4316667e6d30a48fbf730a60c6e70285c3f4699c78e32108108a86e771,
and every one of its 317 current byte, size and mode guards. Current complete
git diff HEAD exactly equals retained full-working.diff SHA256
42e8bb6a88dea1cab503ccbc4d3f01f1699ec65f8a7ed8488014cac6e3397eeb.
SVG remediation SHA256
c503b00928bc055e9421a32abb0bcd1eeecd3e4033c4437fac66b0a01821046d.
Cargo.lock still contains 302 packages. The two binding layout dependency
edges and Word binding PDF edge have named consumers in the approved plan.

## Defects

### D1, empty font inputs change Word measurement fallback

`crates/rdocx-py/src/document.rs:3492`
`crates/rdocx-py/src/document.rs:3539`
`crates/rdocx-py/src/document.rs:4154`

The collector returns Some(empty) for fonts=[], font_dirs=[], or a valid
empty directory. Both measurement methods use layout_deterministic for None
but layout_with_additional_fonts_and_options for Some. The latter explicitly
uses normal system discovery through build_layout_input_with_fonts at
crates/rdocx/src/document.rs:28557 and :28620, while deterministic layout at
:28368 selects bundled-only discovery. A document whose requested system
face differs from its bundled fallback can therefore change pagination,
fragments or page availability merely by supplying an empty input.
The approved design .claude/plans/F-X195-design.md:49 and :57 requires
empty inputs to retain the selected entrypoint's ordinary fallback policy.
The new equivalence assertions at
crates/rdocx-py/tests/test_rendering_threads.py:574 and :575 do not establish
that property across installed system families. Retain nonempty caller-font
selection and restore empty-input equivalence to the existing measurement
entrypoint, with a discriminating ordinary-system-versus-bundled control.

### D2, denied-directory controls expect an exception the loader loses

`crates/rdocx-py/tests/test_rendering_threads.py:593`
`crates/rpptx-py/tests/test_documented_examples.py:5595`
`crates/rdocx-py/src/document.rs:4148`
`crates/rpptx-py/src/presentation.rs:560`

These new matrices chmod an accessible owned directory to zero and expect
PermissionError. Stat metadata for the directory itself can still succeed.
The later read_dir failure is converted into LayoutError::FontParse by
crates/oxml-layout/src/font.rs:35 and :43, losing its I/O kind. Word then maps
it to public LayoutError at crates/rdocx-py/src/document.rs:4153 and
crates/rdocx-py/src/lib.rs:122. PowerPoint maps it to RpptxError at
crates/rpptx-py/src/presentation.rs:565 and crates/rpptx-py/src/lib.rs:140.
Neither path can deliver the expected PermissionError from that failure.
Thus the thirteen method matrices contain a source-demonstrable failing
control even with correct font selection. Reconcile the intended public
error contract and the genuine permission failure control. Preserve strict
refusal, path attribution, detached reads and source atomicity.

## Smells

0.

## Nitpicks

0.

## Aspects checked

Correctness: D1. Configured render_pages at
crates/rdocx-py/src/document.rs:2396 constructs one native layout and paints
that owned bundle for omitted or explicit page selection. Explicit order
and duplicates pass through to the existing raster renderer. TIFF, format,
quality and invalid selections retain existing renderer validation.

Contract: D1. Caller bytes are copied before detachment. Ordered directories
load before explicit entries in both families. Conflicting singular and
plural directories and scalar plural strings are rejected before rendering.
Existing positional arguments, revision selection and return forms remain.
The seven Word and six PPT signatures agree with the installed stubs.

Panics: zero findings in the reviewed delta. SVG selected TTC offsets, header
counts and table spans are checked before parser arithmetic at
crates/rdocx/src/svg.rs:767. Extraction checks allocation sizes and table
ranges. Malformed selected faces retain diagnosed refusal. The new bounds
controls do not earn executed panic-safety credit yet.

OOXML: zero findings. Rendering borrows native documents. Genuine notes and
embedded-font fixtures preserve source/package relationships and assert
nonempty intended owners. This review does not authorize source mutation or
claim native application compatibility.

Tests: D2. The new PNG arithmetic derives glyph ink and advances directly
from the two independent bundled SFNT sources. Collection controls use the
actual second face, and embedded Word controls use a real obfuscated font
relationship. The selected-page test independently discriminates pagination,
ordering, TIFF and return forms. The singular-effect SVG fixture now supplies
an actually unprovable glyph instead of relying on nominal placeholder IDs.
Existing independent 400/700 instance expectations and tolerances remain.

Structure: zero findings. Existing files and concrete operations implement
actual work. No new module, trait, generic facade or output-neutral wrapper
is introduced. The approved Word binding PDF edge paints its already
configured layout instead of adding another layout facade.

SVG correctness and contract otherwise produced zero findings. Full f64
synthetic stroke and baseline-relative shear values preserve the selected
geometry. Ordinary number formatting remains unchanged. Nominal cmap checks
at crates/rdocx/src/svg.rs:336 prevent equal glyph/scalar counts from falsely
claiming scalar ownership after substitution. Unsupported complex shaping
retains searchable Unicode text, total advance and a diagnostic.

## Limits and handback

No Cargo, installed binding execution, native UI or output baseline ran in
this review. R7 remains uncompiled and runtime pending. The native one-layout
counter and installed selection controls are composed evidence, not an
observed installed invocation counter. Prior failed runtime attempts remain
retained. This pass grants no whole-feature, full-gate or sprint completion
credit. Only this review file was written. Review phase ends here.
