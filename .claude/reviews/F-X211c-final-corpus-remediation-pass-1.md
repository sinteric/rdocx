# F-X211c, final corpus remediation, pass 1

**Reviewed**: Corrected3366-file source map `40b2659c618bb378170c2cd31cc03864df4adf27259d31ab57726e2cda107826`, exactly five changed carriers against qualified predecessor89d8. Independently authenticated every worker body, size and mode. Failed first remediation sourceb863 and its nonexistent AlignDirection compile failure remain retained. The successor changes only its two native test references to actual ShapeAlignment, with product changes unchanged.

**Verdict**: 1 documentation defect, 0 smells, 0 nitpicks. The two original implementation defects are addressed in source, but the public alias documentation needs correction. Installed Python/render controls remain pending.

## Defect

### D1. Auto-numbering documentation is attached to bullet_char

`crates/rpptx-py/src/text.rs:1182` through line1186 describes auto_number's scheme strings and scheme assignment. The inserted bullet_char getter immediately follows those comments at line1189, so it receives this documentation. The actual auto_number getter at line1199 loses it. This is inaccurate public documentation, not only a formatting preference: bullet_char returns the canonical bullet choice, including True for numbering, and assigning a string sets a character choice rather than an auto-numbering scheme.

Move the existing auto-numbering description back to auto_number and keep the compatibility bullet-choice description on bullet_char. Do not change the actual checked alias semantics or stub types while correcting documentation. No fix was made in this review.

## Smells

None.

## Nitpicks

None.

## Original source defects and meaningful controls

The new alias getter/setter at `crates/rpptx-py/src/text.rs:1189` and line1194 reuse the actual checked bullet provider, including owner validation, character construction, canonical metadata preservation, clearing and no-op behavior. The stub and typing control declare the matching str/bool/None surface. No new resolver, namespace parser, bullet storage or source-character remapping is introduced.

The complete numbering/four-family regression at `crates/rpptx-py/tests/test_documented_examples.py:10264` checks all original marker families/characters, actual numbering/font/colour/size metadata, source immutability, PDF page/diagnostic presence and saved/reopened XML. Separate invalid empty/True/integer cases check atomic bytes and held-owner validity, plus False/None semantics. Bundled caller inputs keep this regression independent of host Office fonts. They do not by themselves substitute for the frozen specimen's exact caller-font or painted-glyph/native qualification.

The native error at `crates/rpptx/src/lib.rs:37604` names reference context, the existing actual minimum and selected count while preserving InvalidShapeMutation and validation/publication order. The original minima remain selection alignment2, selection distribution3 and slide reference1. Native and Python tests preserve one-shape slide-reference positives and byte-atomic, held-owner refusal checks, rather than only matching a permissive error keyword.

No further correctness, bounds, atomicity, preservation, source-owner, typing or structural finding was found in these bounded implementation repairs. Frozen scripts/support/body digests remain unchanged. The original whole100 candidate is not retried or rescored in this review.

## Actual gate boundary

Read corrected scoped receipt `8232a65fdead71a99df1731ded29c621ed9e4da1b7293e6223251cd180a154e0` under `/private/tmp/S91-F-X211c-final-corpus-remediation-2`. Its all-target/all-feature check, strict Clippy, native regression compile and actual typed minimum/atomic regression rows are qualified zero-exit with safe cleanup. The compile uses the ordinary2MiB setting and the typed runtime uses its explicitly admitted8MiB test setting. Neither is relabeled as a12-control ordinary consumer rider. The failed b863 compile remains separate.

The fresh Python wheel and nine new installed controls have not yet executed. Their full rendering and saved-file facts remain uncredited, as do parent full100/native/rendered gates. Source review is not a claim that the previous94/6 result has become96/4 or that full acceptance has been reached.

No source, test, runtime, CP environment, UI, remote state, dependency, artifact or Git state was changed. No tests or build were rerun. Only this canonical review report was written under the exclusive lease. Review ends and returns control for separate documentation correction and current installed qualification.
