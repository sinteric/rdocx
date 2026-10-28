# F-X268, canonical frame relationships, pass 3

**Reviewed**: Working mechanism 7 on parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. Surrounding diff: 4 source and test files, 5961 additions and 210 deletions. This bounded pass reviews frame relationship occurrence order and current-piece cache lifetime, including D1 and D2.
**Reviewer**: Codex root. Grok implements and transcribes the report.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D1 and D2 are closed in this scope.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, namespace preservation, tests and structure produced no remaining finding in the inspected repair.

Frame head ids, child occurrences and tail ids compose in physical first-seen order. Child collection uses its physical event range, while all wrapper attributes are still validated. A body and tail duplicate is collected at its first actual occurrence. The old full scanner remains the reference and malformed outside-range attributes retain their errors.

The serializer compares the finalized head and tail, including conditional w14. A changed frame produces current piece records with fresh parsed, identifier, canonical and namespace locks, including previously failed facts. Old shared records are untouched. Unchanged-frame publications retain the current records. Resetting a cache does not record a child write. The staged model remains a hint, not a parsed proof. Root mismatch and field retry tests remain active.

The earlier six-test run actually failed 5 to 1, and the separate context diagnostic failed after recording both sizes. These are retained as D2 RED evidence. The corrected batches in `/tmp/fx268-m7-frame-context-test.log` passed 6 tests in 60.36 seconds and 3 reset and retry tests in 0.36 seconds. Both header/footer rows now have staging `[0, 0, 0, 0, 21, 7]`. Kept, fresh and whole relationship vectors agree.

The current debug extension was built successfully and its actual import was recorded in `/tmp/fx268-m7-frame-context-python.log`. The two selected public cases, Table.add_row and Cell.text, passed in 124.42 seconds on Python 3.12.13. The loaded and independently checked archived artifact is `6666e70231748f82614c8aea483373b91cce670c6acbc0b597bf33519f783d06`, at `/private/tmp/s95.4-fx268-m7-canonical-frame-_rdocx.abi3.so`.

The approved six counters are not a total runtime claim. The header/footer diagnostic still records partial canonical-proof input of 2189172 bytes at 50 paragraphs and 7454172 at 200. That growing cost remains visible for the mandatory final timing measurements and any resulting repair. This pass does not establish the 79 flat gates or universal constant time.

The other seven public paths, retained-source block integration, mechanism 8, A3 and A5 composition, release execution, memory, HLD, full feature microscope and final sprint verification remain pending. No complete F-ID or sprint pass is supplied by this bounded review.
