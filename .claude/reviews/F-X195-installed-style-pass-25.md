# F-X195, installed style acceptance, pass 25

**Reviewed**: Complete external R3 candidate at
/private/tmp/S91-F-X195-runtime-r14/installed-style-expansion-prepared/complete-r3.
Three existing carriers, 84578-byte composed patch and43885-byte R2-to-R3
increment. Citations below identify intended candidate source lines.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Identity and scope

Read the approved F-X195 physical-font, independent contour, synthesis and
entrypoint contract, prior pass24 and its three findings, complete R2-to-R3
change, composed changed helpers and the retained SVG companion. Reconstructed
all three complete patched carriers in memory against their actual current
worker originals. Every resulting candidate matches its receipt. Both Python
candidates preserve their complete current source as an exact byte prefix.
Retained R2 candidates match their recorded hashes. AST comparison finds only
the stated helper changes and the PPT notes-scale callsite change in existing
function bodies. The former PDF anchor is replaced by two qualified helpers.
The native companion changes its execution declaration and mode admission,
with prior paint logic retained.

R3 receipt SHA-256:
4fa7b77e0620deed27fd2dc4b5399a7657c680ed09626e5345405905f51990cb.
Composed patch SHA-256:
7453053e97423ac21307e4f985f7e8486b1f1867cfe2b71edfe0cc37023b8799.
Increment SHA-256:
ae1e57566762e6132fe84c8f207c27344001524a38b0fddd139c4e1835aff40c.
Authenticated all317 actual current source bytes, sizes and modes against
source map e9ef0b359a38a2a0d8940f4d91596d84817e013c00d45ab819f12499bfd2acc7.
Carlito Regular, Italic and license match the independently retained asset
hashes. The source remains unapplied. This review does not execute specimens,
installed imports, rendering, Cargo or the mandatory companion.

## Defects

### D1, analytic painter overwrites point size with contour vertices

`crates/rdocx-py/tests/test_rendering_threads.py:1259`
`crates/rdocx-py/tests/test_rendering_threads.py:1263`
`crates/rdocx-py/tests/test_rendering_threads.py:1264`
`crates/rpptx-py/tests/test_documented_examples.py:6121`
`crates/rpptx-py/tests/test_documented_examples.py:6125`
`crates/rpptx-py/tests/test_documented_examples.py:6126`

The new variable points initially holds48 times the physical owner scale.
The analytic branch reassigns it to the four contour vertices, then divides
that list by24 for synthetic bold or uses px times that list divided by1000
for every non-bold axis/synthesis case. These expressions raise TypeError
before creating the independent image. A read-only evaluation of these exact
numeric statements confirms list/int TypeError for both bold and non-bold
branches, without any product import or render.

Consequently all five analytic style cases in each family fail at the oracle,
including the reduced slide-preview cases. The static italic case does not
enter this branch. This is not evidence of incorrect product geometry. Keep
the numeric physical point size and contour vertex list as distinct values
through stroke, bounds, baseline and independent polygon calculations. Retain
the actual owner scale and deciding0.99 positive and negative thresholds.

## Smells

0.

## Nitpicks

0.

## Prior finding assessment

Pass24 D1's mismatched bbox-versus-tight-ink assertion is removed. Candidate
Word :1244 and PPT :6106 compare FreeType advance with direct cmap/hmtx advance
at the identical quantized oversample point size. The unquantized source
advance is used for placement, and Pillow bbox is only a padded independent
raster extent. The corrected formulation compares matching quantities.
Actual FreeType and product paint qualification remain pending.

Pass24 D2's ordinary-test admission failure is addressed by the declared
ignore at crates/rdocx/src/svg.rs:3440. The proposed plan amendment and recipe
require actual104 installed passes, exact --ignored execution, one passed
companion and eight authenticated SVG owners. Missing admission still fails
when this explicit stage executes. Modes are authenticated by ROOT admission,
and the Unix-only PermissionsExt import is removed. Ordinary ignored output
provides no acceptance credit. Apply and execute the mandatory plan contract
with the test rather than allowing the new ignore to become an omitted gate.

Pass24 D3's adapter now starts at reachable page Contents, resolves resource
and Form ownership, composes q/Q, cm, Form matrices and Tm, and converts to
canvas coordinates. Unsupported positioning or multi-stream fixtures fail
explicitly. Synthetic shear is checked against the independent policy before
removing only that style transform from placement. Notes require a body owner
at scale1 and a preview owner at the source-established sldImg scale. Both
owners have positive and negative PDF/PNG paint assertions and receipt output.
The PPT fixture preserves one real styled slide run and one real notes run at
:6302. No arbitrary first anchor or actual-mask registration is introduced.
The three constructed adapter self-checks are preparation evidence, not
actual output or installed acceptance. Real emitted PDFs must still qualify
the parser and owner identification under the unchanged paint threshold.

## Other aspects and limits

The candidate changes test and gate code only. No public API, dependency,
module, product font preparation, OOXML source or baseline changes occur.
Existing provenance and before/after source checks, repetition, physical font
identity, variable controls and eight SVG case identities remain mandatory.
No weakened0.99 comparison was found. Test-only assertions intentionally fail
unqualified fixtures rather than silently accepting unsupported parser forms.

The prepared counts remain Word56 and PPT48, uncollected and unexecuted.
Neither104 installed acceptance nor the native SVG gate is credited. The new
analytic oracle defect must be remediated in a separate phase, then reviewed
and executed. Earlier actual19 installed successes and native gates remain
separate evidence. No source, cache, mtime, Git, UI or formatting action ran.
The sole write is this review artifact, and this pass ends here.
