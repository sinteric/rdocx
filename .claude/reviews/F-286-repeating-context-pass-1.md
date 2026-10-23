# F-286, repeating context, pass 1

**Reviewed**: Focused post-completion B2 typed-context amendment on `sprint/s95.3`. HEAD and comparison base are `3277ed2853f6bc203ed27ae1299de7c04e55d229`. The working diff contains 11 files, 488 insertions and 47 deletions, 535 changed lines. Complete diff SHA-256: `b6d6e4bc0cbc196a6e9ace69fe74db9adeaeaa1e835f7e347e97a48a73e44cc4`.
**Reviewer**: Codex, this actual independent review session. This session did not implement the amendment or delegate its review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the focused amendment. The examined B2 physical-placement and ownership failures are repaired. This is not final sprint pass 5 or a sprint acceptance decision.

Read AGENTS.md, CLAUDE.md, the canonical workflow and microscope command, approved F-285/F-286 designs and applicable HLD contracts, the independent repeating-placement investigation, custom-XML placement review and sprint pass 4. CURRENT_SPRINT and BACKLOG retain F-285/F-286 as done. The explicit amendment request and run-sprint section 8 govern this post-completion review. Historical delivery records are not reopened for new feature credit. B1/B3/B4 retain their pass 4 dispositions. No concrete newly applicable defect was identified that would reopen them.

## Defects

None, 0.

## Smells

None, 0.

## Nitpicks

None, 0.

## Contract and publisher examination

Rechecked the primary [MS-DOCX 2.5.1.11 item rule](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/6b50d4d0-3f5d-4d8d-b507-9879bd342f3e) and [2.5.1.10 section rule](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/b1768ff2-c9fe-4660-86e5-73cd8c94b0a0). Item containment and physical block, row or cell placement are separate requirements. Section content consists of item controls. Inline section refusal follows from combining those requirements. Containment does not impose immediate parenthood. The F-286 amendment applies these existing invariants without adding a general imported-schema validator.

At `crates/rdocx/src/content_control.rs:1552`, every payload passes `validate_repeating_type_change` before marker replacement. That helper independently checks proposed repeating placement, item containment, direct-item retirement and section ownership loss. Empty nested controls and metadata-only customXml cannot establish a physical level. At line 3149, `element_slot` requires qualified Word identity and treats qualified customXml, sdt and sdtContent as transparent. Paragraph, table, row and cell remain physical discriminators. This ancestry assessment is source-derived, including customXml ancestors that the Python story inventory does not generally expose as independent addressed controls.

Move and clone at lines 1647 and 1709 retain two actual insertion routes. Direct block sources delegate to fragment insertion at an owner boundary, including when the destination location names a paragraph. They do not insert inline into that paragraph. Their new admission checks the complete source shell and the destination story owner's enclosing section. Nested relocation uses the actual destination element's grammar and ancestry at line 3126. Full source XML retains inherited namespace bindings for type inspection. `sdt_payload_matches` at line 2995 checks outer and transparent nested repeating markers before payload recursion. Concrete paragraph/table boundaries stop inherited-grammar descent, preserving the bounded placement policy rather than recursively validating every retained subtree.

`control_matches_context` at line 3774 also inspects retained items whose required section lies outside the moved range. A section carried inside the range retains ownership. A destination enclosing section can supply ownership. An item cannot rely on a source section left behind. The direct-story helper at line 3803 inspects the actual story owner span, which matters for cells inside row-level repeated items.

Section retirement at line 3726 refuses loss of the nearest section owner for retained items while permitting an enclosing section to retain them. Direct-item type retirement separately refuses a non-item replacement. Unwrap at line 3815 requires item-only replacement of a direct item and retains at least one direct item. Removing a section deletes its owned content and remains allowed. Last-direct-item movement and removal use `direct_item_section`, avoiding an invented prohibition on every nested item. Unwrap otherwise retains its existing payload and cell-paragraph checks.

Repeat retains the existing template-copy path. Existing-item conversion remains ahead of ordinary block wrapping. Legal row/cell/block branches therefore remain supported, and malformed imported item payloads are not newly subjected to comprehensive validation. Python invokes these native operations without a second binding policy. No new trait, generic, module, dependency, unchecked indexing or panic path was introduced by this diff.

