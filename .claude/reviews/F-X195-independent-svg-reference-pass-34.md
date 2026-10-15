# F-X195, independent SVG reference, pass 34

**Reviewed**: External R26 proposal, four Python sources and one Rust source,
235 lines total, 24 physical owners and 128 path-only SVG inputs. Metadata,
manifest, qualification text and actual retained R24/R25 receipts were also
read. The proposal author is independent of this reviewer.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this prepared supplemental
reference. Compiler, runtime and acceptance qualifications remain pending.

## Contract and authentication

Read CLAUDE.md, WORKFLOW.md, microscope, the approved whole F-X195 plan,
current progress, pass33 and the relevant rendering, deterministic-font and
oracle contracts. Canonical branch is sprint/s91 and F-X195 is in progress.
Canonical status was clean before this report. This review changes no source.

Authenticated all137 manifest files, seven original/static font guards and
54 retained artifact guards by SHA256. All128 SVG files exist and match.
Authenticated all five primary API source hashes and all six candidate
rlib/depfile/fingerprint triplets. Their presence is not ABI qualification.
Manifest SHA is
8dc37e06f8c62bb56e6d35475785db82ffc00487d2d2ea82822812c775b247d8.
Prepared receipt SHA is
92b8b0d776ae8dad1dc966c016b9a6206e9f5e8d5529cffd90618bfb179c881c.
Renderer SHA is
e06ce328e15b2003ebff071266b07aff04ae0f5462fad4890ea7bd9601b66e85.

All24 corpus owner identities, original score tuples and source ROIs match
the actual retained R24 reports. The full SVG canvases match declared physical
page sizes. Origins and preview scales match the exact corresponding owner.
The corpus retains 24 quadratic positives, 24 cubic positives and80 negatives.
Each SVG has only namespace-qualified path children. Duplicate style and
wrong-geometry negatives in the finite fixture remain separately labelled
conditions, not80 distinct geometric shapes.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Aspect assessment

Correctness: source_decoder.py:5 through73 decodes original/static glyf and
hmtx bytes independently, including the admitted composite i, rather than
extracting production outlines. Unsupported composite attachment, rounding
and scaled-offset forms fail assertions. This is a bounded fixture decoder,
not a general SFNT implementation. svg_reference.py:5 through23 preserves
implied on-curve midpoints and contour closure. The quadratic-to-cubic control
points at:19 are the exact affine conversion. Transforming those points
retains that equivalence. Original contour order and nonzero winding remain.

Synthetic geometry: svg_reference.py:27 through37 uses source hmtx advances,
48-point source size scaled by the real owner, centered1/24em round stroke,
and tan12-degree baseline shear. The SVG affine offsets cancel at the run
baseline. Style, three-point origin, wrong geometry and six-degree shear
negatives remain explicit. Source positions are not fitted to pixels.

Contract and tests: prepare_corpus.py:21 through34 authenticates the original
owner graph and carries exact source ROI and retained PNG/PDF paths. run.py:43
through47 preserves the original reference-versus-negative comparison policy,
all0.99 positives and deciding negatives below0.99. Both quadratic and cubic
references must match the actual PNG. It also retains the separate actual
PDF versus independent font-text positive at0.99. All24 scores are written
before the final aggregate assertion. R24's failed bold and genuine-italic
preview results remain retained. The actual R25 four-owner filled-round
success is supplemental evidence and is not relabelled as R24 success.

Renderer isolation: renderer.rs:4 through16 walks the resulting complete
Group tree and rejects Text/Image nodes. At:23 through33 it parses the pinned
public usvg API, uses default empty font database, paints onto a declared-size
white canvas at identity root transform and saves PNG. All admitted inputs
were independently inspected as path-only with no resources. No product
writer, font shaper, subsetter or decoder is linked by the proposed extern
closure. The shared resvg/tiny-skia raster backend is accurately disclosed.
It cannot independently detect a common raster backend bug.

Panics and OOXML: there is no OOXML mutation. Decoder assertions are bounded
source-qualification refusals on the authenticated finite inputs. Subprocess
waits in run.py:30 through38 are bounded and signals address only the owned
process group. Command exits and logs are retained before assertions. No
product input/output publication or user document is involved.

Structure: the reference lives outside tracked production and uses one
concrete standalone renderer with existing pinned public dependencies. No
crate, tracked module, feature flag, product dependency or forwarding public
API is proposed.

## Required qualifications before execution or acceptance

At run.py:7 through12 the actual compiler qualification receipt is still
required. ROOT must produce and inspect the real rustc1.97.1 verbose version,
executable hash, sysroot and host, chosen exact resvg rlib, depfile/source and
fingerprint identity, features/profile, and compatible transitive artifact
and ABI evidence. The boolean ABI field is an assertion of that separately
reviewed evidence, not evidence by itself. No triplet may be selected by
recency or the fact that it is in the shared target. Retain actual compiler
argv, exit/log and new standalone binary hash. Compiler/link failure remains
a setup failure, not geometric RED. Current candidate hashes all match, but
no artifact has been selected or compiled in this review.

At run.py:18 through47 actual standalone execution, complete128 command
outcomes, all24 unchanged-threshold scores and deciding negatives remain
unrun. Retain true terminal exit and output/log hashes. Formal adoption would
also require the explicitly reviewed plan/test amendment, exact interpreter
and image-library identity, immutable artifact lineage and actual repeated
reference evidence where claimed. The prepared corpus does not authorize an
oracle substitution or erase the original installed failures.

The approved plan's mandatory installed SVG gate remains separate. Actual
full104 success must precede the native test invoked with --exact --ignored,
with one passed and zero ignored. This finite24-owner prototype supplies
neither full104 nor eight-owner native SVG acceptance, no type/link proof,
no feature completion and no waiver for remaining font, publication, WASM,
hash or policy riders.

## Review boundary

Only this canonical review file was written. No workers, compiler, Cargo,
product import, rendering, UI, cache or source changes occurred. The review
ends here and returns control to ROOT for its separate execution phase.
