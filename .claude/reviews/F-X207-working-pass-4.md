# F-X207, working remediation, pass 4

**Reviewed**: Complete two-carrier incremental diff73f4cdc726ff1dac58746cb793a4a7e9e3ddc134acbb1a652d9d2bb04e101fc8 and relevant current whole-feature inventory. Current source91ad36623f554a543ab817a98bf7e63261eb0bf365d246aa62fd27df103002dd.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authentication

ROOT and the independent auditor authenticated all2943 current byte, size and mode records. Only crates/rpptx/src/lib.rs and crates/rpptx-render/src/lib.rs changed from reviewed74b38, with all2941 other records exact. Receipt562736d20a62e31291e813ade96ffbbdbbe45e5361066d40b5d5ab09af1f31e6, incremental diff and actual26/27 receipt/log hashes authenticate. Actual binding all-target check exits0 and28 focused tests pass, with no failure or ignore. ROOT freshly confirmed every owned group is absent. Intermediate failed22 through25 remain retained.

## Earlier finding dispositions

- D2 resolved at crates/rpptx/src/lib.rs:11880. Old and proposed anchors use the unchanged actual painter transform. Inverting the actual group linear transform and local axis scale converts world displacement to stored offsets. Nonfinite and noninvertible mapping errors retain staged atomicity. Nine actual nested, scaled, rotated and flipped combinations pass. This is painter geometry evidence, not native PowerPoint paint parity.
- D3 resolved at crates/rpptx/src/lib.rs:12046. Existing TextFont slots change only typeface, preserving pitchFamily, charset and retained attributes. Only absent slots are constructed. Direct, default, field and end-property controls pass.
- D4 resolved at crates/rpptx/src/lib.rs:12743. Unused shared table owners enumerate physical cells and deduplicate by exact part, child path and cell. Layout, master and notes-master controls report and refuse the actual uninstantiated flagged cells.

## Defects

### D5, unused shared AlternateContent text is absent from accounting

`crates/rpptx/src/lib.rs:12769`

An unused shared layout, master or notes-master containing locally namespace-closed mc:AlternateContent with a genuine p:sp/p:txBody/a:bodyPr/a:normAutofit is represented as ShapeTreeChild::AlternateContent. owned_text at crates/rpptx/src/lib.rs:12127 supports typed Shape and Table bodies, so it returns None and the unused-owner traversal skips this child. The whole-part qualified opaque scan at crates/rpptx/src/lib.rs:12539 is in the exclusive else branch and never scans an already parsed common part. The owner therefore disappears from fit_report and refresh can succeed without accounting for it.

The approved plan's every-frame accounting requires opaque unsupported text to have a named location and unused flagged shared owners to refuse refresh atomically. Active-context opaque handling at crates/rpptx/src/lib.rs:12271 already recognizes the finite opaque case. Apply equivalent qualified unsupported accounting to unused shared children without promoting their opaque body into writable typed ownership. Add genuine report, atomic refusal and source-preservation controls for the unused context.

## Not found

No additional OOXML, panic, structural or test finding beyond D5's missing control. No smells or nitpicks. The corrected affine, font metadata and shared-table behavior has no additional finding.

## Handback

Only this review record was written during review. Return to distinct D5 remediation. Installed-package acceptance, native affine paint parity, remaining scoped native gates, hash attribution, archives and final integration remain due.
