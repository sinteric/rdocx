# F-X220, whole feature, pass 1

**Reviewed**: Frozen whole implementation on `work/f-x220-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. The tracked diff comprises51 files,7,966 insertions and484 deletions. Scope includes native Word and PowerPoint font graphs, settings, decoding, selection, Python, CLI, existing HLD/README carriers, the concrete six-document downstream proposal and the narrow timeline correction.

**Verdict**: Two defects, zero smells, zero nitpicks. Both defects concern the downstream documentation proposal. No new production defect was found. This is an implementation review, not complete scoped acceptance or permission to complete the feature.

## Defects

### D1, downstream proposal asserts unqualified native PowerPoint interoperability

`.claude/scratch/F-X220-model/downstream-proposal/skills/pptx/references/python-api.md:56` states that the authored uncompressed EOT is what PowerPoint reads and that PowerPoint uses no embedded font without the embedding flag. `.claude/scratch/F-X220-model/downstream-proposal/skills/pptx/references/recipes.md:363` repeats the native-use assertion beside the example.

The approved contract at `.claude/plans/F-X220-design.md:44` requires genuine producer-format inspection and actual native use of an uninstalled face, with raw-format and missing-flag negatives. That acceptance remains explicitly pending in the authenticated final checkpoint. Current own-renderer font use, the EOT specification and upstream reports do not establish these claims for the authored package and pinned native application. A consumer following this proposed documentation receives an application-use assurance that the feature has not qualified.

Limit the proposal to the proven writer facts, flag behavior and own deterministic rendering. Keep actual PowerPoint use and native producer/save behavior expressly pending until the required application controls establish them. This finding does not claim that native PowerPoint rejects the authored format.

### D2, downstream handle table contradicts the checked font publication provider

`.claude/scratch/F-X220-model/downstream-proposal/skills/pptx/references/python-api.md:28` lists `embed_font`, `remove_embedded_font` and `remove_font` as retiring slides and all descendants.

The actual provider at `crates/rpptx-py/src/presentation.rs:3443` captures unchanged owners and prepares/publishes them through the existing owner provider. Successful font graph edits preserve unchanged slide identities. The genuine installed control at `crates/rpptx-py/tests/test_documented_examples.py:9261` retains a slide before embedding all four kinds and accesses that same held owner after the mutations at line9264. Exact no-op behavior is additionally asserted at line9279.

The proposal therefore teaches the wrong lifetime contract for these feature operations. Remove the font graph operations from the global retirement row and describe their actual preservation of unchanged owners, while retaining the existing semantics of the other listed operations. A documentation-only correction is sufficient. Do not change the production provider to match this stale table.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero additional findings. Native candidates stage font parts, relationships, content types and flags before publication. Replacement uses checked copy-on-write reachability. Removal validates actual target ownership before snapshots or deletion. Word checked listing bounds aggregate projected copies. PowerPoint decoding bounds envelopes and shares physical decoded bodies. Caller faces retain precedence over enabled embedded faces and bundled fallback.
- Contract: the two proposal defects above are the findings. Native/public/CLI behavior otherwise matches the approved explicit-license policy, four face kinds, checked inputs, immutable snapshots and tri-state settings. Word retains its legacy owned snapshot signature and adds the checked listing for the actual new consumers. New public rendering input is documented in the affected HLD carriers. The eight specified HLD files describe current intent.
- Panics: zero findings. Sfnt/EOT indexing follows checked table/header ranges. Face, aggregate, alias and metadata bounds precede significant projections. Python readers continue through short chunks to actual EOF, check cumulative limits before copy and use fallible preparation. Python result snapshots and owner preparation precede publication. Existing bounded invariants justify the reviewed internal unwrap/expect uses.
- OOXML: zero findings. The presentation font model resolves qualified namespace ownership, preserves unsupported/opaque source and uses schema order. Word settings retain source namespace context. Unsafe deletion and ambiguous or foreign graph targets refuse atomically. Unsupported compressed/unknown bodies remain retained and explicitly undecoded. No convenience ZIP writer bypasses the native provider.
- Tests: zero additional findings. Existing entrypoints cover graph ownership, settings, malformed data, bounded readers, actual allocation refusal, owner preservation, deterministic painted font use, caller override and CLI path collisions. The current timeline correction has both unchanged fixed-hash poster coverage and actual glyph plus PNG paint discrimination. Missing native/corpus authorities remain uncredited rather than represented by self-produced fixtures.
- Structure: zero findings. The implementation stays in existing carriers and concrete owners. No new product module, dependency, trait, generic, revision framework or feature flag was introduced. Both wrappers consume native providers. Repeated manager and public input functions serve the existing concrete ownership boundaries rather than adding forwarding abstractions.

## Narrow timeline correction

`crates/rpptx/src/lib.rs:1658` prepares the rendering context, then gives a detached public sample a fresh configured manager at line1662. That manager still incorporates enabled embedded sources and explicit callers through `crates/rpptx/src/lib.rs:12379`. `crates/rpptx/src/lib.rs:8305` uses the supplied prepared manager directly. Animation continues to retain its one prepared manager, rather than resetting on every frame.

The correction separates detached glyph identities from accumulated animation identities without restoring caller-only rendering. Actual stronger font-selection and PNG-paint controls, one-prepare/fifty-frame coverage and mixed-media coverage qualify the correction. The separate independent timeline pass2 is retained. The original broad failure and unchanged poster source/baseline remain retained, with no golden rewrite.

## Authenticated evidence and boundaries

The frozen final checkpoint SHA-256 is `38dad1ee84ab7242f52588680fedce973f7b19395abb28a8f5333b37c04ece73`. Its source map is `e754f9c3ce62b0e715a4c4f0fc32942290d49c7c199a9150acd509df12765bec`. All3,205 tracked bodies match. Current complete binary diff is `dbd2c40ae178d4b99e9bd5657b0bdd2082bbab8b47baa341569825fe699051ec`. Narrow correction delta is `838a7657067562e3cfa83ef4d09b0e3f3c631f8190bfc0a7b41c675cf3057d05`.

All seven current checkpoint evidence receipts and all23 actual row log hashes authenticate, with zero exits and safe owned cleanup. Current PPT library records159 passes and5 ignored. Current strict changed all-target/all-feature Clippy, both WASM graphs and49 unchanged hash outputs are qualified. Exact poster, embedded/caller glyph and paint, one-prepare animation and mixed-media controls pass. The recorded8MiB legacy native stack remains qualified separately from ordinary-stack controls.

Current paired fifth wheels and own isolated installed acceptance retain authenticated receipts. The exact25 collected runtime cases, both strict mypy consumers and both module stubtests pass. Preceding safe-final receipts and their33 row logs also authenticate. Their historical broad native results, actual own deterministic CLI/Python render composition, documentation checks and failed originals retain their stated scope. Earlier wheel generations remain historical. The earlier lost alias-result body remains unavailable, while current distinct successor evidence is retained. No lost artifact is reconstructed or counted as accessible evidence.

All six downstream proposal bodies and the actual patch match their receipt. The patch SHA-256 is `8b2c2834d5d5f8b72a80ada768294045ad694ee1a3afc973c9cbe1777dc068f5`, against the recorded external tree `a41d8553b2dbdbb8c5f7fa4d8668ba06afbf51ae`. This review grants no submission authority and claims no external update.

Native Word/PowerPoint producer qualification, authentic font-bearing corpus, actual uninstalled regular/bold application use and native save/reopen behavior remain required and pending. Own renders do not substitute for those authorities. Root integrated managed composition, current affected actual package archives, inventory/measurements/policy and remaining supply-chain riders remain pending. The two UV corpus successors and four absent historical timeline/SmartArt controls remain uncredited. Early-return215 rows are not native oracle passes. Final scoped verification, structured handoff and feature completion are not granted by this review.

## Review integrity

Read canonical CLAUDE/WORKFLOW/microscope instructions, approved plan, sprint/backlog status, progress, prior model/native/Word/public/CLI reviews and the current narrow timeline review. Inspected source and current documentation directly. No product, Cargo, import, CP, target, UI, Office, network or verification command was run.

HEAD, branch, physical index, logical index, tracked bodies and full diff were authenticated before report creation. Only this review file was written. No source, plan, progress, HLD, ledger, baseline or implementation change occurred. This pass ends here and returns control to root for distinct remediation and subsequent independent review.
