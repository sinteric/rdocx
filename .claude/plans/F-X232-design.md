# F-X232, Render legal revisions and qualify checked XML repros

**Status**: completed
**Sprint**: S93
**Size**: L
**Depends on**: F-X201, F-X211a, F-X211d

## Problem

Tracked rendering forces single underline or strike with the resolved run colour
(`crates/rdocx-layout/src/engine.rs:2265`). It does not provide the legal colour
scheme requested by Issue373 and PR375. Render option dispatch currently selects
the accepted cache using revision_view alone
(`crates/rdocx/src/document.rs:17573`). Exact checked-owner namespace repros also
need qualification against current providers rather than another XML engine.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X232, Render legal revisions and qualify checked XML repros".
- `docs/hld/08-rendering-spec.md`, "Word revision views" and retained layout cache identities.
- `docs/hld/10-bindings-spec.md`, native render options, checked Word XML ownership and Python rendering methods.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", revision-view regressions and "Word checked XML qualification".
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering" and "Packaging".
- `docs/hld/04-opc-and-packaging.md`, checked physical source ownership.
- `docs/hld/06-presentationml-model.md`, "Parts", raw XML preservation.

## Approach

Assess and reuse the qualifying slices of PR375 by hadim, authenticated head
0c608f7a2a1d5a9f8391a4de1dfed661df6e105a. Capture its exact diff and provenance
before adoption. Contributor claims are not local gate evidence. Keep changes
inside existing carriers and exclude unrelated policy/archive observations.

Add RevisionStyle::{Plain, Legal} to existing rendering option flow. User
explicitly approved the pre-1.0 Rust source compatibility change on2026-10-15:
RenderOptions and LayoutInput gain the style field and ParagraphBlock replaces
has_visible_revision with change_bar: Option<Color>. Public NoteLayout also
changes revision_ranges from Vec<Range<usize>> to Vec<(Range<usize>,Color)> so
continuing note fragments retain their owning bar colour. This is the direct
propagation of the approved rendering-structure source change. Update current callers,
struct literals and release compatibility prose. Plain remains the default.

Legal insertion paint is0000FF with double underline, deletion isFF0000 with
strike, move-from is00A000 with strike and move-to is00A000 with double underline.
Override resolved paint only in the transient layout. Stored XML, run formatting
and revision metadata stay unchanged. Bars use the first visible insertion, deletion or move in
projection order, then paragraph mark, else black for property-only changes.
Preserve nested wrapper precedence, all supported story kinds and page fragments.

Unify actual double-underline geometry in existing convert.rs instead of retaining
three independent implementations. Qualify stroke separation and line scaling
independently. Native Word16.113.2 build16.113.26092012 measured double
stroke thickness/centre separation at12points as0.48/1.44points, at24points
as0.96/1.92points and at36points as1.20/3.12points. Keep thickness size/24 and
use centre separation max(size/12,1.5points), with an independently declared
0.36point geometry bound. Maximum width error is0.30points and maximum centre
error is0.12points across these three cases. This measured clarification
replaces the donor's unqualified wider gap and changes no existing raster
threshold. Native static source SHA-25652c35e24f7670ed6b9ae70ccb663216f89aa66ba8d5dc8d128ff82b65dd4669f
and exported PDF SHA-2568af352f574fdc9c0d0c647de376af2f05812356c8248cd09d6013ebb2177c2f9
bind the new evidence. Existing strike metrics retain a documented divergence.
Add style to every relevant retained context, paragraph/table
identity and caller-font/measurement route. Test plain/legal alternation and
mutation invalidation against fresh layout. Retain accepted normal/deterministic
cache behavior for valid accepted/plain requests.

CLI convert/render gains --revision-style plain|legal. Existing Python rendering
methods gain keyword-only revision_style="plain". Native, CLI and Python reject
unknown styles and legal with accepted view before producing output. Reuse
existing errors and physical ownership providers. No new file, module, trait,
generic, feature or dependency is proposed.

