# F-X268, final performance, pass 22

**Reviewed**: Frozen working diff against `8a3c658cd00dcb8be1f307088ca6ac28429746c2` on sprint/s95.4. Nine existing files, 566 insertions and 36 deletions. Read the approved combined normalization design, plan amendment, progress, canonical workflow/microscope guidance, pass21 and frozen diff. F-X268 remains in progress. All nine frozen hashes and the actual debug binding hash matched.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. This is local source and mechanism-evidence review, not final performance or feature acceptance.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Fixed-point equivalence and parser effects

The new path at `crates/rdocx/src/document.rs:10243` keeps the same current writer document and normal frame borrowed throughout. The left side actually executes its first writer, the checked normal-frame read of C1, and its second writer C2. It skips the two right rounds only after actual C1 and C2 both equal the actual right bytes. The would-be right reads therefore repeat exactly that already successful normal-frame read, and the same writer receives the same field-free parsed value. No equality of two differently framed models substitutes for the byte fixed point.

`scoped_normalization_reuse_allowed` at `crates/rdocx/src/document.rs:10205` checks current normalized owner facts and lexical field exclusion in both children and all four source/normal head/tail pieces. Examined the recursive owner predicate and its exclusions for fields, bookmarks, tab stops and nested owners. The lexical guard also excludes field spellings in opaque content or frame bytes. It remains conservative rather than interpreting only modeled fields.

The field parser allocates NEXT_FIELD_SOURCE_ID at `crates/rdocx-oxml/src/text.rs:1412`. These inputs retain the original reads instead of assuming parsing is pure. The other relevant production global is the scoped root-binding state, restored by Drop. No new clock, identifier reservation, media allocation or writer option intervenes between the equivalent rounds. Test-only counters are not production semantic state.

A failed left still evaluates the full right normalizer before returning false. C1 is dropped before a nonfixed right fallback. The original pair comparison, marker absence check and correspondence storage conditions remain. The approved marker clarification is respected: an otherwise pure checked fixed point can omit redundant right rounds, but a marker collision still returns false at the unchanged guard, preserving the caller's whole fallback.

## Producer read provenance

The seed is confined to the actual fresh producer builder at `crates/rdocx/src/document.rs:24999`. It follows the unchanged physical owner partition, body/root admission, actual empty-frame parsing and equivalence, checked source-child reads, source private/field refusals, clearing staged writer authority, and complete proved_document(source) admission. No head, tail, child bytes or scope metadata mutate between those steps and seed consumption.

The helper at `crates/rdocx/src/document.rs:10223` reads the already stored actual parsed child only after matching root declarations, body declarations and body bindings. It rechecks the field/private side-effect guards. The shared consumer checks eligibility again. The ordinary scoped-proof caller passes None and retains its first read. Masked probes retain both checked normalization stages. No current-model child, staged writer child, PartialEq-only result or earlier opening supplies the seed.

The existing source read already checked closure, one-child cardinality, shell serialization and private round-trip restrictions under these exact bytes and frame. Reusing that value removes a duplicate read, rather than creating a new source or writer proof. Every producer child is admitted before the correspondence loop. No new persistent cache, all-child seed vector or predecessor record is introduced, and no work moves to document opening.

## Existing authorities and aspects with no findings

Correctness and contract: zero findings. Both duplicate eliminations remain within the approved shared normalizer. No picture inventory is narrowed to a target. The complete native inventory, related-part validation, current owner selection, binding publication and returned-picture handling remain unchanged.

OOXML: zero findings. The factored reader retains exact closed-body admission, the actual source/normal empty-frame serialization comparison and cardinality. The w14/root binding test runs after the actual first writer as before. Resource admission and actual body_item_xml calls remain. Fixed-point results do not supply writer eligibility or authorize changed private fields.

Error ordering: zero findings. Examined failed left, unequal right, changed frame, missing w14, field-bearing frame and marker branches against the old eager pair. Ordinary fallible parser/writer results remain authoritative. Only proved pure duplicate calls are omitted.

Panics: zero findings in changed production paths. Checked range arithmetic and existing root-frame lookup remain optional fallbacks. No new unwrap, unchecked source slice or fallible operation is assumed successful.

Structure: zero findings. The two private substantive helpers factor actual checked reads and normalization after an actual read. No new source file, module, trait, generic, public feature or cache field. The debug-only binding observation has no stable stub or release API promise. The temporary third Vec result is retained only for the fixed-point comparison, then dropped before fallback.

## Discriminating evidence

Actual old-artifact RED used debug module `107157fb0aa2b0ff5ffc78558ae482fcdd531060d826e8375d3b0f5ba6c35671`, before either optimization. All four exact pinned public cases failed the intended new work assertions. They recorded positive seed/fixed eligibility and actual left/right reader/writer rounds, with whole parsed bytes zero. A fixture error was not called this RED.

