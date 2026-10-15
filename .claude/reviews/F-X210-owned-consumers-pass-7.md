# F-X210 owned consumers, pass 7

**Reviewed:** Complete four-carrier external candidate `/private/tmp/S91-F-X210-owned-consumer-candidates-r1-prepared`, including production and three added inline controls. Complete diff is 18656 bytes, SHA-256 `fbd3e90f1a91c31299134be61ba322c4dc3f463184c9fcf1d2e24ba9cc70f9f9`. Receipt SHA-256 `2afb2abddf10eba6372df90cf61ca267a7d23e5524b36305be908acdfcf28314`. Approved plan, current status/progress, existing provider and consumer controls and bounded contribution consumers were inspected.

**Verdict:** 1 defect, 0 smells, 0 nitpicks. Proposal remains unadopted and uncompiled by this review.

## Defects

### D1, caller aliases are mistaken for authentic physical families

`/private/tmp/S91-F-X210-owned-consumer-candidates-r1-prepared/candidate/crates/oxml-layout/src/font.rs:1537`

selected_request_face accepts membership in caller_aliases as proof that the selected physical face is the requested family. But existing load_caller_font explicitly creates these aliases when the supplied label differs from every parsed physical family at candidate `crates/oxml-layout/src/font.rs:1000`. Aliases are a supported selection route, not physical-family authentication.

Trigger: load genuine Carlito bytes with the caller label Aptos, then contribute an Aptos-requested nonempty run. Selection correctly follows the caller alias, but record_font_approximation returns early at candidate `crates/oxml-layout/src/font.rs:1561`, suppressing the required message even though the drawn family remains Carlito. A genuine Aptos face and an arbitrary face labelled Aptos must not be indistinguishable for the approximation contract.

The same predicate also gates original-symbol preference at candidate `crates/oxml-layout/src/font.rs:1590`. Carlito bytes labelled Wingdings, with covered low-byte explicit symbol code FC, preserve Carlito's Latin glyph rather than applying the authenticated semantic check-mark substitute. The plan gives this precedence to a supplied genuine original-family face with original-code coverage. It does not establish original glyph identity from an arbitrary caller alias.

Preserve accepted caller selection and precedence. Independently authenticate physical family for approximation suppression and genuine-symbol preference, and add discriminating controls for an authentic requested face versus a differently named physical face exposed through a caller label. The proposal itself records these authentic-face controls as still missing in qualification.md.

## Smells

0 findings.

## Nitpicks

0 findings.

## Checked aspects and evidence

- **Correctness:** D1 above. The new diagnostic sink is reset by begin_layout, adds facts only for nonempty Contribution events, retains the first selected-family message per finite Aptos spelling and rolls provisional messages back to the scope's starting length. Existing stale/nested token validation precedes the new truncation. Accepted replay validates the whole event vector before mutating the sink. Trace overflow removes provisional scope warnings while preserving previously committed output facts. The three inline tests exercise nested rollback, accepted replay, overflow, empty/non-Aptos contributions, operation reset and invalid replay. No additional finding in these paths.
- **Contract:** D1 above. Full-output Word and PPT results copy facts into their owned diagnostics vectors. Word measure_content and whole layout begin separate operations. PPT full presentation resets once before slide traversal. Existing output bytes and source models are not rewritten for diagnostics. Frame measurements, notes/handout/timeline/direct-slide propagation, native/Python byte companions, CLI and fit reports are explicitly pending rather than credited by this partial proposal.
- **Panics:** 0 findings. New index access follows internal FontId lookup or bounded selected-face indices. No untrusted glyph indexing or new arithmetic overflow route was found. Existing replay budget validation and invalid-event refusal remain intact.
- **OOXML:** 0 findings. Word conversion is confined to generated numbering display markers with PUA and explicit RunContent::Symbol. PPT conversion is confined to shape_marker after bullet style selection. Ordinary inline PUA is not globally rewritten. Original run content, stored char codes, numbering and bullet models are not edited. The verified eight family/code mappings and unknown-family/code preservation remain bounded. Existing Wingdings F0B7 bullet control retains its separately enabled PPT compatibility route.
- **Tests:** D1's missing authentic-versus-labelled discriminator is required. The new table test requires actual nonzero shaped Unicode glyphs and unknown/other-family preservation. It does not claim native appearance parity. The four public consumer Base controls and seven earned provider controls remain unchanged. Current running Base results, future compilation and all painted-output results are not inferred by this review.
- **Structure:** 0 findings. Exactly four existing source carriers change. The concrete symbol resolver and diagnostic getter have named current Word/PPT consumers. No new module, trait, generic, registry, dependency or repository file is introduced. The diagnostic sink is bounded by the two supported request spellings rather than historical FontId attribution.

## Authentication and limits

All sealed artifact and candidate hashes match. All 2909 actual source records match baseline `7a02e0f5903b4e86380e4dec97e71c47b7b67f21b26db9d921800d444b006e80` in bytes, size and mode. Prospective map changes exactly the declared four carriers. The worker remains work/f-x210-codex and F-X210 remains in progress. The prior provider phase is separate earned authority. Historically self-authored consumer test controls are inspected for preservation, not claimed as newly independent original test design.

Genuine consumer Base outcomes remain an adoption prerequisite. Full approved diagnostic companions and frame/export routes, caller suppression, original-symbol coverage, independent PDF/PNG/SVG paint, metric limitations, no-default/WASM, publication/license inventory, scoped verification, hash attribution and HLD delivery remain mandatory. No whole-feature completion, adoption or runtime credit is granted. Only this report was written. Return the review-only lease to ROOT.
