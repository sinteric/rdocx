# F-X195, installed style acceptance, pass 24

**Reviewed**: Complete external R2 test proposal, three existing carriers,
62339-byte1108-line unified patch. All three candidate diffs were read and
independently reconstructed against actual frozen R14 source.

**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Authority

Read the approved caller-font, physical-instance, independent contour,
mathematical synthesis and full entrypoint acceptance contract, current source
and prior independent reviews. The worker remains work/f-x195-codex at
2a9bf14072231e46fcb34df70f88cf7aa79fbd79 with in-progress sprint records.

Proposal receipt SHA256:
85e3206933b901f82d2e852e878a4dab113b57ee10d733d185f917226b362247.
Patch SHA256:
68c662d479fb76295dc80b0b27c12387ed13983cc54445bfd3a8fc5a3c694b0e.
All317 current source inputs retain exact bytes, sizes and modes. No proposal
has been applied. Citations below identify intended line positions in the
complete external candidates, unless explicitly marked current source.

## Defects

### D1, the FreeType precondition confuses character layout boxes with tight ink bounds

`crates/rdocx-py/tests/test_rendering_threads.py:1086`
`crates/rpptx-py/tests/test_documented_examples.py:5948`

The genuine-italic painter equates the width from Pillow getbbox with the
raw TrueType tight ink width. Pillow includes layout bearings/advance space,
so these are different quantities. A read-only in-memory probe using the
qualified installed environment and exact bundled bytes returned bbox
(0,-529,176,0) at768px for both Carlito faces. The proposed expression is44pt.
The direct hmtx/glyf helper returns41.390625pt for Italic and38.8359375pt for
Regular. Their differences2.609375 and5.1640625 exceed the1/16pt tolerance.
Thus every genuine-italic case fails before observing the product, and its
regular negative also cannot reach its deciding paint comparison.

Retain an independent physical face/advance check, but compare matching
quantities. Obtain tight ink from the independent contour/raster bounds or
compare FreeType advance to hmtx advance separately. Do not relax the paint
threshold to cover an invalid oracle precondition. The probe wrote no file.

### D2, ordinary library tests acquire mandatory coordinator-local admission state

`crates/rdocx/src/svg.rs:3471`
`crates/rdocx/src/svg.rs:3483`
`.github/workflows/ci.yml:180`

The new ordinary #[test] unconditionally unwraps F_X195_ROOT_SERIAL_SVG_STAGE
and coordinator admission paths, then demands104 installed passes, two
qualified wheels and317 current worktree guards. Normal cargo test -p rdocx
and the existing CI invocation provide none of that state. They therefore
panic before testing any SVG. This is a new ordinary-gate failure, rather
than an explicit separate prepared-artifact acceptance stage.

Keep the mandatory installed SVG acceptance, while making its separate
execution contract explicit and compatible with ordinary library tests.
Do not return success or silently skip inside a normal named test when proof
is absent. An explicitly declared external-artifact gate must actually run
with its prerequisites and retain exact one-test/eight-owner acceptance.
The source also imports Unix-only PermissionsExt at:3443, so its eventual
execution route must respect the intended portable native test boundary.

### D3, the PDF anchor adapter ignores graphics transforms and assumes one owner on notes pages

`crates/rpptx-py/tests/test_documented_examples.py:5928`
`crates/rpptx-py/tests/test_documented_examples.py:5931`
`crates/rpptx-py/tests/test_documented_examples.py:6027`
`crates/rpptx-py/tests/test_documented_examples.py:6138`

The adapter treats raw Tm coordinates as physical canvas coordinates without
composing the enclosing graphics state. The actual existing textbox starts
at72pt,72pt. A read-only current-wheel PDF probe has an enclosing
1 0 0 1 72 72 cm and a local1 0 0 -1 7.2 49.2 Tm. The proposed independent
painter places ink at7.2,49.2 instead of the transformed physical origin.
The resulting SSIM failure would be an oracle coordinate error, not evidence
that caller style rendering is wrong.

The notes fixture also styles both slide and notes owners at48pt. Current
render_notes_pages at crates/rpptx/src/lib.rs:10257 includes the real slide
preview. The actual current notes PDF probe returned two matching48pt anchors,
both locally7.2,49.2, so the unconditional exactly-one assertion fails for
notes methods as well. These in-memory probes wrote no document or artifact.

Track the actual composed PDF text position, including q/Q, cm and relevant
form transforms, and identify the intended physical owner independently.
Retain meaningful slide-preview and notes-body coverage rather than discard
one owner or select an arbitrary first text matrix. Source-established
independent geometry remains required, without actual-mask registration.
The same Word adapter should be assessed for any enclosing-transform case.

## Smells

0.

## Nitpicks

0.

## Not found

- Production correctness: this proposal adds tests only, with no product edit.
- OOXML: current source/input XML stays unchanged by this pass.
- Oracle thresholds: no weakened .99 paint threshold or actual-mask alignment.
- Structure: existing carriers only, no dependency, module, trait or feature.
- Provenance: independent retained source hashes and real physical font inputs
  remain mandatory, and variable-resolver limitations are stated explicitly.

## Limits and remaining scope

The104 proposed cases are uncollected and unexecuted. This pass does not credit
them, the SVG companion, TTC or all-script/style coverage. Layout metadata is
not direct glyph-instance evidence, and the existing native configured-layout
controls remain required. Earlier19 installed PASS, source-bound Word366 PASS
with8 historical ignores and actual PPT native evidence are separate scopes.
No source, formatting, cache/mtime, Cargo, baseline, Git or UI change occurred
in this review. The only write is this review artifact, and the pass ends here.
