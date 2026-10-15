# F-X208, date-mode remediation, pass 2

Reviewer: Codex fonts_resume. Verdict: 0 defects, 0 smells, 0 nitpicks. Focused pass 1 D1 is resolved.

## Bound scope

Only the two-hunk native facade delta from original focused source `187122a9db0f7859801556757dafa5271c9258291f102d16b877825373309f6a` to exact source-production-10 facade `a0baff63872be406e8933e238a71275f525e470cb71b7bc13ac71d0e52ea7e5d` was reviewed. Immutable input `/private/tmp/S91-F-X208-date-mode-correction-snapshot-1` contains a reconstructed complete carrier authenticated against the retained actual pre-runtime map, with correction diff SHA-256 `1a9cf5b0cb39945c6d28e00d09871b47605afca0ab40ca99c6179769e18d01d8`. The decompressed carrier and source map were independently authenticated. Later live source changes are excluded.

Source-production-10 map SHA-256 `1ec7d5745237f08078b41881dcacddea643db9c8b6da5673f85a0b5c235723b6`. The delta adds the field-aware text no-op condition and one existing-control extension. There are no other carrier changes in this comparison. This is not confirmation of unchanged review categories, Python batching, transitions or full F208 acceptance. No source edit or runtime replay occurred.

## D1 resolution

`crates/rpptx/src/lib.rs:15665`

Equal visible text returns unchanged only when all paragraph runs are free of typed fields. The automatic-date-to-equal-fixed assignment now takes the existing `body.set_text` path. Ordinary fixed text retains the valid repeat no-op. The predicate examines every paragraph, uses the existing formatting rewrite operation and introduces no serializer or API change.

`crates/rpptx/src/lib.rs:18449`

The existing native footer control now creates Automatic datetime1 with a supplied cache, then assigns Fixed with the exact same visible string. It asserts the initial typed automatic state, Fixed after conversion, Fixed after save/reopen and byte-exact repeated fixed assignment. Original unique-ID, reopened state, repeated deck publication and invalid-text atomicity assertions remain unchanged. Reverting the production condition makes the added Fixed assertion fail because the date reader still sees the retained automatic field.

## Actual evidence

Actual receipt `.claude/scratch/F-X208-start-1/actual-native-date-mode-GREEN-1/receipt.json`, SHA-256 `6e7e9afe3a8144fd7139c5221fb08657dfcac44b08656d5ea2e60298eb11b9d1`, records the exact named footer test under Cargo1.97.1, offline, locked and all features. Authenticated raw log SHA-256 `3ad5c2854f29e34f6fb03d5700825fbbbc04cea64241ebc557d4edab28a2eb18` records 1 passed, 0 failed, 0 ignored. Source-after matches. Recorded group82904 is gone with ESRCH, leader reaped and no signals. The historical status spelling `completed-Base-observation` is preserved and does not relabel old failures or grant a new Base claim.

## Findings and limits

Zero new correctness or contract findings in this predicate. Zero panic, OOXML preservation or structure findings earned by this narrow delta. Zero findings in the deciding mode/reopen/repeat test. Remaining native, installed and full-feature matrices retain their pending status from pass 1. The worker may copy this externally staged report into its review directory under its exclusive writer lease.
