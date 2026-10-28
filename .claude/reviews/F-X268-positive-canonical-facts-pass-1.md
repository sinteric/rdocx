# F-X268, positive canonical facts, pass 1

**Reviewed**: Codex root reviewed the positive canonical-read facts on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 3050 added and 151 removed lines, including earlier reviewed work and the diagnostic probe. This pass also checks the parallel identifier-cache consumer of the same child-scope invariant.
**Verdict**: 1 defect, 2 smells, 0 nitpicks

## Defects

### D3, independent child facts omit the body-frame eligibility proof

`crates/rdocx/src/document.rs:5959`
`crates/rdocx/src/document.rs:7881`

This extends the body-frame invariant from native-addressing pass 2 D3 to new canonical and identifier consumers. The existing addressing repair remains correct. These new readers independently wrap each raw child without establishing that it occupies the same closed parent scope as the whole part. Matching root declarations and body bindings from the empty frame is insufficient when raw bytes close one body and open another.

The actual cross-body probe writes two raw fragments. The first closes the body and opens a body binding q to the drawing namespace and s to the relationship namespace. The second contains q:docPr id 37 and s:id rId7. Old and block writers produce the same bytes. Whole scanners find relationship rId7, drawing 37 and Drawing(37) as an identifier definition. Cached facts instead return positive empty relationship and drawing results and an empty identifier hint.

`CT_Document::from_xml` rejects this two-body form with unexpected element w:body. This is a raw-writer and fallback/error-preservation case, not evidence of identifier loss from a valid imported Word document. The wrong positive certificate must become unavailable, retaining the authoritative scanners and original rejection behavior. Do not add a new raw-XML admission rule. Apply the shared closed-frame and covered-child eligibility to every independent fact reader, including frame and identifier paths, rather than fixing only the new relationship query.

## Smells

### S1, obsolete child empty-proof state blocks the required clippy gate

`crates/rdocx/src/document.rs:7831`

BodyBlockScan.canonical_empty, canonical_child_is_empty and MainPartBlocks.proved_canonical_empty no longer have consumers after the positive-fact conversion. The check reports all three as dead code. This retains a second unused cache policy and prevents clippy with denied warnings. Remove obsolete state, initializers and methods. Retain the frame empty proof still used by the positive path, without blanket allowances.

### S2, composing cached relationship ids retains quadratic membership work

`crates/rdocx/src/document.rs:8395`

The new fast composer calls Vec.contains for every id. K distinct ids therefore require 0 + 1 + ... + K-1 comparisons. Picture-bearing workloads grow this list with the document, so cached XML reads can hide a quadratic metadata phase. This conflicts with the plan's linear registry-owner work requirement. Keep the ordered vector and use a membership set to preserve first occurrence with expected linear lookup work. Retain ordered-dedup evidence and actual work accounting. This is a source-established complexity finding, not a claimed runtime RED observation.

## Nitpicks

None.

## Evidence and limits

`/tmp/fx268-m7-positive-canonical-check.log` records successful compilation with the three S1 warnings. The focused fact test passes. The batch records 22 passed and 1 failed in 67.13s, with only the known original A10 namespace-work diagnostic failing. On its table-row samples, canonical whole-input bytes are now zero, serialized children remain 18, children read remain 9, and identifier input is 3295 or 3346 at both sizes. Namespace work still grows from 254607 to 992583 bytes. Section and list cases have not executed.

The private fragment counter now records input at each actual scanner invocation, including wrapping bytes and short circuits. Its 155196 and 156420 sample values match across sizes. Earlier 12924 observations counted one cache-fill input rather than all validator inputs and are not the same unit.

`/tmp/fx268-m7-cross-body-scope-probe.log` records actual D3 RED, 0 passed and 1 failed in 0.02s, with all three wrong positive facts printed before the assertion. Product code was unchanged for that diagnostic.

## Not found

No additional drawing-selection, span-rebase, replay-authority, current-argument, parser-error fallback or temporary-publication defect was established in this bounded inspection. Cached drawings are selected using current modeled ids and nested relationships, and scanner errors retain the old whole fallback.

Namespace facts, original A10 section and list coverage, current release and clippy gates, Python rebuild, HLD, full composition, mechanism 8, timing, memory, Word and final gates remain pending. The installed extension remains the mechanism 6 build. This is not mechanism 7 acceptance or a whole-feature microscope.
