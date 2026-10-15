# F-X211a, Word namespace oracle authority, pass 2

**Reviewed**: Four amended existing regression cases in frozen source123 against namespace-oracle pass1 `537b211f2d10e1695f23ffdc0893ee1ec7b7db128a7c34fe671be4c0789e9caa`. Exact capsule is `.claude/scratch/F-X211a-start/oracle-correction-review-capsule-123.json`, SHA `01a58b61196b6eb49251f5e536a0ee5a623ef2c81f3e3ec6cd472c0b49d61275`. Regression source SHA is `d4af5abd5323c97f587943487f16c7123c5b35ad8fb2b2d6320216bb3df79968`. Native source116 is unchanged and retains its separate pass5 review. This is a bounded static test-authority review, not current runtime qualification or whole-child acceptance.

**Verdict**: Zero defects, zero smells, zero nitpicks in the four bounded successor oracles. Current source123 compilation and decisive execution are pending. No final corrected-case success is credited.

## Defects

Zero remaining findings in this scope.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Successor proof authority

### Raw payload classification

`crates/rdocx/tests/regression_test.rs:15815`

The ordinary modeled run still requires empty raw and position vectors. The pict run excludes only slots marked by the existing root-attribute classifier, then requires exactly one actual payload, byte-exact pict source, position0 and the correct legacy-horizontal-rule bit. It separately requires exactly one metadata record carrying the actual v/o namespace facts. Serialization emits one pict and repeated parse/serialize is fixed. The same prefix spelling bound to a foreign VML URI must produce a false legacy bit. This distinguishes payload from private facts without accepting arbitrary added opaque children or dropping the original ordinary-run negative.

### Actual qualified comment owner

`crates/rdocx/tests/regression_test.rs:60898`

The fixture explicitly binds ancestor q to Word and locally rebinds q to foreign in its negative branch. The successor requires Body anchor AB and, only in the qualified branch, one actual TextBox at `/word/document.xml` index0 with OTHER. It preserves anchor identity, exact story snapshots and unchanged package bytes across reads and reopen. Both branches retain their original raw payload substring. This tests the original source-qualified owner instead of requiring declaration loss. Real malformed, incomplete, unrepresentable and foreign endpoint refusal coverage remains in the unchanged `comment_anchor_refuses_malformed_or_unrepresentable_without_losing_source` at line59408.

### Revision source and resolution

`crates/rdocx/tests/regression_test.rs:28909`

All three original ancestor declaration positions explicitly bind x to Word. The successor requires actual TextBox index0, text boxed and insertion31. The existing helper at line28713 verifies each revision's membership in the reported story inventory. Listing counts remain equal to actual accept/reject counts through independent saved copies at line28698. The default-namespace case remains intact. Two qualified text boxes require source-order indices0 and1 with distinct revision IDs and texts. Explicit foreign and absent x bindings must create neither TextBox nor revision authority and must preserve bytes on zero-effect resolution and reopen.

Original body, paragraph and run declarations receive original-source accept/reject save/reopen proofs. Supported modified paragraph/run scopes additionally preserve the unrelated after paragraph, actual TextBox owner and accept/reject effects through reopen. The modified-body structural edit is still explicitly unsupported at line29000: listing and accept must fail for the same named shadowed-x namespace reason. Saving must fail with that reason and retain the preexisting sentinel output bytes. This branch preserves the real guard rather than assuming that a read-only positive authorizes namespace relocation after structural modification.

The empty rejected paragraph projection at line28952 now uses the actual optional text API. `crates/rdocx/src/document.rs:12836` returns None when there is no qualifying text and no complex field. This correction does not simply erase the failed unwrap. Rejected successors require the TextBox to remain, zero revisions, empty text, and absence of both boxed bytes and the source x:ins bytes from its raw item XML at regression_test.rs28985 and29033. Positive boxed and mixed-owner text assertions remain nonempty and exact.

### Scoped mutation and snapshot identity

`crates/rdocx/tests/regression_test.rs:61338`

The original ancestor q declaration establishes a real TextBox. The successor requires its actual part/index and Version text before mutation. Count mismatch still returns expected1/found0 and preserves package bytes. An authorized zero-match operation remains a byte no-op. The valid selected operation must replace exactly one outer occurrence, preserve the second paragraph and nested TextBox Version, and preserve every unselected package part. Save/reopen must retain actual namespace declaration, ordered physical owners and target values.

`StoryId` includes a structural fingerprint at `crates/rdocx/src/document.rs:1183`. Comparing its full Body identity after changing body content would demand an unchanged snapshot. The corrected successor compares physical kind/part/index tuples through the existing accessors, requires the complete TextBox identity unchanged and explicitly requires the Body snapshot changed at regression_test.rs61360. It continues using the original unchanged TextBox identity to read its text. Reopen checks physical owners and values rather than granting stale Body snapshot identity. The existing foreign-w object branch retains its exact opaque payload assertion. Actual root-namespace atomic refusal controls remain unchanged at lines17903 and18048.

## Retained actual results

Authenticated every inspected raw log and its receipt. Source117 receipt `98e1dc68f702d51e788984c7260046f7cd140c7db2f475659b079ba644d232a4` compiled and passed the exact metadata and comment successors, then genuinely failed the count/no-op expectation. Source118 receipt `97d9edc4b1af61eaa7e6145d14619bfa2d759a3dbbe32387b14ec7f1f46ed8b2` genuinely failed the unsupported modified-body serialization assumption. Neither failure is promoted to success.

Source119 receipt `d595c5ef000d9725de20f4088337fe1f8ddb91a3e5bfd6bc30a0667674470cf2` records three exact passing unchanged controls on the retained regression artifact: rewritten root namespaces block story splices atomically, used root default namespace fails atomically, and comment anchor refuses malformed or unrepresentable source without losing it. These prove genuine negative guards alongside the amended positives.

Source121 receipt `8674313368ce35c88fd962bc1075efae107d0375f954a18356267728d581e78c` compiled, then genuinely failed the stale Body fingerprint expectation. Source122 receipt `598a701e3fbbf9ab03843e23f3fe8f048f795ac9e7e8607d0af98fda14ffba00` genuinely failed the empty text Option unwrap. Source123 makes the bounded API-consistent corrections and adds the distinguishing snapshot and raw rejection assertions described above. Its current compile is active at review time. No source123 runtime row or full-suite green is inferred.

Original Word114 regression2392 passed, four failed, seven ignored remains failed. Its actual native616 passed, zero failed,13 ignored result remains separate. Historical failures, ignored cases and current proof obligations retain their dispositions.

## Integrity and boundary

Authenticated all3,228 frozen source123 bodies by hash, size and mode against map `ffd4a7c08d98b8eefd08514cadf2320a97b847ff11e7bcab5e9586b74310c6b7`. Compared source116 and123 maps: only the existing regression carrier differs. Compared actual preserved113 regression bodies with current123: exactly the four named test functions change. No production behavior, helper, public API, new file, module or dependency change is introduced by these amendments.

Native116 behavior review remains independent. Finite source123 deciding tests, subsequent applicable gates, installed public bindings and final child/composition acceptance remain outstanding. Audit ran no test, provider, Cargo, target, CP, UI or Git action. Only this canonical review file was written after root granted its writer window. No source, index, ledger or baseline mutation occurred. Review ends and returns the writer lease.
