# F-X268, block namespace proof, pass 1

**Reviewed**: Codex root reviewed the bounded mechanism 7B2 parse and namespace reuse on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 1534 added and 49 removed lines, including the earlier reviewed serializer, writer and parsed-child work. This pass covers current-part proof consumption in flush, relationship inventory and reopen, and the empty-owner namespace fact.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, the empty-owner proof ignores accepted opaque frame content

`crates/rdocx/src/document.rs:7752`

`store_empty_namespace_owners` examines namespace declarations only in children, tail, and the head after the last body start. It does not establish the old namespace scanner's result over the complete head and tail. An admitted raw background containing a nested body and a locally bound unsupported run child is therefore assigned an empty owner list, although the authoritative scanner finds one owner in those same bytes.

The actual `main_part_blocks_namespace_shell_probe` demonstrates this. `document.to_xml` and the block writer agree on 811 bytes. `CT_Document::from_xml` accepts them. The old namespace scanner finds one owner, `proved_document` succeeds, and the retained fact contains zero owners. The probe fails with `retained=0 old=1`. This is a wrong cached fact, not a request to impose new schema admission on preserved raw XML. The constructor, flush and relationship helper consume this fact in place of the authoritative result.

Prove the complete frame with the existing owner logic. A nonempty or unproved frame must keep the checked whole-part fallback. Preserve current bytes and errors, and keep frame work separate from the six approved whole-part and child units.

## Smells

None.

## Nitpicks

None.

## Evidence and limits

`/tmp/fx268-m7-parse-namespace-finish-test.log` records 13 passed and 0 failed in 17.36s before the diagnostic probe was added. The measured cases use 50 paragraphs on each side of a fixed table versus 200 on each side. Warm whole-part parse and namespace bytes are zero. The reference clears blocks before preparation and again before reopening, preserving in-memory package eligibility while forcing the whole main parser. Its whole parse and scan work grows with input. Canonicalization and identifier bytes still grow on the optimized path.

Earlier failures are retained as partial implementation evidence. Constructor proof ordering and the relationship helper were both needed. The unused namespace marker was dropped by both implementations, so that failed assertion was a fixture error. The replacement control uses an existing preserved unsupported run child and checks actual part XML rather than ZIP bytes.

`/tmp/fx268-m7-namespace-shell-probe.log` records the current diagnostic RED: 0 passed and 1 failed in 0.02s. Current release typecheck and clippy have not been rerun for this namespace change. The installed Python extension is still the mechanism 6 build.

## Not found

No additional current-part identity, model-selection, counter-unit, field reread, physical alignment, panic or structural finding was established in this bounded source inspection. The original whole scan remains the fallback when no cached owner fact is available.

Canonicalization, identifier optimization, complete native A10, section and list coverage, Python rebuild, HLD, whole-feature composition, release behavior, timing, memory, Word and final gates remain pending. This is not mechanism 7 acceptance or a whole-feature microscope.
