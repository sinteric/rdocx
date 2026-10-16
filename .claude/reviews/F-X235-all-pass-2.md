# F-X235, all aspects, pass 2

**Reviewed**: renewed frozen working diff against90a25094c929e81c11f5df2eeaf3768256baf7a2,6 existing files,1737 inserted and130 removed lines. Approved plan, cited contracts and pass1 finding read before audit. Production math source SHA256 is `edab6dc340ddde84f4d90fcc58ca88f09d50fdecb14686e7d0dd4eb607b2a039`. No implementation, fixture, baseline or documentation edits were made during this pass.
**Verdict**: 0 defects,0 smells,0 nitpicks

## Defects

None found. D1 from pass1 is resolved.

## Smells

None found.

## Nitpicks

None found.

## Resolved finding and evidence

The private style retains root size. One concrete helper reads the first-level
and second-level MATH percentages, derives each actual size from that root,
and saturates the optical level and MATH size coherently. Radical degrees
advance two levels. Limits and accents use the same context. Fonts without
MATH retain their historical recursive fallback instead of receiving an
unqualified behavior change.

The constructed-font test has distinct72/60percent constants, verifies actual
rendered nested/deep glyph sizes at11and22points, checks radical degree size
and second-level context, and retains the separate actual GSUB alternate
control. Root independently executed the built test and observed its pass.
Restoring the previous recursive first-level calculation produces the retained
semantic assertion failure, rather than a compile-only negative.

Root independently parsed the fresh native comparison and authenticated its
actual product PDF hashes. All six native cases and six exact-expression
clusters pass. All six product PDFs remain byte identical to the preceding
positive candidate, so the nesting correction does not invalidate those
qualified controls. The actual1.01point rendered negative retains its explicit
five-case rejection and radical residual limitation. Historical manifest
fields, source identity, thirteen expression controls and thresholds remain
unchanged. Shared sample baselines remain untouched.

The current full changed-crate suite reports340unit tests and1doctest, with
denied-warning Clippy green. New WASM and hash checks pass. Post-remediation
publication, actual archive numeric/HLD preparation and remaining scoped
policy acceptance remain completion obligations, not inferred passes here.

## Not found

- Correctness: actual-ink fraction clearance, MATH axis/leading, script placement, radical sizing and side-limit attachment show no additional actionable defect after D1.
- Contract: deterministic caller-font qualification, explicit source spacing, current native identities and original diagnostic limitations remain separate. No threshold relaxation or product-side external executable found.
- Panics: face/table parsing, glyph lookup, source ranges, finite ratios and bounded variants retain checked admission. No additional actionable panic found.
- OOXML: transient layout retains typed expression/source data and logical glyph-run text. No new serializer or opaque-preservation change found.
- Structure and layering: the private context/helper remain in existing math.rs. The direct existing ttf-parser edge adds no package, module, trait, public API or family-direction violation.
- Tests: genuine native outputs, actual constructed font data, preserved historical controls and rendered/source mutation failures support the contract. No additional test defect found.

This review pass ends here. Return to sprint orchestration for the remaining
scoped gate and separately reviewed metadata/HLD preparation. No confirmation
source pass is required after this clean review unless source changes again.
