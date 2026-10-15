# F-X195, glyph representation diagnostic, pass52

**Reviewed**: R34 representation-only diagnostic diff from reviewed R32.
**Verdict**: 0 defects,0 smells,0 nitpicks. Causal execution unproved.

## Authentication and isolation

Helper22f1ea81b932488f898a9ddd7b815d40bee22f1d37428d83dfa0940a83110b7d,
preparation3a840059b4d161d1c10a3ebf11e48ca04f485ccba6be16748cc9425226f9df16
and diffa5b2d1e8f9a259ed868151dbdcfa0f0e5084d3f582723da839540607dbdc0e69
were authenticated. AST parsing succeeds. Inherited source/input guards,
SFNT checksum validation, PDF reader and owned-group cleanup are unchanged.
Only the counterfactual font construction, PDF font replacement argument and
experiment inventory change.

Six controls separately swap raw GID3, GID16 or both in each direction.
Two unused GID0 controls are independently reported. All other raw glyph
records remain byte-exact. Used donors must have equal bounds, instructions
and independently decoded quadratic commands. The decoder handles repeated
flags, signed coordinate deltas and implicit on-curve midpoints. Current
simple-glyph ceilings and global bounds are validated without modifying them.
Only glyf/loca and required checksum adjustment change. Metrics, names and
all unrelated tables remain exact. PDF object identities, free entries,
trailer, drawing and other object bodies retain the reviewed strict controls.

Original and repeat renders must equal retained originals before comparison.
All comparisons use the original540x720 full canvas. The baseline remains
three differing pixels at maximum channel delta1. Causality requires exact
bidirectional matching after both used-glyph swaps. Negative GID0 results
remain separate. No image fitting, tolerance, whole-font substitution or
acceptance correction is performed.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness, contract, panics, tests and structure produced no findings in
this fixed-input diagnostic. OOXML production is unchanged. Actual causal
results, any contract-compatible response and complete feature acceptance
remain unproved. This review does not authorize changing a test oracle.

## Review boundary

Only this canonical review was written. No font, PDF, raster, source, proposal,
Cargo, cache, mtime, application, baseline or intake mutation occurred.
Return to orchestration for diagnostic admission and execution.
