# Hanji, VML shorthand watermark colours

**Status**: renderer implementation and local validation complete, publication gates pending
**Base**: 082edbf6cd4f415bd997fac6579aedeea120fd82
**Authorization**: continue app-relevant renderer preview correctness fixes, scoped verification and ordinary ready PRs. No merge or release.

## Problem and spec reference

`crates/rdocx-layout/src/engine.rs`, `vml_color`, accepts named colours and
six-digit RGB. Its projected VML input can contain three-digit RGB. Real Hanji
DOCX 05 uses `fillcolor="#e00"` and loses every SAMPLE watermark, with two
unsupported-colour diagnostics. Changing only the three header colour strings
to equivalent `#ee0000` restores SAMPLE on pages one through four. Page five
stays unwatermarked and the document remains five pages.

References: `docs/hld/08-rendering-spec.md`, "Word watermarks".
`docs/hld/12-testing-strategy.md`, the watermark golden gate.
[W3C VML](https://www.w3.org/TR/NOTE-VML), the shape fillcolor and colour types.
[CSS1 colour units](https://www.w3.org/TR/REC-CSS1/#color-units), three-digit RGB
expansion duplicates each digit. This is a finite colour-projection extension,
not general VML support or native Word certification.

## Approach and implementation checklist

- [x] Accept exactly three or six ASCII hexadecimal digits in the existing
  private VML colour projector, expanding RGB digits to RRGGBB before lowering.
- [x] Keep named colours, six-digit values, whitespace and case handling.
  Reject unsupported lengths and non-hex strings with the existing diagnostic.
- [x] Add an owned renderer regression that fails before the correction,
  checks colour, transform, opacity, page content and cold/warm equality.
- [x] Verify malformed and unavailable colour forms remain rejected.
- [ ] Add a source-owned Hanji public-API test after an immutable renderer
  revision is published, with exact SVG, PNG and HTML shorthand/control parity.
- [ ] Re-render real DOCX 05 and require source-derived watermark counts
  [1, 1, 1, 1, 0], unchanged body output, page count and input bytes.
- [ ] Run changed-crate verification, hashes, policy and a bounded review,
  publish the renderer PR and explicitly dependent immutable-pin integration.

The parser, retained XML, VML authoring model, public carriers, fonts, theme
colour transforms, page geometry and raster backends retain their contracts.
No new dependency, module or source file is required. No finished upstream
sprint ledger is rewritten for this Hanji fork custom fix.

## Test plan

The regression gate is `short_hex_vml_watermarks_match_expanded_rgb_layout`.
It compares complete owned page output against expanded RGB, requires the
watermark and actual colour, and checks identical warm output. A focused
negative lexical gate rejects invalid colours. Existing watermark selection,
blank variants, scoped media identity, opacity, rotation and geometry tests
remain intact. Hanji validates the same group through all three output formats
on native, byte-only and WASI paths, with source-owned OPC parts.

## HLD impact

- `docs/hld/08-rendering-spec.md`, Word watermarks, three-digit RGB support.
- `docs/hld/12-testing-strategy.md`, the shorthand equivalence regression.

## Risk routing

The diff changes rendered watermark colour projection and makes previously
omitted groups visible. Read rendering and colour conventions, use bundled
fonts for deterministic checks, and run the 49-entry hash gate. Do not modify
the deliberately retained Word tint/shade implementation. There are no parser,
serialization, font-distribution, public API or binding changes. Refresh the
changed layout package measurement because source size changes, run the
existing policy/README checks and preserve the archive budget. Exact-head CI
is required before ordinary readiness.

## Hash harness

Expected unchanged. Existing hash samples author no watermarks. The declared
behavioural delta is restoring only supported shorthand-colour watermarks,
matching the equivalent expanded RGB, with no pagination or body changes.

## Open questions

None. The user has authorized the bounded core fix. No architecture decision
requires user input because the existing private colour decoder, watermark
model and group rendering path are sufficient.

## Renderer completion evidence

The changed layout crate passes 319 unit tests and one documentation test.
Strict Clippy, formatting, prose, generated adapters and all 49 unchanged
output hashes pass. The package verifies against the reviewed local dependency
graph with offline and allow-dirty flags, and its README example compiles.
The refreshed archive has 282,348 compressed bytes, 1,511,843 normalized
member bytes and 15 members. The bounded correctness review records zero
defects and smells. The full repository-policy gate passes all 139 cases with two expected skips.

The Hanji and real-document checklist items above are external consumption
gates. They stay pending here until the renderer revision exists. Exact-head
CI is required before the PR becomes ordinary ready. No merge or release is
part of completion.
