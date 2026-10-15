# F-X211, native owner, pass 1

**Reviewed**: Frozen native owner checkpoint54 with normal-stack successor57. Seven current owner carriers against claim base `36cfafaa5999709a4899f74ad4ccef773981a876`, within the complete working diff of13 files,4048 insertions and106 deletions. The source34/source54 hash comparison identifies exactly seven changed carriers, not a reconstructed historical textual diff.
**Verdict**: 1 defect,0 smells,0 nitpicks. Native owner checkpoint only, not the full feature.

## Defects

### D1, retained foreign w binding changes the table and cell namespace

`crates/rdocx-oxml/src/table.rs:2827`
`crates/rdocx-oxml/src/table.rs:2202`
`crates/rdocx-oxml/src/text.rs:510`
`crates/rdocx-oxml/src/text.rs:543`

The newly retained table/cell root attributes are appended to canonical `w:tbl` and `w:tc` start tags without protecting the canonical element binding. A valid input can use `q` for Word and `w` for an unrelated producer namespace, for example `<q:tc xmlns:q="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w="urn:producer" w:keep="x"><q:p/></q:tc>`. Root attribute capture retains `w:keep` together with `xmlns:w="urn:producer"`. The shared attribute writer skips only the canonical Word declaration and writes the foreign declaration onto the generated `w:tc`. The generated root and its inherited `w:` children consequently resolve to the producer namespace. The table route has the same defect. This is a static serialization finding, not a claimed runtime observation.

Preserve the producer attribute's expanded name while keeping generated elements in the Word namespace, or refuse this unsupported collision before publication. Cover both direct typed round trips and checked table/cell replacement with a genuine failing control. Assert expanded names, retained producer attribute values, modeled content after save/reopen, and unchanged state on any named refusal. The existing foreign `x`/`y` attribute positives do not exercise this collision.

## Smells

None found in this bounded pass.

## Nitpicks

None found.

## Not found

- Correctness outside D1: no additional finding in physical cell selection, staged candidate publication, identical replacement handling or section reference qualification.
- Contract: five operations remain crate-private. Public bindings, PowerPoint XML, complete known invariant profiles and the frozen100-script corpus remain explicitly unqualified.
- Panics: no new finding in the bounded owner routes. Actual successor57 authenticates eight rows and20 controls on the normal2097152-byte stack, including maximal mixed lifecycle controls.
- OOXML outside D1: no additional finding in qualified root admission, typed owner relationship scope, modeled child ordering or intentionally opaque wrapper handling.
- Tests outside D1: no additional finding. Failed setup55 is retained separately from actual successful direct-binary successor57 and owned cleanup56.
- Structure: no new unsupported trait, generic, crate or forwarding abstraction found in the seven-carrier boundary.

## Evidence boundary

All3203 entries in source54 were independently checked for current bytes, size and mode before writing this report. HEAD and checkpoint57 SHA `9cabd6ae03520b36944b7824f66d8cc30df114aef14d90e3c62a7dfa0eee5e20` match. Actual normal-stack receipt SHA `f72951ce2a749a93523c6e123e16534ec4ac3fedc49ccdcbf62301ed17b80663` matches. Source34/source54 comparison is SHA `a0332da016eb488a0ef8278ae7b4d9a8c51aadb822c1dd9c88d20f077026c2aa`. The current seven-carrier claim-base patch is SHA `ee3f4d5ed785541f1d682bb20e2b30c2f743e063bd6e6d5b3c3b7945e4c97d0b`.

This pass modifies only this review file. Remediation is a separate phase.
