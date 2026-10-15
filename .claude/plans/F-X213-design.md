# F-X213, Preserve TOC entry boundary paragraphs during rebuild

**Status**: completed
**Sprint**: S91
**Size**: M
**Depends on**: F-279

## Problem

Issue340 demonstrates that the generic source splice in existing
`crates/rdocx/src/field.rs:2070` retains two empty marker paragraphs when
Word stores the field begin and end inside entry paragraphs. The scanner
at `field.rs:7100` also refuses a one-paragraph TOC. PR341 proposes a bounded
repair, but its property replacement and moved marker namespaces can lose
opaque source. Its started-style scanner records only an opening tag.

## Spec reference

- `docs/hld/04-opc-and-packaging.md`, The package, staged generated-table cache edits and expanded namespace ownership.
- `docs/hld/08-rendering-spec.md`, Word bookmark field pagination, immutable deterministic TOC target placement.
- `docs/hld/12-testing-strategy.md`, dynamic TOC differential and source-preservation regression matrix.
- `docs/hld/14-development-backlog.md`, F-X213 bounded entry-boundary rebuild contract.

## Approach

Reuse exact source/test hunks from PR341 head
`15e799e891cfab6a65d988f621e5081bfc27c1a6`, excluding contributor archive
measurements. Intake assessment is `/private/tmp/S91-PR341-intake/assessment.md`,
SHA256 d95a5b3d50eb04f85c91233d617c8b8d4c713ba0a49097e2c9d1a895086f9f96.
The three-file reuse patch SHA256 is
0796fcaa574eba5ce863fdf2f1c0ee75dad05fdd450760a01244e2f78769789d.
Keep code and tests in existing field.rs, regression_test.rs and test_core.py.
No public method, dependency, module, trait, generic or product file is added.

Recognize entry-owned boundary paragraphs from qualified Word properties
and existing resolved TOC styles. Reuse source offsets and staged source edits.
For unchanged counts, retain the original number of entry paragraphs. For
changed counts, move begin/separator/end to generated first/last entries.
A single entry retains both boundaries. Zero entries retain a balanced owner
without an obsolete TOC entry style and allow a later positive rebuild.
Existing distinct marker-only paragraph behavior remains unchanged.

Preserve paragraph root attributes and opaque pPr children exactly. Overlay
only owned generated entry properties in schema order. When a boundary cannot
be safely overlaid or moved, retain the existing safe generic layout or return
a named atomic refusal for the previously unsupported single-paragraph case.
Do not use fallback to excuse ordinary valid entry-boundary cases. Support
namespace aliases/default namespaces by expanded URI identity. Foreign or
rebound lookalikes remain opaque. Retained end runs and sibling suffixes carry
their actual namespace closure when their former paragraph root disappears.
Track a started/ended pStyle as one complete element, including zero-entry
style removal. Duplicate or malformed owned properties refuse before commit.

Retain nested field balance, hyperlink/bookmark relationship ownership,
original instructions, unrelated producer parts and current update_fields
policies. Preserve F-X200 numbering and F-X206 causal publication when composing
later owners. Do not adopt parked F-X192 source or expand F-283 scope.

## Read-only namespace inventory compatibility

For an unchanged main CT_Document with no modeled relationship mask candidates,
inventory qualified relationships from the exact retained main XML. A repeated
save and ordinary reopen preserve the complete main-part bytes and package
member payloads. This supersedes only F-X186's historical initial read-only save
failure qualification for retained table, cell and ambiguous paragraph namespace
owners. It does not broaden comment placement or mutable owner admission.
Dirty/add/move paths still require the existing unique namespace replay proof.
Named ownership failures preserve all source members, comment metadata and
identifiers atomically. Rich anchor queries still require unique canonical
namespace replay and retain their named refusal for ambiguous producer owners.
The qualified raw-source literal guard proves unchanged SOURCE anchoring
independently without promising that richer query will accept the package. Ambiguous mutation outcomes require actual focused
qualification, never an inferred result from successful read-only serialization.

