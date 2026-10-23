# F-285, custom XML placement, pass 1

**Reviewed**: Focused post-completion B2 amendment on `sprint/s95.3`, against `8fd3cd292798fdbde155430e0b322b1631b776ea`. Includes separate process-only commit `4ebd83bd788400029eec4a94b73652d1bd3171f7` and the seven working-tree files. Total delta is 12 files, 189 insertions and 23 deletions, 212 changed lines. The complete reviewed diff has SHA-256 `fd5cabe27fa81478c6577e2a7554b24fefa677ae9e1cd99364f440b9ef1cd2ea`.
**Implementer**: Codex, the separate amendment implementation session identified in the existing progress record. Grok implemented the original F-285/F-286 workers. Git author Atul Sharma does not identify the implementing host.
**Reviewer**: Codex, this actual independent review session. This session did not implement the amendment or impersonate a prior reviewer.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this focused amendment. The applicable B2 customXml admission bypass is repaired. This is not final sprint pass 5, sprint acceptance or a reset of reviews 1 through 4.

Read AGENTS.md, CLAUDE.md, the canonical workflow and microscope command, independent sprint pass 4, the F-285/F-286 contracts, current HLD03 and the amendment progress and actual evidence. The explicit post-completion review request governs this amendment despite the ordinary microscope restriction to in-progress features. Completed delivery records remain historical. B1/B3/B4 retain pass 4's bounded accepted dispositions. No concrete new applicable defect was found that would reopen them.

## Defects

None, 0.

## Smells

None, 0.

## Nitpicks

None, 0.

## Contract and publisher examination

F-285 A3/A4 require compatible relocation and unwrap, identity handling and atomic refusal. F-286's repeating conversion requires an existing item or a block sequence. The amendment clarifies that existing contract in `.claude/plans/F-285-design.md:204` and `docs/hld/03-architecture.md:1861`. It does not introduce a general imported-document schema validator.

At `crates/rdocx/src/content_control.rs:2971`, customXml now inherits the same `ParentSlot` through its payload. Only a qualified `w:customXmlPr` immediately owned by customXml is skipped as metadata. Other children still undergo admission. CustomXml properties and attributes are inspected without rewriting them. Recursive customXml and sdt branches retain the caller's slot, so alternating wrappers cannot reset a row, cell, block or inline requirement. Empty-element handling does not introduce a payload to misclassify.

Examined the entire admitted-child policy at `crates/rdocx/src/content_control.rs:3075`. The context-dependent admitted structural wrappers are sdt and customXml. Inline-only hyperlink, field and revision containers do not become admissible block, row or cell content through this amendment. Paragraphs, tables, rows and cells are the concrete slot discriminators. This review does not demand recursive schema validation of every retained paragraph, table, inline container or opaque metadata subtree.

Traced both relocation routes at `crates/rdocx/src/content_control.rs:1656` and line 1707. Direct block sources check their block payload before delegating to the existing fragment boundary operation. Nested sources reach destination admission at line 3127 before splice/publication. A direct-body source relocated relative to a paragraph remains a body-boundary operation. It is not inline insertion into that paragraph. The corrected block mismatch fixture nests the source inside another control, while compatible block controls remain direct body children.

Unwrap reaches the same inner admission through `parent_can_hold` at line 2905 and the staged rewrite at line 1790. Remove does not publish the removed payload elsewhere. Ordinary repeating conversion at line 3460 now calls the same transparent-child policy instead of a separate name-only block predicate. `set_content_control_type` at line 1575 wraps only after admission. Existing repeating-item recognition at line 3453 remains ahead of ordinary conversion admission, preserving the approved existing-item branch. Repeat at line 1605 clones an existing item within its existing section and does not convert an arbitrary payload into a new grammar. No new promise to validate historical imported repeating items is inferred.

The Python wrappers at `crates/rdocx-py/src/document.rs:8574` and line 8588 still invoke the actual native relocation methods on staged candidates. The installed regression calls those methods directly. No substitute API or binding-only policy is credited.

Decoded boundary checking remains shared through `payload_has_nonspace_text` at `crates/rdocx/src/content_control.rs:3031`, including customXml's boundary before recursive descent. Literal text, CDATA and decoded references are distinguished from text inside legal runs or paragraphs. Repeating's outer boundary predicate has equivalent event handling. The amendment adds no unchecked byte indexing, arithmetic, trait, generic, module or forwarding abstraction. The process commit sharpens review instructions without claiming product remediation. Both generated adapters' source hashes match their canonical commands.

## Evidence examined and challenged

The installed matrix at `crates/rdocx-py/tests/test_core.py:6641` constructs row, cell, block and inline payloads under direct customXml and alternating customXml/sdt/customXml wrappers. Its 16 incompatible move/clone combinations assert the error and complete unchanged package bytes. The 16 compatible combinations distinguish move identity from clone identity and check retained wrapper attributes, properties and exact customXmlPr bytes after save/reopen. Eight unwrap cases retain customXml while removing the addressed shell. Repeating refuses the six non-block cases atomically and converts both block cases. The source selector inspects the addressed control's own property boundary, avoiding an outer container selected only because it contains the nested tag. These fixtures reject name-only admission without rejecting compatible opaque metadata.

