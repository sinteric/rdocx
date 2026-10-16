# F-X235, all aspects, pass 1

**Reviewed**: frozen working diff against90a25094c929e81c11f5df2eeaf3768256baf7a2,6 existing files,1660 inserted and130 removed lines. Approved refined plan and its rendering, native-oracle, font and graph contracts read first. No source, fixture, baseline or documentation edits were made during this audit.
**Verdict**: 1 defect,0 smells,0 nitpicks

## Defects

### D1, nested scripts ignore the separate second-level font scale

`crates/rdocx-layout/src/math.rs:66`
`crates/rdocx-layout/src/math.rs:334`
`crates/rdocx-layout/src/math.rs:1523`

The new style context tracks a saturated second script level, but its size
keeps multiplying by the first-level ScriptPercentScaleDown. The separate
ScriptScriptPercentScaleDown is never read. A nested superscript or subscript
therefore has the wrong font size, and deeper levels keep shrinking even
after the style level saturates. The radical degree path also multiplies the
first-level scale twice while advancing the optical alternate level only once.

Root independently read the authenticated Cambria Math face. Its first and
second level values are73 and60percent. At11points the current nested path
produces8.03 then5.8619points, whereas its second-level target is6.6points.
The installed parser's own primary API documentation distinguishes these
two font constants. This is a reachable supported nested OfficeMath tree,
outside the six new single-script fixture cases.

Derive first/second script sizes from the same root style size, saturate both
size and optical alternate level coherently, and route the degree path through
the correct second-level context. Keep the non-MATH fallback contract and all
historical measurements intact. Add a constructed-font nested/deep script and
degree control with distinct first/second constants, then refresh affected
native/scoped evidence before a new review pass.

## Smells

None found.

## Nitpicks

None found.

## Evidence checked

All prior manifest fields remain exactly unchanged. Historical source identity,
thirteen expression-family tuples,1point geometry,150DPI and0.99SSIM remain
preserved. Root independently parsed the final explicit-default receipt,
observed all six native cases and six exact expression groups passing, and
authenticated every current product PDF hash against that receipt. The exact
formula and native save/reopen sources were captured by root through the
actual Word UI, independently of product output.

The earlier mismatched-font control and implicit-default v1 controls remain
separately identified diagnostics. V2 explicitly fixes paragraph spacing and
math pre/post spacing in source rather than normalizing comparison geometry.
Actual1.01point rendered negatives reject five cases. The radical signed
residual cancellation remains explicit rather than being reported as a
universal mutation rejection.

The graph change adds only the already-resolved ttf-parser0.25.1 direct edge
to rdocx-layout. It adds no new resolved package or cross-family edge. The
three actual affected archive inventories and final publication acceptance
remain worker completion obligations. Shared baselines are untouched.

## Not found

- Correctness: fraction ink/rule separation, actual axis, display ink leading, radical variants and simultaneous side-limit attachment have no additional actionable defect beyond D1.
- Contract: production invokes no external executable and uses existing deterministic caller-font APIs. Native external font bytes remain oracle inputs without redistribution.
- Panics: inspected checked face/table admission, glyph lookup, bounded variant selection, source indexing and GSUB lookup/subtable handling. No additional actionable panic found.
- OOXML: layout preserves typed/source expressions and logical text. No new parser or serializer rewrite found.
- Structure: private context and measured helpers remain in existing math.rs. The approved existing dependency avoids a new public font API, module, trait or generic layer.
- Tests: actual native geometry and raster measurements, constructed font constants/GSUB controls, preserved historical families and rendered mutation controls supply independent evidence. D1 needs the stated nested-context regression before completion.

This review pass ends here. Return to sprint orchestration for a distinct
remediation phase, then a fresh independent source pass. No completion or
baseline update is authorized by this finding-bearing review.
