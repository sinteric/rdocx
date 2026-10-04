# F-X170, High-level Word style formatting API

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-246, F-X169

## Problem

Issue 264 asks for a convenient style creation and management API. `Document::add_style`, `set_style`, `style`, `styles`, `set_default_style` and `remove_style` already provide management (`crates/rdocx/src/document.rs:19235`). `StyleBuilder` still requires callers to construct `CT_PPr` and `CT_RPr` for ordinary formatting (`crates/rdocx/src/style.rs:411`). The high-level paragraph and run facade offers these choices directly.

## Spec reference

- `docs/hld/10-bindings-spec.md`, the native Rust style facade and `StyleBuilder` contract.
- `docs/hld/14-development-backlog.md`, "F-X170, High-level Word style formatting API".

## Approach

Add fluent methods to the existing `StyleBuilder` for paragraph alignment, space before and after, left indentation, font name, font size in points, bold and hex colour. Match the established `Paragraph` and `Run` conversions, including font script slots and theme-attribute clearing. Keep `paragraph_properties(CT_PPr)` and `run_properties(CT_RPr)` for advanced formatting. The methods update the same builder fields, so callers may combine convenience and typed settings in an explicit call order. Document `Document`'s existing create, update, lookup, default and removal operations with one complete example. No new type, module, crate or feature flag is needed.

## Rejected alternatives

- Add a second style manager. `Document` already owns staged style graph validation.
- Mirror every OOXML property as a method. The existing typed property escape hatch covers uncommon settings without a larger public surface.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| unit | `style_builder_convenience_matches_typed_properties` | Common settings produce the same `CT_PPr` and `CT_RPr` as the explicit builder form |
| round-trip | `high_level_style_survives_save_and_reopen` | A custom style is added, applied, saved, reopened and resolves to the intended formatting |
| integration | README example and style CRUD checks | Public-only example compiles and the existing management operations remain valid |
| policy | `cargo publish --dry-run` and package inventory | The public API and package remain publishable with the measured archive size |

**Test gate**: round-trip. The convenient settings survive save and reopen, apply to document content and match their typed property equivalents.

## HLD impact

- `docs/hld/10-bindings-spec.md`, the native style facade convenience methods and advanced typed fallback.

## Risk routing

- Unit conversion: read `docs/hld/01-glossary.md` units and the deliberate conversion notes in `CLAUDE.md`. Match existing `Length` to twips and `HalfPoint::from_pt` behavior and run the hash harness.
- Public API of a published crate: read `docs/hld/10-bindings-spec.md` and `CLAUDE.md` structural rules. State the additive 0.14.0 semver effect, run `cargo publish --dry-run`, and check the measured `.crate` archive.

## Hash harness

Existing sample output should remain unchanged. A newly authored style example is an additive test, not a baseline update. Any existing entry change requires a separate reviewed behavior commit and expected delta.

## Implementation checklist

- [x] Add focused fluent methods without changing the style graph owner.
- [x] Add public-only example and round-trip parity tests.
- [x] Run scoped verification, the public API riders and microscope.

## Open questions

None. The request permits a focused first surface for common settings, and the typed fallback remains available for all other properties.