The authenticated current module is `/private/tmp/s954-normalization-green-site/rdocx/_rdocx.abi3.so`, SHA-256 `486c6cc7c17af21fb40a1394954c61ea224ddedd9d27ed20d021a2b86302ea0c`. The exact pinned Document.add_picture and Cell.add_picture at25/100 show all56/139 producer seeds and fixed points consumed. Actual total child reads fall from349/847 to181/430, satisfying3P+13. Whole parsed bytes stay zero. Left read2 and write1/write2 remain positive, while the duplicate left read1 and both right rounds disappear. Real parser attempts and body/frame writer attempts/bytes are recorded at their call sites, including failed writer attempts. Zero cannot be achieved merely by abandoning the producer proof.

The repository Python test at `crates/rdocx-py/tests/test_core.py:15118` is a simpler functional/work fixture and uses a looser additive allowance20. It is not substituted for the separate exact pinned driver, which actually enforces13. Returned bytes and reopened picture inventory are checked. The installed broader batch retains both picture publishers and existing owned publication controls.

The native controls at `crates/rdocx/src/document.rs:51182` cover actual seed use, mismatched scope refusal, generic unseeded first reads and a real uncached reference with both actual right rounds. Separate controls exercise fields, bookmarks, tab stops, failed left, unequal bytes, a changed frame, lexical field-bearing frame, missing w14, force-off and marker rejection. The field/private/failure controls require actual right reads and writes, rather than accepting an unsupported shortcut.

The first GREEN native run had20 passes and one invalid positive fixture: declaration-less factory XML correctly failed the unchanged producer root guard before exercising the optimization. The correction preserves that input as a negative control and supplies actual declared canonical XML to the positive case. The corrected group has3 passes, overlapping2 already passing controls. Verified21 unique native passes from actual test names, not23.

Python reports26 passes. Both affected-crate clippy and corrected-native clippy exited zero. Hash harness matched49 entries. Recorded fmt, prose, adapters and diff checks pass. No reviewer build or test was run.

## Source identity and remaining qualification

Verified the first51181 lines of the already-built binding's native source snapshot and current native source are byte-identical, SHA-256 `167a69bf85917bd72a22cf669d10c204f4e805b6059b4e40fe70cb8021fde3c2`. This is the exact recorded prefix before the corrected native test body. Independently compared the complete pre-tests production prefix as equal. The retained correction diff changes only cfg(test) fixture/comment code. The plan/HLD marker wording change records Root's approved clarification. A new production binding build is not inferred or needed for those test/prose-only changes.

Plan and HLD03/04/10/12 describe the exact producer provenance, pure fixed-point guard, actual counted work, unchanged inventory/binding contract and pending final gates. HLD14 remains the unchanged in-progress qualification record.

No final release timing, full A9 rerun, memory qualification, Word oracle, feature completion or sprint acceptance follows from this local repair. Both picture timing gates remain to be measured on the final reviewed normal artifact. The 79-call, five-linear, Original407, memory and external riders remain required. Q25/F288 and the separate prepared font qualifier were not changed or accepted in this review.

## Frozen source authentication

| File | SHA-256 |
| --- | --- |
| `.claude/plans/F-X268-design.md` | `89d6b3a762bcbd76c4bb58a4d534e5ec7174df2db97f9e5983d69c65af2a17c2` |
| `crates/rdocx-py/src/document.rs` | `e8b66469d5355e4118e50db188142f5b64ef59f2bb17f1c13ebf8b00fa662608` |
| `crates/rdocx-py/src/lib.rs` | `3e022b4e9d5cb02ed35906b6797bbf478c228af75e7e784c0a673c589e2b8642` |
| `crates/rdocx-py/tests/test_core.py` | `39a3d9300620d09c539792d954404ca907ff8dd294190937209baccdf21ce72a` |
| `crates/rdocx/src/document.rs` | `f970f62befbb880f8d4eea74e118f8ef197c8e1db67885fabc00e16f76459a89` |
| `docs/hld/03-architecture.md` | `65623713114c0663a92622951e887a6308e698710872d21c0f35abf1325f498a` |
| `docs/hld/04-opc-and-packaging.md` | `1e8f4f1ba31b08b8fecaec80a71e001919ace28b92c4fa5f75bc06e14543724e` |
| `docs/hld/10-bindings-spec.md` | `d46c64c52481f94098d8750ad9410cda352864f22267e4d0d6c9021bf810630e` |
| `docs/hld/12-testing-strategy.md` | `a93903dcc4003c88b9003b3a79c8d2c3d87f11c7098cdab036f38cf1d82ecbdf` |

