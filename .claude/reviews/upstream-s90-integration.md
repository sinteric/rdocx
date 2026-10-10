# Upstream S90 integration review

Base: `6482b1cbe8aefd9f6586a05931987950d6d63608`.
Upstream: `733ce6f7b2b816c5760692abf5c99d68b29ae897`.

This integration retains both histories and the existing Hanji layout extensions.
The 28 merge conflicts are resolved without replacing the fork with upstream.
The upstream workflows and the existing pinned refs are retained.

The source conflicts are in `rdocx-layout/src/paginator.rs` and `table.rs`.
Header and footer paragraph spacing is applied before the upstream anchored
DrawingML placement. Table indentation retains authored-versus-absent provenance
and the fork's RTL leading-margin conversion. The upstream legacy positioning
adjustment still applies only to non-RTL, non-floating top-level tables.
The related layout suite passes all 335 cases, including existing fork cases.

Both rendering manifests are copied from the immutable upstream revision before
running the harness. F-X183's reviewed table side-padding and legacy placement
changes supersede the older fork alignment deltas. The deterministic hash harness
matches all 49 entries without recording goldens or changing comparison code.
A final rerun covers the subsequent equivalent Rust compatibility adapters.

Rust 1.99 introduces a constant chunk-size lint. Fixed-size chunk iteration uses
`as_chunks` or `as_chunks_mut`, available below the workspace's Rust 1.93 MSRV.
All callers continue to discard the same trailing remainder. Array reference
comparisons in test helpers are adapted without changing expected values.
Boolean simplifications retain short-circuit order and side effects.
These adaptations change no public signature or intended rendering behavior.

Archive footprint rows are remeasured from the actual locked local packages on
Debian 13.6, x86_64. The existing normalized member counts and 64-byte gzip
comparison tolerance are retained. Historical performance observations retain
their original platform and date.

The required PPTX corpus has 49 verified decks locally. The final Google Slides
export is blocked by the Cloud network with a tunnel HTTP 403. All 22 affected
corpus checks are recorded as blocked, not passed. The unchanged CI fetches and
verifies the complete 50-deck corpus before executing those checks.
The complete Word regression suite and external Office comparisons have not
been rerun locally. Compilation is distinguished from WASM execution.

Hanji remains pinned to the previous fork main. A future S90 consumer repin needs
its own API adaptations and preview acceptance evidence. This PR provides no
claim of Flutter or Hancom Print acceptance.

Review aspects: correctness, preservation, schema ordering, panic boundaries,
test expectations, and repository structure. No additional source defect was
found in the conflict resolutions. The missing corpus input remains an explicit
verification limit and is not resolved by weakening a test.
