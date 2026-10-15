# F-X209, master style expected fixture, pass 8

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: unadopted one-hunk expected-object proposal, against immutable source49 copied carrier and unchanged source45 style control. This is test-proposal admission, not production or whole-feature review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. The bounded correction is admissible. Successor execution remains due.

## Authentication

Proposal diff SHA `689ba0a114f596401405589d5353d444998aaafb8050a9eafe9e634eee383116` is exact. Before49 carrier SHA `72f4ca21403d4a944ddd2d321d9b80dc6625d0fe7611ccfc24d1dab25113703d`. Proposed after SHA `4e93cd25942782749acaacc0130028f28c2773b800526ebf9bed350d2b16c750`. An independently reconstructed unified diff matches the supplied proposal exactly. Source45 carrier SHA `00a2428e8d9d923a75b1e088afacfe88594772daa8222f4ce5c2469365eb5a5c` matches its source map. The complete style-control body is byte-identical between source45 and the proposal's source49 preimage, despite shifted lines and unrelated later source49 additions. Those additions are excluded.

## Assessment

The control clones a parsed paragraph property object, changes font size to 4000 and typeface to Carlito, then explicitly authors it. Its private original-source XML and projection still describe the prior parsed operand. Existing paragraph-property equality compares both modeled fields and closed source-owner identity, so direct comparison with that mutated yet un-reparsed operand compares inconsistent provenance states.

The proposed expected object is derived solely from the requested operand through the existing public write_xml and namespace-qualified lstStyle/from_xml providers. It is not derived from the product getter or saved product output. Full property-object equality remains, with the requested size/typeface and all remaining modeled/raw fields in the independently serialized expected operand. See `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/master-style-normalized-expected-proposal.rs:20821` and `:20829`. The relevant existing provider behavior is `crates/oxml-drawing/src/text/paragraph.rs:2741` and `:2968`: equality retains source-owner closure and the writer detects changed modeled projection before serializing it.

The proposal does not alter production, input fields, the original complete object comparison, repeat whole-package equality, reopened size, opaque root bytes, unrelated part bytes, invalid-name/level atomic refusals or body levels1..5. Those assertions remain at `master-style-normalized-expected-proposal.rs:20834` onward. It normalizes expected provenance rather than weakening observable authoring requirements. The setter's own source patch and provider serialization are unchanged and are not awarded broader acceptance by this pass.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: 0 findings in expected-object normalization.
- Contract: 0 findings. All deciding semantic/source/refusal assertions remain.
- Panics: 0 new findings in this test-only proposal.
- OOXML: 0 findings. The expected wrapper declares the actual DrawingML namespace.
- Tests: 0 findings. Expected derives from the requested source operand, not observed product output.
- Structure: 0 findings. One existing test hunk, no new API/module/dependency.

## Evidence and limits

Original source45 failed receipt SHA `ab9f1355bc1639955c3b1c720f955eccc79a0f18a13318b1e0caef55aee96bb0` and style log SHA `aa5fbacf07cc9ea34a2419875bf6bb8c179dfb50516b16e44e0f7074130eaa8e` are authenticated. The exact style row exited101 at the original full-object assertion. The separate background refusal row in that attempt passed and remains separate. No successor runtime, native paint, Python or whole-feature acceptance is claimed. No source or runtime changes were made by this reviewer. Review ends at this report.
