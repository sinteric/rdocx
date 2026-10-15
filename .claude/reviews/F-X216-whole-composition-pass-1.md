# F-X216, whole working composition, pass 1

**Reviewed**: Whole working diff against ClaimBase `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`: 19 tracked carriers, 12,458 additions and393 deletions. Approved canonical plan SHA-256 `1a0dc6c6d4def67e9b455ef6ace104983f067858332edf2413243fb14d33b5b4`. Frozen3,296-body source map `69d907193b234a821689ef06c10e8f5616b5b6419406a0c7ac8b6a619b306d33` matches every current body, size and mode before and after audit. Accumulated bounded native, inheritance, public, CLI, normalization, registration, placeholder and test-authority reviews were read as their stated scopes, not promoted to whole acceptance.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, formatting reuse discards relationship-owner metadata from graph equality

`crates/rpptx/src/lib.rs:14338`, `crates/rpptx/src/lib.rs:14377`, `crates/rpptx/src/lib.rs:14427`, `crates/rpptx/src/lib.rs:8746`, `crates/rpptx/src/lib.rs:8809`

`formatting_parts_equal` compares the ordinary part body, content type and typed relationship edges, recursively checking internal target graphs. Its edge projection keeps only public Relationship fields. It never compares the retained relationship XML root, record metadata, opaque children, physical order or their namespace authority. `adopt_source_layout` and `adopt_source_master` immediately reuse an existing destination graph on that boolean.

A valid source and destination layout with identical body bytes, relationship IDs, edge types, target modes and equal image/theme/master targets, but different foreign metadata on the source layout relationship owner, is reported equal. For example, one actual owned Relationship can carry source-only `z:record="source"` or a retained child in a declared foreign namespace. A differing opaque root attribute or child supplies the same witness. Reuse then keeps only destination metadata and bypasses source-preserving graph copy, losing the source owner instead of admitting complete equality or retaining a distinct graph. This contradicts approved plan39's complete graph equality including opaque metadata.

Admit equality of the actual relationship scope alongside the proven edge/target map, ignoring only the precise owned IDs and targets mapped by graph comparison. Preserve or refuse unsupported scope equivalence rather than silently ignoring it. Add valid otherwise-equal layout and master graph controls with distinct root and record opaque metadata, alias/default scope differences and reordered opaque payload. Prove non-reuse or named atomic refusal, source/destination byte preservation, and continued reuse for truly equal repeated imports. A successful copied-graph preservation test does not cover the reuse shortcut.

### D2, resource target maps are passed as relationship ID maps

`crates/rpptx/src/lib.rs:14652`, `crates/rpptx/src/lib.rs:14664`, `crates/rpptx/src/lib.rs:14961`, `crates/rpptx/src/lib.rs:14973`, `crates/oxml-opc/src/relationship.rs:230`, `crates/oxml-opc/src/relationship.rs:282`

`transfer_imported_property_resources` clones selected relationships without changing their Target. It separately builds a map from relationship ID to the destination-relative target path, then supplies that map as the `ids` argument of `rebind_for_import`. Source notes-master resource staging repeats the same pattern. The existing provider interprets map values as proposed relationship IDs, not target paths. A path such as `../media/image.png` is not an ID of any proposed cloned record. The original record is consequently removed and the unchanged clone appended. The computed owner-relative target is never assigned. Per-record opaque source metadata is lost during that remove/add path.

If a presentation default picture bullet is transferred from `/ppt/presentation.xml` into `/ppt/notesSlides/notesSlide1.xml`, a source Target `media/image.png` must become `../media/image.png`. The current proposed item still names `media/image.png`, resolving below the wrong directory. Notes masters at custom paths provide another concrete owner-depth witness. Standard notes-master and notes-slide directories can share the same relative depth and mask the target failure, but cannot justify dropping record metadata.

