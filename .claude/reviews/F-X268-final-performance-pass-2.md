# F-X268 final performance remediation, pass 2

**Reviewer:** Independent Codex agent `/root/review_source_pieces`.
**Scope:** Only pass 1 D1's declaration-name guard correction and its new regression. The full unchanged implementation assessment and evidence remain in `F-X268-final-performance-pass-1.md`. This is not a broader feature audit.
**Source:** Canonical dirty `crates/rdocx/src/document.rs` against `67e161046bef0d3e3d22b40ab93a548259866118`. Current file SHA-256 `89939cc161617467abfdb5ef1769b1cb9afb56a50c849ba99efe5d8bba53eb26`.
**Method:** Read-only source and existing execution-evidence inspection. No builds, tests, source edits or delegation.
**Verdict:** D1 is resolved. Zero defects, zero smells and zero nitpicks in this bounded follow-up. Pass 1's unchanged source assessment applies to the corrected batch. Performance and memory acceptance remain pending.

## D1 closure

The guard now tests the actual declaration name `xmlns` (`crates/rdocx/src/document.rs:23748`). That matches the representation stored by `apply_namespace_declarations`, rather than looking for an impossible empty string. Body-local declarations continue to decline independently. Canonical `w` producer header normalization remains eligible, while a root default declaration takes the checked whole-reader fallback. No source parser or writer policy was weakened to obtain the rejection.

The new test combines the producer-normalizing XML declaration with canonical `w:document` carrying `xmlns="urn:producer"` (`crates/rdocx/src/document.rs:48541`). It actually opens the package, requires `source_blocks` to decline and verifies the public parsed paragraph remains `alpha` (`crates/rdocx/src/document.rs:48557`). Thus it distinguishes unsupported fast-path admission from ordinary valid-input parsing. It does not reject or canonicalize away the valid fixture merely to make the control pass.

## Evidence and limits

- `/private/tmp/s954-final-performance-review-red.log`: actual pre-fix execution failed the new fallback assertion. This is a semantic RED, not a compile failure.
- `/private/tmp/s954-final-performance-review-green.log`: all five `final_gate_*` tests passed in 10.56 seconds after the guard correction, including the four prior producer, namespace-preflight, scoped-normalization and cell-route controls.
- The serial follow-up clippy log had begun checking rdocx at review inspection. No completion is inferred from that progress line. The parent must retain its actual final exit separately.
- The unchanged full affected native batch and precise qualified-tool retries remain the evidence recorded in pass 1. They are not represented as a newly repeated full batch.

No additional correctness, panic, resource, structural-smell or nitpick finding was established. The original pinned release timings still contain 15 required misses until actual affected remeasurement establishes otherwise. Memory qualification includes the new caches and remains pending. Native Word execution permission and all other outstanding final riders retain their existing status. No final feature, sprint or release acceptance is claimed.
