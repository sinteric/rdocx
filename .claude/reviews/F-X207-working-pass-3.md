# F-X207, working implementation, pass 3

**Reviewed**: Complete sixteen-carrier working diff,3844 insertions and64 deletions, current source74b38a46c471de4e7fd3cfbeddc4a0fdd18c0242e1bc433b37308bd4252ebf54. Independent parallel auditor read the complete native, layout, render, Python, stub, test and HLD diff against the approved plan. Root authenticated the same2943-byte/size/mode source inventory during targeted pass2.

**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Defects

### D2, resized shapes do not preserve their painted affine anchor

`crates/rpptx/src/lib.rs:11888`

The resize displacement applies flips before rotation and treats the rotated displacement as unscaled local coordinates. Actual paint applies rotation then flip then translation and group_transform at crates/rpptx-render/src/lib.rs:1877. Transform::then applies self before next at crates/oxml-layout/src/transform.rs:44. Parent scale is already applied to bounds at crates/rpptx-layout/src/context.rs:2792 and split separately at:3802. A horizontal top anchor with height100 to200 points, child rotation30 degrees and parent scale2 by1 needs local horizontal displacement minus12.5 points. The current minus25 displacement moves the painted anchor by25 points. Unit scale with horizontal flip also needs the opposite horizontal sign from the current result. Rechecking text containment does not prove anchor equality. Existing standalone rotated controls omit the distinguishing parent scale and flip. Use the full actual painter transform and checked inverse conversion, with genuine scaled, rotated, flipped and nested owner controls. The invalid native group capture is excluded from this finding.

### D3, explicit fitting family loses imported font-owner attributes

`crates/rpptx/src/lib.rs:12002`

Replacing existing latin, ea and cs slots with TextFont::new discards retained pitchFamily, charset and foreign attributes. Existing TextFont captures those attributes at crates/oxml-drawing/src/text/paragraph.rs:423, but new creates an empty attribute collection. An imported font with typeface Old, pitchFamily34, charset0 and foreign marker keep loses all three when fit_text_at receives an explicit family. The approved plan preserves unknown properties and original owners. Update the existing typeface while retaining its captured attributes across direct runs, fields, defaults and end properties, creating a new owner only when absent. Add source-exact saved/reopened controls.

### D4, unused shared-part table cells disappear from the inventory

`crates/rpptx/src/lib.rs:12695`

owned_text with cell=None skips tables, while table cells require a concrete row and column at crates/rpptx/src/lib.rs:12083. The unused shared scan at:12465 therefore omits flagged cells in unused layouts, masters and notes masters. Successful refresh can claim every-owner accounting without reporting or refusing such an unmeasured owner. Enumerate physical cells in the unused shared scan and return the existing named unresolved-context refusal. Add actual unused shared table-cell controls with whole-source atomicity.

## Not found

All six aspects were audited. Correctness and contract have D2/D4. OOXML preservation has D3. Tests have the discriminating gaps attached to those findings. Panics and errors: zero additional findings, internal expects follow checked bounds and family guards. Structure: zero findings. Smells: none. Nitpicks: none. Atomic staging, immutable Python snapshots, checked location fingerprints and original read-only rendering have no additional findings within the inspected scope. Earlier fixed-width D1 remains resolved by pass2.

## Authentication and handback

Worker HEAD8f57b6ac5e6c9e2d8ac40cce1224201744a964d1 and all2943 records matched for both independent audit and root targeted authentication. Only this review file was written during this pass. Return for distinct remediation, then an independent fresh pass. Installed, full native, native group paint, hash, WASM, archive and integration acceptance remain unproved.
