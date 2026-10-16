# Current Sprint, S94

**Milestone**: M24, X, Python and CLI parity and bounded conversion.

**Goal**: Let first-draft python-docx and python-pptx scripts and the existing native signing and form surfaces work from Python and the CLI. Complete the finite 73 input LaTeX conversion follow-up to Issue365. The four parity stories reuse existing native semantics, while F-X247 explicitly defines bounded conversion normalization. Five stories total 14 to 19 developer-days of estimated effort.

## Spec references

- `docs/hld/14-development-backlog.md`, F-X238 through F-X241 and F-X247 define the five story contracts. M24's final stability boundary remains S99.
- `docs/hld/03-architecture.md`, the document-family boundaries, staged mutation and shared binding/tool support constrain adapters and dependency edges.
- `docs/hld/04-opc-and-packaging.md`, package signatures, coverage, XML preservation, schema order and atomic publication constrain signing, forms and equation round trips.
- `docs/hld/06-presentationml-model.md`, retained notes and opaque source preserve PowerPoint ownership while exposing compatibility names.
- `docs/hld/08-rendering-spec.md`, existing deterministic OfficeMath layout and unit semantics constrain the bounded conversion and silent-zero refusals.
- `docs/hld/10-bindings-spec.md`, Python/CLI parity, immutable snapshots, causal owner publication, native signing and form providers define the public surface.
- `docs/hld/12-testing-strategy.md`, regression, external structural/native differential, installed binding and mutation-sensitive checks define acceptance.
- `docs/hld/15-build-and-toolchain.md`, pinned producers, packaging, feature reachability, archive inventories and actual artifact measurements constrain shipped signing and render evidence.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X238 | Restore python-docx Word names from the first-draft corpus | L | done | - |
| F-X239 | python-pptx notes names and refusal of silent zero values | M | done | - |
| F-X240 | Python and CLI signing and signature verification | M | done | - |
| F-X241 | Python and CLI form reading and filling | M | done | - |
| F-X247 | Preserve the common LaTeX inputs reported in Issue365 | L | done | - |

## Sequencing note

Rows list the accepted scope. Implementation waves follow dependencies and explicit file leases in the approved plans. Design all five stories before implementation. F-X247 consumes completed S93 F-X234 and F-X235. F-X238 and F-X239 overlap Word formatting and PowerPoint slide bindings and integrate serially. F-X240 and F-X241 share Word binding/CLI entrypoints and also serialize their changes there. Independent math and parity work may share a wave only when their actual source, test and HLD section leases do not overlap.

A blocked worker does not block independent stories without formal dependencies. After F-X238 fully checkpoints and explicitly releases its overlapping source, test, HLD and cache leases, F-X239 may start from clean canonical source while F-X238 stays blocked and unintegrated. It may share disjoint implementation time with F-X247, but the math worker retains the archive/catalog lane until explicit release. Later claims and integration preserve all scoped, microscope and final union gates. F-X238 resumes only after lease return and reconciliation against the current consumers.

Keep installed environments and build caches isolated or exclusively leased. One story per wave may change deterministic baselines. Shared archive observations and delivery totals belong to the integrator. Developer-day estimates describe effort, not elapsed delivery time. S95 prioritizes the new repair intake. SPRINT_PLAN.md assigns the remaining repairs and carryovers to the explicit S95.1 through S95.4 boundaries.

## Definition of done for this sprint

- Complete all five approved contracts through implementation, independent zero-defect and zero-smell microscope, scoped verification, integrated full verification and clean sprint review.
- Execute the authenticated PR333 first-draft inputs unchanged, keeping their original reports separate from F-X211. Independently qualify every requested Word name, including add_section, stream save, highlight and first-page headers, as compatible behavior or an error naming the supported alternative. The three specified divergence messages retain their exact required pointer tokens.
- Preserve successful Cell.merge behavior. Retain the original runner's stale divergence-marker result separately from saved-file and semantic qualification. A raw runner result alone cannot qualify needs-labelled scripts or erase a diagnosed observer contradiction.
- Qualify notes presence versus empty content and notes text mutation, the requested enum aliases and constants, and all five exact silent-zero assignments. Refusals name the correct units and leave complete document bytes and owner authority unchanged. Positive explicit-unit controls still serialize correctly. Native unit constructors keep their existing truncation behavior.
- Sign and verify both DOCX and PPTX through actual installed Python and CLI entrypoints using admitted DER and PEM. Reports preserve signer identity, precise issues, cryptographic validity and coverage. Typed edits, saved tampering, partial and unsigned controls have the approved distinct outcomes and CLI exits. Verification does not assert certificate-chain trust.
- Ordinary shipped wheel and CLI configurations enable signing, including the existing no-default-features CLI path, with actual dependency, licence, artifact and package measurements. Native family and shared support default features keep their approved boundaries. Unexecuted target builds remain unclaimed.
- Read and fill plain text, checkbox, drop-down and date controls plus a legacy text field through Python and CLI --values. Independently inspect saved XML and reopen values. Preserve existing callable snapshots, --tag behavior, custom XML, held owners, strict new mixed-selector refusal and whole-operation atomicity. Fill/sign interactions operate on current package state.
- Execute the exact 73 LaTeX inputs through public add_equation and save/reopen. Retain the original unconditional FAIL label and use a separately identified summary-only correction with independent semantic assertions. Preserve symbols, script bases, upright/text roles and meaningful spacing under the explicitly designed OfficeMath normalization. Qualify actual pinned Pandoc outputs and native paint where required, retaining oracle limitations and fraction-style boundaries.
- Keep deterministic fonts, XML/schema preservation, parser/resource caps and historical oracle identities. Do not weaken geometry, raster or mutation gates. Reconcile all 49 hashes and the pixel manifest, with any intentional delta separately declared and reviewed.
- Pass the integrated union of binding, typing, stub, target, publication, supply-chain and differential riders. No release or publication is part of this sprint. M24's final gate remains a later boundary.
