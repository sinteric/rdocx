# F-X198, Preserve alternate text-box copies during scoped replacement

**Status**: completed
**Sprint**: S91
**Size**: M
**Depends on**: F-X187

## Problem

Scoped replacement selects only the modeled physical story at
`crates/rdocx/src/document.rs:26846`. It splices only that owner's paragraph
at `document.rs:27012`. Story discovery deliberately selects the first
Choice containing a text box at `document.rs:8221`, leaving Fallback opaque.
The result can edit Choice while the VML Fallback retains old text.

## Spec reference

- `docs/hld/03-architecture.md`, physical story ownership and one document model.
- `docs/hld/04-opc-and-packaging.md`, The package, opaque XML and atomic mutation.
- `docs/hld/12-testing-strategy.md`, Checked Word scoped literal replacement.
- `docs/hld/14-development-backlog.md`, F-X198, full Issue301 and F-X187.

## Approach

Use the issue's explicitly permitted atomic-refusal outcome for paired
AlternateContent text boxes. Leave document-wide replacement unchanged.
Before a positive scoped edit can publish, inspect the selected text box's
physical enclosing AlternateContent in the actual retained source. Use
existing namespace-qualified story scanner conventions and exact span
containment, not matching text or a global substring search.

Add a small private detector in existing document.rs. It returns whether the
selected owner is contained by a qualified text box in an MC AlternateContent Choice with
another compatible text-box representation in a Choice or Fallback. Refuse
with Error::Other explaining that scoped edits of alternate text-box copies
require document-wide replacement. This maps through the existing Python
adapter to RdocxError. Standalone/single-representation text boxes keep their
supported scoped behavior. Opaque foreign lookalikes do not masquerade as MC.
A selected table cell inside that text box follows the same containment guard.
Main-body cells outside text boxes retain their existing scoped behavior.
The detector uses the containing text-box span rather than StoryKind alone.
Count mismatches and zero no-ops retain their existing outcomes in these cells.
Ambiguous alternate ownership refuses rather than selecting an identical box.

Perform the guard before splicing/publishing the candidate. Existing invalid
text, checked source selection, expected-count mismatch and zero no-op retain
atomic semantics. Candidate preparation never changes the live document.
Preserve source bytes, comments, formatting, revisions and unrelated members.
No public method, type, module, file or dependency is added.

The required repository-policy archive inventory checks exact normalized
member bytes. Rederive the affected rdocx archive in the existing local
measurement generator before preparation, update only its observed README
row and scripts/readme_doctests.py tuple, and rebuild until the actual records
agree. Keep inventory and tolerances unchanged. Integration rederives shared
records after combining workers. This is gate maintenance, not publication.

## Rejected alternatives

- Edit the first matching Fallback text, which can redirect to an identical box.
- Reimplement global replacement with a scoped substring filter, losing
  physical ownership and existing counted transactions.
- Claim Choice-only edits are valid, leaving producer alternates inconsistent.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | scoped_text_box_replacement_refuses_alternate_copies_atomically | Issue301 source-built paired box errors with byte-equal full package and live model |
| regression | scoped_single_representation_text_boxes_keep_their_supported_edit | Nonpaired box still edits selected paragraph and counts once |
| regression | alternate_text_box_refusal_tracks_physical_owner_and_namespace | Multiple identical boxes, aliases, shadows, unequal fallback and opaque siblings cannot redirect ownership |
| integration | scoped_text_box_count_mismatch_and_zero_noop_preserve_every_copy | Count mismatch, stale locations, invalid text and zero matches remain atomic |
| regression | scoped_cell_inside_alternate_text_box_refuses_without_mutation | Publicly inserted cell inside a paired box retains count mismatch and zero no-op outcomes, and refuses a positive edit atomically |
| round-trip | document_wide_text_box_replacement_updates_both_copies_once | Existing global replacement produces both PIN copies with count1 after save/reopen |
| regression | test_scoped_alternate_text_box_refusal_is_rdocxerror_without_mutation | Installed Python StoryItem refusal, unchanged revision, handles and bytes |

Use existing native regression_test.rs and existing Python replacement test
file. Build controls from current public add_text_box_to_story and generated
XML for alternate namespace/topology discriminators. Read contributor updates
before implementing and attach a later Issue301 PR to this F-ID.

## HLD impact

- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Parser/serializer: HLD04/HLD06, qualified MC/Word/VML ownership, exact opaque
  byte retention, positive namespace aliases and negative shadows.
- PyO3: actual installed Python tests and both WASM target checks. Workspace
  tests exclude both binding crates.
- Public behavior: a deliberate safety refusal of unsupported paired edits,
  not a new public API. Package dry-run and 10 MiB gate for affected carriers.
- No new file, crate, module, trait, generic, flag or dependency.

- Archive measurement carrier: HLD15 Packaging and the release command.
  Actual local archive records, exact members and existing size ceilings.
  No manifest, lockfile, version, tolerance or publication changes.

## Hash harness

Expected unchanged. No sample scoped alternate-text-box replacement is used.

## Exclusive file claims

- `crates/rdocx/src/document.rs`
- `crates/rdocx/tests/regression_test.rs`
- Existing Python replacement test entrypoint
- Listed HLD files and all Cargo/binding/test execution

- Existing README rdocx archive row and script measurement tuple

## Implementation checklist

- [x] Prove Choice-only failure at exact Base.
- [x] Add qualified physical-owner alternate detector and atomic refusal.
- [x] Cover standalone, multiple boxes, namespaces and producer topology.
- [x] Verify global replacement, count mismatches and save/reopen retention.
- [x] Run native/installed Python gates, scoped verify and clean microscope.
- [x] Prepare validated handoff.

## Open questions

None. Issue301 expressly permits atomic refusal. Use that outcome to retain
opaque XML without guessing equivalent alternate structure.

## PR317 reconciliation

Contributor head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0 supplies staged
paired-success mechanics for F-X202. Keep the independently tested qualified
owner guard and atomic refusal in this story. Do not replace it with the
contributor local-name-only txbxContent/AlternateContent routing, or move
whole-box topology refusal ahead of established expected-count/noop checks.
F-X202 reuses namespace-closing fragments, per-copy match counts, descending
splices and prepare/reopen publication only after proving physical pairing.
The contributor matcher is not adopted here and no new source change is
needed to finish the approved safety contract.
