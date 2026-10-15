# F-X211a, clone isolation, pass 1

**Reviewed**: Actual source18 to frozen source19 delta in `crates/rdocx-oxml/src/content_control.rs`, one file,54 additions and one deletion. The production portion replaces derived `SdtContent::clone` with concrete dispatch and one isolated leaf helper. The rest adds one existing-module seven-variant test. Compared the actual preserved pre-edit carrier, not a reconstructed source or sprint-wide diff.

**Verdict**: Zero defects, zero smells, zero nitpicks in this bounded clone repair. The maximum checked transaction still fails and this report does not qualify the complete Word owner feature.

## Defects

Zero findings in the reviewed clone delta.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. `crates/rdocx-oxml/src/content_control.rs:596` dispatches the recursive ContentControl variant directly. The exhaustive helper at608 clones each of the other six exact concrete variant payloads into the same variant. Its ContentControl fallback is also equivalent if called directly. No field, raw byte, ordering slot or alternative is omitted. `CT_Sdt` remains derived Clone at670, retaining properties, revisions, namespace bindings, root/content attributes, slots, original run sources and pending field edits declared at671.
- Contract: zero findings. Only cloning implementation and its distinguishing test change. No admission limit, namespace classification, table depth, XML replacement behavior, stack setting, timeout or source authority is changed. The existing concrete Clone trait remains available on all seven variants. Standalone32-table/256-control clone qualification is distinct from the approved complete lifecycle and transaction obligations.
- Panics: zero findings. The new clone implementation adds no indexing, arithmetic, unwrap, unsafe operation or input-dependent recursion beyond the existing actual content-control structure. `#[inline(never)]` at607 isolates large modeled leaf cloning from the recursive control frame. The normal-stack measurements qualify the exact retained input, without asserting stack safety for every native lifecycle or arbitrary model graph.
- OOXML: zero findings. The repair does not parse, normalize or serialize XML. Each existing typed payload uses its original Clone implementation and RawXml clones its owned byte vector at616. The seven-variant test at2182 asserts complete value equality, identical serialized bytes, exact foreign XML including attributes/entities/comment, and original raw bytes unchanged after clone-only mutation. No foreign child is promoted and no schema child order changes.
- Tests: zero findings in the bounded repair. The existing standalone diagnostic at source19 `crates/rdocx/src/document.rs:37029` independently builds32 nested tables and256 nested controls, proves every typed level and terminal text, checks original package bytes, clones and drops both staging Document and CT_Document, and drops the original. The seven-variant test is deliberately Standalone context so each concrete carrier is exercised without changing production owner admission. Source18 ran the same two standalone controls and both aborted. Source19 passes both with the same input and ordinary2MiB stack. Those are genuine distinguishing successors, not method-presence or output-derived expectations.
- Structure: zero findings. One concrete existing-file helper, no new type, module, file, dependency, generic, registry or framework. It isolates the leaf frame with exhaustive direct operations rather than adding forwarding wrappers or another cloning model. Existing parse/write/field definitions remain unchanged.

## Authenticated source and evidence

Reviewed worker is `work/f-x211a-codex`, HEAD `564bb645e5e93fbc986b86162a27da9d8a31bbe4`. Capsule `clone-isolation-checkpoint-19.json` SHA-256 is `5fdd0d80656021f72fe25f04f8f2fe79ec2f09028412bcc5018e5fd3479aaf8f`. Source19 map SHA-256 is `e041fa9e019b81d89aea06b7cd4ee08ccb73e01c5c00d8eb9bf5b4406eaaab03`, with3,208 entries. Independently matched the complete current map at the initial snapshot and found only content_control.rs differs between source18 and19.

Actual pre-edit content_control.rs SHA-256 is `0435d8d50946a0505f3b43dfe189b323f2b127560f3b22e7b8407d1e2a467e2a`. Retained and current reviewed source19 carrier SHA-256 is `e3f8c53f1a08ea49bca0ee7f8d1e5e9744fe2d5fd077ee4a0bd0f7760302ef2f`. Retained source19 document.rs SHA-256 is `b6b0be738d313719f023232922aecd0ccf3080e917ea2397d37431ab02c9a848`, identical between source18 and19. Later root-authorized cfg(test)-only flush diagnostics may change the working document carrier, so test citations and this review bind to that retained source19 version. The production clone carrier remains frozen and independently matches before and after review.

Sealed source19 receipt SHA-256 is `d8a575b8dbcc1913192c665faad2f206b0fdb5e6e3edac32752a992631c6e753`. Authenticated all six row logs, source map, retained native binaries and cleanup facts. Three preparation rows pass: affected all-target check, the seven-variant debug control and release native test-binary compilation. Both actual standalone clone/drop rows pass one control each on2MiB stack with30-second deadlines. Both complete without timeout. The source14 failed input and both source18 input artifacts have the same SHA-256 `57730bd394238c3aa749f759d907ed9b3dedf2f21a400f7e7973546789b157a5`, also pinned by the source19 standalone rows.

Original source18 receipt SHA-256 is `926979655e7238d44df5921cd69f57dccf7014d00a3dff03abdb6e69b544b5de`. Its two genuine standalone clone failures are exit-6 and remain preserved. Independently authenticated their logs and both retained binaries. Current retained source19 Word binary SHA-256 is `d316880f66abfbff00dc2b68e5abc2dafe11f62f3ae19d1e753812017c039a26`, OXML binary is `4329f8053d497a5d9d82fe568b758d6d90ea9077e2d6daeebb1dae83fa7f2106`. No compilation or runtime was performed by this review.

## Unfinished qualification

The final source19 maximum transaction row remains actual exit-6, qualified=false, no timeout. Its log reaches clone-after then flush-before before stack overflow. The exact flush cause is not established by this review. All six recorded process groups are gone and leaders reaped, including the failed row. The sealed receipt explicitly distinguishes clone positives from this negative transaction, retaining the original running receipt rather than relabeling the entire driver as passed.

No complete selected-owner transaction, maximal85 resolution, complete resource lifecycle, installed public five-owner surface, frozen100 rate, whole child acceptance, scoped completion, hash baseline or native Office acceptance is inferred. The root-authorized separate diagnostic phase owns the remaining flush investigation.

## Review integrity

Read canonical workflow/microscope, approved parent and childa contracts, progress and cited current model/package preservation sections. No source, test, target, process, CP, UI, network, plan or ledger mutation occurred. Only this workflow review file was written in the existing F218 worker, as explicitly assigned by root. Its pre-existing tracked files, diff, HEAD and physical/logical index remain unchanged before and after. Retained source19 evidence and reviewed live production carrier authenticate before and after. Review ends here and returns control to root and the childa writer.
