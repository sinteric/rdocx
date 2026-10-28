# F-289, native CJK metrics, pass 1

**Reviewer:** Independent Codex reviewer `/root/review_shared_proof_batch`. This reviewer prepared the earlier external native metric assessment but did not author this production implementation or its tests.
**Reviewed:** Frozen working changes against `8f1ec6d690a2cdcf7c85b26dec7b2c3fc3aef042`, four existing files,503 insertions and11 deletions after excluding Root's separate PowerPoint subsection in HLD12. Product changes are confined to private Word metric acquisition/projection in font.rs. Engine changes are tests only. Read the approved F289 repair boundary, progress, canonical microscope/workflow and applicable HLD policy. No builds, tests, rendering, native UI, RSS or source remediation were performed.
**Verdict:** Zero defects, zero smells, zero nitpicks in this bounded source repair. Final dense-form A7 and integrated acceptance remain pending.

## Defects

Zero.

## Smells

Zero.

## Nitpicks

Zero.

## Source and contract examination

The actual raw OS/2 version/length guards precede access to both codepage words, and only bits17through20 of word1 classify the face (`crates/oxml-layout/src/font.rs:609`). Requested names, scripts and Word slots do not select the branch. Supported classification requires positive hhea ascent and nonpositive descent, and converts the latter through i32 before negation (`font.rs:613`). Invalid hhea, missing OS/2 and old/truncated codepage data keep established fallback behavior. The non-CJK USE_TYPO_METRICS and unsigned Windows metric branches remain intact. No untrusted indexing or arithmetic overflow exposure was identified in these new operations.

Fractional leading is computed after conversion to f64 and before scaling (`font.rs:2270`), so fifteen percent is not rounded in design units. Each component receives that amount independently, and the CJK acquisition stores zero gap. This matches the approved symmetric allocation and avoids counting hhea gap or the extra leading twice. Shared FontManager::metrics and shaping are unchanged. Actual policy acquisition stays attached to the resolved physical LoadedFont at `font.rs:2164`, rather than an edit-dependent or requested-family cache. The public method signature is unchanged. The existing Word convert and engine callers continue owning text-line, mark and reflow restoration. No paragraph-mark minimum, grid intervention, font-registry lookup or pixel offset was added.

The native evidence distinguishes numeric behavior from local synthetic arithmetic. The preserved independent four-face inputs match98 embedded mapped glyph outlines and advances to exact physical vendor sources. Their hhea/Win/gap contrasts and six sequential baseline measurements discriminate gap-free hhea and symmetric leading. Earlier theme/EA, retained mark-size and no-grid negative observations remain counterevidence to the abandoned explanations. This review consumes that provenance without treating it as a successful final eight-state render differential.

## Tests and actual receipts

The raw font controls at `font.rs:3759` vary each supported codepage bit, adjacent non-CJK bits, a misleading requested name, typo precedence and multiple sizes with divergent hhea/Windows/typo/gap values. They assert the supplied physical bytes actually resolve and preserve shared metrics and shaping. Boundary controls at `font.rs:3822` cover OS/2 version/length, second-word-only bits, absent OS/2 and invalid hhea. The implementation itself retains fractional design-unit arithmetic. Existing bundled assets are not patched, and the32-face test confirms no bundled face currently activates this policy (`font.rs:3896`).

The Word controls at `crates/rdocx-layout/src/engine.rs:17621` exercise plain and multilingual items, text with an inline object, single/proportional/atLeast/exact rules, nearest-text break carriers, a blank mark and object-only proportional spacing. They assert actual ascent/descent and advance, not only total height. The source-built integration control at `engine.rs:17764` exercises multiple wrapped lines and real float reflow, with flagged physical synthetic font bytes, nonzero covered CJK glyphs and separate exact spacing baselines. Existing exact and object behavior remains protected by the consumed selections.

Consumed `/private/tmp/s954-f289-cjk-final-receipts.json` and its actual logs: full font library151passed and1unchanged external smcp oracle ignored, new CJK Word2passed, existing convert11passed and exact-word3passed. The new Word, convert and exact selections are distinct tests. The full font selection already includes the new font tests, so they are not additional counts. The genuine prior-policy RED is red3, one failed bit17 metric assertion with two passing controls. Earlier compile/fixture setup failures are not semantic RED. The first clippy receipt exited101 for a test-only lint. The correction receipt records the subsequent scoped font rerun and both-crate all-target/all-feature clippy exit0. Format/prose/diff receipts passed. No new test execution was performed by this reviewer.

The inline storage receipt is compiler/platform-specific: WordLineUnits12to16bytes, Option remains16 and LoadedFont remains2024 on this aarch64 toolchain. Source introduces one fixed bool and no heap payload/history edge. This is not measured RSS or permission to reuse an unauthenticated final memory artifact. Final full-source refresh/calibration remains explicit.

## Frozen identity and pending gates

All four reviewed file hashes match `/private/tmp/s954-f289-cjk-frozen-hashes.json`:

- font.rs `9818ded50756f340ae278e337d1a808300bcc0ae3aa02eb22c24a3da15adbf4b`
- engine.rs `448bf7a7a8f1c3107105bae29cf714f4aaba17715a30558b2c948013483afa99`
- HLD08 `c730bf42bf150ad8c4030d54b6f2fab5ff6f154f0de9189f991dcd5df050a345`
- HLD12 `22c5d5d396cd19f79ffd28bae8242dc42cd8421082409ae411e2d2a503a2fb20`

HLD08 describes the actual Word-only policy/fallback and HLD12 distinguishes native qualification from arithmetic controls. Root's PowerPoint reference subsection and other dirty PowerPoint work were excluded. No public API, dependency, module, source file or bundled font was introduced.

Normal final artifact rebuild, integrated hash harness and applicable final riders, final source/memory refresh and eight native dense-form Rust/Python A7 comparisons remain pending. The existing equal-page/text/checkbox and0.95 per-page144DPI gate is unchanged. This clean bounded implementation review does not claim feature delivery, native A7 completion or release acceptance.

## Not found

Correctness: zero findings. Contract: zero. Panics and untrusted indexing: zero. Font/raw-table preservation and OOXML-facing behavior: zero. Tests: zero remaining local findings. Structure/public surface: zero. Smells and nitpicks: zero. Review ends here and returns the writer lease.
