# F-X218, native semantic export, pass 1

**Reviewed**: Frozen native phase on `work/f-x218-codex`, HEAD `a86e1f9142e602c6e34d4801136c8b33efb652d1`. Four tracked carriers, 2,024 additions and six deletions, including the narrow plan status update. The exporter, reexports and existing integration entrypoint form the product diff.

**Verdict**: Two defects, zero smells, zero nitpicks. Full-feature acceptance remains pending.

## Defects

### D1, internal links use a different part identity from the package

`crates/rpptx/src/html.rs:575`

Internal slide links compare `slide.part_name == target`. A valid source relationship targeting `SLIDE2.XML`, while the stored slide part is `slide2.xml`, resolves successfully through the package's ASCII-case-insensitive identity but fails this comparison. HTML and Markdown then drop the supported link and emit a missing-target diagnostic. The completed package identity contract at `docs/hld/04-opc-and-packaging.md:54` applies to required PowerPoint parts and relationship targets. Match the actual source slide using that identity before deciding whether it is omitted by hidden filtering. Qualify the case-varied target through both formats and through a hidden-target negative.

### D2, magic-byte recognition admits invalid picture bodies

`crates/rpptx/src/html.rs:617`

Picture preparation calls `ImageFormat::sniff` and checks only MIME equality before embedding or staging the complete bytes. The sniff implementation at `crates/oxml-media/src/lib.rs:181` returns PNG for the eight-byte PNG signature alone. A source package with a valid picture relationship and PNG content type but that truncated body therefore exports and publishes a broken image successfully. JPEG and other magic-only truncations have the same boundary. This violates the approved requirement to diagnose invalid media before output publication. Validate the supported image body with bounded existing format machinery before producing a destination or asset. Add invalid-body controls with correct MIME and valid source ownership, including save refusal with no published document or asset. The current MIME-mismatch control does not distinguish this case.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: D1 affects supported internal-link resolution, and D2 affects picture admission. No further finding in source-order traversal, title-once identity, grouped recursion, list hierarchy closure, table-origin emission, notes and hidden-slide selection.
- Contract: D1 and D2 violate existing native obligations. No additional finding in concrete options/results, detached assets and diagnostics, caller-independent semantic export, explicit loss reporting, escaping, read-only source ownership or existing outline stability.
- Panics: zero findings. Fixed nine-level style arrays are checked for paragraph access, exporter slide indices come from the current slide inventory, list pops follow nonempty checks, and base64 indices remain bounded. Owned output staging uses create-new files and no-overwrite hard links with rollback on failure.
- OOXML: D1 is the identified part-identity violation. Zero additional findings in actual image owner relationship lookup, typed source metadata, namespace-aware inherited source use or source preservation. The exporter writes semantic documents without modifying the native package.
- Tests: D1 and D2 lack distinguishing controls in the qualified set. Zero additional findings in actual HTML parse-error checks and malformed-nesting negatives, independently specified Markdown semantic records, parser grammar negatives, deterministic resource deduplication or native source/output assertions. The still-pending full corpus and native source authority are not represented as completed tests.
- Structure: zero findings. Changes stay in existing carriers, with concrete types and no new trait, generic, crate, module, dependency, feature flag or fixture asset. Existing import-only items retain their feature guards while semantic export is available without the default template feature.

## Authenticated evidence

Checkpoint SHA-256 is `209664203efdee4a4bfeb3d359267075fb210fe912ebfbe7fcd2c4fdb05f9001`. Complete tracked diff SHA-256 is `6e1436bb4845eb55d3fba69854795cc56adb255cfb154de03aaca7859c3945a3`.

Final native receipt `b1580d0ecb6587401e35f28a25deecd44e841d03d167bd24d2e396e08a391b58` and every actual log hash authenticate. It records terminal zero exits for default all-target check, fifteen focused native controls, the official cmark oracle, strict library Clippy, MSRV 1.93.0 no-default library check and scoped formatting. Each row retains exact source guards, no timeout, a gone owned process group and a reaped leader. The actual focused log reports fifteen passed, zero failed and zero ignored. This is authenticated worker execution, not an audit rerun. Existing warning output is retained and is not described as a workspace strict-Clippy pass.

Official cmark-gfm qualification receipt `5cae559d8b5090e600afe192d9e3d4ad910306b654904d502ff087c0ab42481e` authenticates seven exact input records and three grammar negatives. Its script, inputs and executable hashes match their files. The pinned source is commit `587a12bb54d95ac37241377e6ddc93ea0e45439b`, version `0.29.0.gfm.13`, archive SHA-256 `9214b42f13a471fc2984e9806c37bf0f42a13027854ab19103bd6d8085eb3416`, executable SHA-256 `f05a86b6961401e2f732f91b7864a43fe6d61598c67b96a17e5d4eda51eb94b2`. The oracle uses declared table and strikethrough extensions and independently specified textual structure. The earlier alternative renderer defect, intake 403 and unqualified setup failures remain separate from deciding evidence.

## Full-feature boundary

Complete pinned corpus semantics, selected compatibility and local-rebinding controls, actual PowerPoint source authority and unchanged save/reopen, Python and CLI consumers, fresh installed wheel and typing, affected archives, both WASM graphs, final 49-output hashes, policy gates, HLD changes and whole-feature microscope/scoped verification remain required. The limited native corpus control and source-built controls do not complete those obligations. This report neither waives later acceptance nor grants a full-feature handoff.

## Review integrity

Read repository CLAUDE.md and canonical WORKFLOW.md, microscope/risk guidance, the approved plan, current status/progress and relevant cited source-order, package identity, API and test contracts. All 3,203 source file hashes, HEAD, branch, physical index, logical index and complete tracked diff matched the frozen checkpoint before and after this review. Physical index remained `74d5287ceed23c1aac004126d4332e20bbc83920338bee52d0dbdc386546b5e6`. Logical index remained `ce0be2590c52a95f9c0101559ccbbcbab4becc608667c1ab5fa8e67909e0b8ac`.

Only this review file was written. No source fixes, test runs, Cargo, product execution, CP, Office, Git mutation, intake, plan or shared ledger changes occurred. Review stops here and returns the writer lease to the orchestrator for a separate remediation phase.
