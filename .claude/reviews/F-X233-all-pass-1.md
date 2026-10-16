# F-X233, all aspects, pass 1

**Reviewed**: frozen working diff against6f3da0b72659f58c29646875f548f8b42f7df5f9,6 existing files,1194 inserted and38 removed lines. Approved plan and cited units, rendering, related-story and deterministic qualification contracts read first. No source, fixture, baseline or documentation edits were made during this audit.
**Verdict**: 1 defect,0 smells,0 nitpicks

## Defects

### D1, wrap reservations omit the new top band on the second reflow pass

`crates/rdocx-layout/src/paginator.rs:3693`
`crates/rdocx-layout/src/paginator.rs:3750`
`crates/rdocx-layout/src/paginator.rs:3757`

The first reservation pass sees the original padded lines, but the second
replaces them with fresh unpadded line-breaker output. It starts intersection
tests at the paragraph origin, while actual text is painted below the new top
border band. The band is reapplied only after both reservation passes. With a
20point line and11point top band, a square drawing intersecting the actual
first content line below its unpadded20point extent can receive no first-line
reservation on the second pass. The early no-intersection return can even
discard the first pass's reservation entirely. Text can then paint inside the
floating drawing. The existing reflow test uses an80point-tall drawing, so both
the padded and unpadded bands intersect it and the test misses this boundary.

Use the same border-adjusted text band for every wrap intersection pass and
for final paint, without accumulating padding. Add an actual short-wrap
boundary control that checks the resulting line reservation or glyph/object
non-overlap, alongside the repeated reflow equality control. Preserve the
unbordered path and refresh native/hash qualification after remediation.

## Smells

None found.

## Nitpicks

None found.

## Evidence checked

The independent native12page receipt remains within the declared0.60point
frame and0.36point width bounds, with maximum observed0.50 and0.195points.
The two nonzero-spacing group controls retain continuous sides, maximum frame
error0.30924points and native31.92/product32.00001point baseline advances.
The genuine Base/candidate no-paragraph-border invoice retains table borders
and byte-identical PDF/PNG hashes. Five retained mutations fail semantic
assertions and restore source. Final scoped layout evidence reports346unit
tests and1doctest, with denied-warning Clippy green.

The final candidate attribution has exactly18entries and equals the prior
candidate values. Every affected sample contains paragraph border owners,
and XML/resources remain unchanged. This supports the declared attribution
but does not authorize recording a baseline before D1 is resolved and the
affected checks are refreshed. The prior legal-underline feature_showcase
PDF deltas remain the starting values for this incremental attribution.

## Not found

- Units: sz/8 and truncating unit constructors remain unchanged. No invented size-conversion fix found.
- Contract: inferred15point and raw15 inputs remain distinguished from the missing original. The original foreign-property native refusal is retained separately from the normally opened minimal controls.
- OOXML: this story modifies transient layout, leaving stored source and opaque XML untouched. No schema rewrite found.
- Panics: private group indices and note slices follow constructed adjacency/range invariants. No additional actionable panic found.
- Structure: private metadata remains in existing block, header/footer and note carriers. No new public field, module, dependency, trait or generic abstraction found.
- Cache and group spacing: transient clones preserve retained cache blocks. Body/cell/header/footer/note tests cover maximum and additive spacing, and the body bridge stops at new pages. No additional actionable finding found in these paths.
- Fragment geometry: first-top/middle-sides/last-bottom masks and single/double corner ownership have native and focused coverage. No additional finding found.

This review pass ends here. Return to sprint orchestration for a distinct
remediation phase, then a fresh independent pass. Baseline recording remains
blocked until the source review is clean.
