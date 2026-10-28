# F-X268, opened staging guard, pass 1

**Reviewed**: Mechanism 3 unchanged-main admission in the five-file working diff, 1232 insertions and 63 deletions. This is a bounded guard review, not the full feature microscope.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, serialization equality replaces the old model equality gate

`crates/rdocx/src/document.rs:18301`

`opened_main_still_serializes` admits solely on source bytes, namespace context and serialized model bytes. Both `flush_document_to_package` and the opaque relationship inventory then assume the old parsed-model equality condition is true. Serialization is not injective. For example, removing a redundant canonical `xmlns:wp` entry from the opened model can retain the same serialized bytes because `CT_Document::root_start` inserts that declaration automatically, while the parsed model is unequal. The debug fresh-parse assertion panics on this admitted state. The release path instead skips the old equality decision. Preserve cached parsed-model equality in addition to successful serialization equality, and prove the unequal-model, equal-serialization case misses before either shortcut.

## Smells

None.

## Nitpicks

None.

## Remaining scope

The complete canonicalization and comparison changes still need their bounded review after remediation. This report does not discharge A3, A5, A10, memory measurement, or final integrated verification.
