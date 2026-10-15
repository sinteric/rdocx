# F-X216, OPC import metadata query, pass 5

**Reviewed**: frozen relationship.rsf897c94554b8bb103941cb3c850cac52dee813bd6b9ff348588848a96d4d3fd0 against the immutable pass4 predecessor114c1cb9. One existing carrier differs by 41 inserted and 7 deleted lines, including its defining control. Frozen manifest9531728b9d865c5ec1a1482f22f339ada233df67ba17cb6726f71806a699c28a retains the exact source and receipt.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the bounded namespace-closure query correction.

## Prior execution credit withdrawn

The pass4 statement that the query control executed one passing test was incorrect. The retained actual-native-font-table-controls-1/opc-record-metadata-control.log:5 says running0 tests and :7 reports0 passed. Its receipt91ec19ca5ecc88b16eddbab3c3b51b38c86744a6d4aa0d1356829b84c3187ac4 had exit0 but the tests:: selector omitted the actual relationship module. That row earns no query execution credit. The predecessor report remains immutable.

The repeated query row in actual-native-font-table-debug-controls-2/opc-record-metadata-control.log:14 and :16 likewise collected and passed zero tests. Withdraw its query-control credit in the embedded-face pass2 evidence paragraph. The actual two font helper nodes and actual strict Clippy rows are unaffected. These are evidence corrections, not retrospective test successes.

All cited old log paths are under /private/tmp/rdocx-f-x216-codex/.claude/scratch/F-X216-start/.

## Defects

None found in the correction. Metadata filtering at crates/oxml-opc/src/relationship.rs:531 excludes only namespace declarations and the caller-owned unqualified assigned relationship fields. Qualified foreign attributes with the same local spelling remain metadata. The effective namespace map at :573 chains root declarations before record declarations, so record-local bindings correctly override inherited root bindings. Decoded URI values come from the already reviewed parser. Declaration placement and equivalent encoded URI spelling no longer falsely distinguish source records from their preserving copied closure.

Opaque records still require equal complete effective namespace authority, equal metadata attribute count and order, actual expanded names and exact retained lexical attribute slices at :585 through :601. Interior bytes remain exact. Foreign local shadowing cannot compare equal merely because root bindings match. No generic semantic XML comparison, namespace-value rewriting or positional record matching was added. Plain records without opaque metadata or payload retain the prior semantic fast path. Missing qualified record identities return the existing relationship error.

The query takes immutable inputs, materializes only checked local copies through rebind_for_import and performs no input publication. Existing root and record QName recognition, BOM coordinates and lexical serializer preservation remain dependencies rather than new fixes.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual evidence

Authenticated current receiptabf210754e57e94fd765b0830ba63655a8c709cbbe2993971ca497dc4894d7d6, each raw log hash and three exit0 qualified rows with safe owned cleanup. Strict OPC Clippy is actual exit0. Both corrected selectors include relationship::tests and match existing named tests at crates/oxml-opc/src/relationship.rs:1441 and :1468.

The current fx216_import_record_metadata_ignores_redundant_closure_declarations.log:5 through :8 records running1 test, the exact named node ok and1 passed,0 failed,0 ignored. The current fx216_import_record_metadata_checks_opaque_namespace_authority.log:4 through :7 independently records the same exact membership and one-test result. Both paths are under actual-opc-closure-metadata-successor-1 in the evidence root above. These are current qualified executions, not reclassified zero-collection rows.

The first control compares actual source metadata with merge_for_import copied metadata, refuses a record-local foreign namespace shadow and accepts entity-equivalent local closure. The second checks inherited decoded equivalence, foreign authority refusal, nested foreign ghost rejection and plain remapped records. The implementation is nonmutating by immutable receiver and local-copy construction. No separate executed input-byte sentinel assertion is inferred from these controls.

Genuine retained RED3 receiptcd789d27213b4c62a5772b1934eb60b6da83cf5904eae1170f897d130dab0590 and its raw log authenticate one actually collected failing test with exit101. It exposes source-versus-copy redundant closure inequality. That failed predecessor remains failed and distinct from the corrected current two-node successor.

## Not found and boundary

Correctness, decoded namespace ownership, qualified record identity, conservative opaque authority, raw lexical retention, nonmutation, regression meaning and structural rules produced no additional bounded finding. Frozen source and live relationship carrier matched f897c945 before and after review. Only this canonical review was written. No source edit, build, test, runtime probe, Git, CP or application operation ran.

Full table transfer, font pipeline, composite table abort, old release image repeat failure and F216 acceptance are outside this verdict. This pass ends and returns control.
