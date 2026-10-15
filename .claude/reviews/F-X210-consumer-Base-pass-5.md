# F-X210, consumer Base tests, pass 5

**Reviewed**: Focused R1 to R2 correction in `/private/tmp/S91-F-X210-consumer-Base-tests-r2-prepared`. Two removed test lines, no production change. Receipt SHA256 `47ddece8f1249bcd1eedb052432def2c422242c7136023a68326556b82d9c601`. Incremental diff SHA256 `70a6d56ae2fd333c94c4cd71f1a6615c44bcfe1f55b827f426c43845b221c7b0`. Complete test-only diff SHA256 `4a60f96d0540ecf3faa7d6d80f94adc893909ddf61322fa6db3e068acfcb53e7`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

Zero. Review4 D1 is resolved in this successor. The immutable prior review and defective R1 remain retained.

## Smells

Zero.

## Nitpicks

Zero.

## Exact correction and retained contract

The PPT candidate removes precisely the nonexistent `frames[0].diagnostics` access and its dependent assertion. Inverse insertion recovers the complete R1 candidate exactly. `crates/rpptx/src/lib.rs:13612` and13613 retain the existing `text_layout_deterministic(1.0)` call and real frame-count positive setup. The full-presentation Aptos diagnostic discriminator, four visible Carlito contribution preconditions, repeat equality and package-source preservation remain unchanged. The PPT bullet control and both Word controls remain exact. All four command selectors, order, counts and deciding markers remain byte-identical.

The measurement-owned diagnostic projection remains mandatory later implementation and GREEN acceptance, as explicitly recorded in the successor receipt. It is not claimed by frame count, and no prototype field or diagnostic shim was introduced to manufacture Base. Existing API definitions support the remaining references examined in review4. No additional future-field compilation blocker was identified by this focused static review.

## Authentication

All2909 current provider byte, size and mode guards match source map SHA256 `0a46980f5c88eef9654d376daad798b03cc48fbfb6e17108983a8c546a35f781`. All packet artifact hashes and sizes match. The Word candidate and commands are byte-exact R1 copies. The PPT candidate SHA256 is `aa8c802defc3ec9806c4e98dc3b4da5c949c48f05e9624ae3da34aaf972ad6dd`. The correction changes no fixture, expected symbol mapping, public API, asset or product implementation.

## Aspect results and limits

- Correctness: Zero findings in the focused removal.
- Contract: Zero findings. Existing-API Base is restored while future projection acceptance remains required.
- Panics: Zero findings. Existing authored-fixture preconditions are unchanged.
- OOXML: Zero findings. All original source and save/reopen assertions remain unchanged.
- Tests: Zero findings. The three other tests and all four exact selectors remain intact. Compilation, preconditions and named semantic RED still require actual execution.
- Structure: Zero findings. No new repository construct is introduced.

No Cargo, source edit, product import, native UI or execution occurred. The bounded controls prove positioned semantics only after actual execution, not independent paint or full feature acceptance. Review4 broader coverage limitations remain applicable.
