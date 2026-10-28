# F-X268, positive namespace spans, pass 1

**Reviewed**: Working mechanism 7 on parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. Surrounding diff: 4 source and test files, 5371 additions and 315 deletions. This bounded source pass examines positive namespace spans and shared replay policy in `crates/rdocx/src/document.rs`. Compilation and focused executions of this step have not yet run.
**Reviewer**: Codex root. Grok implements and transcribes the report.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, the span shortcut still scans the whole current part

`crates/rdocx/src/document.rs:9025`

Every call to `proved_namespace_spans` invokes `closed_word_body` on the whole main part before using the frame or child caches. That helper scans story owners and fingerprints their XML. A warm replay therefore still reads all unrelated body XML, including when every piece span is already cached. This whole-part parser input is recorded only in the separate closed-body counter. The approved staging scanned slot does not expose this growing work.

The positive owner and replay path must compose a proved frame and closed child parser-state certificates, or retain an honestly counted whole fallback when composition is unavailable. Exact current bytes, context and partition checks remain necessary. Removing the guard without replacing its invariant is not a repair. The old parser state around a Word body inside opaque XML must still invalidate unsafe composition.

A discriminating warm N and 4N test must inspect guard and partial namespace inputs as well as the six staging slots. It must fail while this unconditional scan remains. Cache-off evidence and the original whole policy remain the reference. Byte comparisons and metadata composition are separate disclosed costs.

## Smells

None.

## Nitpicks

None.

## Not found

No additional source finding was established in correctness, panics, namespace preservation or structure within the inspected change. The shared old owner and replay policy remains authoritative. Frame spans retain background and tail candidates, and the child parser-state check is deliberately stricter than editable story-owner discovery. Compilation, new focused tests, the canonical pairing diagnostic, Python gate repairs and full feature acceptance remain pending. No current Python pass is claimed for this source.
