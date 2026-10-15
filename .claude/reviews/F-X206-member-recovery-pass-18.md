# F-X206, member recovery, pass 18

**Reviewed**: Unapplied external member-recovery packet, nine existing carriers. The complete diff has753 lines,479 additions and42 deletions. Reviewed correctness, contract, panics, OOXML, tests and structure against the approved F-X206 recovery policy and current foundation/provider source.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, boundsless gradient failures evade the unresolved count

`crates/rpptx-layout/src/context.rs:640` in the complete candidate.

The new unplaced-member validator checks only solid fill colors. A boundsless ordinary shape with a supported linear gradient containing an unresolved phClr stop or an unknown preset color returns success here. It is silently omitted with count0 and no failed-owner diagnostic. The same placed shape fails through concrete_fill, whose gradient-stop loop resolves each color at context.rs:1808. Stop-color resolution does not require replacement dimensions. The same gap applies to gradient line fills. This defeats the approved independent omitted-member count and allows the strict corpus count check to accept these unresolved owners. Preserve the existing unsupported-gradient boundaries, but validate dimension-independent colors for supported gradients and add placed/unplaced negative controls.

### D2, the native timeline diagnostic companion loses recovery facts

`crates/rpptx-render/src/timeline.rs:67` in the complete candidate.

The public layout_timeline_slide_with_diagnostics starts with an empty vector and returns only shaping diagnostics. It neither consumes unresolved_member_count nor propagates timeline.slide.diagnostics. Thus a recovered timeline member can produce a successful PageFrame and an empty diagnostic companion, including when a boundsless failed owner has no fallback geometry. The new incomplete summary is added only by layout_presentation_impl at rpptx-render/src/lib.rs:334, which this route bypasses. The presentation facade separately copies owner messages at rpptx/src/lib.rs:4927, but still does not consume the count, and that does not repair the public renderer companion. Propagate owner messages and slide-qualified count facts consistently without duplicating them in facade aggregation. Add a direct companion control and a facade timeline control for fallback and omission.

## Smells

0.

## Nitpicks

0.

## Checked without additional findings

- Member matching stays outside fatal background/context/package/XML/backend admission. Authored mutation refusal behavior is unchanged.
- Fallback geometry checks finite transformed corners and preserves ancestor composition, rotation and flips. Valid sibling order and source ownership are retained.
- The structured count increments before fallback eligibility. Both existing strict corpus assertions consume it independently of drawable fallback geometry.
- All current ResolvedSlide literals found in the existing crates receive initialization. The public struct field remains an explicit pre-1.0 source break requiring the planned archive/API riders.
- Prepared native counts are four resolver tests, two renderer tests, one public integration test, one CLI test, two existing strict corpus tests and one adapted connector test. The separately prepared Base discriminator uses existing APIs. Installed selection has six parameter cases. None is execution evidence.
- No new module, dependency, generic framework or test binary is introduced. No additional panic or raw-source mutation defect was found in this incremental packet.

## Identity and limits

Independently authenticated all eight receipt-listed artifacts and all nine before/complete/production carrier hashes. The current authoritative source map contains2714 entries, all byte, size and mode matches. Receipt SHA256 is d47e4017bdf46bfe087eb1a8546ba7e1460341ac049fc05edb00d8756be2cc6c. Complete diff SHA256 is472addd09e3ed9608019841a670ae13d39cb36128df99e66c70caa34adc17a77. Source-before map SHA256 is e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4.

No source was adopted or edited. No compilation, installed test, rendering, native UI, corpus execution or baseline change occurred. Full F-X206 acceptance remains pending. Review ends with this report.
