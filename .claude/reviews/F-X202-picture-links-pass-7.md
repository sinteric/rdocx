# F-X202, picture and links, pass 7

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable picture-links-capsule-r7, fourteen complete product/test before/after carriers, approved plan, full source map and context-inclusive current.diff of 166,154 bytes. Scope is new physical Picture inventory/resize, configured picture import, internal and heading links, Python publication and typing. Previously reviewed tables, paired text-box replacement and metadata entity correction are excluded except shared transaction ingredients.
**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Defects

### D1, supported related-story sources cannot author internal or heading links

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx/src/document.rs:19607`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx/src/document.rs:19697`

append_internal_hyperlink_staged unconditionally refuses a source paragraph outside the main document part. add_heading_hyperlink_at applies the same restriction to its source. Thus a valid live header/footer paragraph cannot link to a complete bookmark or heading in the same Document. The approved Bookmarks and internal links contract permits exact supported source owners and same-document targets in another supported owner. An anchor-only hyperlink requires no external media relationship that justifies this blanket part restriction. The worker confirmed there is no later approved narrowing. Retain exact owner and bookmark validation while supporting these source owners, with related-source save/reopen and refusal atomicity controls.

### D2, internal-link target validation omits the promised bookmark grammar

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx/src/document.rs:19604`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx/src/document.rs:19612`

The new path validates XML characters and then checks only matching-name cardinality, marker issues and a complete range. It never applies the existing native bookmark-name grammar required by the approved approach. A unique complete imported bookmark with a grammar-invalid name such as Bad Name can therefore be used to author a new anchor. Inventory completeness does not imply authored-name validity. Validate the requested bookmark through the existing native policy before any staging or allocation, and add a complete imported invalid-name target with exact unchanged-package refusal. The worker confirmed no separate grammar preflight exists in this slice.

### D3, Picture typing exposes a constructor that runtime does not support

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx-py/python/rdocx/_rdocx.pyi:67`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-links-capsule-r7/source/crates/rdocx-py/src/document.rs:243`

Picture is a frozen factory-created snapshot with no runtime new method, but its stub has no private constructor declaration. Static users can call Picture() through the inherited object constructor, although that call cannot create a runtime Picture. This contradicts the requested nonfabricable snapshot boundary and the existing factory-only typing convention. Declare the existing private constructor boundary in the stub without adding a runtime public constructor. A negative typed construction control should preserve the published factory and snapshot property types.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: D1 and D2. Contract: D1, D2 and D3. Panics: 0 additional findings. OOXML preservation: D2, 0 additional findings. Tests: 0 additional findings, the current positive controls do not decide these negative/related-source cases. Structure: 0 findings.

## Reviewed preservation and ownership

The physical resize route re-resolves the complete frozen occurrence and checks its current correspondence before staging. It distinguishes one occurrence from relationship-wide resizing, preserves the separate legacy relationship operation, transports numeric edits onto original source attribute ranges, and publishes only after staged reopen. Picture snapshots use existing ContentPath origin validation, so identical relationship strings from another Python document do not authorize mutation. No new global identity registry is introduced.

Configured body, run and cell insertion allocates image relationships on the checked actual story part and preallocates returned Python snapshots with prepared revisions before committing the candidate. Heading link creation stages bookmark and source hyperlink together. First-character # dispatch and ordinary URL fragment behavior are explicit. These observations do not resolve the three findings above or grant unrun acceptance.

## Authentication and actual limits

Capsule receipt SHA256 77131c3285681ee4e8116a069b42d4eb6c72b9fbf5cf94f2b0c79f9c49e6724d. Manifest SHA256 13a2394fb00476c69d11fab20b856ad5ee5a7e2be9a745cda239eb621b197b24. All 32 manifest artifacts were authenticated by exact size and SHA256. Source map SHA256 1b91bdaf9ecbc3f119dac6a045582721f19f5f838ec5d93808d60fa211cd8ba0 binds the copied source.

The subsequent actual-native-batch-32 receipt was independently read at terminal, SHA256 34f565fefd6dd71b59362153dcc52041f5dc7c51a485032ee9fa9b0cd75d8fb9. All three raw logs match their recorded hashes. Combined native/binding check exited 0. The heading-link control and configured body/header/cell/text-box run-picture control each passed exactly 1 test with 0 failed and 0 ignored. sourceAfterMatches and ownedLeaseSafeToRelease are true. This qualifies those finite positives only. Earlier setup/compile failures remain preserved by the worker.

Installed fresh-wheel, strict typing, full physical inventory matrix, native Word/LibreOffice output, full contract and final scoped gate acceptance remain due. Later worker source and new controls were not audited. No source, runtime or canonical record was changed. This review ends at this report.