Qualify Issue373's exact two-picture svgBlip repro with local versus ancestor
namespace declaration, then header/footer paragraph and run owners, tables and
first/even variants, save/reopen and atomic refusal. Repair only a proved uncovered
case in the existing provider. Self-rendered coloured fixtures do not satisfy
independent native paint acceptance. Pin actual new Word/Poppler identities and
retain historical native evidence separately.

## Rejected alternatives

- Mutating stored colours to implement a render preset.
- A second options hierarchy to preserve Rust literals after the user approved
  the explicit source change.
- A parallel checked-XML engine or adopting donor test claims as qualification.

## Test plan

The **test gate** is **regression**. Use existing source test modules, integration
entrypoints, CLI integration tests and Python test_core.py.

| Category | Test | Asserts |
|---|---|---|
| regression | `fx232_legal_revision_paint_preserves_stored_formatting` | All four legal kinds, mixed/property-only bars and nested controls have exact paint and unchanged source |
| regression | `fx232_render_style_participates_in_every_cache_identity` | Cold/warm plain/legal alternation, mutated source, tables, related stories, caller fonts and measurement equal fresh layout |
| regression | `fx232_double_underline_has_two_separate_strokes` | Actual line primitives and raster retain independently visible scaled strokes |
| regression | exact Issue373 checked-owner controls | Body/header/footer paragraph/run and ancestor namespace variants preserve source and qualified references through reopen |
| integration | native/CLI/installed Python rendering parity | Same selected projection and paint, invalid values refuse before file output, stored XML unchanged |
| differential | pinned native legal paint | Independent native colours and decoration geometry corroborate product paint, with exact source and producer identities |

Run changed-crate checks/tests/clippy, focused namespace/preservation matrices,
fresh installed Word Python checks, both WASM graphs, publication riders,
scoped verify and an independent zero-defect/zero-smell microscope.

## HLD impact

- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/15-build-and-toolchain.md`

## Risk routing

- Layout/shaping: deterministic bundled fonts for all baselines and exact
  declaration of product versus independent native raster changes.
- Published APIs: explicitly approved source break, verified publication dry-runs,
  denied-warning rustdoc and actual archive assertions below10MiB.
- PyO3/WASM: both target graphs, fresh installed binding parity, native atomicity
  and workspace full tests excluding both extension crates at final integration.
- Parser/serializer if a proved provider repair is required: qualified namespace,
  schema order and byte-identical opaque payload round trips.
- External oracle: record actual native build, rasterizer, DPI and comparisons.

## Hash harness

Default plain decoration can change only where corrected double underline is
actually present. Enumerate affected entries and independently justify each
stroke delta before recording. Legal fixtures are new explicit mode inputs.
Unrelated outputs remain identical. Never rerecord to conceal a mismatch.

The deterministic check attributes exactly two changed entries to the static
double underline sample on page 3 of `feature_showcase`, generated by
`crates/rdocx/examples/generate_all_samples.rs:267`. The PDF bytes change from
`b4fe360e1dc26d0f3d848b3d9be0bcba8af613cb417e3d310b8ee17b150f993b` to
`53e38b257703bfb26eeb8c2ff2534827057d05cc26cec48767e6173bd918e60b`.
The PDF pages hash changes from
`f0578124411527a6dcccbcd0701908226ce570dfc94c62bb55961f4df60d9fa2` to
`bbc3b255794d0a97f7630203e3d059cc5e534ac19ae74f1ee5b1d34028070213`.
Every XML, resource, page 1 PNG and other sample entry remains unchanged.
The approved scale-aware double geometry and independently measured native
12, 24 and 36 point controls justify these two changes. Recording awaits a
clean independent microscope and a separate labelled behavioral commit.

## Implementation checklist

- [x] Approve all designs, capture donor head/diff and claim isolated worktree.
- [x] Establish Base failures and implement legal paint and complete identities.
- [x] Qualify double underline, exact owners, public parity and native paint.
- [x] Review intentional output deltas and compatibility evidence.
- [x] Pass scoped verification, independent microscope and prepared handoff.

## Open questions

Resolved: user approved the explicit pre-1.0 Rust source compatibility changes.
No unresolved material product question remains.
