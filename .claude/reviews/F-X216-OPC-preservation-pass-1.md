# F-X216, OPC import preservation, pass 1

**Reviewed**: Frozen relationship.rs98778e1aa6e34fd2b93ba3faba7b61ba5d8719b7b9a2ec821607e2ea12bc9692 against claim Base5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572. One existing carrier,493 additions and3 deletions, including inline controls. Other native/public/global-resource carriers are outside this pass.
**Verdict**: 2 defects, 0 smells, 0 nitpicks

## Defects

### D1, Raw splice offsets do not account for an input UTF-8 BOM

`crates/oxml-opc/src/relationship.rs:603`, `crates/oxml-opc/src/relationship.rs:620` and `crates/oxml-opc/src/relationship.rs:625`

relationship_xml uses reader.buffer_position directly as offsets into the original complete XML bytes. The pinned quick-xml0.41 reader removes the initial UTF-8 BOM in its initialization without incrementing state.offset, which buffer_position returns. Its buffered-reader BOM implementation consumes the three bytes without a position argument. Every later recorded range is therefore three bytes early for a BOM-prefixed relationship part. Rebinding a target or removing a record splices the wrong original bytes. Merging also copies wrong root-attribute ranges and inserts at wrong child/root boundaries. A valid BOM-prefixed source can be rejected or corrupted before reparse. Translate parser positions into the original byte coordinate space while retaining the original BOM. Add defining existing-entrypoint controls covering BOM-prefixed rebind and either-side merge, with and without a declaration, and untouched opaque payload.

### D2, Root metadata conflicts use raw names rather than expanded attribute identities

`crates/oxml-opc/src/relationship.rs:361` through `crates/oxml-opc/src/relationship.rs:366`

merge_for_import searches only equal raw attribute names. Two independently valid scopes can declare xmlns:z="urn:metadata" with z:mark="left" and xmlns:y="urn:metadata" with y:mark="right". Their mark attributes have the same expanded namespace/local identity, but the raw-name check sees different keys and appends both to one root. This creates duplicate expanded attributes, which namespace-well-formed XML forbids, instead of the promised root-conflict refusal. Current from_xml projection does not validate these non-namespace root attributes, so the items equality check cannot catch the malformed merged root. Compare namespace-qualified attribute identities, handle namespace declarations separately and refuse conflicting values atomically. Equivalent aliases with equal values must not emit duplicate expanded attributes. Add deciding alias-equality and alias-conflict controls.

## Smells

None found.

## Nitpicks

None found.

## Not found

No additional defect found in direct qualified root-child relationship projection, explicit ID-map membership, removed records, new records, source relative/external targets, source-owned record child preservation, ID collisions, invalid proposed modes/targets, staged rollback or the supplied two-scope namespace closure. Structure adds concrete import methods and local records in the existing carrier, without a new trait, generic, module, dependency or feature flag. The two independent defects above block this helper checkpoint.

## Actual evidence and boundaries

Root independently verifies the five raw rows in scoped successor63ac263d15798175c8121c1f736f130983bd6745b097d0b855dde01121e5b5d9: release compile and four exact controls all exit0, qualified, naturally safe, with matching logs. Existing-API Base c7fd9412 genuinely exposes nested foreign projection and mutable-model opaque loss. Current scoped helper fixes its supplied fixtures, but these do not cover BOM offsets or expanded-name alias conflicts. No new runtime or test was run by this audit.

Legacy direct items mutation followed by to_xml remains explicitly outside the scoped import-preserving helper. No global serializer fix is asserted. Source import callers must use the checked methods. A later labelled parser commit must state the expected foreign-projection behavior change and actual deterministic hash outcome.

The frozen carrier hash still matches at audit end. Only this canonical review file was written. Return to a separate remediation phase after the current native guard is safely terminal.
