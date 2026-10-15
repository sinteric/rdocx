# F-X208, review7 remediation, pass 8

**Reviewed**: Immutable source31 four-carrier correction against source24. Capsule receipt SHA-256 23da38a23109d1bfd1b2c5d1e1d54ae2f99d9b9c4ef62875e97b931b32975cd7. Read approved footer/cache/diagnostic contract and all correction hunks. Authenticated four compressed candidates and eight execution receipts. This is a bounded remediation review, not whole-feature completion.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, nonpositive effective footer extents still publish invisible content

`crates/rpptx/src/lib.rs:15804`

The new preflight accepts any present extent. An imported footer transform with cx or cy equal to zero, or negative, therefore passes this check and publishes footer content. The actual renderer at `crates/rpptx-layout/src/context.rs:3043` refuses these extents and resolve_ordinary_shape at :1362 omits the shape. This retains the original silent missing-footer failure for a concrete unsupported geometry. Apply the same strictly positive extent admission before publication and cover zero/negative extents plus byte-identical refusal in existing controls.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: compatible datetime alias preserves an existing cache and field identity. Explicit empty cache remains distinct. Contract: uncached automatic fields produce diagnostics without a clock, and the HLD no longer claims slidename. Panics: no new unchecked indexing in the correction. OOXML: template source and ordinary field representations are retained. Tests: three semantic Base failures and three subsequent deciding passes are distinct, prior fixture and lint failures are retained. Structure: concrete existing files and receiver remain, no new module, trait or lint suppression. Four affected footer/field controls actually pass even though the enclosing first Clippy batch failed. Later six-crate Clippy actually passes. Formatting and final installed artifacts remain worker gates.