## Assertions and executed challenges

Read the assertions in `crates/rdocx-py/tests/test_core.py:6641` and the existing 48-scenario customXml matrix. The new test distinguishes inline item ancestry from physical placement, ambiguous empty wrappers from matching block positives, section/item retirement from compatible nested retirement, and illegal unwrap from item-only replacement. It exercises legal block/row/cell set and repeat after reopen, nested item containment, typed relocation refusal, compatible row-section relocation and row-item ownership loss. Refusals assert complete package-byte equality. Compatible relocation distinguishes retained move ids from fresh clone ids. The Rust test at `crates/rdocx/tests/integration_test.rs:26566` independently checks native error variants and atomic bytes for the core negative cases.

This reviewer executed both Python matrices directly from their current AST definitions using Python `-B`, in-memory packages and the authenticated installed extension. Both passed. No pytest cache or fixture file was written.

Five additional bounded public-operation probes passed:

- Move and clone of an ordinary nested rich shell carrying a deeper typed section into a paragraph both refused atomically. This challenges nested type inspection independently of the matrix's outer typed section.
- Moving the last direct item refused atomically.
- Cloning a direct cell-story rich control carrying an item dependent on an enclosing row section into the body refused atomically.
- Cloning the same source to the end of its compatible cell story succeeded and retained two source-tagged controls. This discriminates actual destination ownership from blanket rejection of direct-story sources.

These are installed-library observations, not native Word parity. Qualified ancestry, marker retirement predicates, transparent boundary coverage and staging were additionally traced in source. No exhaustive schema or namespace conformance claim follows.

## Source and retained evidence

Independently matched all 11 entries of `/tmp/s95.3-repeating-context-review-source.json` to current files. Product source SHA-256 is `cffcddc4d2043860e2ffd27914b6328177a3119473540deaa9ddb1eceacb00d7`. Python test source SHA-256 is `ccfeacbc74685b33f65f285fcaf19692afd22013ad64b596a253ae489bd0c86b`.

Authenticated the installed Python 3.12 native before execution as `e07f191d8dd34d036eb35ad14c60aa5e6b499227c432e2626f97143302cd539b`. Independently matched the retained wheel and embedded native to the artifact receipt. Wheel SHA-256 is `dcd30088fd29c60c19979737dfca3daf8e401f90cf05e30a49715da928a43a7b`. The receipt explicitly describes working source, not a clean Git build.

Read the retained RED, build, matrix, integration, installed, typing, lint, WASM, policy and final baseline logs. The actual old-native `3958f3a4` RED fails on accepted inline item publication. The remaining newly covered negatives are not individually relabeled as historical RED. Current installed receipts authenticate native identity and child exits 0 for full Word 1065, exactly 42 priorities on each interpreter, strict typing and stubtest. Both retained matrix runs pass. The Rust aggregate remains failed with 411 passes, one fixture failure and eight ignores. The corrected marker fixture then passed alone with 419 filtered out. This supports 412 composed cases, not a successful rerun of the aggregate.

Retained scoped clippy and both WASM logs finish successfully. Policy reports 192 tests with two skips, meaning 190 passes. Final hash, deterministic pixel and README logs report 49 unchanged hashes, seven matching page-one pixel buffers under Poppler 26.01.0 and successful README checks. These are consumed observations, not checks executed by this reviewer. Archive measurements remain no-verify inventory evidence.

## Not found and limits

Correctness, contract, panics, OOXML preservation, tests and structure each produced zero findings within this amendment. F-285 A3/A4 and F-286 A7/A8 are the affected acceptance contracts. Other typed payload semantics and Q24's documented native-default differences remain outside this focused reassessment. No whole external issue closure, new rendering parity or unrelated resource obligation is inferred.

A genuine clean verified affected archive, actual updated hosted artifacts and independent final sprint pass 5 remain pending. Historical successful hosted run 39806338556 does not qualify changed Word source. This clean focused review supplies none of those missing qualifications and consumes no final sprint review pass.

Only this report was written. No build, broad gate, native UI, subagent, remediation, source/scratch/state/plan/ledger edit, commit, push, publication or sprint decision was performed. Review ends here.
