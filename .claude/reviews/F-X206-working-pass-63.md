# F-X206, whole working implementation, pass 63

**Reviewed**: Actual worker `work/f-x206-codex`, HEAD `7b1e5ef29464e93ae30e68df1523ce09143578ff`. Working diff: 96 files, 32809 insertions and 2525 deletions. Approved whole plan, current HLD, progress, original whole working review and retained component reviews, including source-union pass 53 and HLD pass 59.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authentication and scope

All 2714 actual tracked source byte, size and mode records match `/private/tmp/S91-F-X206-HLD-union-r2-prepared/root-adoption-1/source-after.json`, SHA-256 `3ba5a6c7fe47334e0b31f85764935e99bccfa2b9586094bf7ae85b2d03d7777f`. Actual worker semantic index, raw `git ls-files --stage -z` bytes, has SHA-256 `c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c`, matching the retained index authority. The earlier 3a63 observation was the physical Git index file hash, a different representation, and is not evidence of index mutation. No source or index writes occurred in this review.

The review covers the current complete feature contract and its composed native, model, binding, painter and HLD implementation. Retained independent component reviews qualify unchanged inherited components. Direct examination focuses on current publication and relocation paths, table/grid descendants, picture replacement, internal links and namespace ownership, group raw-member movement, connector rerouting, alignment, caps/highlight and member recovery. Accepted F195 and F200 prefixes are retained as composed dependencies, not excluded by filename. I authored historical external namespace and geometry remediation and HLD proposals. This pass does not supply a new independent certification of those original authored hunks. Its finding concerns the separately authored replacement owner-proof helper.

## Defects

### D1, flattened-text equality keeps changed run descendants live

`crates/rpptx-py/src/presentation.rs:1344`

`replaced_text_owners` compares only flattened `ShapeRef::text`, cell text or notes text, then uses that comparison as the entire `text_changed` proof. Both presentation replacement (`crates/rpptx-py/src/presentation.rs:1202`) and slide replacement (`crates/rpptx-py/src/slide.rs:284`) publish through this helper. A false flag leaves the old descendant Text state live because owner preparation only retires it when that flag is true (`crates/rpptx-py/src/presentation.rs:1055`).

A concrete trigger is a shape with two ordinary runs, first text `A` with bold formatting and second text `B` with italic formatting. Call `Slide.try_replace_text("AB", "AB", expect=1)` or the presentation equivalent. The native cross-run algorithm replaces the first run's matched tail with the complete replacement (`crates/rpptx/src/lib.rs:10743`), then removes the second run's matched prefix (`crates/rpptx/src/lib.rs:10757`). The resulting runs are `AB` and empty, so the accepted run contents and formatting distribution changed. Flattened text remains `AB`, and this helper marks the owner unchanged. Previously held paragraph/run/font/hyperlink descendants remain valid against changed run content, with no recorded replacement retirement cause.

This is not an unchanged native operation. The approved plan requires descendant retirement for affected text ownership and explicitly forbids treating displayed-text equality as sufficient no-op evidence (`.claude/plans/F-X206-design.md:475`, `.claude/plans/F-X206-design.md:477`). The frame route already retires the selected owner's Text state for a nonzero native replacement count (`crates/rpptx-py/src/text.rs:365`). The slide/presentation routes must identify changes to accepted text-body/run facts or retain an equivalent native affected-owner proof, while preserving untouched owners and stable slide/shape/frame ancestors. A deciding installed control should hold the two formatted run descendants, perform this same-string cross-run replacement, verify saved run distribution and replacement count, and require the actual public replacement cause on retired descendants. It should also retain unrelated-owner stability and zero-match/error atomicity controls.

## Smells

0.

## Nitpicks

0.

## Six-aspect results

- Correctness: D1. No additional findings in checked owner relocation, staged candidate publication, grid retirement, foreign-owner validation, geometry preflight, connector rerouting or recovery paths.
- Contract: D1 violates the accepted no-op and descendant-text ownership boundary. Required picture filling, distinct tcPr anchoring, internal slide links, native placeholder naming and ancestor-aware highlight surfaces are now present. Their presence is not an acceptance claim.
- Panics: 0 additional findings. Checked path, range, geometry and owner preconditions guard the examined publication and native transform operations. Trusted test assertions are not classified as production panics.
- OOXML: 0 additional findings. Examined namespace/relationship overlays, opaque group member movement and owner metadata refusals preserve the retained classification and schema-order contract. No blanket opaque-group waiver or foreign-token normalization is accepted by this review.
- Tests: 0 additional findings apart from the missing deciding coverage described with D1. Retained Base and source controls are not rewritten or credited as current GREEN. Actual SVG/Q/C, physical-font and installed affine/picture controls remain required under their original assertions.
- Structure: 0 additional findings. Current concrete owner/provider machinery is used rather than a second registry or framework. No structural change is proposed or applied in this review.

## Acceptance limits and handback

The original five semantic Base controls and additional highlight/picture Base history remain retained. Current production GREEN, full changed-crate/scoped verification, fresh paired wheels and installed acceptance, typing/stub checks, Issue323 paint/native application/no-repair evidence, hash, WASM, archive/source compatibility and policy gates remain separate required evidence. Their pending execution is not manufactured into a source defect, and this review grants no runtime PASS or whole-feature completion.

Only this review file was written. The review pass ends here and returns the sole canonical review writer lease to ROOT.
