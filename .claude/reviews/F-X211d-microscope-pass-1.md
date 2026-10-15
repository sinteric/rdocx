# F-X211d, microscope, pass 1

**Reviewed**: Approved corrective Word contract, progress, inherited carrier custody and frozen3348-file working-source map `0cf61d2a75c7d6df9b2ab5314fa087585719193536a7720c1e9a088c829b237d` on claimed HEAD `902d68956aefcb75e2fad332915ff3e49b6e6582`. Independently authenticated every mapped worker body, size and mode. Source was not changed during review.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. The new merge operation has an opaque-property loss path and is not ready for a zero-finding gate.

## Defects

### D1. Merge deletes unmodelled properties of removed non-anchor cells

`crates/rdocx/src/table.rs:1108` checks direct cell.extra_xml and existing vertical-merge state, but does not inspect opaque XML inside the cell properties. `crates/rdocx-oxml/src/table.rs:1771` stores unmodelled tcPr children in a distinct extra_xml field. The parser records unknown empty children at line1804 and unknown nonempty children at line1837, with their namespace closure. The ordinary writer retains them at line1928 and subsequent schema boundaries.

For a two-column horizontal merge, put a namespace-complete foreign child inside the second cell's tcPr, such as `<x:keep xmlns:x="urn:opaque" marker="preserve"/>`. Its direct cell.extra_xml stays empty, v_merge stays absent, and the new preflight accepts it. The candidate copies only cell content at `crates/rdocx/src/table.rs:1122`, then removes the second physical cell at line1142. Its tcPr and opaque child are discarded. Validation and publication at line1159 do not restore the lost bytes.

The same omission affects a removed cell's opaque border children. Their actual retained storage and namespace-closing parse are `crates/rdocx-oxml/src/table.rs:256` through line269, with raw writer emission at line307. Removing the cell removes that separate retained border payload too. These are the same metadata-preflight hole, not separate findings.

Direct tc root attributes are stored in the cell's retained root record and are covered by the existing nonempty cell.extra_xml check. Existing topology preflight at `crates/rdocx/src/table.rs:754` checks wrapped rows/cells, legacy horizontal merge and topology, but does not check either nested opaque-property store. No additional root-attribute or namespace-only defect is asserted.

The operation must preserve opaque payload or refuse before publication when merging would discard it. This follows `.claude/plans/F-X211d-design.md:38` through line43 and the checked physical merge/rollback contract at line49 through line52. A successful merge of plain cells is insufficient qualification. The new native merge control at `crates/rdocx/tests/integration_test.rs:25310` tests absent-row refusal and plain merge/split but does not exercise this metadata case. Keep a genuine current failing control before remediation, then a successful atomic preservation/refusal successor. No frozen corpus expectation needs editing.

## Smells

None.

## Nitpicks

None.

## Areas without additional findings

The retained-root namespace helper at `crates/rdocx-oxml/src/text.rs:13793` subtracts only declarations from the actual root attribute record. Namespace-only roots retain external child bindings. The foreign reserved w binding remains explicitly protected at line13819. Callers pass actual physical table, cell and section roots and continue the existing bounded depth and qualified-name parsing. This avoids the failed predecessor's blanket local-declaration subtraction. No additional exact-byte or closure defect was found in this bounded successor.

Python Cell.merge at `crates/rdocx-py/src/table.rs:2150` checks same physical table after both handles validate, returns the same object for a same-cell no-op, stages the native edit, and prepares the resulting captured Cell before revision commit at line2197. Allocation or native failure precedes publication. The underlying D1 preservation defect remains independently blocking.

HeaderFooterAppend at `crates/rdocx-py/src/document.rs:2072` and ParagraphAppend note/equation publication at line5119 and `crates/rdocx-py/src/paragraph.rs:771` retain surviving ancestors through their checked inventory routes. Affected descendants still retire through the selected-owner ancestor test at `crates/rdocx-py/src/document.rs:4876`. Conservative Document.add_paragraph behavior remains separate. No owner resurrection or foreign-source bypass was found in these bounded changes.

The four section margin aliases at `crates/rdocx-py/src/document.rs:1386` through line1415 use the existing snapshot values and geometry consumer at line1214. They preserve checked units and the existing EMU-to-Twips truncation rather than introducing a second conversion.

Lazy requested table-style publication at `crates/rdocx/src/document.rs:28371` accepts only the two canonical ID/name spellings, retains an existing definition and builds an atomic candidate. It validates the style graph and leaves factory defaults unchanged. Independently read the existing pinned python-docx1.2.0 default.docx with SHA `2094b5bddffe9cf973d61fe03388413804f034160718494a65db7e98da40d35d`. The inline LightShading-Accent1 definition matches its exact ordered expanded element/attribute/value structure. The TableNormal base also matches after the explicitly intended removal of its default attribute. No new dependency, asset or source file is added.

Inherited four c Word carrier identities are explicit in `.claude/scratch/F-X211d-inherited-carriers.json`. Their prior reviewed hyperlink/grid behavior and the root-admitted shared stale prefix remain separate custody, not new unreviewed owner rules. No additional dependency, structural or causal-error finding was found.

## Execution and acceptance boundary

Read the retained actual native successor receipt `.claude/scratch/F-X211d-current-native-gates-2/receipt.json`, SHA `4fe541ed91971a5594d616975ffb6e98924a26db456a85608dc772ee3a9387d6`. Its three new controls and existing owner/opaque/37 namespace controls do not cover D1. Preserve the original namespace failures and the installed nine-control predecessor RED receipt `1cfb2d42648dc98701e71ff9c7c96f504986a90bcc1755fc111861531c3e54a9`.

A source build cannot qualify the untested metadata-loss case. Fresh installed, typing, policy and parent corpus/native gates remain separate actual authorities. No first100 failure became success, no denominator or body changed, and no completion is credited here.

No source, test, installed runtime, CP environment, output artifact, dependency or Git state was modified. No build, test execution, installation or native UI was performed. Only this unstaged, untracked worker review file was written under the delegated exclusive lease. Review ends and returns control for separate remediation.
