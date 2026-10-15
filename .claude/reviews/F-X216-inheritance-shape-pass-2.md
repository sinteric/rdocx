# F-X216, inheritance and shape utilities, pass 2

**Reviewed**: the four frozen utility carriers in manifestd0100b2ed7a67afede26f3bffcce6fb5a9b495d5d8902dab718c501f0928c1f1. Prior shape source21ca64fe to current354df893 changes157 inserted and6 deleted lines, including the two defining controls. Other three utility carriers remain byte-identical to pass1. Exact cumulative Base patchf4efd6cfd9effc8361331abb39469feb93464e8dcdddd9e51bd56f2b9df81317 is authenticated. Native consumers are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior D1 and D2 are resolved. New D3 remains.

## Prior findings

D1 is resolved at /private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:209 and :609. Qualified raw grpFill now occupies the fill choice before defaults.fill is copied. The same slot rejects a foreign lookalike as a modeled choice. D2 is resolved at :140 through :150 and :558. Opaque root attributes compare actual expanded identities, including the fixed XML namespace, while complete effective contexts remain required for opaque value authority. Equal-value aliases reuse one local attribute and unequal opaque aliases refuse.

## Defects

### D3, identical raw bytes collapse opaque children with different namespace authority

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:243` and `:248`

The raw-child inheritance branch skips an unknown parent child if any local raw child has equal bytes. It performs that deduplication before closing the inherited child in its original physical namespace scope. Equal lexical names are not equal producer XML when their inherited bindings differ.

Both individually valid property roots use the actual DrawingML namespace:

```xml
<!-- Local -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:local"><z:payload/></a:spPr>
<!-- Defaults -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source"><z:payload/></a:spPr>
```

Neither root has an opaque non-namespace attribute, so the earlier root metadata conflict checks do not refuse. Neither child is a modeled schema-slot member. The byte-equal <z:payload/> triggers :243 and discards the inherited child before :248 can preserve z=urn:source. The returned object contains only the local {urn:local}payload. The distinct source {urn:source}payload is lost without a named error. The narrow transform/fill/line reparse comparison does not detect that loss.

Require source-qualified opaque equality before deduplication, including inherited and local scope authority needed by unknown QName or prefix-valued payload. Alternatively retain the inherited child with its actual source closure or return the approved named atomic refusal when the merge is ambiguous. Do not assume equal expanded element names alone proves equality for opaque values. A defining control should distinguish equal raw bytes under different bindings, equal effective bindings with different declaration placement, and unknown namespace-valued content while preserving both input byte sentinels. This is a static witness, not reviewer runtime evidence.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual evidence

Authenticated genuine RED receiptae8446bc1d150746f4eb05f0e95e3cc58372ff0fedceccc18439300a68b2dcb3 and both raw logs. Each exact shape_props node actually collects1 test and exits101 with1 failed,0 passed,0 ignored. Both failed rows have safe owned cleanup and remain unqualified.

Authenticated current successorb442a54df05c576d49caa38a1219287c3ae1ef0644be4b1918d4f2d5551bfb2b and all three raw log hashes. Both exact fully qualified nodes actually run1 test and each reports1 passed,0 failed,0 ignored, exit0 and qualified safe cleanup:

- shape_props::fx216_local_group_fill_blocks_inherited_fill_choice
- shape_props::fx216_opaque_attribute_aliases_reuse_or_refuse_expanded_identity

Strict current oxml-drawing and rpptx-oxml all-target/all-feature Clippy also exits0 with qualified safe cleanup. The first control covers local grpFill, the converse typed fill, foreign lookalike retention, reparse and unchanged inputs. The second covers actual expanded attribute count, equal aliases and unequal-value atomic refusal. Neither exercises D3's equal-byte unknown children under different inherited bindings.

Earlier successor1's Clippy replace_box failure is retained and uncredited. Its consumer correction is outside this utility scope. The earlier ten-control receipt remains separately qualified against its own source and retained binary. No whole-current ten-control rerun or consumer acceptance is inferred. No runtime command was executed by this reviewer.

## Frozen source and boundary

Live and retained sources matched before and after review:

- crates/oxml-drawing/src/text/paragraph.rs: c17d7a472aacef09a4f60d38c4355ed3fd9cff7e43abfa7a95c0a5ae0cbb92b4
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/oxml-drawing/src/shape_props.rs: 354df8937602b481e44a1aa7f1762c75767a360dce305b9eb352ccd8235ac2b9
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3

No additional bounded defect was established in sparse transform precedence, schema-slot choice, expanded attribute refusal, unchanged text scalar behavior or exact notes showMasterSp handling. Only this canonical review file was written. No source fix, test, build, binary probe, Git, CP or application operation ran. Full native imports, resource retargeting, appearance, archives, public controls and F216 completion remain outside the verdict. Review ends and returns control for distinct remediation.
