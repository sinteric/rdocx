# Hanji, zero-radius preset corners

**Status**: independent fork custom fix, local implementation and scoped validation complete
**Base**: 082edbf6cd4f415bd997fac6579aedeea120fd82
**Authorization**: continue independent bounded app-relevant preview work during PR review, with no merge or release

## Problem and spec reference

Real Hanji PPTX 12 renders seven slides but emits seven round2SameRect
geometry failures and retains those shapes only as bounds. Two authored source
shapes, in slide 2 and its shared layout, explicitly carry adj1=16667 and
adj2=0. The official generated preset has the same defaults. Its two bottom
corners evaluate to arcs with both radii zero, and flatten_arc rejects the
entire path, losing its supported rounded top corners.

References: `docs/hld/08-rendering-spec.md`, Preset geometry. The checked-in
`tools/gen-presets/presetShapeDefinitions.xml` is the pinned official source.
[Microsoft DrawingML arcTo](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.arcto?view=openxml-3.0.1)
defines the radii and angle-based current/end point relation. With both radii
zero the geometric arc is a point and does not move the pen. SVG endpoint
arc behavior is not assumed to define DrawingML's center-based commands.

A measured default-size scan of all 186 unique official definitions identifies
round2SameRect and round2DiagRect with this radius failure. Three circular
arrow definitions separately fail formula arity. Those are not part of this
slice. No generated definition will be edited.

## Approach

Allow exactly the both-zero case in the existing private arc flattener after
finite-value validation. Emit no cubic and retain the current point. Keep the
strict radius gate for every other unsupported nonpositive radius combination,
with its existing diagnostic. Positive-radius arcs and their segment budget
retain their code path. No public surface, dependency or source file is added.

The branch is based on verified merged main and has no dependency on the
pending VML renderer PR. Source graph differences are explicit. Any Hanji
consumer pin must later include both reviewed fixes or follow their merges,
so this branch must not silently replace the pending watermark pin.

## Test plan and implementation checklist

- [x] Add an owned preset regression that fails at old code and requires both
  affected defaults to keep two rounded corners, finite coordinates and closure.
- [x] Prove a both-zero arc leaves the current point unchanged by comparing a
  following positive arc with an independently constructed no-op control.
- [x] Keep negative, single-zero and non-finite rejection and positive arc
  segment limits. Exercise each focused boundary.
- [x] Run all drawing-crate tests, strict Clippy, formatting, the 49 hashes,
  repository policy, scoped archive/README verification and bounded review.
- [x] Repeat the 186-name default probe, requiring exactly the two known radius
  failures to disappear and every previously successful path to stay identical.
- [x] Run the owned zero_radius_preset_corners_resolve_for_slide_and_layout_shapes
  regression plus a focused shared Presentation layout reproduction and record the
  restored geometry and remaining diagnostics without native certification.

## HLD impact

- `docs/hld/08-rendering-spec.md`, Preset geometry, point-degenerate arcs.
- `docs/hld/12-testing-strategy.md`, the owned geometry regression and guards.

## Risk routing

Geometry affects shared Word and Presentation layout, so read rendering and
units contracts and use deterministic bundled fonts for the unchanged 49-entry
hash harness and focused layout rider. There is no unit-conversion, parser,
serializer, font-distribution or public API surface change. No WASM binding
carrier changes. Verify both changed drawing and Presentation layout packages against reviewed
local patches, refresh their archive evidence and compile their README examples. Exact-head
CI is required before ordinary PR readiness. Do not rewrite completed sprint
ledgers or import unreviewed generated data.

## Hash harness

Expected unchanged. The seven existing samples use no affected degenerate
preset corner. The declared output change is restored supported rounded
geometry for both-zero corner arcs rather than whole-shape bounds fallback.

## Open questions

None for the measured both-zero correction. Single-zero-axis arc semantics
remain unsupported in this bounded slice and require separate evidence.

## Completion evidence and publication boundary

The drawing crate passes 147 unit tests with two existing ignored oracle tests.
The Presentation layout crate passes all 142 tests with all 50 pinned decks
verified by SHA-256. Strict scoped Clippy, formatting, prose, adapters, 139
repository-policy cases with two expected skips, both package verification
builds and README examples pass. All 49 hash entries remain unchanged.

At (360, 180), exactly round2SameRect and round2DiagRect recover. Every one of
the 181 previously successful default results is byte-identical in its complete
debug representation. The three circular-arrow arity failures remain. Real
PPTX 12 restores exactly seven curved shapes, with all other resolved state
retained and both unrelated chart diagnostics unchanged. All 490 rendered text
runs and their accumulated transforms remain exactly equal. Seven slides remain
seven, with 96-DPI changed-pixel counts [0, 240, 6, 6, 6, 6, 6].

The source input hash is unchanged. This is a deterministic shared-renderer
reproduction from the reviewed local worktree, not an immutable Hanji consumer
revision or native PowerPoint oracle. Both archives remain below 10 MiB. The
drawing archive records 182,758 compressed bytes and 1,230,174 normalized member
bytes with 24 members. A verified repack is 182,751 compressed bytes, within
the existing 64-byte tolerance. Presentation layout records 83,782 compressed
bytes, 486,090 normalized member bytes and 11 members. Packaged source equals
the final positive source byte for byte.

The publication boundary is the user's at-most-one useful custom-fix PR per
fork convention. Renderer PR 6 and Hanji PR 82 are already ordinary ready.
Keep this independent commit local while PR 6 remains open. After a clean
publication path exists, consume the watermark and arc fixes together through
a reviewed main descendant or explicitly coordinated dependency. Never replace
the pending watermark pin with this independent main82-based branch. No merge,
release or deployment is included. Exact-head CI remains required for any
future published arc candidate before ordinary readiness.
