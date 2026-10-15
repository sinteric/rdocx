# F-X222 native pass 1

**Reviewed:** Frozen three-carrier native working diff against claim HEAD `f76bce4c654f862af26029416aa200e4182b1cc0` on `work/f-x222-codex`. The diff contains 738 insertions and 67 deletions in existing document, comments and field carriers. The document carrier includes the same existing-inline ignored benchmark used for Base and result measurements. Production result is restored.

**Verdict:** Zero defects, zero smells, zero nitpicks within this bounded native implementation review. Performance and full-feature acceptance are not decided by this pass.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Reviewed implementation

`crates/rdocx/src/document.rs:19062` obtains one operation-scoped source list, applies the same physical story identity de-duplication as existing `stories()` and retains owner/source order. Owner, control and paragraph namespace scopes are inventoried in batches. Paragraph output retains physical spans and existing ContentLocation paths before namespace-closing copies are constructed.

`crates/rdocx/src/document.rs:6585` scans each owner from its namespace-closed excerpt, preventing an empty owner from reading beyond its physical boundary. It retains the existing item/control path projection and source span sorting. The new control excerpt mapper at `crates/rdocx/src/document.rs:10758` uses checked subtraction/addition and an upper physical-source bound. Source namespace declarations remain on their original elements, so aliases and local rebindings retain their qualified ownership.

`crates/rdocx/src/document.rs:18794` shares the located source/span inventory with text-box cache projection. It builds text-box owner lists per source, selects the narrowest enclosing owner and retains physical paragraph de-duplication and established owner-relative ordinals. This avoids the old repeated source materialization per paragraph. It does not claim that every remaining enclosing-owner lookup is constant time.

The three additional existing native callers at `crates/rdocx/src/document.rs:20113`, `crates/rdocx/src/document.rs:20316` and `crates/rdocx/src/document.rs:20785` use the new single-owner batch shape while retaining their checked source/location selection. The rich comment namespace replay path also retains duplicate-path refusal and restored-source boundary mapping.

`crates/rdocx/src/comments.rs:660` extracts range pairing into a private concrete helper over the already-owned paragraph inventory. `crates/rdocx/src/field.rs:13106` reuses one inventory for pairing and name counts. A pairing failure still permits the independent name-count path at `crates/rdocx/src/field.rs:13128`, preserving established accepted fallback behavior. No new public identity, global cache or parser is introduced.

## Tests and evidence limits

The repaired existing-API controls start at `crates/rdocx/src/document.rs:34986` and `crates/rdocx/src/document.rs:35016`. The source-build counter records calls to the source-list builder, not every serialization, owner scan or namespace-parser byte. Its successful value of one is evidence for operation-scoped acquisition, not a complete proof of global linear runtime or evaluator reuse counts.

The exact old/new equivalence control at `crates/rdocx/src/document.rs:35480` compares locations, closed paragraph bytes, comment namespace replay and text-box cache coordinates across eight story kinds. Its authored source includes nested tables and controls, fields, bookmarks, comments, shared header/footer sources, namespace aliases and local declarations. It compares every batched owner span against the retained old reference. This is stronger than visible-text equality but does not replace the remaining complete native suite, invalid/foreign scope controls, saved-package equivalence or public tests.

Authenticated the genuine Base receipt and its source/log hashes. The corrected Base control executed two genuine runtime failures with exit 101. The earlier test-placement compiler failure remains unqualified and is not RED credit.

Authenticated result1 source inventory and all three row logs. Default native all-target checking exited zero, the original two controls passed and the exact old/new equivalence selector passed once. Every row retained qualified terminal results and safe owned-process cleanup. All 3,201 result-inventory file hashes match the current worktree except document.rs. Removing the single exact retained benchmark-inline block from current document.rs reproduces the saved qualified result carrier exactly. Comments and field carrier hashes match their saved qualified result byte-for-byte.

The Base release run is recorded as complete with an actual 562.51-second test runtime. This bounded pass makes no result-timing, speedup, near-linear scaling or full performance claim. The result benchmark is owned by ROOT, and its terminal review and Base/result qualification remain with ROOT. No new execution was run here. The ignored benchmark does not count as an ordinary automatically executed regression test.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no defects or smells within the reviewed native implementation. The operation-scoped inventory preserves order, de-duplication, namespace ownership, accepted range pairing and unmodelled source. No new trait, generic, public type, dependency, source module or test binary was added.

Remaining acceptance includes actual performance/output equivalence, explicit source/scan counter coverage and malformed/foreign/empty-owner save/reopen controls, affected complete native suites, Clippy/format/WASM/hash49, fresh public Python and CLI behavior, archive measurement, HLD and full-feature microscope. These obligations remain due and are not waived by this phase report. Bibliography and navigation remain out of scope.

## Review integrity

Captured HEAD, branch, physical index, logical index, tracked binary diff and all 3,201 tracked file hashes before review, immediately before writing and after writing. Complete tracked-source aggregate remained `77fc8a419df0ee6bdb830a41670a967491183a1bba1c86ab472e9fa258cf88d0`. Tracked binary diff SHA-256 remained `61044653fe05dcc7fd246500a8569444034b6ace72892f80176eb92ea1603f2b`. Physical index remained `1088315a7813d1e4a541127c5cba401de656cac4551cfade98e29e4bdc31cfc8`.

Only this review file was written. No source, plan, progress, HLD, test or formatting edits were made. No Cargo, new test, CP or Office command was run. This pass stops here. It does not hand the source lease to a writer while ROOT's result command owns it.
