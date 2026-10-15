# F-X195 installed style acceptance, pass 26

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the narrow external R3-to-R4 remediation. Product rendering, installed104 and the mandatory SVG companion remain unexecuted.

## Scope and identity

Reviewed the approved F-X195 independent font/style geometry contract, progress, existing worker status, pass25 D1, full R4 increment and both complete painter functions with their surrounding physical-scale, independent-bound and comparison helpers. The worker remains on work/f-x195-codex, F-X195 in-progress in both ledgers. This is external candidate review, not application or completion.

R4 directory: /private/tmp/S91-F-X195-runtime-r14/installed-style-expansion-prepared/complete-r4.

Receipt SHA-256: `d72ae56143aeacc451acc587678dd888e8ca4e833c874c9a838ab7cdd6a0be61`.

R3-to-R4 diff: 3794 bytes, SHA-256 `824346d16669499e96d92a36b31377969be54b7aed2f505e29044919b7bfdc44`.

Complete patch: 84594 bytes, SHA-256 `e8f0c20af96a80f33bf04bb843eacc285348e4e59b4e4c894b662b340c130c13`.

All three full candidate hashes and sizes match the receipt and retained R3 originals. Independently reconstructing both complete and incremental diffs gives the exact packet bytes. All317 current source sizes, SHA-256 values and modes match the bound source map e9ef0b359a38a2a0d8940f4d91596d84817e013c00d45ab819f12499bfd2acc7. Both Python candidates retain the complete current worker file as an exact byte prefix. AST comparison identifies only _fx195_independent_style_paint as changed in each carrier. The SVG candidate is byte-identical to R3.

## D1 remediation

Candidate crates/rdocx-py/tests/test_rendering_threads.py:1259 and crates/rpptx-py/tests/test_documented_examples.py:6121 now store contour coordinates in vertices, leaving points as the physical numeric48pt times owner scale. Bounds, stroke radius and unquantized advance consume the numeric point size. Vertex iteration and independent polygon coordinates consume the separate contour list. The static italic branch remains unchanged.

Independent read-only evaluation of the exact extracted arithmetic assignments passes20 combinations: five analytic cases, body scale1 and preview example scale0.5, in both families. Total advance, ordered bounds and all four numeric polygon vertices are checked. This reproduces neither Pillow paint nor product output and is not installed acceptance. It establishes that the previous list/int TypeError is removed from the deciding numeric branch.

## Six aspects

1. Correctness: 0 findings. Numeric physical sizing remains separate from analytic contour geometry through bounds and stroke/shear placement.
2. Contract and API: 0 findings. Test-oracle changes only, with no product API or error-policy change.
3. Panics and failures: 0 findings. No remaining numeric type collision appears in the changed branch. Actual painter/runtime failures remain possible until execution.
4. OOXML and output preservation: 0 findings. No product XML, font preparation, source-document handling or rendering baseline is changed by R4.
5. Tests and evidence: 0 findings. AST checks and isolated arithmetic are properly bounded preparation evidence. The deciding0.99 positive/negative comparisons, independent ROI, physical provenance and repetition assertions are unchanged.
6. Structure and HLD intent: 0 findings. The small variable correction adds no module, file, dependency, trait or generic. Mandatory gate policy remains separate from this source-only remediation.

## Mandatory gate preservation

The mandatory-gate-plan-proposal.diff and execution-recipe.md match R3 byte-for-byte. The PDF anchor adapter, adapter self-check and style-font guards also remain byte-identical. The recipe continues to require actual installed Word56 plus PPT48 cases and a separate explicit ignored native SVG stage with exactly one passed companion and eight authenticated SVG owners. Ordinary ignored-test output is not acceptance. The retained SVG candidate preserves that declaration and source-admission requirement.

No semantic segments outside the two painter variable fixes changed. No tolerance, fixture expected contour, physical font identity, required400/700 control or negative-paint discriminator is weakened. Earlier actual installed/native evidence remains distinct, with no credit supplied for these prepared104 cases.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

The only write in this pass is this review artifact. No source, scratch, Cargo, installed product import, cache, mtime, Git mutation, native UI or formatting action occurred. Review ends here.
