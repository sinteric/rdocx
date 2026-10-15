# F-X196, Validate visible paragraph content without false empty warnings

**Status**: completed
**Sprint**: S91
**Size**: S
**Depends on**: none

## Problem

`crates/rdocx-cli/src/commands.rs:2318` counts trimmed text only. Paragraphs
containing pictures and other visible nontext content trigger empty warnings.

## Spec reference

- `docs/hld/10-bindings-spec.md`, CLIs, validation advisory findings.
- `docs/hld/12-testing-strategy.md`, Test taxonomy, regression.
- `docs/hld/14-development-backlog.md`, F-X196 and full Issue298 criteria.

## Approach

Keep the advisory body-paragraph inventory and metadata warnings. Replace
its text-only predicate with a private content-aware check in existing
commands.rs. Text-bearing paragraphs remain nonempty. Scan the existing main-part XML
read-only with quick-xml to build one visible-nontext flag per admitted body
paragraph. Follow direct body paragraphs and recursively typed block-control
content. Direct body tables remain excluded. Tables inside block controls are
entered, matching CT_Body::paragraphs and CT_Sdt::collect_paragraphs exactly.
Properties and unsupported wrappers remain excluded. Zip the
flags with the existing facade paragraph inventory and reject an inventory
mismatch. This preserves its exact counting scope, including nested block
controls and their owned table cells. No new public facade surface or dependency is needed.

The scanner detects actual text and Word drawing/pict/object, field elements,
symbols, tabs, breaks, footnote/endnote references and OfficeMath content.
Descend through content-control content and supported inline wrappers. Do not
count paragraph properties, bookmarks, range markers, deleted text or
lastRenderedPageBreak producer hints as visible content. Properties inside
sdtPr are not rendered children. Use expanded names rather than local-name
matching foreign producer elements. A truly empty content control remains
empty, and a content control with a visible nontext child is nonempty.
The scan is read-only and never rewrites the source or changes revision.
Malformed snapshot input reports the existing CLI error rather than being
silently classified as empty. Preserve the original counting scope.

The required repository-policy archive inventory checks exact normalized
member bytes. Rederive the affected rdocx-cli archive in the existing local
measurement generator before preparation, update only its observed README
row and scripts/readme_doctests.py tuple, and rebuild until the actual records
agree. Keep inventory and tolerances unchanged. Integration rederives shared
records after combining workers. This is gate maintenance, not publication.

## Rejected alternatives

- Test only drawings, which leaves every other listed content kind broken.
- Count any XML tag as visible, which treats bookmarks and properties as text.
- Add a forwarding public facade method solely for this CLI predicate.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | picture_paragraphs_do_not_trigger_empty_paragraph_warnings | Reported three-picture document gives metadata warnings only |
| regression | visible_nontext_paragraph_content_is_not_empty | Every Issue298 content kind and nested content-control child is recognized |
| regression | paragraph_properties_and_range_markers_do_not_invent_visible_content | Empty, whitespace-only, marker-only, property-only and empty-control paragraphs remain empty |
| round-trip | validating_nontext_content_preserves_document_bytes | Snapshot/validation is read-only and title/author diagnostics remain unchanged |

Add tests to the existing commands.rs test module and existing CLI
integration entrypoint. Use source-built XML and generated image bytes.

## HLD impact

- `docs/hld/10-bindings-spec.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Parser: read HLD04 The package and HLD06 parser conventions, qualify expanded
  names and preserve opaque bytes. Positive namespace aliases and foreign
  lookalikes distinguish the scanner.
- Published CLI behavior changes only the incorrect advisory warning. No
  library API, version, dependency, flag, trait, generic, module or file change.

- Archive measurement carrier: HLD15 Packaging and the release command.
  Actual local archive records, exact members and existing size ceilings.
  No manifest, lockfile, version, tolerance or publication changes.

## Hash harness

Expected unchanged. Validation reads documents without rendering or mutation.

## Exclusive file claims

- `crates/rdocx-cli/src/commands.rs`
- Existing CLI integration test entrypoint
- HLD files above and shared Cargo/test execution

- Existing README rdocx-cli archive row and script measurement tuple

## Implementation checklist

- [x] Prove the three-picture warning failure at exact Base.
- [x] Add private content-aware predicate and qualified XML scan.
- [x] Cover every visible kind and empty/metadata controls.
- [x] Verify source preservation, scoped gate and clean microscope.
- [x] Prepare validated handoff.

## Open questions

None. Empty means no visible content, as explicitly requested in Issue298.

## Approved approach revision

Inspection found paragraph_story_location deliberately returns None for nested block-control paragraphs at document.rs:20117. The original per-paragraph snapshot approach cannot cover the existing recursive body paragraph inventory at document.rs:11744. The read-only main-part scan above keeps the same source and counting scope without a forwarding public API. This technical revision has no product ambiguity and adds nested block controls and table inventory controls to the existing test plan.

The nested-scope regression found that block_paragraphs is the heading/link
inventory, not Document::paragraphs. The latter calls CT_Body::paragraphs at
rdocx-oxml/document.rs:2518 and CT_Sdt::collect_paragraphs at
content_control.rs:1197, which enters owned tables within block controls.
The approved main-part scanner must follow this exact existing inventory:
direct body tables excluded, control-owned tables entered through qualified
row/cell/control paths. The source-built test distinguishes both cases.

## Approved opaque-control remediation

Independent microscope pass1 found duplicate sdtContent and controls with
invalid typed properties were incorrectly admitted. The public regression
failed with an inventory mismatch before production remediation, exit101.
Use the existing CT_Sdt::story_raw_is_typed admission helper with the correct
physical owner and active Word namespace prefixes before entering a control.
A cloned read-only namespace reader bounds its source span without disturbing
the main scan. Admit only the first expanded sdtContent container, matching
the native RootSlot::Content rule. Self-closing containers remain raw slots
and do not consume that typed content slot. Entire opaque controls and later
raw containers stay excluded from paragraph inventory and visibility.

Nine public discriminators cover duplicate block/inline containers, expanded
empty versus self-closing first slots, invalid-property block/inline controls,
namespace aliases and nested opaque controls. Preserve metadata diagnostics
and exact input bytes. This revision uses an existing native admission API
and adds no library surface, source module, dependency or writer.
