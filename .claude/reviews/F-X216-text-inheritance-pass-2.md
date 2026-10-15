# F-X216, text inheritance and notes flag utilities, pass 2

**Reviewed**: three frozen carriers in manifestba79b91d801c7b564823b3b182d1eb628a72fcedf48fe4f7395a171170277019 against Base5b0f63d9 and prior pass1. Exact cumulative patch7b9af4e5ae36896f827becf3739a23f919f2d07eb8e5204a8711068f5c2a1811 has458 inserted lines and zero deleted lines. Current approved plan2e7222f4 retains the same text inheritance contract.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the bounded utility review. Prior D1 is resolved.

## Defects

None found. The correction at /private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/text/paragraph.rs:6203 now copies inherited root namespace bindings only when transferring an opaque attribute. Known unqualified scalar properties transfer their original value token without adding unrelated namespace declarations. This preserves the local root's default and prefix authority, including retained local opaque children and namespace-valued local attributes.

Opaque attributes retain the preceding expanded-identity/value/context checks at :6158 and :6190, with named refusal on conflicting or unrepresentable authority. Actual inherited element children still receive their original scoped closure at :6253. The fix does not remove opaque facts, invent namespace bindings or use a foreign local-name lookalike as owned schema membership.

Character and paragraph group precedence, qualified alternative slots, schema insertion boundaries and nested defRPr/defPPr operations remain unchanged. The unchanged list helper at /private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/text/list_style.rs:247 retains concrete staged cascade and parsing. The unchanged notes getter and setter at /private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/notes_parts.rs:127 and :142 act only on the exact unqualified showMasterSp attribute. Prefixed foreign lookalikes remain retained metadata.

All inheritance methods read immutable inputs and return staged parsed candidates. The correction adds no new carrier, type, trait, dependency or global namespace policy. No additional bounded correctness, namespace, opaque retention, ordering, nonmutation or structural defect was found.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual retained evidence

Authenticated genuine RED receiptad42d876d7cab01e0a03562257f3cd7de32d108982642757d0301f8165371fd6 and its raw log. It actually collects the named text::paragraph::fx216_inherited_scalar_retains_local_opaque_namespace_authority control and fails its Unbound namespace assertion, exit101 with one failed and zero passed. Cleanup is safe. The failed predecessor remains failed.

Authenticated successor receipt22dd747746d957069d5b2685c6e979c73f3b462c99a44cde44c9c2e92009c578, three raw log hashes and safe owned cleanup. Current native check exits0. Both exact fully qualified control names appear as actually running1 test and each reports1 passed,0 failed,0 ignored:

- text::paragraph::fx216_inherited_scalar_retains_local_opaque_namespace_authority
- text::paragraph::fx216_inherited_scalars_keep_false_zero_and_harmless_aliases

The first current control proves the local opaque element remains unbound while inheriting boldtrue. The second proves explicit false and zero dominate, missing size inherits, harmless alias roots remain accepted, the unused parent default URI is not emitted and the result reparses equally. These are retained worker executions. No reviewer execution occurred.

The broader eight external XML pairs remain advisory and unexecuted. No full bullet-group, list-level, notes-flag or consumer pipeline runtime qualification is inferred from these two focused nodes. Compile0 supplies compile evidence, not full feature acceptance.

## Frozen source and boundary

Live and retained sources matched their manifest before and after review:

- crates/oxml-drawing/src/text/paragraph.rs: c17d7a472aacef09a4f60d38c4355ed3fd9cff7e43abfa7a95c0a5ae0cbb92b4
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3

Only this canonical review file was written. No source fix, build, test, binary probe, Git, CP or application operation ran. Native consumers, source-format appearance, notes materialization, archives, public surfaces and whole F216 acceptance remain separate required gates. This review ends and returns control.