Set the proposed relationship's Target to its checked destination-relative target and supply only the actual ID remapping to the preservation provider, empty when original IDs are retained. Keep opaque root/record metadata and order under the original scope, then let the destination merge supply its real ID map. Add source-built presentation-default resource and custom-path notes-master resource controls with deliberately different owner depths, foreign record metadata and ID collisions. Require correct saved target resolution and exact media bytes, proper reference mapping, unchanged inputs and late-refusal rollback. External target/mode should remain unchanged. Do not repair output afterward or fabricate a new relationship serializer.

## Smells

None.

## Nitpicks

None.

## Composition aspects checked

- Correctness: complete native request validation, selected caller ordering, source snapshots, staged batch publication, graph copies and sharing, layout/master reuse, global resources, comment identity, sections and jumps were audited together. The two findings concern shortcuts at graph reuse and original-to-local resource scope transfer. No additional concrete finding was established.
- Contract: native `import_slides` at`crates/rpptx/src/lib.rs:7910` validates insertion/layout, empty selection, duplicate selection and pre-insertion existing map. Expanded graph transfer and direct source appearance stay within the approved ownership boundary. `keep_imported_sections` at8172 preserves caller-order runs rather than sorting selection. The reviewed notes registration and skipped-placeholder reference refusal remain present. Pending HLD candidates are unapplied documentation proposals, not current completion evidence.
- Panics: zero additional findings. Public/native admitted paths check bounds before actual indexing. Copy maps are populated before recursive graph copying, and visited identities prevent graph cycles. Python single-result indexing follows the concrete one-slide native request. This is not a claim of exhaustive arbitrary-depth or allocation-failure qualification.
- OOXML: D1 and D2 are preservation/ownership defects. Other previously reviewed namespace admission, modeled edit handling, schema placement, opaque child order, lexical jump release, notes field retention and source-scoped style preparation remain composed. The existing relationship engine stages raw edits and validates resulting items. D2 is its caller misuse, not a reason to relax that provider's ID contract.
- Tests: existing defining graph, comments, table/default style, face conflict, notes namespace/occupancy/registration, jumper sound/foreign payload and atomic refusal controls were inspected with their prior review boundaries. They do not supply the missing D1 reuse-metadata or D2 different-owner-depth resource witnesses. Historical failures remain evidence. Whole integration384PASS/6FAIL/24IGNORE is still FAILED, not whole-feature green.
- Structure: zero findings. Concrete copied graph maps and inheritance operations have actual consumers. No new product module, crate, dependency, feature flag or speculative framework appears in the19-carrier diff.

## Public and CLI composition

`crates/rpptx-py/src/slide.rs:1210` prepares a native candidate, allocates diagnostics, returned paths, Slides and tuple before publication at1265. Empty selection returns without preparing or publishing owners. `InsertMany` at1233 preserves existing slide owner addresses through current provider preparation. Existing-owner foreign identity precedes stale lookup in the reviewed layout paths. Batch index normalization and precise error mapping retain the corrected test authority. Collection survival, fresh returned owners and source bytes remain explicit tests, not inferred solely from class presence.

`crates/rpptx-cli/src/commands.rs:1532` protects both input paths before reads, validates one-based ranges/duplicates and checked insertion, delegates to the shared native candidate and uses path-aware package serialization at1563 before atomic publication. The Base default duplicate path now reads actual dirty modeled owners. Table and font import continue through their qualified resource and source-description providers. Those providers do not eliminate the two composition defects above.

## Evidence and acceptance boundary

The frozen full map remains exact for3,296 tracked carriers throughout this pass. No source writes, tests, compiler invocation, CP, application or Git mutation occurred. Only this report was written. The approved plan and current source were the authority. Prior bounded zero reports retain their narrower conclusions and original failures, without certifying these newly identified whole-composition witnesses.

Current native-source Office captures, full source-format metrics, final installed current union, affected archives, scoped policy/riders, corrected broad environment successors, HLD application and formal completion remain separately pending. Missing original native timeline/SmartArt assets do not become passing by omission. Zero findings in another scope cannot substitute for feature or sprint acceptance. This whole working source pass ends and returns the implementing writer for distinct remediation.
