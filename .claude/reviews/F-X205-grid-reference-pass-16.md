# F-X205 Grid reference, pass 16

**Reviewed**: Immutable integration-grid-reference-proposal-4. One existing test carrier, one hunk, 23 additions and 10 deletions. No production or fixture changes.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound evidence

Manifest SHA `9c1dc4241e481476cae6b5a88180bc1fc1dd368af947d0fb847b9e7e2868fd74` and all four declared artifacts authenticated by size and SHA-256. Candidate SHA `f98ce35169046dc0fb91d49ba93e095374e7129f7c1a67ca3fa30040de3a54e3`. Diff SHA `977a813be5662db4faa7c4e3124f0ea35954156b06956184e580a711ea5e5db3`. The approved F-X205 contract and current source were read before this review.

## Reference correctness

`/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/integration-grid-reference-proposal-4/integration-candidate.rs:18497` checks all 16384 cell colours against an independently expressed coordinate formula and channel-directed exponent 1.875. The expected calculation calls no production interpolation helper. Coordinates are algebraically equivalent to the resolver's cell centres at `crates/rpptx-layout/src/context.rs:2054`. Their absolute maximum is the rectangle distance at2061. Missing path kind is explicitly Rectangle in `crates/oxml-drawing/src/fill.rs:1071`, so the selected distance is justified by this actual fixture.

The maximum distance is127/128 and the minimum is1/128. Independently calculated minimum red is0.014598353264312913 and maximum blue is0.9998880610298823. Thus the previous requirement for a sampled red below0.01 was impossible. The correction replaces that predicate with a stronger all-cell reference at tolerance1e-12. The directed red and blue expressions agree with the pinned primary Microsoft authority already qualified in pass15 and production channel direction at `crates/rpptx-layout/src/context.rs:4365`.

The candidate retains exact128-by-128 dimensions, requires exact vector length and zero green, retains full alpha, diagnostics, original path XML and save assertions. Earlier 256 PDF intervals, exact PDF endpoints, axis, raster orientation and thresholds are byte-equivalent in this bounded diff. No product colour rule, output baseline or source geometry changes.

## Evidence limits

The original failed Grid predicate and pass13 reference approval remain historical failed evidence. Proposal authority records expected values, not measured RGBA. This review authenticates source and mathematical predicates only. Actual corrected selector execution and whole-feature acceptance remain due. No worker file was modified and no Cargo, binding or Office runtime was executed.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings in this bounded correction. This is not a confirmation review of the whole feature.
