# F-X266, all, pass 2

**Reviewed**: Frozen working diff on `work/f-x266-grok`, Base and HEAD `64a15b2c08393353cc77e03e3761e5f77e1d11eb`. 16 tracked files, 3112 insertions and 246 deletions. Binary diff SHA-256 `12493927d39b26591149276dc46118ebc8d2e39362354d7c6ac998adf6b395bf`.
**Reviewer**: Codex root, independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 0 defects, 2 smells, 0 nitpicks

## Defects

No additional runtime correctness defect established.

## Smells

### S1, Owner construction and lookup still escape the counter

`crates/rdocx-py/src/document.rs:6995`
`crates/rdocx-py/src/document.rs:5816`
`crates/rdocx-py/src/document.rs:5884`
`crates/rdocx-py/src/document.rs:4537`
`crates/rdocx-py/src/document.rs:4631`

The original canonical-inventory, scope-sum, retirement and body-part scans now count their visits. The pre-operation denominator correction is also valid. S1 remains on other actual consumers of the same metric.

`word_owners_with_states` transforms each owner address into a WordOwner in an uncounted iterator map. The qualified path calls it both while capturing the prepared result and while committing inserted owners. This is owner-address work during prepare and commit, not an excluded debug comparison. `WordOwnerIndexes::find` also dereferences owners in its route-bucket predicate without incrementing the new counter. `word_owner_capture` reads its indexed owner record without recording that visit.

Count these actual transformation and lookup sites. Keep a single count for the same iterator yield rather than adding a duplicate marker in its caller. Add a discriminating public owner-lookup control that does not read XML, for example acquiring a paragraph handle after resetting counters. This distinguishes an uncounted lookup from the already covered held-handle XML getter. Confirm its real old-instrumentation outcome before calling it RED. Keep setup and handle acquisition outside the N XML-getter measurement.

This is the same incomplete-accounting invariant as pass 1, not a new product defect. Escalate the retained S1 to Codex root's design and review role before another broad repair. The source sites above define the bounded remaining correction. No budget factor or public stable API change is authorized.

### S2, The new binary-search data parameter has only one actual type

`crates/rdocx-py/src/document.rs:5254`
`crates/rdocx-py/src/document.rs:5515`
`crates/rdocx-py/src/document.rs:5527`

`counted_partition_point<T>` is called only with slices of `Option<rdocx::ContentLocation>`. Two predicate closures justify the callback's generic type, but not a separate arbitrary element type T. Use the concrete slice element type unless a second existing element-type instantiation is demonstrated. No new consumer should be invented to justify this parameter.

## Nitpicks

None.

## Closed portions of S1

The reviewed fixture has distinct physical body and header owners. Capturing its 33 visible owners before the operation gives a conservative lower bound for the actual owner or scope-state count. Omitted section and hyperlink owners keep the bound stricter. The inserted-owner lower bound of 1 is also conservative. Measurement resets after those queries.

The observed 546 visits are within `16 * 33 + 16 * 1 + 16 = 560`. Earlier misses against the stricter body-only bound remain recorded. The removed duplicate marker counted the same `next_owners` yield twice, while the iterator still counts each yield. That correction is not removal of the underlying visit. The formula and tuple shape did not change.

## Not found

- Correctness: no additional mutation, lifetime or serializer regression found in this remediation.
- Contract: pre-operation lower-bound proof now agrees with the approved budget. Native scope-identity iteration is disclosed separately and uses the linear identity set, rather than an owner-inventory scan.
- Panics and OOXML: no new issue found. The root-only w14 normalization and previous direct-read byte comparisons are unchanged.
- Structure: S2 is the new parameter issue. No new module, crate, trait or feature flag.
- Evidence: the checked successful build and authenticated import precede the new test. Earlier stale-module runs are now explicitly excluded.

## Evidence consumed

- `/private/tmp/s95.4-fx266-counter-remediation-ready.txt` and `/tmp/fx266-s1-raw.txt` retain 4 and 16 direct-read visits, 1627 and 6508 emitted bytes, whole-body work `(0, 0)`, and the 546-visit mutation observation.
- Successful dev extension SHA-256 `4bb60b704de01660be40b4fb698198d75d0666717de98df8088e937eebccb9fe`. Python 3.12.13, feature extension-module. `/tmp/fx266-s1-pytest3.txt` reports the selected counter test passing.
- Formatting and binding all-target/all-feature clippy pass. Previous unaffected native and Python observations retain their stated applicability, without being rerun or relabelled.
- Root performed source review only in this pass and changed no implementation files.

## Acceptance boundary

Preparation is still blocked by S1 and S2. Final release timing, full verification, hash, installed suite, WASM and archive qualification remain pending. This pass ends here and returns control to the orchestrator without remediation.
