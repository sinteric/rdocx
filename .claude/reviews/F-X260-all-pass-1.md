# F-X260, all, pass 1

**Reviewed**: uncommitted working tree on `work/f-x260-grok` after `c776041c`. 16 files, 2114 insertions and 147 deletions.
**Verdict**: 1 defect, 0 smells, 1 nitpick

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

### D1, parsed namespace records do not override admission prefixes
`docs/hld/03-architecture.md:82`

The new architecture sentence says a parsed root or body declaration overrides the prefixes `w:document` always declares. `direct_body_paragraph_context` copies `root_namespace_declarations` and `body_namespace_bindings` into the admission map (`crates/rdocx/src/document.rs:6644`). Those vectors store attribute names such as `xmlns:w` (`crates/rdocx/src/document.rs:2753`). `expanded_name` looks up the prefix before the colon (`crates/rdocx/src/document.rs:7805`). A stored `xmlns:w` entry does not replace seeded `w`, and a stored `xmlns:w14` entry does not bind `w14`.

A loaded document that rebinds `xmlns:r`, or that declares `xmlns:w14`, therefore keeps only the four writer prefixes during a plain local replacement. The A5 packages are constructed documents with an empty declaration list, so their visit tests stay green. The spec sentence is still false.

## Smells

None.

## Nitpicks

- `docs/hld/03-architecture.md:93`, the connector sentence breaks after "A connector,".

## Not found

Correctness produced no other defect. A plain direct body paragraph is one with no comments part, no commentsExtended part, and a `CT_P` whose companions are empty and whose runs contain only text, tab, break, symbol or a special character (`crates/rdocx/src/document.rs:6604`). Anything else, including shared media and `remove_content_range`, stays on `clone_for_staging` (`crates/rdocx/src/document.rs:26916` and `crates/rdocx/src/document.rs:26940`). Section-property mismatch returns the existing section error before assignment (`crates/rdocx/src/document.rs:26880`). PowerPoint local fill, local geometry and local XML each restore the edited child when validation fails (`crates/rpptx/src/lib.rs:11400`, `crates/rpptx/src/lib.rs:39993` and `crates/rpptx/src/lib.rs:6144`). A connector still clones the presentation and notes one visit per slide (`crates/rpptx/src/lib.rs:39961`).

Contract produced no other defect. A1 through A4 stay handle tests. A5's visit bound is the library test of that name. The release ratio stays final-sprint evidence in the plan (`.claude/plans/F-X260-design.md:59`) and in HLD 12 (`docs/hld/12-testing-strategy.md:3326`). A6 is half-open and does not walk table children. A7's shared-media and refusal controls were not narrowed. PR393's head is still named and is not merged (`docs/hld/14-development-backlog.md` F-X260 entry). The extra edited files are the Deviations list, not a second registry. Q8, Q9 and Q10 are unchanged. The worker acknowledgment of `659abb21` and `a6d8d4d7` is present (`.claude/scratch/F-X260-progress.md:49`).

Panics produced no finding. The new plain-publication `expect` calls are birth-count invariants on a body paragraph inventory the passing removal test already builds. User XML errors return `Result` before the native assignment.

OOXML produced no other finding. The local paragraph writer is the existing `CT_P::to_xml`. Child order is unchanged. A paragraph with retained root attributes is not plain because `extra_xml` is non-empty, so `w14:paraId` stays on the package transaction.

Tests produced no defect. Each of the four library tests asserts the small and large visit counts are both 1 for every one of the seven operations (`crates/rdocx/src/document.rs:56154`, `crates/rdocx-py/src/document.rs:12656`, `crates/rpptx/src/lib.rs:40676` and `crates/rpptx-py/src/presentation.rs:4708`). The same name in the two Python files checks the edited owner and one unrelated owner, and it does not claim the ratio. Reverting the local note to a package-length note fails those assertions. The fx211 admission subset was rerun after the namespace seed: `fx211_imported_local_picture_snapshot_remains_namespace_complete`, `fx211_drawing_identity_preserves_existing_repetitions_and_refuses_new_collision`, `fx211_nested_section_profiles_are_preserved_or_refused_atomically` and `fx211_selected_paragraph_xml_stages_noops_and_late_refusals` each passed. Affected pytest passed: Word 4 selected tests, PowerPoint 3 selected tests. Debug timings were not used as A5 evidence.

Structure produced no finding. No new trait, generic, crate, module, file or public type was added. The visit counters are `cfg(test)` thread locals in the four crates that already own the operations.

## Final gate debt

Hash harness expected unchanged. The release-build ratio below 3, the installed product suites and the full sprint union stay pending. This pass does not tick the final acceptance checklist item.
