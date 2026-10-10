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
matched all 49 entries after the final Rust compatibility adapters, without
recording goldens or changing comparison code. The reviewed deterministic
animation GIF and Motion JPEG AVI manifest also passes after those adapters.

Rust 1.99 introduces a constant chunk-size lint. Fixed-size chunk iteration uses
`as_chunks` or `as_chunks_mut`, available below the workspace's Rust 1.93 MSRV.
All callers continue to discard the same trailing remainder. Array reference
comparisons in test helpers are adapted without changing expected values.
Boolean simplifications retain short-circuit order and side effects.
These adaptations change no public signature or intended rendering behavior.

The first normal CI MSRV run finds one additional integration mismatch in
`direct_table_alignment_overrides_style_and_reopens`. This test is introduced by
upstream F-X182 and is absent from the fork base. Its RTL center shift expects
physical x to increase by 162 points. The existing fork converts a leading-side
offset to physical left, so that shift decreases physical x by 162 points.
The base already contains `rtl_table_alignment_and_signed_indent_use_the_leading_margin`
and the same physical conversion. The targeted MSRV case reproduces the mismatch.
The test now converts its logical expected shift to physical x for RTL cases.
All direct/style, nested, compatibility, signed-indent, border, fill and
preservation assertions remain. Renderer code and goldens are unchanged.
These RTL expectations describe the existing fork contract, not new native
Office evidence. The reconciled integration case passes on Rust 1.93, as does
the unchanged signed-indent unit case. Whole-workspace Clippy, 49 output hashes,
formatting, refreshed archive inventory and policy checks pass after the adapter.
Final CI must be rerun on this reconciled test head.

Archive footprint rows are remeasured from all 22 locked local packages after
the compatibility adapters on Debian 13.6, x86_64. Two measurement rounds converge
and the complete README inventory validator passes. The existing normalized
member counts and 64-byte gzip comparison tolerance are retained. Historical
performance observations retain their original platform and date.

The required PPTX corpus has 49 verified decks locally. The final Google Slides
export is blocked by the Cloud network with a tunnel HTTP 403. All 22 affected
corpus checks are recorded as blocked, not passed. The unchanged CI fetches and
verifies the complete 50-deck corpus before executing those checks.
The complete Word regression suite and external Office comparisons have not
been rerun locally. Three additional native library tests stop on external
Poppler version assertions. The required 26.01.0 download also receives a tunnel
HTTP 403, while the installed pdftotext reports 25.03.0 and pdftoppm reports
26.05.0. Those tests are not counted as passes and their assertions are retained.
Compilation is distinguished from WASM execution.

Hanji remains pinned to the previous fork main. A future S90 consumer repin needs
its own API adaptations and preview acceptance evidence. This PR provides no
claim of Flutter or Hancom Print acceptance.

Review aspects: correctness, preservation, schema ordering, panic boundaries,
test expectations, and repository structure. No additional source defect was
found in the conflict resolutions. The missing corpus input remains an explicit
verification limit and is not resolved by weakening a test.

Whole-workspace Clippy with all targets and features passes on Rust 1.99 after
the equivalent compatibility changes. Rust 1.93 native library checks execute
125 passing oxml-layout cases, 72 oxml-opc cases with one ignored, 96 oxml-pdf
cases with one version-gated failure and one ignored, 513 rdocx cases with two
version-gated failures and seven ignored, and 85 rpptx cases with four ignored.
The ignored cases retain their upstream external evidence requirements.
Formatting and both WASM binding compile checks pass. The repository policy
module executes 139 passing cases across the complete run and missing-tool
retry, with two explicitly gated published-family checks skipped.
Normal CI fetched the complete corpus and pinned external tools. The first head
passes the related hard gates and bindings but fails the new RTL test assumption.
The final reconciled PR head needs its own complete CI result.
