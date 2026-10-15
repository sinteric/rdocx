# F-X206, caller-selected text-body owner, pass 71

**Reviewed**: Current namespace projection in the R21 source661a freeze and the actual R22 native continuation. The complete worker diff contains96 carriers,35032 insertions and2549 deletions. This pass addresses the concrete caller-wrapper boundary, not a new whole-feature acceptance claim.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, stored DrawingML owner overrides requested PresentationML wrapper

`crates/oxml-drawing/src/text/paragraph.rs:40`

The ownership predicate accepts A_NS unconditionally, regardless of the namespace requested by the caller. An imported DrawingML a:txBody is therefore considered an eligible source spelling when the caller requests p:txBody. CT_TextBody::write_xml_as at crates/oxml-drawing/src/text/mod.rs:286 replaces the requested tag with that spelling. The shape writer at crates/rpptx-oxml/src/shape_tree.rs:1396 explicitly requires PresentationML p:txBody, but receives DrawingML a:txBody. The saved shape has a wrong expanded owner. Presentation parsing does not recognize that child as the shape text body, losing rendered and extracted text after reread.

The actual R22 Word library fails document::tests::large_word_and_presentation_pdfs_preserve_logical_reading_order at crates/rdocx/src/document.rs:37149 and word_and_powerpoint_chart_pixels_are_identical at crates/rdocx/src/document.rs:36587. Its actual log SHA is7f5e3d82f4bc80ce0474729f58ad7f908b8cb78c0c89cdeeb2709eb7a2eb6807 and failed receipt SHA is a492797710929cb37fa9efd7dea49d1b3e8895bf11dd41516068772789158d2c. These remain failed. The second test also includes an intentional placeholder-name difference, so correcting wrapper ownership alone does not justify accepting its existing expected package hash.

Preserve an imported alias only when its expanded namespace matches the requested wrapper owner. Converting a DrawingML shell into a PresentationML wrapper must retain child and opaque namespace closure without retaining the wrong wrapper owner. Add real cross-owner controls in both directions and saved Presentation reread controls. Existing finite-wrapper, alias, default-namespace and foreign-root refusal controls must remain intact.

## Not found

No additional contract, panic or structural finding is established by this bounded pass. No smells or nitpicks. Earlier incremental review70 remains evidence for its own reviewed changes, not proof of the now-exercised cross-owner boundary.

## Handback

Only this review record was written in the review phase. Return to a distinct remediation phase for D1. Full feature completion remains unproven.
