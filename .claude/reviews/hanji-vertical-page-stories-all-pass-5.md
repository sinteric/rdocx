# Hanji, vertical page stories, all aspects, pass 5

**Reviewed**: working diff from 873df08, ten source/spec/measurement files, 1160 inserted and 54 removed lines, plus the approved plan and prior reviews
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: D1's keep-next lookahead prices selected measures.
D2's note registry includes every selected vertical measure and keeps the raw
endnote width. D3's row replacement/refitting is confined to vertical pages.
S1 is resolved by cloning preceding numbering only when an alternative width
actually differs. The Option expectation follows that exact predicate.

Source conservation: split Unicode cell text has contiguous source starts,
all logical text and short-cell content appear once, markers advance once,
and rich bidi cursors are recorded before visual ordering. Generated hyphens
consume no input. Tabs, figures and explicit breaks retain atomic identity.
Continuation anchors are removed after their first placement. Word height and
grid restoration remain in the existing conversion path.

Panics: variant row/cell shapes come from the same accepted AST. Table selection
is total for the two existing private-trait implementations. Cursor slicing is
checked and text splits use scalar boundaries. No new untrusted index path
found.

OOXML: production parsing and writing are unchanged. Alternative table
measurements reuse existing provenance, style, numbering and cache keys.
Diagnostics from alternate measurements are retained without duplicates.

Tests: 125 shared-layout tests and 315 Word-layout tests pass, plus four docs.
The three new cursor cases, selected-width notes and keep-next cases, split
cell source/numbering/cache checks and nine tall-story alignment combinations
exercise the new behavior. Changed-story warm results equal fresh results.
The owned Hanji gate fails at the original immutable pin and passes with the
corrected source. All six independent API contracts pass.

Structure and compatibility: public carrier fields, feature flags, crate graph,
and existing row-splitting guards are unchanged. The existing LayoutBlockLike
trait and split function are instantiated for LayoutBlock and SharedLayoutBlock.
New line-module functions are additive. Active story widths are finite, while
retained cache budgets stay unchanged. Ordinary line breaking avoids cursor
character-count work.

Validation: scoped strict Clippy, formatting, prose and generated-adapter
checks pass. The 49-entry deterministic hash harness is unchanged. Browser
WASM compilation, strict changed-crate documentation, no-default shared tests,
and oxml-layout publish dry-run pass. The 139-test policy suite passes with two
existing skips, followed by a successful final package-record/README rider.
Archive measurements were rederived, with member counts unchanged and both
changed crates below 10 MiB. No baseline was regenerated to hide an output delta.
Native Word fidelity remains unverified. No merge, release or deployment runs.
