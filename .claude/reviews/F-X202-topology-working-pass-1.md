# F-X202, table topology slice, pass 1

**Reviewed**: Immutable topology-capsule-r1, seven existing complete before/after carriers and owned diff. Scope is empty append, logical columns, full merge split, Python table style/look and physical nested-table routes. Approved F202 plan, resolved table contract and its packaging, binding, table-layout and risk references apply. Claim Base is 30775a483ae73c06e448dd594b002dbb4ea2881d. All 29 manifest records authenticated. Pictures, hyperlinks, paired edits and other ongoing implementation are excluded.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, column insertion loses the referenced column before choosing width, formatting and omission policy

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/topology-capsule-r1/source/crates/rdocx/src/document.rs:21476`

The public operation converts the caller's existing logical column index into an insertion boundary, then discards whether the reference was the column before or after that boundary. Table insertion chooses width from boundary minus one at `source/crates/rdocx/src/table.rs:1000`, and cell formatting from the preceding physical cell at `:1041`. This violates the approved rule that width and formatting come from the referenced column covering the caller's index.

For a two-column grid with widths1000/2000 and different cell shading, insert_table_column(0,1,true) must copy column1's 2000 width and formatting into the new independent cell before it. The current code inserts width1000 and copies column0's formatting. The same mismatch exists in the related-table Python route at `source/crates/rdocx-py/src/table.rs:986`.

Omission policy also uses only the boundary at `source/crates/rdocx/src/table.rs:1011`. With gridBefore1, reference column0 is omitted. Inserting after column0 gives boundary1 equal to the first physical cell, so current code inserts a physical cell instead of extending gridBefore. Conversely, inserting before a trailing omitted reference at the last physical boundary inserts a cell instead of extending gridAfter. This changes the row's intended logical occupancy. The contract explicitly makes omission behavior depend on the referenced column, not only the insertion point, in F-X202-design.md:357 through :364.

Retain referenced logical neighbor information through the existing edit implementation. Add unequal-width and unequal-format before/after controls, plus insertion after the last leading omission and before the first trailing omission. Require exact content, spans, width policy and producer preservation and unchanged candidate on refusal. The worker confirmed there is no later approved contract exception. Current equal-width/edge controls do not decide these cases.

## Smells

None.

## Nitpicks

None.

## Other bounded review results

The existing row_cell_ranges and topology validator enforce positive active-grid widths, exact row coverage including omissions and matching vertical continuation ranges. Mutations clone a candidate and validate before assignment. Deletion narrows spans and omitted ranges, refuses the final column/last physical cell, retains opaque row boundary positions and uses checked fixed-width arithmetic. Complete vertical split maps a continuation to its restart and contiguous equal logical range, keeps origin content once and refuses distinct continuation content or unsupported producer ownership. Empty append retains last-row occupancy and formatting but creates empty paragraphs and clears merge/content identities. Opaque formatting refuses before cloning a new owner. The native operation stages, reconciles removed comments/bookmark partners where required, prepares and reopens, then publishes once.

The bookmark scanner uses existing qualified modeled-story and accepted-visibility providers. Opaque producer wrappers and foreign markers stay outside cleanup. Main-table inventory/provider functions remain unchanged, and the direct table after a control-owned table is tested against physical ownership. Nested tables use refreshed checked ContentLocation routes instead of guessing an ordinal from a top-level body index. The direct body-content API rejects a nested related handle.

Python strict table styles resolve an actual table-style ID or unambiguous name and refuse undefined/wrong-type values. Look setters preserve other flags and invert through the existing native TableLook mapping. Topology operations stage native work before scope publication, add_row allocates the returned Row under a prepared successor scope, and existing rows/cells become stale on structural publication. Unmerged split returns without bytes or revision publication. The stub shapes match these added surfaces. This inspection does not credit installed topology or stale-handle acceptance that has not run on the successor wheel.

## Not found

Correctness and contract: D1 above, no additional findings. Panics: 0 findings in the bounded delta. OOXML: 0 additional findings. Tests: 0 additional findings beyond the missing deciding D1 cases. Structure: 0 findings. No new product file, module, trait or speculative abstraction is introduced.

## Evidence and limits

- Capsule receipt: a5e14c6bcab033d2daaa951c00f04241eeeab5bd89f748e301da955e539325c5.
- Manifest: 7cd1a1a91cc95091d34006c8f8cc75ac9df0573ebb63915ee56fd254c581aceb.
- Owned diff: c001e096bd4e1322e343113222640ea7b83f6f9d0810f5f95bed1eff8e2f83b3.
- Current source3148 map: 83cd954ccd1ec60290958adbf6981b34b8618bb22f9d3cf544a3ef526cb14eb4.
- Claim index: 6d68e3492f4a0020077140f47574953b37e8dddaf29248263a37123071e586a7.
- Preservation11 receipt: f47a11e8ee80ac30f79ecdf06dc9d923842f0beb1c2d96e8620c7152754cfb8b.
- Preservation12 receipt: ab6f584f86fefe1fb82efa9a0d484bc2792e3b29896a8af66daa7b5c6463a260.

Actual preservation11 native/binding all-target check exited0. Its topology batch recorded7PASS1FAIL0IGNORE. The failed opaque-copy positive used a boundary that requires creating a cell and correctly refused cloning opaque formatting. Preservation12 changed only the regression carrier to an interior span insertion, where no new opaque owner is cloned. The exact successor recorded1PASS0FAIL0IGNORE and scoped native/binding Clippy exited0. Independent source-map comparison confirms this sole test-carrier delta, so the seven unaffected prior controls plus the exact successor compose eight controls, not a freshly rerun eight-control GREEN suite. All recorded source-after guards matched and owned groups naturally ended with reaped leaders, ESRCH and no signals. Earlier routing and invalid setup results remain retained, not acceptance credit.

No runtime, Cargo, CP or native Office actions were performed by the reviewer. No source or test changes were made. Genuine native row evidence informs the approved contract, but current native Word reopen/render, installed topology/typing, full feature, HLD reconciliation and remaining risk gates are still due. Paired Base failure in the complete regression copy is expressly outside this slice. Review ends at this report.
