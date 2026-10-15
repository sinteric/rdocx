# F-X218, native semantic export remediation, pass 2

**Reviewed**: Frozen native remediation on `work/f-x218-codex`, HEAD `a86e1f9142e602c6e34d4801136c8b33efb652d1`. Six tracked carriers, 2,614 additions and eleven deletions. This includes the admitted existing manifest and lock dependency edges and narrow plan routing update.

**Verdict**: Zero defects, zero smells, zero nitpicks within this bounded native phase. Prior native pass 1 D1 and D2 are resolved. Full-feature acceptance remains pending.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. Internal slide destinations normalize through OPC resolution and compare ASCII-case-insensitively before hidden-target selection at `crates/rpptx/src/html.rs:571`. Body validation runs before image destination or asset creation at `crates/rpptx/src/html.rs:624`. PNG validates every chunk extent and CRC, excludes animated PNG, requires terminal IEND and decodes pixels. JPEG requires actual scan termination through the checked segment and entropy walk and then strict pixel decode. GIF traverses and decodes every frame. TIFF traverses and decodes every page and verifies complete returned buffer length. Supported uncompressed BMP verifies header, palette and physical pixel extents. Original resource bytes are exported unchanged.
- Contract: zero findings. The source-owned resource route, inline and detached output options, read-only source package and existing safe publication contract remain intact. Unsupported animated PNG and compressed BMP return explicit errors. Input bytes, dimensions, pixel allocation, cumulative GIF frame and TIFF page bytes, and frame/page/scan counts have concrete bounds. These repairs do not claim pending public-consumer or full-corpus acceptance.
- Panics: zero findings. Chunk and marker extents use checked input access and arithmetic. JPEG entropy iteration checks every byte. BMP direct pixel indexing follows validated dimensions, checked stride/end and physical input extent. PNG dimensions are checked before pixel decode. JPEG headers and output sizes are checked before full decode. GIF per-frame buffers and TIFF complete layouts are checked before pixel allocation. Frame/page traversal remains bounded.
- OOXML: zero findings. D1 now uses the package part-identity contract for actual source slide relationships. Media stays under the actual source picture owner and source MIME must match detected format. Semantic output and refusal controls preserve the native package. No OOXML serializer, child order or opaque-source rewrite is introduced.
- Tests: zero findings. The retained three genuine RED controls distinguish case-varied source targets and signature-only PNG/JPEG bodies. The stronger genuine JPEG missing-EOI RED distinguishes decoder recovery from complete-body validation. Final controls cover both semantic formats, hidden-target omission, inline and detached export/save refusal with an empty publication directory, unchanged source, complete encoded PNG/JPEG/BMP bodies, two GIF frames, two TIFF pages, corrupted tails and oversized dimensions. The independently pinned cmark reference remains qualified against seven input records and three grammar negatives.
- Structure: zero findings. Root-admitted existing workspace tiny-skia, gif, zune-jpeg and tiff become unconditional native facade dependency edges. Cargo.lock changes only the rpptx edge list, with no new crate version. The implementation stays in existing carriers with concrete helpers and no new module, file, trait, generic or feature flag. HLD03 graph routing is explicitly added to the approved plan and its actual HLD update remains pending.

## Authenticated evidence

Remediation checkpoint SHA-256 is `b96e9c2cd9439f9bd620f17be903b9afe9ffa9204084b34c48b6a34cec8bb79f`. Complete tracked diff SHA-256 is `b3266a0f89946dd78746af7e3e86fc76d4b8963dd4323f02049b50e7890d10c5`. Prior native pass 1 SHA-256 is `18899be7809b3363a702600ce0258d28eddfc68f8e1c4c8cbaebc0a0291b8505`.

Original defect RED receipt `27de6378db27cc4358e615a4751a1553f7fa53f8200baace1451e5c254439ebd` retains actual exit 101 with three runtime failures. Stronger complete-body-tail RED receipt `4588387b976fde32f6a12e40eae375d114bd8c5fad56102f88f1e501533d431a` retains actual exit 101 when strict JPEG decode admitted the missing EOI body. Actual log hashes and source inventory receipt hashes authenticate. Earlier API spelling and unguarded setup failures remain unqualified and are not substituted for genuine RED evidence.

Final receipt `d04f81e9bbe446543440de69331d63cb6d60ea3167eb53a361c3526ec6575413` and all six actual log hashes authenticate. Its complete source inventory exactly matches the frozen checkpoint. Actual deciding logs record terminal zero exits for default all-target check, 21 focused native controls with zero failed or ignored, the official cmark reference, strict library Clippy, MSRV 1.93.0 no-default library check and scoped formatting. Every row records exact source guards, no timeout, a gone owned process group and a reaped leader. The expected caught HTML grammar-negative panics remain visible in the successful focused test log. Existing all-target and no-default warning output remains visible and is not represented as a workspace strict-Clippy pass. Tracked diff whitespace check is clean. This is authenticated worker execution, not an audit rerun.

Official cmark-gfm qualification receipt `5cae559d8b5090e600afe192d9e3d4ad910306b654904d502ff087c0ab42481e` authenticates all seven exact input hashes, three grammar negatives, the script and executable. The pinned source remains commit `587a12bb54d95ac37241377e6ddc93ea0e45439b`, version `0.29.0.gfm.13`, archive SHA-256 `9214b42f13a471fc2984e9806c37bf0f42a13027854ab19103bd6d8085eb3416`, executable SHA-256 `f05a86b6961401e2f732f91b7864a43fe6d61598c67b96a17e5d4eda51eb94b2`. Prior alternative renderer and intake failures remain separate from deciding reference evidence.

## Full-feature boundary

Complete pinned corpus semantics, selected compatibility and local-rebinding controls, actual PowerPoint source authority and unchanged save/reopen, Python and CLI consumers, fresh installed wheel and typing, affected archives, both WASM graphs, final 49-output hashes, policy gates, listed HLD changes and whole-feature microscope/scoped verification remain required. This clean bounded pass neither waives those obligations nor grants a full-feature handoff.

## Review integrity

Read repository CLAUDE.md and canonical WORKFLOW.md, microscope guidance, the approved plan, current in-progress sprint/backlog status, progress and prior native pass 1. All 3,203 frozen source file hashes, HEAD, branch, physical index, logical index and complete tracked diff match before and after this review. Physical index remains `74d5287ceed23c1aac004126d4332e20bbc83920338bee52d0dbdc386546b5e6`. Logical index remains `ce0be2590c52a95f9c0101559ccbbcbab4becc608667c1ab5fa8e67909e0b8ac`.

Only this review file was written. No source fixes, Cargo, product or oracle execution, CP, Office, Git mutation, intake, plan or shared ledger edits occurred. Review stops here and returns the writer lease to the orchestrator.
