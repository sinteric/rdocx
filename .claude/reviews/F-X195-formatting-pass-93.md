# F-X195, formatter equivalence, pass93

**Reviewed**: Proposed formatter delta in three existing Rust carriers, against current worker preimages. Candidates were prepared by ROOT using rustfmt 1.97.1, edition2024, stdout emission. No source adoption occurred during this review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and equivalence

Manifest SHA `6e3463d579aa4cc9abe6d9e2456d195d0c301fefbc692e55a1f945cac45d0e5b` and diff SHA `7845deb0fc23c3d036e3ee57d27caa523b7bf41661b86ce8d0525f9832c801fc` authenticate the proposal. All three current preimage and candidate SHA values match the manifest. Reconstructing the unified diff from those exact bytes reproduces the proposed diff exactly.

Ordered lexical comparison preserves literal and comment bytes. `crates/rdocx/src/document.rs:36978` changes only whitespace. Two other hunks normalize optional trailing argument commas, so this is formatter-only equivalence rather than literal whitespace-only identity. `crates/oxml-layout/src/font.rs:1161` removes the optional trailing comma from the sole saturating_add argument. `crates/rdocx/src/svg.rs:1522` adds an optional trailing comma to the sole f64::from argument. All remaining ordered tokens are identical. Neither comma changes arguments, evaluation order or operations.

Strings, numeric operands, source ownership, font axes, synthetic paint, regular golden assertions, independent SSIM thresholds and oracle inputs remain unchanged. The SVG definition rule and selected-instance controls are only reflowed. The font trace capacity calculation remains the same expression.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. Only whitespace and the two optional argument commas differ.
- Contract: zero findings. No behavior, accepted scope or public signature changes.
- Panics: zero findings. Existing unwraps and failure paths are unchanged.
- OOXML: zero findings. No string, namespace, preservation or serialization operation changes.
- Tests: zero findings. Assertions, thresholds and expected values are identical. Original source-qualified runtime receipts remain immutable and authoritative for their tested source. The exact formatter equivalence permits a separately recorded source transition, without relabeling those receipts or claiming a fresh runtime result.
- Structure: zero findings. No files, modules, dependencies or abstractions are introduced.

## Handback

Only this report was written. No formatter, Cargo, source, cache or environment mutation occurred during review. Actual adoption and current format gate remain separate ROOT actions. This pass ends and returns the review writer lease.
