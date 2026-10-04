# Hanji, page-specific vertical story measures

**Status**: completed renderer implementation and validation for the approved fork custom-fix task
**Base**: 873df08311ad74cfaebefc920e3641225afa3b7b

## Problem and spec reference

The engine prepares vertical paragraphs and percentage tables using the largest
active header/footer band. A tall first header therefore narrows later pages
using a shorter default header. The owned public-API cases show an 84 pt lost
band and seven versus eight pages for the same continuation body.

References: `docs/hld/08-rendering-spec.md`, "Vertical Word text" and "Tables".
`docs/hld/03-architecture.md`, paragraph/table ownership and bounded retained
layout state. `docs/hld/12-testing-strategy.md`, grid and vertical geometry,
Word render fidelity and the hash harness.

## Approach and implementation checklist

- [x] Record logical consumption alongside shared line breaking, before bidi
  reordering. Preserve existing public carriers and ordinary line output.
- [x] Resume retained paragraph input at that cursor on a new measure. Preserve
  Word heights, grid, Unicode/source order, tabs, objects, fields, first-fragment
  markers, indentation and anchors.
- [x] Prepare the finite distinct active story widths through existing table
  layout and width-aware cache keys, with cloned numbering state so alternative
  measurements do not advance the document twice.
- [x] Select width-specific rows and rebreak only remaining cell content at a
  page transition. Preserve structure/source identities and header repetition.
- [x] Use authored page geometry plus selected-story room while preserving the
  largest-band preparation as a safe input bound.
- [x] Validate source conservation, cold/warm equality, existing custom fixes
  and six owned public-API cases. Review before preparing the fork PR.

The corresponding Hanji checkout contains the six owned API regressions and
updated preview documentation. After this renderer commit is immutable, pin
that revision and complete downstream validation before opening the integration
PR. The renderer PR and Hanji PR are separate reviewable changes.

## Test plan

Shared cursor tests cover indexed/Unicode text, markers, tabs, images/figures,
explicit breaks, generated hyphens, multilingual visual ordering and invalid
cursors. Renderer tests cover tall first and varying even/default headers and
footers, paragraphs crossing story changes, percentage tables, split cell rows,
numbering/source maps, selected-width footnotes, keep-next lookahead, accepted paragraph joins and deterministic cold/warm
layouts. Preserve the nine tall-story alignment cases and section isolation.
The six owned Hanji API fixtures must restore continuation bands and equal
seven-page overflow while preserving text and horizontal output.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/12-testing-strategy.md`

## Risk routing and validation

Layout/pagination/line breaking: deterministic fonts, focused changed-crate
tests, scoped strict Clippy, formatting and a deliberately reviewed hash result.
Additive shared API: no public carrier fields change. The new line-module
functions are additive. Verify the oxml-layout package with publish dry-run and
the 10 MiB archive assertion, plus wasm compilation and strict documentation.
Refresh only measured archive records for the two changed crates in their
READMEs and scripts/readme_doctests.py. Run prose and generated-skill policy checks. Record native
Word fidelity as unverified, rather than treating geometry tests as its oracle.

## Hash harness

Expected unchanged for existing samples without unequal vertical story bands.
Any observed delta must be independently reproduced and explained before
baseline acceptance. Do not regenerate a baseline merely to pass the check.

## Process boundary

This follows the established `Hanji, ...` fork custom-fix convention. Preserve
the completed upstream sprint records and ancestry. One fork PR contains the
coherent fix and its reviewed evidence. No merge, release or deployment is
authorized. The supplied design approval covers this implementation scope.
