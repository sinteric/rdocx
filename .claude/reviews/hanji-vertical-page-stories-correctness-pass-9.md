# Hanji, fractional landscape measure correction, bounded review

**Reviewed**: correction after 79e8ab89, five table selection call sites, the canonical measure selector and one source-owned renderer regression
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Effects checked

The pass 8 finding is resolved at the caller. Vertical table selection reads
the selected page geometry's body measure before transposition reconstructs
the margins. That is the same raw height used when the engine prepares its
finite variants. Horizontal pages retain their active column width. Row starts,
split transitions, whole-row transitions and keep-next lookahead all use the
same selector. No tolerance, rounding, cache key change or public field is added.
Exact table variant equality and exact width-aware cache identity remain intact.

The owned fractional landscape regression failed before the correction on
the first table's lower bound. It now requires y=336.05..475.9 pt on page one
and y=436.05..475.9 pt on page two. It preserves the two cell strings and the
authored empty separator's source identity, checks contiguous scalar ranges,
and requires four distinct cold table builds plus four exact warm cache hits.
Cold/warm source maps and complete layout results match. Header/footer bounds
and body origins stay inside the selected bands.

All 317 Word-layout tests and its documentation test pass. Strict changed-crate
Clippy, formatting, prose, adapters, three focused archive-policy checks and
all 49 unchanged deterministic hashes pass. The measured archive remains
below 10 MiB at 281,377 compressed bytes, 1,507,762 normalized member bytes
and 15 members. The earlier source/numbering and percentage-spacing gates pass.

Review is bounded to the known width-lookup finding and this correction's
selection, cache and geometry effects. Downstream validation will use an owned
fractional landscape API regression and the exact immutable renderer commit.
Native Word fidelity remains unverified. No merge or release occurred.
