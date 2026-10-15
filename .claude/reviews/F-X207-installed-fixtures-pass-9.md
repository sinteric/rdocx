# F-X207, late-unfit fixture exception, pass 9

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

crates/rpptx-py/tests/test_documented_examples.py:7098 expects rpptx.LayoutError, which is not exported. The public family defines RpptxError at python/rpptx/__init__.py:10 and exports it at line61. The implemented refresh maps native failures through rpptx_to_pyerr at src/presentation.rs:346, whose domain fallback is RpptxError at src/lib.rs:135. The original intended floor refusal therefore never executes because exception-class lookup raises AttributeError. Use the existing public RpptxError while retaining the original physical-location/floor message and whole-deck atomicity assertions. No new public exception is required.

## Smells

None.

## Nitpicks

None.

## Not found

Actual no-install receiptddfe08358bae0b8b8f0a6d92affc7720db26076b0a5b86cdc7eaa07fe6cabf16 reports14 passes and this single fixture failure. Both previous handle corrections passed. Actual late-unfit refusal and four typing rows remain unrun. No production failure or complete gate is inferred. Only this review file was written during review. Return to a separate one-operand fixture correction and unchanged installed-pair renewal.
