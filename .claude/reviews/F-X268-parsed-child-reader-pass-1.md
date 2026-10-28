# F-X268, parsed child reader, pass 1

**Reviewed**: Codex root reviewed the bounded mechanism 7B1 reader on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 1066 added and 39 removed lines, including the separately reviewed serializer and write prerequisites. This pass covers the parsed child cache, empty frame, constructor selection and two reader tests.
**Verdict**: 1 defect, 1 smell, 0 nitpicks

## Defects

### D1, an empty frame is counted as a child read

`crates/rdocx/src/document.rs:7723`

`prove_empty_body_frame` increments `children_read` before even checking whether head is empty. It then parses a shell containing zero body children. This changes the approved sixth slot from actual one-child read attempts into child attempts plus frame attempts. An empty-body serialization therefore reports one child read although no child was read. Keep frame work visible separately without changing the six approved units. Count actual child attempts, including failures and retries, at their real sites.

## Smells

### S1, the cache regression permits rereading every safe child

`crates/rdocx/src/document.rs:44176`

The fixture contains exactly two safe children. After both are proved, the next read accepts any count below 3. A broken implementation that reparses both children on every call reports 2 and passes. Require the actual zero-child-read warm result and distinguish the field-bearing repeated-read case with exact counts. Preserve functional equality assertions in release.

## Nitpicks

None.

## Not found

No additional model-selection, output-byte, checked fallback, physical alignment, panic or new structural finding was established in this bounded source inspection. The constructor keeps the authoritative whole parser when the frame or a child is unproved. Parsed children are cached separately from serialized staged values and keyed by root and body namespace context. Empty and multi-root raw inputs fall back.

## Evidence and limits

`/tmp/fx268-m7-read-model-check.log` records the successful scoped check after three test-code compile errors were corrected. `/tmp/fx268-m7-read-model-test.log` records 12 passed and 0 failed in 16.84s. These are GREEN observations, not historical product RED evidence. Raw counter output and current release typecheck have not yet been retained. The table-row fixture grows rows within one child, so it is not the complete many-neighbor A10 proof.

Namespace, identifier and canonicalization work, all native A10 edit kinds, first-four byte bounds, Python rebuild, HLD, full composition, release behavior, timing, memory, Word and final gates remain pending. This is not a whole-feature microscope or mechanism 7 acceptance.
