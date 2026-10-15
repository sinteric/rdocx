# F-X206 cell-anchor production review, pass 29

## Scope and evidence

Independent static review of the unapplied nine-carrier R2 proposal at `/private/tmp/S91-F-X206-cell-anchor-production-r2-prepared`. This reviewer did not author the proposal. No source, proposal, runtime, cache or timestamp changes were made.

Authenticated receipt SHA-256 `d66e9bce03af1fabee4cb10f66851304efc5647b26ce21f26ac04481251fdeaa`, complete diff `93ecccdc79274a8f0c77a4e3c8f8586f2b7c5fda0b835ad3e0af925ff57634da`, incremental diff `38d5f174d3d4797c96cfcb8810a442f5457a74cc2618c003572a0cfd08d0f209`, and all receipt-linked evidence. Reconstructed the complete diff from current worker bytes and all nine candidate bytes. Every candidate size/hash and current mode matched. All 2,714 current worker input hashes matched retained source map `53debc893276ef3db4baa380c971dc994ba0d835df8406f933335d75dd57c875`. Only the table model differs from R1. The other eight candidate files remain byte-identical to R1.

Read the approved F-X206 contract, its referenced specifications, current progress, preceding reviews and the full candidate delta. Reviewed direct cell-property anchoring separately from text-body anchoring, qualified owner selection, inherited/default/alias/rebound namespaces, opaque attribute spellings and child payload, lazy clear/no-op semantics, existing live-handle publication, shared enum conversion, native layout precedence, stubs and controls.

## Findings

### D1. An embedded patch header makes the layout candidate invalid Rust

`/private/tmp/S91-F-X206-cell-anchor-production-r2-prepared/candidates/crates/rpptx-layout/src/context.rs:6687` contains the literal line `++ b/crates/rpptx/src/lib.rs`, between two `#[test]` attributes at lines 6686 and 6688. This is a patch-header fragment in the Rust module, not a comment or string. It prevents the candidate from compiling, including the new cell-anchor layout discriminator at line 6689. The duplicated test attribute should also be resolved when removing the fragment. R2 retains this file byte-identically from R1, so the table-only R2 correction does not address it.

Remove the transport fragment and retain one test attribute in a separate remediation phase. A successful patch-application check and Python AST check do not validate Rust syntax. Actual compiler and meaningful semantic test results are still required.

## Six-aspect results

1. Correctness: one defect, D1. No other confirmed defect.
2. Approved contract and public API: zero additional findings. Cell anchor uses the existing five-value enum and direct `a:tcPr` ownership. Clearing does not synthesize a text body. Existing checked read/edit publication remains the binding route.
3. Panic, failure and atomicity: zero additional findings. Invalid anchor conversion precedes mutation. Missing/identical anchor writes bypass materialization. Runtime validation is pending.
4. OOXML and preservation: zero additional findings in this bounded delta. The concrete namespace scope follows ancestor bindings and per-element overrides. Foreign lookalikes are captured as opaque siblings rather than consuming the owned cell-properties slot. Unqualified owned anchor is separated from qualified foreign anchor attributes. Raw foreign spelling and opaque descendants are retained. This is not a claim that unchanged descendant parsers have been universally audited.
5. Tests and oracle independence: zero additional findings beyond D1 blocking compilation. The field-free Base control uses existing `CT_Table::from_xml` and `to_xml`, with a valid positive setup before the foreign-owner discriminator. It does not obtain RED credit from a missing new field. The installed controls retain physical text descendants, body-property independence, lazy reads, idempotence, invalid input atomicity and save/reopen checks. No test was executed during this review.
6. Structure and maintainability: zero smells and zero nitpicks. Existing enum conversion has two real consumers. No new module, file, trait or dependency is introduced by the proposed tracked delta.

## Qualifications

A provisional fixture-owner concern was checked and rejected. Native `resolve_layouts` iterates master relationship XML order at `crates/rpptx/src/lib.rs:6660`, not lexical part-name order. In the authenticated default template, index 6 resolves to Blank. Its latent placeholders are excluded by `add_slide` at `crates/rpptx/src/lib.rs:3424`, so the appended table is the first shape in the installed controls at candidate `crates/rpptx-py/tests/test_documented_examples.py:7343` and `:7399`. The retained naming reference enumeration alone was insufficient to infer current API order. No finding is recorded for that concern.

The default template hash remains `8ecd98d4e52c8ece061cb36c8baa9f0424b362454d820f6df1e425c662b9a057`. Namespace, layout and installed assertions are uncompiled and unexecuted. Actual Base RED, scoped GREEN, fresh bindings and typing, renderer effects, native no-repair qualification, hash-harness attribution, documentation and public-source/package riders remain separate required work. This report neither approves execution nor completes F-X206.

Result: one defect, zero smells, zero nitpicks. Review phase ended. Return source ownership to the orchestrator for distinct remediation.
