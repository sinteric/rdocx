# F-289, native, pass 3

**Reviewed**: Root Codex independently reviewed the frozen native working diff on `work/f-289-grok`, HEAD `c0b58c7fa8e4e919b9bfe5c02932f74827cea36b`. Diff SHA-256 `6e32f5774b43b98cc4717e8caa47a5459c38ac7bd607ad67171edb7903767d6f`. Six tracked files, 4751 insertions and 41 deletions. This is a bounded native checkpoint, not the complete F-289 implementation.
**Verdict**: 2 defects, 0 smells, 0 nitpicks

## Defects

### D1, retained field bytes do not establish physical ownership
`crates/rdocx/src/content_control.rs:4535`
`crates/rdocx/src/content_control.rs:4562`
`crates/rdocx/src/content_control.rs:4576`
`crates/rdocx/src/content_control.rs:4532`

The repair now obtains the accepted field inventory from F-288, but it locates its source by choosing the first equal byte substring anywhere in the part with the same text-box classification. It does not establish that the occurrence belongs to the accepted paragraph, run, or control. Give the opaque decoy in the existing legacy-ordinal test the exact same field bytes as the later live field. The opaque occurrence wins, so the live field's anchor is outside its actual enclosing control. The overlap check can then accept that control and its own legacy descendant together. The current fixture gives the decoy a different name and value, so it cannot reject this implementation.

A newly authored accepted field has no parsed source fragment, and line 4532 silently omits its anchor. Flushing the cloned model does not turn that field into a parsed field. The accepted inventory still contains it, so absence of a retained fragment is not proof that there is no overlapping target.

The required invariant remains one accepted physical owner used by inventory, overlap checks, and mutation. Raw equality and an opaque-versus-text-box bit do not supply that owner. Two repair attempts have now left D1 open on different inputs. Escalation to Root Codex as semantic designer and independent reviewer is required before another repair. Preserve the dirty diff and sole-writer ownership.

### D3, the new font publisher still creates unqualified Word attributes
`crates/rdocx/src/content_control.rs:5903`
`crates/rdocx/src/content_control.rs:5907`
`crates/rdocx/src/content_control.rs:5951`
`crates/rdocx-oxml/src/content_control.rs:1716`

For an imported inline control whose unchanged run is `<r xmlns="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><t>...</t></r>`, the retained inline source keeps that spelling. Assign a checkbox state with a declared state font when that run has no rPr. The new publisher inserts `<rPr><rFonts ascii="..." hAnsi="..." eastAsia="..." cs="..."/></rPr>` and returns before the qualified-attribute writer runs. Default namespaces qualify elements, not attributes. Word therefore does not receive the declared state font. Creating a fresh display through display_run has the same empty-prefix font constructor.

The previous default-namespace checked-child case, quoted greater-than attribute case, and exhausted-prefix case are fixed. This new caller of the same namespace invariant still bypasses qualified publication. Cover both an existing inline run without rPr and an inserted display, preserving opaque attributes. A run with existing qualified rFonts does not exercise the failing branch.

## Smells

None.

## Nitpicks

None.

## Closed findings and evidence

D2, D4, D6, D7, and D8 remain closed on the examined paths. D9 is closed by per-part grouping, sorting physical spans, deduplication, and enumeration inside each group. D5's selected glyph and font provider now uses CT_SdtPr::checkbox_state, reports invalid current glyphs as issues, and preserves the selected font through the normal qualified-run path. The required placeholder checkbox discriminator is covered. The remaining default-namespace font publication is D3.

Consumed actual local evidence: cargo check, fmt and clippy passed. The final native integration batch passed 81 tests, consisting of five original form groups, eleven review2 cases, and 65 affected existing tests. The render-fallback case used CLI SHA-256 `3c8e2c858215109fefbbedf4ebf7c97d5bbad83d0bc8afee544087faeb1efda3`. This review inspected the assertions and provider code. It did not rerun tests.

The two early invalid fixtures and the later reset-value fixture correction are not semantic RED. The unavailable remediation-1 raw log remains unavailable. No fabricated independent failures are credited.

## Not found

No additional panic, schema-order, structural abstraction, or unrelated behavior finding was established in the examined native repair paths. Zero smells and zero nitpicks are intentional.

## Pending scope

Python classes, bindings, exports, stubs, typed public use, related-owner publication, three required Python groups, and the five HLD updates are not implemented at this checkpoint. F-289 A7 Word qualification, hash harness, full verification, WASM, installed typing and publish riders remain pending. This report does not approve whole-feature completion.
