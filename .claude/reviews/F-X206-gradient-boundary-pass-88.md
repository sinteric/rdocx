# F-X206, gradient boundary, pass 88

**Reviewed**: R35 external two-carrier diff97871ccfb8e00b42b1cebf8cf5b8a91d98c04a8cfe26563a9214462d933444e7 against current d7e9dd6e.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, global rotation bypass also skips supported placed gradients

`crates/rpptx-layout/src/style.rs:290`

The proposed substitute_fill bypass treats every rotate_with_shape=false gradient as unsupported. Current context.rs1346 through1360 explicitly clears that flag for an actual placed shape with no effective rotation, making the gradient supported. Skipping placeholder substitution globally leaves phClr unresolved in that supported case, including a fill style with a real reference colour. Restrict the unplaced validation boundary without changing placed supported-gradient substitution. Add a placed unrotated phClr/reference positive and a rotated unsupported partner alongside the original unplaced control.

## Smells

None.

## Nitpicks

None.

## Not found

Missing-colour stop ordering, background rotation normalization, source preservation and dependency structure produced no other findings. Proposed runtime remains unexecuted. Review ends before separate remediation.
