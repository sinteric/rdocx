# F-X205, fill preservation correction, pass 6

**Reviewed**: Immutable fill-review4-correction-capsule, three complete existing carriers. Bounded scope is the optional transparency getter/stub and the original-reference constraint on native image pruning, plus its added preservation assertions. Other source27 implementation is context, not a new whole-feature review. Reviewer: fonts_resume, independent of this correction.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Evidence

Capsule: `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/fill-review4-correction-capsule`.

Manifest SHA-256: `c16307e561c358a4af7c87ac7bb0258dd1cc78bb74822e89918e6238312ce816`. All three candidate sizes and SHA-256 records authenticated. The source27 map SHA-256 is `942bce561522a59ac408843dfcd5e71a4baacc22228082f3714d9976ec796dbb` and contains the exact three copied candidates.

The retained source26 picture-preservation receipt SHA-256 is `72f8a58f55f280d80d58bb0844813a97b1a2687378228fc519ab09bf2b7ed3a9`. Its raw picture log authenticates the exact native picture-fill test with 1 passed, 0 failed and 0 ignored. That same wrapper failed at the subsequent notes-handout row, and remains failed. The source27 notes-only successor receipt `f18536a5a03ecbf1992e8d1ff67a49b24077dcf1161c7770153d1c634c61337c` records one separate notes test passed with natural cleanup. It is not a fresh picture test or an all-fill runtime pass. Both copied raw logs match their receipt hashes.

## Defects

None. Pass4 D1 and D2 are resolved in this bounded source correction.

At `crates/rpptx-py/src/dml.rs:632`, the checked transparency getter now returns `Option<f64>`. It retains owner validation, returns None for absent/non-solid fills without authoring a fill, and wraps the unchanged computed solid alpha result. The stub at `crates/rpptx-py/python/rpptx/_rpptx.pyi:613` now matches `float | None`. Existing non-solid setter refusal and finite range checks remain unchanged.

At `crates/rpptx/src/lib.rs:4987` and `crates/rpptx/src/lib.rs:5036`, both staged replacement routes collect actual qualified original-slide references before publication. Cleanup at `crates/rpptx/src/lib.rs:5090` now removes an internal image edge only if its ID was referenced before the selected mutation and is no longer referenced afterward. Because these operations change only the requested fill, that reference difference constrains cleanup to released source ownership. Pre-existing unreferenced image edges cannot satisfy the new predicate. Shared current users and external relationships remain protected. Scanner or publication errors still affect only the staged candidate.

The native control at `crates/rpptx/src/lib.rs:20154` adds an unrelated image relationship and distinct media payload, replaces and clears the selected picture fill, and proves the unrelated edge and payload survive. It retains the original malformed-image atomicity, shared-user retention, last-user pruning, absent-owner refusal, repeat-byte stability and ordinary native save/reopen controls. The new unrelated-edge subcase uses the genuine deterministic OPC writer for its final reread because the pre-existing native debug validator rejects unused IMAGE edges. It proves preserved OPC relationship and media bytes, not successful native `to_bytes` acceptance of that unsupported validation state. This limitation is explicit and does not weaken the original valid controls.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation/schema: 0. Tests: 0. Structure: 0.

## Limits

No fresh installed transparency/typing execution is claimed. Later pattern/backend paint, metadata, media and export semantics are excluded. The picture pass is retained at its actual source26 authority, and source27 contains separate context changes. This source review does not relabel the failed two-row wrapper or claim full feature acceptance. No code or runtime was changed during this review.
