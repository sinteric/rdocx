# F-X211b, native owners, pass 1

**Reviewed**: Frozen six-carrier native diff against0a4f9259c6d98f281075244cdc6b7b39da8edd83,2393 insertions and55 deletions. Worker /private/tmp/rdocx-f-x211b-codex. Public-only changes may proceed in disjoint carriers. This pass covers native checked owners, parsers and recorded native controls, not the public implementation or whole feature.
**Verdict**: 2 defects,0 smells,0 nitpicks. Both defects below are independent static code traces, not newly executed regression results.

## Defects

### D1, shape properties bypass modeled child-order and choice checks

`crates/rpptx/src/lib.rs:1141`

The checked-input walker switches PresentationML parents to DrawingML children only for txBody and graphicFrame/graphic. p:spPr is not included. The finite order vocabulary at line963 also has no spPr case. Consequently its DrawingML property children receive no modeled ordering/multiplicity checks, despite being consumed by the native shape-properties model.

A captured shape replacement retaining its valid nonvisual identity but containing both a:noFill and a:solidFill under p:spPr reaches assign_xml_shape at line4838. `crates/oxml-drawing/src/shape_props.rs:202` models the first fill, then returns false for a second fill because fill is already present. At line174 that second known choice becomes raw_children rather than a refusal. The writer at line238 emits the modeled fill and retained second fill. An out-of-order fill followed by xfrm similarly lacks checked-source ordering enforcement and is normalized by the writer.

Candidate validation at `crates/rpptx/src/lib.rs:4998` checks nesting, shape IDs, table merges and serialization. It does not close this shape-property validation gap. Thus the new checked XML operation can publish conflicting modeled fill choices or accept invalid modeled order rather than refusing atomically. This is a concrete modeled vocabulary gap, not a request for full XSD validation. Existing rPr choice tests at `crates/rpptx/tests/integration.rs:31010` do not cover shape properties.

Corrective qualification should include one conflicting shape-fill choice and one out-of-order shape-property source under the actual qualified p:spPr/a-child boundary, alongside foreign opaque preservation. Do not fix it by treating every same-local-name foreign child as modeled.

### D2, image relationship validation accepts truncated signature-only targets

`crates/rpptx/src/lib.rs:5141`

The actual image-target check requires only any image/ content type and ImageFormat::sniff returning Some. `crates/oxml-media/src/lib.rs:180` returns Png for the eight PNG signature bytes alone. Those bytes are not a complete image, but this new checked-owner relationship route accepts them as actual image content. It also does not match the detected format to the declared image content type.

Concrete input: an imported package containing an internal IMAGE relationship to an eight-byte PNG-signature part declared image/png, then an otherwise valid changed regular-run XML with rPr/blipFill referencing that relationship. Existence, type and internal mode checks pass, as does this signature-only condition. The native owner parsers do not decode the referenced image. Presentation reopening at `crates/rpptx/src/lib.rs:1748` initializes MediaStore by scan and does not establish complete image validity. The selected owner can therefore publish a relationship to unusable media instead of reporting the required actual-target refusal.

Use an existing bounded format-aware structural/media validator and check format/content-type agreement before publication. Retain unsupported-format/source-preservation boundaries explicitly. The existing wrong-target/mode control at `crates/rpptx/tests/integration.rs:30629` tests a theme target and external image embedding, but not this signature-only image target. No new image baseline or application result is asserted by this finding.

## Smells

None found.

## Nitpicks

None found.

## Other aspects checked

- Correctness: D1 andD2. The finite PptXmlOwner dispatcher at `crates/rpptx/src/lib.rs:798` binds actual part, shape path, cell and paragraph/run selectors. Public run replacement selects regular TextRun::Run records rather than fields or breaks at line4964. IDs and table continuation rectangles are checked before candidate publication.
- Contract: D1 andD2. The concrete root/kind gate at `crates/rpptx/src/lib.rs:1085` rejects multiple/wrong roots, DTD and modeled container character data. Shape kind conversion and selected nonvisual-ID changes refuse at line4870. Unknown relationship-role additions require retained source facts, and cleared noaction is narrowly admitted from existing source at line5049. No arbitrary cross-part importer is added.
- Panics: zero findings. owner_data/owner_part_name validate part indexes before later indexed access at `crates/rpptx/src/lib.rs:4074`. Descendant lookup uses get/checked owner helpers. Nesting admission precedes modeled group parsing. Candidate clone/assignment/validation/serialize/reopen complete before the original Presentation is replaced at line4833. No new unsafe code or externally constructible unchecked owner token was found.
- OOXML: D1. Root closure and retained namespace/QName source are exercised separately. Parser changes reject trailing roots for regular runs, paragraphs, cells, tables and presentation parts. The selected-part force-write at `crates/rpptx/src/lib.rs:5232` handles opaque root edits that rendering equality deliberately ignores. Original signature material remains present with genuine cryptographic invalidation evidence, rather than being silently removed. Image-target admission D2 remains required package qualification.
- Tests: the retained16 controls provide actual finite evidence for their exact inputs, not the untested D1/D2 negatives. They cover nine owner no-ops, selected writes, five shape kinds, relationships/actual modes, aliases, opaque QName meaning, model attributes/order, grid/merge checks, inherited part namespaces/siblings, signatures, retained cleared actions and modeled nesting. Source-built expectations verify actual saved part names and surrounding bytes. Debug constructor stack failures and missing remaining suites stay uncredited.
- Structure: zero findings. The concrete owner enum serves the nine current operations. Existing parser/state/part providers are reused. Existing concrete Box enum payloads are retained. No new crate, module, file, dependency, feature flag, trait, generic framework or owner registry is introduced. This review does not certify later Python property-view retirement.

