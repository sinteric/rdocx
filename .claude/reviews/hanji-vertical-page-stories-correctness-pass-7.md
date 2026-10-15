# Hanji, vertical page stories, final continuation review, pass 7

**Reviewed**: correction after d1966e99, selected-template fragment margins, one renderer regression, its HLD contract and measured archive record
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Pass 6 D1 is resolved. Fragment refitting copies selected-template left and
right margins, and the bottom margin only for cells with remaining blocks.
The removed top margin and completed cells' zero bottom margin stay removed.
The available line measure and row height use the selected spacing. The
plain-row guards still exclude rotations, merges, clipping, anchors and nested
tables. Horizontal pagination does not reach the vertical refit.

The new end-to-end test derives its expected half-gap from the selected first,
even and default story bands. It failed before the correction at page two,
origin 232.2 pt against an expected 229.8 pt. It now verifies active-band
containment, selected spacing, every indexed Unicode source scalar once,
contiguous source spans, three finite table measurements and cold/warm equality.
Large entries may exceed the unchanged retained budget. The existing smaller
table case still proves three actual cache hits and changed-story warm/fresh
equivalence. No cache budget or source identity mechanism changed.

Paragraph logical cursors, markers, anchors, source ranges and numbering paths
are unchanged by the correction. Alternative layout state stays cloned from
the preceding primary state. Header/footer reservation and story selection
are unchanged. The independent API reproduction retains the original 4.2 pt
later-page spacing error as a negative-control artifact.

All 316 changed Word-layout tests and its documentation test pass. Strict
changed-crate Clippy, formatting, prose and adapter checks pass. All 49
deterministic output hashes still match. The changed crate archive has
280,803 compressed bytes, 1,503,006 normalized member bytes and 15 members,
with its README and gate record refreshed from the generated package.
Unchanged workspace suites were not repeated.

Downstream pin integration will select the new immutable correction commit
and rerun the focused public API cases. Native Word fidelity remains
unverified. No merge, release or deployment is authorized or performed.
