# F-X208 bounded OXML failure attribution, pass 10

Reviewer: fonts_resume, independent of the F208 implementation.

**Verdict: 3 defects, 0 smells, 0 nitpicks.** Three inherited parser or writer integration defects account for five actual failures. One source-preservation test assumption accounts for the sixth and is explicitly refuted as a product defect. This is attribution of the six named failures, not a full feature review or remediation.

## Bound evidence

Worker HEAD `a6f3f1c1c014f510f5ff79827eae02974f574d01`. Actual source map `.claude/scratch/F-X208-start-1/source-production-35-geometry.json` SHA-256 `1bc9831212ac9dd13fef4fec6e2d83a795c36212f4af449323c04a7e11a706cb`. Receipt `actual-scoped-riders-1/receipt.json` SHA-256 `0af24270951a7bf341559bc0b45930f35d3206bc5e7e501c8a88f6b4cb18af72`, log `OXML-integration.log` SHA-256 `593358d1277b5df520795bc9d746d4722e5eff20139a472973ebab2217277592`. Actual result is 150 passed, 6 failed, 0 ignored. Child101 is retained as failure. Owned PG77305 was reaped and gone with ESRCH, no signals.

All seven inspected carriers matched this source map. Shared text/mod.rs, paragraph.rs, list_style.rs, comments.rs, diagram.rs and integration.rs are byte-identical to both claim a6f3f1c1 and canonical2bdf Git blobs. Presentation.rs differs only in F208 appended root-attribute methods. The defaultTextStyle parser body is unchanged. These are inherited seams exposed by the full suite, not newly attributed F208 behavior changes. Citations are relative to `/private/tmp/rdocx-f-x208-codex` at the authenticated source map.

## Defects

### D1. Comment text-body parsing drops the inherited namespace environment

`crates/rpptx-oxml/src/comments.rs:700`, `crates/rpptx-oxml/src/comments.rs:851`, `crates/oxml-drawing/src/text/mod.rs:93`, `crates/oxml-drawing/src/text/mod.rs:259`.

The three failing comment controls at integration.rs5546, 5609 and5865 contain valid d:bodyPr/lstStyle/p with DrawingML xmlns:d declared on the surrounding comment list or comment. Their a prefix deliberately belongs to a producer. Capturing q:txBody retains physical child bytes but removes the inherited declarations. Calling from_xml_as reparses it with an empty namespace environment. text_owned_name at text/paragraph.rs247 therefore cannot recognize d:bodyPr, and the body reports missing properties.

This is a real parser transport defect, not a missing-bodyPr or invalid-alias fixture. Retain the physical raw fragment for source serialization and provide the authenticated enclosing namespace entries to typed parsing. Preserve foreign a payload and the original unsafe rewrite refusal in integration.rs5875. Do not repair it with global prefix substitution.

### D2. SmartArt writer assumes a source-free canonical wrapper

`crates/rpptx-oxml/src/diagram.rs:1746`, `crates/rpptx-oxml/src/diagram.rs:1752`, `crates/oxml-drawing/src/text/paragraph.rs:77`, `crates/rpptx-oxml/tests/integration.rs:559`.

write_diagram_text requires emitted bytes to begin with exactly <dgm:t>. The valid source has x/y namespace declarations and x:keep on that owner. CT_TextBody now retains the root spellings and emits those attributes through text_wrapper_start. The exact OPEN check consequently rejects a valid edited text body as a malformed canonical wrapper. Parse the actual qualified emitted wrapper and carry its body without discarding root ownership or duplicating attributes. Preserve opaque root attributes and the existing reopen assertion.

### D3. Presentation default-text parsing conceals modeled alias children

`crates/rpptx-oxml/src/presentation.rs:252`, `crates/oxml-drawing/src/text/list_style.rs:72`, `crates/oxml-drawing/src/text/list_style.rs:148`, `crates/oxml-drawing/src/text/paragraph.rs:257`, `crates/rpptx-oxml/tests/integration.rs:2598`.

The enclosing presentation declares d as DrawingML. The captured defaultTextStyle is parsed standalone, losing d. Its d:lvl1pPr is then opaque instead of modeled, so the fixed-list-style descendant validation does not see the deliberately foreign canonical a binding. The test expects refusal but gets success. The same seam can omit valid imported default-text properties from the model. Preserve the actual inherited namespace scope through this parser boundary, retaining the original strict typed-boundary negative as well as an alias-positive case.

## Refuted product finding: shape-order control demands a renamed prefix

`crates/rpptx-oxml/tests/integration.rs:3895`, `crates/rpptx-oxml/tests/integration.rs:3920`, `crates/oxml-drawing/src/text/paragraph.rs:37`.

The fixture supplies a genuine PresentationML q:txBody. The writer preserves this qualified source alias, but assert_order searches for literal p:txBody. The actual log shows q:txBody after p:style with expected DrawingML children and owner declarations. Correct the test to inspect expanded owner identity and child order, retaining alias/source-preservation and structural reopen predicates. Canonicalizing production merely to satisfy this string search would regress the selected source contract.

## Six aspects and limits

- Correctness: D1 through D3. Zero additional findings in the bounded paths.
- Contract: D1 through D3. Zero additional findings. No schema-invalid fixture accounts for these six failures.
- Panics and errors: 0 findings. The observed unwrap panics are test sites exposing returned parser or writer errors.
- OOXML: D1 through D3. Zero additional findings. Actual enclosing declarations establish the alias URI. Raw foreign content remains required.
- Tests: 0 separate defects. The lexical p-prefix expectation is refuted as a product finding above. All original six failure predicates remain retained, with no runtime rerun or assertion edits in this pass.
- Structure: 0 findings. Existing namespace-scope and typed parsing seams provide bounded remediation without a new module or dependency.

No source edits, product imports, Cargo commands, test execution or environment mutation occurred. Only this external review file was written. Return to distinct remediation.
