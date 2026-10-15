# F-X216, embedded face helper, pass 1

**Reviewed**: frozen presentation.rs766594e05253d161d9589bd37b9f2c3f3541c0bc0a4194737e801b13a3b77dc6, exact helper control and binary snapshot manifest4a4d2eb185a0cfdce759a7e8a66f5327c1a34f3364465528f28166148a94e292 against claim Base5b0f63d9.
**Verdict**: 1 defect, 0 smells, 0 nitpicks in the bounded helper.

## Defects

### D1, Opaque QName-valued metadata can compare equal under different bindings

`crates/rpptx-oxml/src/presentation.rs:434` through `:438`, `:479` through `:490` and `:742` through `:778`

import_description_matches and the existing-face equality branch compare font_expanded_names and then strip namespace declarations before lexical equality. font_expanded_names records element and attribute name bindings only. It does not record the bindings used by known namespace-valued attributes such as xsi:type or MC Ignorable/ProcessContent. Equal lexical QName values can therefore silently carry different namespace authority.

A precise description witness uses equal typeface FX216 and identical retained root mc:Ignorable="z", with mc bound to the actual Markup Compatibility namespace in both inputs. The destination inherited z binding is urn:destination, while the source inherited z binding is urn:source. One owns regular and the other supplies italic. The descriptions contain the same names and lexical bytes after face removal and namespace stripping, so the helper admits them as equal and keeps only the destination description authority. They are not equal producer metadata. The same problem occurs when an existing face and imported face have identical opaque mc:Ignorable="z" prefix lists but different z bindings. Root closure declarations are stripped, name-only facts match, and the reuse branch accepts the conflict.

Include actual known namespace-valued attribute dependencies in the existing equality proof, or use a named atomic refusal when their authority cannot be preserved. Do not infer dependencies from arbitrary opaque strings. Add meaningful source/destination QName-only conflict and equal-binding controls for both the description and existing-face branches, preserving exact source/destination bytes on refusal. A full font-pipeline verdict is outside this pass.

## Actual evidence

Authenticated receipt1b56c21aa1406a6281bcabea8895d83b137020e5ce69228a0cc78d8649d807a6 records three exit0 qualified safe rows with exact raw log hashes: strict three-crate Clippy, release compilation and the one helper control. The frozen control proves an added italic face retains a rebound source z:marker attribute and z:payload element, preserves the destination regular face, survives repeat/save/reopen and rejects different opaque marker values. Those element/attribute-name dependencies exercise font_expanded_names, but not QName-only values. Initial missing notesSz setup remains uncredited.

## Not found and boundary

No additional bounded defect found in checked relationship-ID edits, unchanged destination faces, schema insertion order, source face child payload retention, ordinary namespace closure, staged helper assignment or explicit overwrite refusal. Smells:0. Nitpicks:0. The helper is concrete in the existing carrier and adds no module, dependency or framework.

The frozen presentation carrier and retained control matched their manifest hashes before and after review. Live lib.rs and evolving integration tests were not treated as reviewed or frozen. No source fix, test, Cargo, probe, Git, interpreter or application ran. Only this canonical review file was written. Whole selected-font closure, licensing/bytes/relationships and F216 completion remain outside the verdict. Review ended and returned control for distinct remediation.