## Authenticated actual evidence

Capsule SHA `bda0c74035050ad7388571e54e33fa047645c9a5ce5e2de585d2d3c40668ac0a` authenticates current and actual retrieved Base bytes for each of the six native carriers. Independently git-retrieved Base bytes match their retained copies. Native patch SHA is `9526aafe9254014f59a701723f3550605772ce273a193146368e9efee1c16e2c`. The historical complete3208-file native snapshot has SHA `dfa7ea8ab9c4fb765caa849006dc656a9e7bf31ef8c85be609f866e63f59ba20`. It is phase evidence, not a claim that disjoint public files remain unchanged during this review.

Release6 receipt SHA `a5326381904d09b8ee950c0d88f1723c058f4359c740c1f55c2a3657b8ecd634` and log SHA `cb4ec542f5e68ed78b066032014cb0ac9baffa6092779efd40e9d30667e1bb33` authenticate actual16 passed,0 failed,0 ignored,350 filtered,0.42-second runtime. The command was locked/offline release/all-features rpptx integration with no RUST_MIN_STACK override and normal2MiB defaults. Its leader is reaped and owned group gone. Current integration executable rehash is `f3742f9a3631d6ea3f74d4c1402efce2fb2c2c31f2516d2c541b7ce3eeae96e8`. No historical binary digest was recorded in release6, so that current rehash is not misrepresented as a retained before/after executable proof.

Current-source strict three-crate all-target/all-feature Clippy receipt `fa633cdfc74f3fc3ffe3b5323161bd8979ed5760ed14067900ea474cf9508e3e`, formatting receipt `bb92db4d07bfab80701fbac5c83c8ed1fb0c37765f27b573ec598ca154f82a1d` and49-match hash receipt `892a86bc626a7102a5e562947a9d526385141cc90fbb62f50dba878819728236` authenticate their actual exit0 logs and safe groups. Check6 receipt `c051be4aa609db7098b26348ec8d8a9d31cce2c722593a35501ddd376fd6fd73` remains actual earlier all-feature qualification, not a final feature gate.

## Originals and incomplete qualification

The four parser RED controls actually failed before the parser repair, receipt `84fe56a972d2a69677b49f490812871af598c083f5c291776997e870d12955f5`. Their unchanged four-control GREEN receipt is `1bff6a658326933e0f9c97f62c18a5479daa0bef90a03872bd919e20ec1a9684`. Their raw logs authenticate.

Wrong-target relationship RED receipt `536cd03c398bee5a4761fb9bcfc1e4d5704311117d5f6d53d77e484470057934`, opaque-source release3 failure `f28dcbca456ab8799c2caa60cfb6e626e2d77810549a32e2ce94a399fe76cbd0` and inherited-closure setup failure `aea74f6579f603ca3e2edd09ebb1937e2f50f667412c000c54ab828c44c1c955` are retained. The latter assumed no XML declaration and is not a behavioral RED. Its actual corrected fixture receipt is `c046a31435bf67019639986d5dc0b10eb001dc97fc1743871c2ea9bc12b03837`.

Cleared-action RED1 receipt `27c975dd92c1f4d29d5bdc6ecf52f5c8fc76bf6289549ca7c16302493292bc2f` is a compiler setup failure on nonexistent id(), not a runtime regression. RED2 receipt `e182c6da425ebfcb41413982be952b1a4e8c5b0615f09c6befe3ee81350c59c1` authenticates an executed failure on the legitimate retained empty-ID/noaction source. Release6 qualifies its actual successor. All these original log hashes were checked, and none is relabeled passed.

Ordinary debug constructor SIGABRT attribution remains unresolved. Release qualification does not explain it. Remaining changed-crate suites, MSRV/WASM/package riders, public nine-owner APIs, property-view/descendant retirement, installed wheel/typing/allocation evidence, native Office, frozen corpus, root composition and whole-feature scoped acceptance remain pending. Neither this report nor the16 controls certify them.

## Review boundary and source guards

The native six-carrier freeze and disjoint-public writer agreement were confirmed with the implementing agent before source review. HEAD remains0a4f9259c6d98f281075244cdc6b7b39da8edd83, branch work/f-x211b-codex, index SHA `d1c274930f3047dc8ae8da4d21c9512eda688892ebb8b97a5c7624fa7f9034ef`. All six carrier hashes, native patch, HEAD and index authenticate before and after this report. Canonical existing tracked/dirty sources, plans and prior review files remain exact. Only this canonical review file is written.

Canonical CLAUDE/workflow/microscope, parent/child plans and matched parser/serializer/API risk references were read. No tests, source fixes, worker writes, target mutation, UI, Office, managed CP, baseline or ledger changes occurred. Review ends here and control returns to root for remediation decisions.
