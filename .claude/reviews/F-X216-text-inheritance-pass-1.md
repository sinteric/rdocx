# F-X216, text inheritance and notes flag utilities, pass 1

**Reviewed**: the three frozen carriers, 407 inserted lines and zero deleted lines, in manifest e660a0af9143748e2e5564c58cf08b956aa1bce80193d7cedd98ed4427ef5fd1 against Base5b0f63d9 and exact patch1564342050502d3883cbdd488933bc5fe38ad3379d632ccc7b46f0df41a21ea5. Contract is the corrected approved plan's three concrete inheritance methods and exact notes decoration flag.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Static utility review only.

## Defects

### D1, copying a known attribute can rebind retained local opaque children

`/private/tmp/rdocx-f-x216-codex/crates/oxml-drawing/src/text/paragraph.rs:6190`, `:6203` and `:6219`

For every missing inherited root attribute, text_inherit_property_xml copies the entire inherited physical namespace map onto the local root. The opaque-context guard runs only when the incoming attribute is not known. Thus copying an unqualified modeled property such as b can introduce an unrelated default namespace and change existing local opaque element ownership without an error.

Concrete source witness, with A equal to the actual DrawingML namespace:

```xml
<!-- Local -->
<a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><opaque/></a:rPr>
<!-- Defaults -->
<a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns="urn:parent" b="1"/>
```

The local opaque child originally has no namespace. Both property roots are correctly qualified a:rPr. The known b attribute bypasses :6190. The absent local default prefix falls into :6212 and adds xmlns="urn:parent" to the local root. The original local child bytes are retained, but <opaque/> now resolves to urn:parent. Reparse does not reject it because it remains an opaque child. The same common helper serves paragraph and list inheritance, including nested defRPr operations.

Copying a known unqualified scalar must not transplant unrelated parent namespace authority. Retain the local root and opaque scopes. Where an inherited opaque attribute genuinely needs additional bindings, prove that adding them preserves existing local authority or return the approved named atomic refusal. Add a defining namespace-resolution control for this witness, alongside explicit false/zero and harmless alias positives. No runtime reproduction was performed in this static pass.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Other assessment

The common helper groups qualified character alternatives through character_property_slot and paragraph alternatives through paragraph_property_slot. Existing local slots dominate inherited slots, preserving explicit modeled false and zero values by physical attribute membership rather than truthiness. Paragraph bullet colour, size, font and choice occupy separate groups. Nested defRPr, defPPr and list-level merges are staged and reparsed through the existing concrete property APIs. List cascade is parent defPPr, parent level, local defPPr and local level. Opaque root attribute conflicts use expanded identity and conservative namespace-context refusal. Copied inherited element children receive original namespace closure before insertion. These inspected paths produced no additional bounded finding beyond D1.

The exact notes getter at /private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/notes_parts.rs:127 recognizes only the unqualified showMasterSp key. It returns presence-sensitive Boolean values and an error for invalid values. The setter at :142 removes or replaces only that exact key, retaining prefixed foreign lookalikes. It introduces no unrelated notes master or rendering change.

The methods take immutable source inputs and return a staged candidate. No mutation of caller-owned defaults or local objects is performed. No new crate, module, dependency, type or generic inheritance framework was introduced. The private common XML operation has the three present concrete consumers.

## Evidence and boundary

Before and after review, frozen and live sources matched these exact hashes:

- crates/oxml-drawing/src/text/paragraph.rs: da3d5f013f2916eee2e4ba5e7226fbeb4c2c64d97781224e845e02ba6b7da7a1
- crates/oxml-drawing/src/text/list_style.rs: 07f23259a1ef85820851635df13cce59a35c35758994badb55b13f9b4f29a01f
- crates/rpptx-oxml/src/notes_parts.rs: 18543d06862113ad739ea91547e43d7662b194930779be7e28a11b5516c09fc3

Current compile check exit0 is only compile evidence. The eight external XML pairs remain advisory and unexecuted. No utility runtime control, native pipeline, appearance equivalence, archive, Python surface or whole F216 success is credited by this report.

Only this canonical review file was written. No source fix, build, test, binary probe, Git, CP or application operation ran. This review pass ends and returns control for distinct remediation.
