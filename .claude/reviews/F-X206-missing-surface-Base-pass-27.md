# F-X206, missing surface Base tests, pass27

**Reviewed**: Bindings-authored external test-only proposal, three existing carriers,64 added lines and4,798-byte patch. Scope is two native semantic discriminators and two installed public-capability discriminators. This is test admission, not a production or completion review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt SHA-256 is0e0e1c139ce3723d46327b2b68c025b48609ccae93fec6b477e5870e1ef812af. Patch SHA-256 is43eb299925ba1a749b04254ee76640eeb152a94f3509f2545b0c8a8545fbedca. Actual2714 current source bytes, sizes and modes match source-map53debc893276ef3db4baa380c971dc994ba0d835df8406f933335d75dd57c875. Worker branch is work/f-x206-codex, HEAD7b1e5ef29464e93ae30e68df1523ce09143578ff. Canonical sprint and backlog record F-X206 in-progress. The approved full plan, current progress, workflow, microscope contract and relevant model/render/binding sections were read.

Independent in-memory application verifies every hunk against current bytes and reproduces all three declared candidate hashes and sizes. context.rs adds15 lines, native lib.rs adds18 lines and the existing Python test carrier adds31 lines. Python AST parsing succeeds without import. Native candidates have not compiled or executed. No source file was changed or temporary candidate tree created by this review.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect coverage

Correctness has zero findings. The table fixture uses the existing admitted table resolver and a real single cell. Its conflicting bodyPr bottom and tcPr center make the expected effective cell anchor a semantic discriminator. Current context.rs:6679-6688 provides the corresponding valid bodyPr-only positive path. The new test uses existing native functions and types, rather than a missing prototype whose compiler failure could masquerade as Base evidence. Table/body presence preconditions must genuinely succeed at runtime before the center assertion earns semantic RED credit.

The naming test uses the actual Title and Content layout, native add_slide, package save/reopen and native shape inspection. Two placeholder owners and distinct IDs precede the generic-name discriminator. Current rpptx/src/lib.rs:3432 calls rpptx-oxml/src/shape_tree.rs:945, whose actual names remain Placeholder plus ID at:959 and:962. Rejecting that generic form is a meaningful bounded Base control. It deliberately does not establish exact PowerPoint spelling, numbering, every supported layout or imported-name preservation. Those remain full acceptance obligations. Save/reopen byte equality is retained, not replaced by a loose parsed-content assertion.

Contract has zero findings in this finite admission scope. Cell tcPr ownership, PPT internal run targets, picture placeholders and native naming remain F-X206, rather than new Word F-X202 scope or a template-only F-X205 workaround. The retained immutable PR317 head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0 supplies the tcPr/bodyPr ingredient once. Its unrelated contributed bindings, colours, archive or baseline claims are not adopted as evidence. Completed F-X194 bodyPr and F-X197 colour contracts remain intact.

Tests have zero findings within the declared Base limits. The Python cell test first constructs a real table cell, confirms empty text and absent frame anchor, then checks the distinct Cell descriptor. The run test first constructs a real owned run and reads its actual Hyperlink.address, then checks the proposed target_slide descriptor. These are capability-absence controls only. A descriptor could exist without correct setter semantics, so future GREEN on them does not establish either feature's behavior. Hyperlink.target_slide remains subject to coordinator signature confirmation before adoption, as explicitly recorded in ownership-and-limits.md. Neither test invokes a missing API to obtain a compiler or parser failure.

Picture-placeholder behavior has no discriminator in these four tests. The packet expressly records that gap. Generic picture insertion followed by placeholder deletion is not an acceptable substitute. Full future native and installed acceptance must prove owner-aware replacement, retained ID/placeholder metadata, child order, inherited geometry, landscape/portrait crop, image bytes and relationship closure, failure atomicity, independent paint and save/reopen. Internal run target acceptance must likewise prove target ownership, ppaction://hlinksldjump, pruning, invalid and cross-owner refusals and the existing cell-link refusal before mutation. Cell acceptance still requires the actual enum/absence/no-op/edit/reopen and preserved grid/text-owner matrix. All-layout native naming and PowerPoint open-without-repair remain separate requirements.

Panics has zero findings. Fixture unwraps and indexing are in source-built test controls. A setup panic, parser failure or compile failure receives no semantic Base credit. Tests must be observed reaching their intended assertions with the positive controls intact. No untrusted product path is changed.

OOXML has zero findings. The literal fixture uses the actual DrawingML and PresentationML parent context and standard table child order. tcPr is not relabelled as bodyPr. Production parsers, source preservation, namespace handling, opaque children and serialization are unchanged. The capability tests read source before any prospective mutation. Full future native/installed semantics must retain qualified aliases, rebound and foreign payload, no-op identity and repeated save/reopen controls.

Structure has zero findings. The proposal changes only inline native tests and the existing installed Python test entrypoint. It introduces no product file, module, trait, generic, dependency, fixture asset, public implementation or duplicate provider.

## Handback

Only this canonical review report was written. The proposal remains unapplied and unexecuted. No Cargo, compiler, product import, installed test, native UI, cache, mtime, baseline or source mutation occurred. Genuine Base evidence requires ROOT admission and actual named runtime results with successful setup. The report grants no full F-X206 acceptance or completion credit. This pass ends here and returns control to orchestration.
