# F-X216, inheritance and shape utilities, pass 3

**Reviewed**: four frozen utility carriers in manifestce11d6b3d0f2a48cfda9663cdffe4fa569c1c1f49a3ef285fb1526b9741a3ceb. Shape predecessor354df893 to current3c65ba69 changes52 inserted and1 deleted line, including the defining D3 control. Exact cumulative patchbe62f4aba29505b22e351957a99f8db5128a0f5eaabeb15d2178cd2f70c3e240 is authenticated. Native consumers remain excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior D1, D2 and D3 are resolved. New D4 remains.

## Prior findings

D3 is resolved at /private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:243. Unknown raw children are deduplicated only when the complete effective owner namespace contexts compare equally. A different source context keeps its child and closes it through the existing transfer helper at :252. Equal byte names and unknown namespace-valued attributes no longer justify collapsing different authority. Prior qualified group-fill and expanded-attribute collision corrections remain intact.

## Defects

### D4, inherited typed fill loses the namespace context of retained opaque children

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:215`, `/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/fill.rs:220` and `:1371`

The method closes inherited raw children of spPr, but copies an inherited typed fill directly by Clone. A typed SolidFill itself retains unmodelled XML children in its own raw_children. Those bytes can depend on the original spPr namespace declarations. SolidFill does not store that enclosing scope, and its writer emits a bare a:solidFill plus retained raw bytes. Copying it under the destination local root changes their namespace authority.

Concrete individually valid qualified owners:

```xml
<!-- Local -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:local"/>
<!-- Defaults -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source"><a:solidFill><a:srgbClr val="FF0000"/><z:payload meaning="z:item"/></a:solidFill></a:spPr>
```

The parent fill parser at fill.rs:1348 through :1373 retains <z:payload meaning="z:item"/> after the modeled colour. Inheritance at shape_props.rs:215 clones that typed fill into the local candidate. The local root retains z=urn:local, and the fill writer at fill.rs:214 through :220 adds no source closure. The imported payload and its opaque prefix-valued meaning now carry urn:local instead of urn:source. It is not in the spPr raw-child loop, so D3's corrected closure does not protect it. The reparse equality check compares the same typed colour and raw lexical child bytes, not their effective namespace authority, and does not refuse.

Retain original scope authority for opaque descendants of the inherited modeled group, or use the approved named atomic refusal when the existing representation cannot preserve it. Review the actual analogous typed-group transfer paths rather than treating an owned typed root as proof that all its descendants are modeled. Do not add blanket root namespace bindings that would rebind local siblings. A defining control should inspect both original and resulting expanded payload names and opaque namespace-valued authority, exact non-namespace bytes, save/reparse and unchanged input sentinels. This finding is source-backed static analysis, not reviewer execution.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual evidence

Authenticated genuine D3 RED receipta43012eea463f01ae491779657e2eb2ce93ad60a6a0d5f7bee3a2cedf8431b76 and its raw log. The exact shape_props::fx216_opaque_children_with_equal_bytes_keep_distinct_namespace_authority node actually runs1 test and exits101 with1 failed,0 passed,0 ignored. Its observed namespace set contains only urn:local. Safe owned cleanup is authenticated, and the failed predecessor remains unqualified.

Authenticated current successorec9aaaaac4688611b7c7a79330925a4806b5477e3a2ea95027c01ce6c91cd298 and all four raw log hashes. Three exact shape_props nodes each actually run1 test, report1 passed,0 failed,0 ignored and exit0 with qualified safe cleanup:

- fx216_opaque_children_with_equal_bytes_keep_distinct_namespace_authority
- fx216_local_group_fill_blocks_inherited_fill_choice
- fx216_opaque_attribute_aliases_reuse_or_refuse_expanded_identity

Strict two-crate all-target/all-feature Clippy also exits0 with qualified safe cleanup. The D3 successor uses an unknown prefix-valued meaning attribute, asserts both source and local namespace identities survive, checks self-inheritance deduplication, unchanged inputs and serialization fixed point. These controls do not inspect opaque children retained inside an inherited typed fill and therefore do not close D4.

Prior ten utility controls remain separately qualified at their actual sealed source and runtime environment. They are not reclassified as a current whole-feature gate. No build, test or native probe ran during this review.

## Frozen source and boundary

Live and retained utility carriers matched their manifest hashes before and after review:

- crates/oxml-drawing/src/text/paragraph.rs: c17d7a472aacef09a4f60d38c4355ed3fd9cff7e43abfa7a95c0a5ae0cbb92b4
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/oxml-drawing/src/shape_props.rs: 3c65ba69d3a2d4aec5a2340035614d737bbd5cda3ae6cb7e672944d6c1022e48
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3

fill.rs is cited as the existing concrete parser/writer dependency, not as a newly modified carrier or independently qualified pipeline. No additional bounded finding was established in raw context comparison, group choice, attribute aliases, sparse transform precedence, unchanged text methods or exact notes flag. Only this canonical review file was written. No source fix, runtime, build, Git, CP or application operation ran. Whole import graphs, resource retargeting, notes appearance, archives, public surfaces and F216 completion remain outside the verdict. Review ends and returns control for distinct remediation.