## Rejected alternatives

- Reopen full F-283 navigation in S91. The user explicitly parked that contract.
- Replace complete contributor carriers or archive tuples. They would overwrite accepted source and misstate local evidence.
- Discard opaque pPr or guess namespace prefixes. Both violate source preservation.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | `toc_rebuild_keeps_the_field_markers_in_the_first_and_last_entries` | Genuine Base adds two empty paragraphs, corrected three-to-three rebuild retains count and no empty entry |
| regression | `toc_rebuild_accepts_a_single_paragraph_toc` | Genuine Base semantic refusal, one-to-one/many/zero and later positive rebuild preserve balanced markers |
| regression | Contributor remaining seven boundary controls | Count changes, old-level styles, content control/PAGEREF links, generic fallback and marker-only paragraphs retain approved semantics |
| regression | `toc_entry_boundaries_preserve_opaque_properties_and_namespace_closure` | Foreign attributes/children/entities, end-root aliases and suffixes survive exact source-aware overlay and repeat/reopen |
| regression | `toc_entry_boundaries_remove_complete_started_styles` | Started/ended pStyle zero-entry removal leaves valid XML and repeat stability |
| regression | `toc_entry_boundaries_qualify_aliases_and_adjacent_fields` | Alias/default/rebinding, two adjacent TOCs, nested instructions and opaque wrapper boundaries retain ownership |
| regression | `test_rebuild_toc_keeps_the_field_markers_in_the_first_and_last_entries` | Fresh installed Python public API retains paragraph count and marker/source facts |
| integration | Existing deterministic TOC layout and field update controls | No added blank entry line, exact field identity and original update policy, source unchanged after refusal |

Run existing native test entrypoints with Base controls before production,
then focused corrected tests, changed-crate checks/tests/clippy, fresh installed
Python tests and applicable typing/stub/WASM gates. Use deterministic font mode
for layout. No invented pixel threshold or unpinned viewer is an oracle.
Run scoped verify and independent microscope. Final full gate remains integrated.

## HLD impact

- docs/hld/04-opc-and-packaging.md, preservation of boundary paragraph properties and namespace closures.
- docs/hld/08-rendering-spec.md, bounded entry-owned TOC rebuild layout.
- docs/hld/12-testing-strategy.md, boundary topology and source-preservation controls.
- docs/hld/14-development-backlog.md, delivered F-X213 acceptance.

## Risk routing

- Parser/serializer: read HLD04 The package and HLD06 Preservation strategy, require expanded namespace ownership, schema order and byte-exact opaque round trips.
- Layout/pagination: read HLD08 Word bookmark field pagination, deterministic layout and hash comparison with every intentional delta separately attributed.
- Published native behavior: read HLD10 Native Word facade stability and structural rules, no signature break, verified affected archive dry-run/source membership and size assertion.
- Installed Python consumer: read HLD10 Python API shape, fresh installed tests and typing/stub checks. Check both WASM graphs. Workspace tests exclude rdocx-py and rpptx-py.

## Hash harness

Expected existing49 entries unchanged unless an existing sample contains the
reported entry-boundary topology. Measure rather than copy contributor values.
Any actual blank-entry removal delta requires independent attribution and its
own labelled behavior commit. Baseline changes remain ROOT-exclusive.

## Implementation checklist

- [x] Claim isolated worktree after dependency and approved-plan checks.
- [x] Reproduce three-entry and single-paragraph Base failures.
- [x] Reuse exact contribution once and correct all preservation gaps.
- [x] Execute focused native, installed, deterministic and risk gates.
- [x] Obtain clean independent microscope and scoped verify.
- [x] Update listed HLD and prepare structured integration handoff.

## Open questions

None. The user authorized new issue and PR repairs in S91. The bounded existing
TOC API repair is separate from the parked complete navigation contract. Opaque
source retention and URI-qualified ownership follow repository invariants.
