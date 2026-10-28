# F-X268, final performance, pass 19

**Reviewed**: Frozen working diff against `d4736c7619d3812548ddb3f66f3a7ba899e7c581` on `sprint/s95.4`. Five existing files, 245 insertions and 18 deletions. Read the canonical microscope/workflow guidance, approved plan amendment, progress record, frozen manifest and actual diff. Every file hash matches `/private/tmp/s954-field-word-frozen.json`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. This is local source and evidence review, not final feature acceptance.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Lexical equivalence and lifetime

The new inline fact at `crates/rdocx/src/document.rs:8135` records exactly the old case-sensitive byte-window predicate. `holds_field_words` at `crates/rdocx/src/document.rs:8620` retains the same token order and short circuit. Test-only instrumentation counts windows actually visited, including rejected windows and early matches. Foreign attributes, text and other lexical false positives still count as field-bearing bytes. This does not introduce namespace interpretation, case folding or entity decoding.

Inspected every `BodyBlockScan` constructor and clone path. Actual writer construction at `crates/rdocx/src/document.rs:8945`, `from_source` at `crates/rdocx/src/document.rs:9958` and `child_scan` at `crates/rdocx/src/document.rs:10458` compute the fact from the exact bytes they retain. The frame-only constructor has empty bytes and correctly sets false. The sole direct malformed test fixture also has no field token. Derived cloning preserves both bytes and fact. `fresh_context_piece` copies the fact only while copying identical child bytes and clearing contextual proofs.

Byte edits at `crates/rdocx/src/document.rs:8726` reconstruct each changed block through `from_source`. Multiple edits of one child therefore recompute against the current edited bytes each time. Unchanged neighbors retain their immutable Arc. Tail edits create fresh contextual pieces with identical child bytes, preserving only the lexical fact. The former test-only direct byte assignment now reconstructs the block, preserving the immutable-bytes invariant rather than introducing a stale test memo. No remaining direct byte mutation was found.

## Consumer safety

All nine block-specific consumers replace the same old predicate with the corresponding block's exact fact. Current model/private-owner checks, staged equality, field-sensitive rereads, frame and scope comparisons, actual parsing, normalization and forced fallback remain in place. A false lexical fact cannot admit a changed private tab stop, bookmark or typed field by itself. A true fact still prevents the same writer, scoped and namespace shortcuts as before.

The separate whole-item owner scan retains its actual scanner at `crates/rdocx/src/document.rs:30543`. No block fact is applied to unrelated assembled bytes. The inline value supplies no source, namespace, parser or writer authority, and no previous-opening chain is added.

No additional findings in correctness, contract scope, panics, OOXML preservation, error ordering or structure. The plan and HLD describe the narrow lexical meaning and retained guards accurately. The measured 808-byte struct layout before and after is target-specific layout evidence, not a declaration that memory qualification is unnecessary.

## Discriminating tests and evidence

The warm test at `crates/rdocx/src/document.rs:50224` exercises actual repeated unchanged writer checks. Its control traverses the same block bytes with the old scanner and must record positive work, while the optimized checks require zero token windows. Constructor controls cover empty/short input, both tokens, casing, escaped spellings, foreign attribute and text occurrences, staged versus source records, clone and frame-copy behavior. Exact early-exit counts prevent replacing this with a constant function-entry counter.

The lifetime test at `crates/rdocx/src/document.rs:50343` adds both tokens through multiple edits, removes each independently, checks byte restoration and unchanged neighbor identity, and changes the frame while confirming that contextual proofs clear. Existing private fallback tests now also establish false lexical facts for non-field private cases while still requiring the authoritative writer.

Consumed actual logs without running builds or tests. `/private/tmp/s954-field-word-red.log` records the intended pre-repair failure with 233,700 visited windows and a successful compilation. `/private/tmp/s954-field-word-green.log` records all three mechanism tests passing in 0.07 seconds. Both print an 808-byte block size.

The enlarged-stack affected batch records 69 passes, zero failures and one ignored external Word oracle in 176.75 seconds. Five integration range tests pass in 7.62 seconds. The structured verification receipt records successful child exits for those runs, scoped all-feature clippy and hash. The actual clippy log completes successfully, and the actual hash log ends with all 49 entries matching. The initial default-stack abort is not treated as a pass. No source change followed that abort before the successful rerun.

## Limits and identity

The current source SHA-256 is `62d4acadb2c6a269f7faf602c39be01fe5f6fedc6b56efa95d20184d1a013309` for `crates/rdocx/src/document.rs`. All five manifest hashes were checked against the frozen tree.

No full A9, final release build, timing, memory or native Word acceptance is inferred. The inline fact belongs to the existing parent cache's Q22 footprint. The unchanged 79-call timing, full catalogue and final integrated gates remain required.

The reviewer changed no source and ran no builds or tests. Review is complete and the source freeze may be released to the orchestrator.
