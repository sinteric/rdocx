# F-X203, boundary correction, pass 2

**Reviewed**: Independent bounded remediation review of pass1 D1 through D3. Four immutable native carriers and the incremental patch in `immutable-boundary-correction-2`. Later text-report, validation and other moving F203 implementation are excluded.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Pass1 D2 and D3 resolve. Pass1 D1 resolves for top-level owners but remains for nested accepted owners.

## Source identity

Root input is `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-boundary-correction-2`. Receipt SHA `3126fead191c58857f61d3f6a1e3a75b04ed9488a973a9a5ef4eb4f4a3267403` authenticates all15 artifacts with zero mismatch. The four before copies equal pass1 source exactly. Incremental diff SHA is `a9217f34b7f8d9ebd936ff0ea2f3ad6c554cc8dfef52bb651888b1b4a0dfef27`. Full3111-record map SHA is `2d02cebaa3491e923449e6d6bc17bc21c3b0b5120140f8771e521d37e65a4936`. Recorded HEAD/index in the actual receipt remain `dd7c627b534301c253615d30e35e8748b455a4d0` and `c203016ed951117303df8410bfd0993697878991ea37fce9a229fc1b0770f17d`.

All findings cite immutable `after/` files within this input. The approved Issue332 contract still requires boundaries at accepted-view structural owners.

## Defects

### D1, nested accepted owners still lose their boundaries

`after/crates/rdocx-oxml/src/text.rs:6271`
`after/crates/rdocx-oxml/src/text.rs:6300`
`after/crates/rdocx-oxml/src/text.rs:5884`
`after/crates/rdocx-oxml/src/text.rs:6045`

The correction marks the start and end of the complete top-level `owned` vector. That vector is still produced by helpers that recursively flatten nested accepted controls, insertions and moveTo owners without carrying their internal boundary indices. An accepted outer control containing bold literal `A`, an accepted nested control containing bold `B`, then bold literal `C` therefore supplies one contiguous equal-format run sequence between the outer boundaries. Markdown still emits a single `**ABC**` span across the nested owner. A nested accepted owner within an insertion has the same loss through `accepted_paragraph_runs`.

Transport actual entering/leaving indices through nested accepted-owner traversal in the explicit Markdown projection, without changing default accepted-view consumers or inserting fake source runs. Add deciding nested-control and accepted-revision combinations with equal marks on both sides, source equality and default-view equivalence. The new top-level control/insertion case proves that top-level path, not this remaining nested path.

## Resolved findings

- **Pass1 D2**: `after/crates/rdocx-html/src/markdown.rs:424` flushes marked cached text and inserts explicit boundary sentinels around line and column breaks, matching the direct-run behavior. The new simple-field case asserts three independent bold spans with original hard-break syntax and exact serialized source.
- **Pass1 D3**: `after/crates/rdocx-html/src/markdown.rs:151` tests true emptiness rather than trimmed emptiness. Literal whitespace-only ordinary paragraphs now reach the original-text output branch. The new case retains spaces and tab outside emphasis. Its genuine old-production RED is authenticated separately.
- **Top-level portion of pass1 D1**: Actual indices are returned separately from the typed paragraph, offset through accepted paragraph joins and consumed by Markdown before coalescing. Wrapper recursion transports its returned indices. No fake run or source XML mutation is introduced.

## Smells

None.

## Nitpicks

None.

## Six aspects and preserved consumers

- **Correctness**: The remaining D1 above. No additional finding in cached hard-break or literal whitespace corrections, join index offsets or link-map offsets.
- **Contract**: Nested accepted structural boundaries remain uncovered. D2 and D3 now meet their bounded requirements. The constructor is unchanged and not re-reviewed.
- **Panics**: Zero additional findings. Boundary values originate from the actual projected run counts and are used as membership facts rather than unchecked indexes.
- **OOXML**: Zero findings. Source models/raw XML are not edited by this projection. Default `accepted_view` takes the paragraph component of the same false projection. That mode produces no boundary facts and retains the previous accepted selection/flattening. HTML chooses the false mode and ignores the new private block metadata. Accepted joins retain identical run/hyperlink behavior in that mode.
- **Tests**: The real whitespace RED and full14-test GREEN are authentic for the cases executed. Nested control/revision combinations are absent from the deciding correction case and remain required for D1. No new or broad runtime claim is made.
- **Structure**: Zero findings. The tuple exposes genuine projection facts to the current Markdown consumer in existing carriers. No new module, trait, dependency, registry or forwarding framework is introduced.

## Actual evidence and limits

Copied RED receipt SHA `b51f6ae0bb5ae67cb0ba0e6219d7802413cdba103a3f1bbca92c08e685c71005` records the exact existing selector failing0 passes/1 failure/0 ignored on old-production whitespace omission, with actual left newline and expected original spaces/tab. This loop stops at that first case, so it is not separate runtime RED proof for every later matrix item.

Copied GREEN receipt SHA `22b3c0cb2b6078193990643a8a63ffea3ba0d5b20e65452af3f01684edc45c6a` records HTML14 passes, zero failures and zero ignored under map `2d02ceba`. Source-after matches. Both owned process groups are naturally reaped with ESRCH and no signals. Neither result tests the missing nested-owner case.

No new runtime, source edits, installed-wheel, native Word, full-feature or output-baseline claims are made. This review ends with this report.
