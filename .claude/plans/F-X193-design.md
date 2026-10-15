# F-X193, Preserve semantic mark highlights through nested HTML runs

**Status**: completed
**Sprint**: S91
**Size**: S
**Depends on**: none

## Problem

`ComputedStyle::inherited` at `crates/rdocx/src/html.rs:186` drops backgrounds.
The semantic mark at `html.rs:3305` therefore disappears on a nested strong,
em or span. Simply inheriting background would change normal CSS behavior.

## Spec reference

- `docs/hld/03-architecture.md`, Inbound HTML belongs to the rdocx facade.
- `docs/hld/12-testing-strategy.md`, Test taxonomy, regression and round-trip.
- `docs/hld/14-development-backlog.md`, F-X193.

## Approach

Assess Pedro Assumpcao's PR294 at head
`63cd119dcb8fa9c367bf93141f79780db7f64886`. Reuse its semantic
mark_highlight field in the existing private ComputedStyle, its inherited
copy and its effective run highlight selection. After the mark's CSS cascade
has run, capture its final background as the semantic inherited highlight.
A child's explicitly scoped background takes precedence over inherited mark.
Nested marks choose their own effective highlight. Unmarked siblings and
ordinary CSS backgrounds retain existing behavior. No public type or method
is added. The native pre-1.0 API is unchanged.

The required repository-policy unittest regenerates published archives and
checks exact normalized member bytes. Source additions therefore require a
fresh local rdocx archive measurement in existing
`scripts/readme_doctests.py` and the root README footprint row. Rebuild the
actual archive with the reviewed local graph, update only the observed rdocx
measurement, and rerun the policy suite. Do not widen compression tolerances
or change package inventories to make the measurement pass. Integration must
regenerate any shared measurement after combining workers.

## Rejected alternatives

- Inherit all backgrounds, which makes normal CSS paint unrelated children.
- Reimplement the contributed patch before assessing it, duplicating work.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | mark_highlight_reaches_formatted_children_without_leaking_to_siblings | Contributor's exact bold/italic and sibling reproducer fails at Base then passes |
| regression | semantic_marks_preserve_css_precedence_without_inheriting_ordinary_backgrounds | Custom mark background, nested mark, child own background and ordinary span/div background remain distinct |
| round-trip | semantic_mark_highlights_reopen_with_nested_run_formatting | Public import, save and reopen retain exact text, highlight, bold and italic |

Use existing html.rs tests. Record actual scoped rdocx lint and tests, never
adopt the contributor's test claims as local evidence.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Parser semantics: read `docs/hld/04-opc-and-packaging.md`, The package, and
  `docs/hld/06-presentationml-model.md` reader/writer conventions. No OOXML
  parser/serializer changes. Round-trip imported formatting and retain opaque
  unrelated source through save/reopen controls.
- Archive measurement carrier: read `docs/hld/15-build-and-toolchain.md`,
  Packaging, and `.claude/commands/release.md`. The existing required policy
  suite checks the measured archive, exact member bytes/count and size ceiling.
  Rebuild the actual local rdocx archive and inspect the README and script
  measurement diff. Confirm no manifest, lockfile, version or publication
  behavior changes. This is reversible gate maintenance, not publication.
- No new file, module, dependency, trait, generic or feature flag.

## Hash harness

Expected unchanged. The sample generator does not import semantic mark HTML.

## Exclusive file claims

- `crates/rdocx/src/html.rs`
- HLD files listed above
- `scripts/readme_doctests.py` rdocx archive measurement
- Root README rdocx footprint row
- Scoped rdocx Cargo and the shared regression/test gate execution

## Implementation checklist

- [x] Add contributed reproducer and prove failure at immutable Base.
- [x] Assess and adapt exact PR294 semantic highlight change.
- [x] Add CSS precedence and save/reopen controls.
- [x] Rederive local rdocx archive measurements and pass repository policy.
- [x] Run scoped verification and independent zero-finding microscope.
- [x] Prepare validated handoff, preserving contributor provenance.

## Open questions

None. Existing CSS cascade and source preservation are retained.
