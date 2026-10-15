# F-X205, working, pass 12

**Reviewed**: Independent full working implementation against the approved Issue308/PR318 contract. The immutable source49 diff contains32 carriers,6757 additions and184 deletions, including all eight approved HLD carriers and the existing default template asset.

**Verdict**: 0 defects, 0 smells, 1 nitpick. This is a source review, not feature completion or an integrated verification pass.

## Source identity

The review reads `whole-source49-capsule/current` under `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1`, not later mutable worker source. Its manifest SHA is `e7f7ac18d3d1e37dca1108901f264e6cb41c190542184af614e3185e37ae1743`. All69 declared artifact hashes and sizes authenticate. The complete working diff SHA is `767741889f6cbc5a8469482a1177492f910af94f65a3d4e49e26da0d0c724731`. Full source-map SHA is `667c8f64bf0439ac76df4c5adc947c98ba6ca13cdd1cfd09f9e63066b1edf1bf`. Recorded HEAD is `338754a70e5c4dffc78d84069b8d0294e9395f2e`, with the captured index artifact SHA `ac793a72091a40351f92b0b226aa8814ff6e77e4430743e0c61ce73c16287318`.

The immutable review-input manifest SHA is `630d728972818e934a02cfb1d45b643414c53518eb132e8e91645557f871c49e`. Its four records authenticate, including approved design SHA `62e3c2baf9d37b674620bb59ca4dc24603240fc75b8d2700759dc8e41108fea2` and evidence-index SHA `bd039030302854c180f9b6dc892b0b8486becf766083c5d9dd54efecea0ec57f`. All61 indexed receipt files and98 indexed raw logs match their recorded hashes. Failed predecessors remain failures.

## Defects

None.

## Smells

None.

## Nitpicks

### N1, pattern-fill API documentation describes the previous renderer

`whole-source49-capsule/current/crates/rpptx-py/src/dml.rs:593`

The public `FillFormat.patterned` documentation says rpptx does not draw pattern fills yet. This source implements concrete sampled pattern paint across PDF, raster and SVG. Update this obsolete sentence to match the bounded supported behavior. The current HLD already describes that behavior accurately.

## Six aspects

- **Correctness, zero defects or smells.** Checked direct font baseline and spacing conversions, language and independent script slots, numbering schemes and independent bullet properties, table flags/style validation, gradient-stop identity and lifecycle, transparency, picture-fill ownership and pruning, properties, sections, media and export result shapes. Mutations validate a staged candidate before publication. Property source patchers retain unrelated XML and refuse ambiguous owners. Gradient IDs remain runtime identities rather than guessed XML identities or value equality. Raster selection, dimensions, formats and quality are checked before layout. ODP returns ordered native reports.
- **Contract, zero defects or smells.** The complete approved surface is present in existing native and Python carriers, with matching typed exports and stubs. Native absence handling distinguishes missing notes/handout owners from present malformed relationships. Synthetic defaults remain read-only and use presentation logical page dimensions. The template has exactly the original39 member paths. Changes are confined to core metadata and the declared master/layout geometry. Fresh metadata has fixed2000 timestamps and absent creator, lastModifiedBy and description. Imported/user metadata remains explicit. The inherited F-X211 Paragraph.add_run promise and deferred reconciliation are not reclassified as completed F205 work.
- **Panics, zero defects or smells.** Examined new indexing, slicing, numeric conversions, checked dimensions, PID/token allocation and section assignment. The new section indices are validated as the exact complete ordered slide sequence before indexing. Invalid scalar ranges and unsupported owners return errors before publication. No new unchecked externally controlled arithmetic was found in the reviewed paths.
- **OOXML, zero defects or smells.** Checked modeled child ordering, retained character-font attributes, language source-token edits, qualified property-owner selection, custom-property PID/fmtid/order retention, picture relationships and unsupported-source refusal. The reviewed path boundary precedes phClr resolution only for unsupported geometry. Supported centred paths still resolve source fillRef/theme colors. The asset edit does not add or remove package members. Alias, foreign-binding and opaque-source controls remain deciding controls, not canonical-prefix assumptions.
- **Tests, zero source defects or smells.** Genuine installed Base collected33 cases, with32 failures and one existing positive. Native metadata Base failed semantically on the old dates. Existing controls exercise source retention, repeat edits, save/reopen, invalid/atomic refusal, physical script faces, shared picture ownership, cache budget and supported/refused paint distinctions. Prior fixture corrections preserve their original assertions and failed receipts. Source49 adds the missing genuine shape preset to the previously nondeciding path control. Header signatures alone do not establish visible export acceptance.
- **Structure, zero defects or smells.** No new product module, file, dependency, trait or speculative generic framework is introduced. Existing typed property helpers have current consumers of multiple concrete types. Grid is the shared concrete paint representation with real PDF, raster and SVG consumers. The narrowly documented PyO3 argument-count exception preserves the approved public keyword signature. No forwarding-only framework was found.

## Prior findings and runtime limits

Independent bounded passes1 through11 retain authority for unchanged reviewed bytes. Earlier spacing, bullet documentation, fixture setup, optional transparency, unrelated-image pruning, Grid heap accounting and path-reference fixture findings are resolved in this inspected source. Their failed predecessors are retained. This whole pass also covers the later property/media/export APIs, template behavior, HLD and mechanical lint changes that those bounded passes excluded.

The immutable evidence index now contains actual terminal source49 attempt6, receipt `c1d80461c476ddd0e291989691d3fc527e321661e448867c13aad0cbf5b59111`. Its exact path control passes1 with zero ignored. Raw executable labels establish `rpptx`135 passes/5 existing ignored and `rpptx-render`124 passes/1 existing ignored. Both rows exit0 with guarded source and safe owned cleanup. Earlier authenticated scoped receipts provide the remaining changed-library and Clippy results with their individual source maps. A failed aggregate wrapper is not relabelled as a passing suite.

The retained installed40-case composite and typing evidence concern their recorded wheel/source versions. Newly supplied current installed7 evidence is outside this immutable review index and receives no independently authenticated fresh credit in this report. Final current wheel/member equivalence, integrated hash, WASM, affected archives/metadata/policy and remaining declared native/oracle qualifications require their actual receipts before completion. Native source-derived pattern programs, palette weights and logical8pt tiles are qualified separately from the recorded unresolved cross-engine raster phase differences. Gradient comparison improvements are not a substitute for the declared visible native acceptance. No pixel threshold, native fidelity obligation, source-preservation boundary or labelled behavior-commit requirement is waived by this clean source verdict.

No source changes or runtime commands were performed in this pass. The review ends with this report.
