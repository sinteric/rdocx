# F-X206, run link namespace, pass35

**Reviewed**: External ten-carrier namespace R3 candidate,165101-byte diff. This pass examines namespace ownership and unknown attribute preservation in proposed hyperlink mutation. ROOT independently confirmed two findings supplied by a separate read-only reviewer who did not author this candidate.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, opaque unqualified id is rewritten as the relationship id

`/private/tmp/S91-F-X206-run-target-slide-namespace-r3-prepared/candidates/crates/oxml-drawing/src/text/paragraph.rs:869`

An owned q:hlinkClick may retain an unqualified id attribute as opaque source, alongside an aliased relationship id or no relationship id. Parsing at :753 correctly promotes only URI-qualified relationship attributes. During retargeting, the writer maps any lexical key named id to the modeled relationship field even when source_id_key names another attribute or is absent. It consequently overwrites or removes the opaque attribute. Keep exact opaque token spelling and update only the actually qualified relationship token. Test absent and aliased relationship cases through retarget, clear and save/reopen.

### D2, first click inherits a foreign relationship prefix

`/private/tmp/S91-F-X206-run-target-slide-namespace-r3-prepared/candidates/crates/oxml-drawing/src/text/paragraph.rs:815`

A valid imported owner can bind q to DrawingML and r to a foreign URI, with no existing owned click. Native set_run_target_slide creates a default TextHyperlink, whose writer emits canonical r:id. The character-property writer at :1303 closes generated DrawingML a descendants, but text_close_generated_child at :42-54 does not close the relationship prefix. The new r:id therefore remains foreign after save/reopen and the target disappears even though its slide relationship was published. Choose an available qualified relationship alias or declare the relationship URI locally on the newly generated click. Preserve inherited foreign tokens. Add first-creation controls with rebound r for ordinary and grouped imported owners, including save/reopen and graph ownership.

## Smells

None in this bounded pass.

## Nitpicks

None.

## Not found

Both findings affect proposed unadopted source. No current worker mutation or acceptance regression is claimed. Existing raw relationship-ID parsing and retargeted imported-click tests do not cover these two triggers. No runtime or compilation was performed in this review.

## Handback

Only this canonical review report was written. The candidate remains unadopted. Return to orchestration for a separate correction and fresh review. The complete ten-carrier and full F206 acceptance gates remain outstanding.
