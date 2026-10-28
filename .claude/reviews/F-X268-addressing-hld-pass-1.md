# F-X268, addressing HLD, pass 1

**Reviewed**: Codex root reviewed the mechanism 6 edits to the five approved HLD files and the plan evidence list on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. The six-file diff is 15 added and 8 removed lines, including replacements of existing long paragraphs.
**Verdict**: 0 defects, 2 smells, 1 nitpick

## Defects

None.

## Smells

### S1, distinguish the address scan record and child boundaries

`docs/hld/03-architecture.md:60`

The new text calls the address record the current last opening. MainBodyScans is a separate scan record that also exists for a new document with no opening record. Describe one retained address record without a history chain. The same paragraph describes a raw span crossing that frame, but the guarded crossing is a body child boundary. The split-raw paragraph regression remains inside one body frame and still needs the full fallback. State that boundary precisely.

### S2, reconcile actual evidence names and retained instrumentation debt

`.claude/plans/F-X268-design.md:346`
`docs/hld/12-testing-strategy.md:4455`

The A3 evidence column still names body_span_serialization_matches_existing_xml, while the amended test list names the actual body_spans_match_existing_to_xml_on_current_fixtures. Reconcile the span alias in every active evidence reference. Keep main_body_block_reuse_preserves_physical_owners_and_publication explicitly planned as the broader composition proof rather than treating the seven address cases as its completion.

The testing-strategy replacement also removes the explicit statement that the two child-stage counters are unimplemented. That work still belongs to mechanism 7. Retain an explicit pending statement so zero child slots are not presented as proof of skipped serialization or parsing. No criterion or performance bound changes.

## Nitpicks

- `docs/hld/14-development-backlog.md:8668`, remove the newly duplicated delivery sentence, The feature is not delivered followed by Not delivered.

## Not found

No other contract, scope, consumer or HLD-impact mismatch was established in the edited text. TableLocation::native is the actual table consumer name. The descriptions retain exact byte proofs, current source ownership, checked fallbacks, cumulative counter semantics and separately disclosed whole costs. No stable Python API or total constant-time claim is added. The feature remains approved and in progress. Final release execution, composition, measurements, Word, hash and full sprint gates remain pending.