Existing native nested-control and reference discriminators at `crates/rdocx/tests/integration_test.rs:26462` and line 26665 remain relevant supporting coverage. They are not mislabeled as new customXml RED evidence.

This reviewer additionally executed 18 bounded, in-memory public API probes using Python `-B` and the authenticated final installed extension. An alternating customXml/sdt/customXml payload was challenged through clone, unwrap and repeating conversion. Decimal nonspace, a predefined nonspace entity and nonspace CDATA refused with complete byte equality. Decimal space, hexadecimal space and whitespace CDATA succeeded and retained metadata. All 18 passed, process exit 0. These are boundary-policy probes, not native Word parity, a full installed rerun or evidence of a valid-input historical RED.

Retained observations were read from `/tmp/s95.3-b2-customxml-*` and ignored `.claude/scratch/S95.3-takeover-evidence/b2-amendment/`:

- The actual old hosted native `5c2997d2cc6ad01d8946ffac013469887d9059b88856af87ef072669aa7eed06` produced the semantic `DID NOT RAISE` RED on the initial row mismatch. The attempted pip installation failed with `No module named pip`. Its subsequent apparent GREEN used old local native `7cea4027...` and supplies no new-artifact evidence. The proper uv installation authenticated pre-clippy native `5d6e1ec6cb9811c54dda712226f7211d7eae9fbab6d611bfe5c836b95b4e96a0`. Its first matrix exposed the mistaken direct-body fixture assumption. The corrected matrix then passed. Those outcomes remain distinct.
- Clippy's subsequent if-let collapse was syntax-only. The final rebuild log reports successful release compilation and wheel creation. Independently hashed the final wheel and its embedded native and checked all hashed RECORD entries. Wheel SHA-256 is `c3fc52ffcf9edf4eb114a87601bb7f5c93b48851a1901f4698fe12cdf221421c`. Native SHA-256 is `3958f3a45317ddb22158867a541e42cbb1c7e05a78b6548e458605c9e9e2101f`. Both installed Python 3.9 and 3.12 extension files match that final native, not either historical binary.
- Final installed receipts and logs authenticate that native and record actual child exits 0 for the complete Word suite, 1,064 passed, both 42-case priority suites, strict mypy over nine source files and stubtest over eight modules. Both interpreters' final expanded metadata matrices report one test passed. The earlier full-suite test file hash predates the added metadata assertions. The final expanded matrices separately cover those assertions against unchanged product source.
- The final native aggregate command remains failed. Its log contains 710 library passes with 14 ignored, followed by 407 integration passes, four failures and eight ignored. The four exact viewer-dependent cases subsequently passed outside the sandbox, with 415 filtered out, under pinned Poppler 26.01.0 and LibreOffice 26.2.5.2. This is not a rerun of all 407. Earlier wrong-Poppler and sandbox-conversion failures remain failed historical observations. The separate final doctest log reports two passed. Final scoped clippy and both affected WASM logs finish successfully.
- Archive measurement 1 records 3,851,079 compressed bytes, 52,612,641 member bytes and 37 members, matching the changed README and measurement catalog. Measurement 2 records 3,851,073 compressed bytes with the same member bytes/count and `catalog_match: true`. The six-byte gzip difference is retained as an actual second observation. Both are no-verify inventory measurements, not genuine clean verified archive qualification.

Independently matched every entry in `s95.3-b2-customxml-final-source.json` to current bytes. Product source `crates/rdocx/src/content_control.rs` hashes to `4985cc18113bbaf09ffdf44c4e31e8532601ce69e6826401701de9388bd767c3`. Final test source hashes to `78e0ef1686be6fb35d8d24a1921a6c6d3700844aa2e998a8705a5c1ea5fe93a0`. HLD03, README and the measurement script also match that manifest. These authenticate the reviewed working source and its retained observations, not a clean-tree build claim.

## Not found and limits

Correctness, contract, panics, OOXML preservation, discriminating tests and structure each produced 0 additional findings. Row, cell, block and inline wrappers, alternating depth, metadata separation, decoded boundary events, direct and nested relocation, unwrap, repeating conversion and the existing-item preservation branch were examined. Supported opaque content was not reclassified as a general validation failure.

Genuine updated clean verified archives and actual updated hosted qualifications remain explicit final sprint riders. Old hosted wheels and the prior verified archive do not qualify changed product source. This focused review neither supplies those missing observations nor decides final sprint acceptance. No new Word UI observation, render parity claim or B1/B3/B4 reassessment is made.

Only this report was written. No build, broad gate, native UI, sub-agent, remediation, source edit, plan edit, scratch update, ledger/state change, commit, push or sprint decision was performed. Review ends here.
