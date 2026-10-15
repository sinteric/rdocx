# F-X216, inheritance and shape utilities, pass 1

**Reviewed**: four production utility carriers and the ten exact controls in immutable manifestaa6a7bee2a711fff8625eef7da1917acbdc713fa42563a8c2fbb0d342b1eaa1d against Base5b0f63d9. Production utility delta is656 inserted and2 deleted lines. Exact sealed patchdc2de84bbff91bae2c0e9b330e65f3c01b8daa12d49c44a1053b9e78dca41fd0 is authenticated. Evolving native consumers are excluded.
**Verdict**: 2 defects, 0 smells, 0 nitpicks in the bounded utilities.

## Defects

### D1, a local group fill does not block inherited fill

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:179`, `:490` and `:545`

A qualified a:grpFill is a member of the shape fill choice. The existing parser retains it as raw source, and raw_boundary_after explicitly recognizes its fill boundary at :522. The new inheritance method copies defaults.fill whenever local typed fill is None, before computing raw occupancy. Its new slot recognizer uses is_fill, which omits grpFill. A valid local group fill therefore does not occupy the fill choice during inheritance.

Concrete witness with the standard DrawingML declaration on both roots:

```xml
<!-- Local -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:grpFill/></a:spPr>
<!-- Defaults -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></a:spPr>
```

The returned model has both the retained local raw grpFill and inherited typed solidFill. The writer emits raw boundary2 before the typed fill, so the package contains both members of a single schema choice. The final permissive reparse comparison does not detect it because grpFill remains raw and the solidFill typed value still matches. This violates local precedence and schema cardinality.

Recognize actual qualified raw group-fill occupancy before inheriting a typed fill, without treating a foreign same-local-name element as a choice member. Add local-group-fill versus inherited modeled-fill and converse controls that assert exactly one owned choice, raw local byte retention, save/reparse and unchanged inputs. No runtime reproduction was performed by the reviewer.

### D2, opaque root attribute aliases can create duplicate expanded names

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/shape_props.rs:140` and `:157`

The shape root attribute merge searches only for the same lexical key. It compares complete source namespace vectors for opaque attributes, but does not resolve their expanded identities. Equal namespace contexts can contain two aliases for the same URI. An inherited attribute using one alias is then appended beside the existing local attribute using the other alias, even though both attributes have the same expanded name.

Concrete individually valid inputs share the same declarations and differ only in the opaque attribute spelling:

```xml
<!-- Local -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:producer" xmlns:y="urn:producer" y:keep="local"/>
<!-- Defaults -->
<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:producer" xmlns:y="urn:producer" x:keep="parent"/>
```

The namespace contexts match, so the new method appends x:keep to the root already holding y:keep. The serialized result contains two attributes named {urn:producer}keep, which violates XML namespace attribute uniqueness. Existing raw attribute capture and the current reparse check do not reject that expanded-name collision. Equal attribute values would still produce an invalid duplicate.

Use actual expanded attribute identity for local precedence and opaque collision checks, preserving QName-valued context and applying the approved named atomic refusal when authority conflicts. Do not rewrite arbitrary opaque values or drop namespace facts. Add equal-binding alias reuse and unequal-value alias refusal with both input byte sentinels and exact expanded-name uniqueness assertions.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Other assessment

The prior text scalar namespace leak remains resolved in the unchanged c17d7a47 paragraph utility. The eight current text controls are now executed evidence rather than advisory only. Concrete known-property groups, false/zero precedence, bullet groups, nested defaults, list aliases, foreign fill lookalikes and opaque scope closure/refusal are exercised. No additional bounded text or notes utility finding was established.

Shape coordinate inheritance copies only absent optional coordinate fields from defaults. A present local transform retains its current rotation and flip values, including explicit zero and false, as the approved source/native precedence requires. Inherited raw element children receive source namespace closure. Results are staged from immutable inputs. The tested opaque same-key conflict refuses without input mutation. Those valid controls do not cover the two defects above.

## Actual evidence

Authenticated receipt5606d6603088cd36abf6f9458ccf23cf229d0ae395f14497fd4c19a38cb85906 and all22 raw log hashes. It records current check, integration compilation, ten exact membership checks and ten selected runtime nodes. All rows exit0, qualified, groupGone, leaderReaped and leaseReleaseSafe. Each runtime log, distinct from its membership listing, contains running1 test, the actual exact named node ok and1 passed,0 failed,0 ignored.

The ten actual nodes are:

- fx216_text_inheritance_char_explicit_false_zero
- fx216_text_inheritance_paragraph_explicit_zero
- fx216_text_inheritance_bullet_four_groups
- fx216_text_inheritance_nested_run_defaults
- fx216_text_inheritance_list_alias_level
- fx216_text_inheritance_foreign_fill_lookalike
- fx216_text_inheritance_opaque_mc_conflict
- fx216_text_inheritance_opaque_local_default_shadow
- fx216_shape_defaults_keep_sparse_geometry_false_and_opaque_scopes
- fx216_shape_defaults_refuse_conflicting_opaque_owner_context_without_mutation

The actual debug binary matches a6185ada581e2ef7335fab21db69adb7af3328a3635a42a44cad233ee6a5f74a at review time. Runtime uses the documented HLD15 corpus/import8MiB environment. This does not qualify a Rust default2MiB test run. Historical scanner compile failures remain uncredited. No reviewer build or execution occurred.

## Frozen source and boundary

Authenticated retained carrier hashes before and after review:

- crates/oxml-drawing/src/text/paragraph.rs: c17d7a472aacef09a4f60d38c4355ed3fd9cff7e43abfa7a95c0a5ae0cbb92b4
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/oxml-drawing/src/shape_props.rs: 21ca64fe24f86a3888d2458a973ac9ded5028533efce81ece194b2036c0155c0
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3
- Retained integration entrypoint: c2f25891c2651c85bbfba312dd9d2fdb00c800cad10b3aab5716499b66b3df5c

The four live production utility carriers also matched their sealed copies. The retained ten-control entrypoint is the review authority, not evolving live tests. Only this canonical review file was written. No source fix, build, test, runtime probe, Git, CP or application operation ran. Native import consumers, notes/master appearance, whole source-format graph, archives, public acceptance and F216 completion remain outside this bounded verdict. Review ends and returns control for distinct remediation.
