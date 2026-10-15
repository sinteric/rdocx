# F-X216, inheritance and shape utilities, pass 4

**Reviewed**: four utility carriers in frozen manifest7e0c52ace5141e7896372d3cc4e0a71f1e2e4af1614f63058be69c6736794b57, retained Base copies and exact Base diffd849f9e0a5c040e139eee102102d97b38de1648cb8dea8453e4478a85fb84e02. Shape predecessor3c65ba69 to current4af332fe changes253 inserted and1 deleted line, including controls. The other three utilities remain unchanged. Native consumers are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior D1 through D4 are resolved. New D5 remains.

## Prior findings

D4's namespace authority loss is resolved by inheritance-only source processing at /private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:276 and :677. The original source is retained alongside its ordinary projection. Unchanged models use physical source for scope closure. Changed models use their current projection when original nested declaration authority is absent, and return a named error for the ambiguous edited nested-declaration case. Closure follows actual ancestor bindings, qualified direct modeled groups and their descendants, including absent default namespace as empty. It keeps local siblings outside the transferred group scope.

Source and projection fields are excluded from existing PartialEq and ordinary writer behavior. Legitimate offset, preset and line-width edits remain represented by current modeled fields. Prior raw group-fill choice, expanded root attribute identity and raw-child full-context deduplication corrections remain intact.

## Defects

### D5, typed-group reparsing still discards opaque root metadata

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:313`, `:314` and `/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/fill.rs:200`, `:214`

The source-closure helper adds namespace declarations to original typed group elements, then reparses them through existing typed parsers. Some typed parsers do not retain opaque attributes of their own root. SolidFill::from_element retains colour and raw children but never captures start attributes. Its writer emits a bare a:solidFill. Therefore original source preservation alone does not establish opaque metadata retention after the utility reparses and clones the group.

Use identical physical root scopes for both local and defaults, with a bound producer extension explicitly marked ignorable:

```xml
<!-- Local -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:producer" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="z"/>
<!-- Defaults -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:producer" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="z"><a:solidFill z:meaning="z:item"><a:srgbClr val="FF0000"/></a:solidFill></a:spPr>
```

These actual root contexts and MC metadata match, so the shape root metadata checks permit the merge. Original source includes the producer attribute z:meaning on the inherited solidFill. The unchanged source branch closes it and reparses at :314. The concrete SolidFill parser discards z:meaning, and the returned inherited typed fill serializes without that attribute. No error is raised. The final typed reparse equality has no field for the discarded attribute, so cannot detect it. This witness concerns legitimate ignorable producer metadata, not an invalid xsi schema type.

The inherited utility must preserve such actual source metadata or return the approved named atomic refusal when that modeled representation cannot retain it. This can remain an inheritance admission boundary without changing ordinary parser/writer behavior. Check the analogous actual modeled groups for unsupported opaque root metadata before claiming complete source retention. Do not hide legitimate later modeled edits behind unchanged raw group replay. Add a defining source attribute/value/context assertion and both input byte sentinels, with a real modeled edit where the supported representation permits it. This is a source-backed static witness, not reviewer execution.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual evidence

Authenticated all pinned receipts and raw log hashes. Genuine REDc031b0ef70fcc4f3b578a215e7045f9a0365cc00027da3b695e84fa80dcace49 actually runs1 named typed-fill test and exits101 with1 failed,0 passed,0 ignored and safe cleanup. It remains unqualified.

Current successor7adb8d8196bb7ec61f5e10bf6bed64b6871dc64855a5689cc466bf4e6508a4c7 has five exact named tests, each actually running1 test, named ok,1 passed,0 failed,0 ignored, exit0 and qualified safe owned cleanup:

- shape_props::fx216_typed_inherited_fill_preserves_source_opaque_namespace
- shape_props::fx216_typed_groups_close_actual_ancestor_scopes_and_preserve_edits
- shape_props::fx216_local_group_fill_blocks_inherited_fill_choice
- shape_props::fx216_opaque_attribute_aliases_reuse_or_refuse_expanded_identity
- shape_props::fx216_opaque_children_with_equal_bytes_keep_distinct_namespace_authority

The second named test contains eight distinct group/context cases. It obtains actual inherited bindings from outside-spPr p:sp ownership, checks local and inherited opaque namespace separation, default namespace shadowing and legitimate offset, preset and line-width edits after parsing. It also proves edited nested declaration authority refuses without changing the input. Those positives cover opaque children and the line root attribute retained by its existing model. They do not assert preservation or refusal for a SolidFill root attribute its parser discards.

Strict scoped Clippy receipt1ed11e57cf052b4e5fd1d98f0c3d59cf5d22d334e0ec7074dd4db04070a0af46 is actual exit0, qualified and safe. Initial Clippy receiptf1a41df4 and its failed row remain unqualified. Intermediate positive receipts stay separately scoped. The preserved current unit binary matches6dd3b9feb361e02872a71d8231fbe86f22079abf00350a3f9d6d2a4ac5b63750. Runtime is the documented8MiB native test environment and default debug profile, not default2MiB qualification. No command was rerun by the reviewer.

## Frozen source and boundary

Before and after review, retained and live utilities matched:

- crates/oxml-drawing/src/shape_props.rs: 4af332fec0c7e430fbb6c273301217a7aae49bf7baf46a92d2c1c08f17b9746d
- crates/oxml-drawing/src/text/paragraph.rs: c17d7a472aacef09a4f60d38c4355ed3fd9cff7e43abfa7a95c0a5ae0cbb92b4
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3

Only this canonical review file was written. No source fix, runtime probe, build, test, Git, CP or application operation ran. No further bounded finding was established in the inspected scope closure, transform precedence, equality bookkeeping or unchanged text/notes utilities. Full native consumers, resource retargeting, appearance, archives, public surfaces and F216 completion remain separate required acceptance. Review ends and returns control for distinct remediation.
