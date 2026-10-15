# F-X220, CLI and Word metadata, pass 1

**Reviewed**: Frozen CLI delta on `work/f-x220-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Six existing CLI carriers contain1,259 additions and zero deletions. The sole authorized native delta adds22 lines to `crates/rdocx/src/document.rs`. Seven carriers and1,281 additions are reviewed in this bounded pass. Prior native and public implementation remains preserved apart from that exact metadata method.

**Verdict**: Zero defects, zero smells, zero nitpicks. Full-feature acceptance remains pending.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. Word font listing at `crates/rdocx-cli/src/commands.rs:3165` uses checked snapshots and resolves actual package metadata separately. `crates/rdocx/src/document.rs:29282` checks record/face indices and returns the stored part name and byte length through the existing relationship provider. That provider at `crates/rdocx/src/document.rs:29302` resolves the real font-table owner, unique internal font relationship, case-compatible target and ODTTF content type. PPT listing at `crates/rpptx-cli/src/commands.rs:1952` reports the native format enum, real relationship/part facts, stored length and optional decoded length. Neither CLI invents a successful decoder result for unsupported PPT formats.
- Contract: zero findings. Font grammar at `crates/rdocx-cli/src/main.rs:21` and `crates/rpptx-cli/src/main.rs:19` requires explicit license authorization and identity, accepts exactly four face kinds and exposes list/embed/remove/settings. Word explicit or derived GUID and subset flag reach its actual native API. Both settings commands distinguish omitted options from explicit true, false and removal. `crates/rdocx-cli/src/commands.rs:3128` and `crates/rpptx-cli/src/commands.rs:1919` require output exactly when settings are supplied. The existing JSON envelope keeps schema1 and stable native traversal order. Reads have no settings publication path.
- Panics: zero findings. Local readers at `crates/rdocx-cli/src/commands.rs:3020` and `crates/rpptx-cli/src/commands.rs:1822` require regular files before and after open, request bounded chunks, check cumulative64MiB before append and use fallible reserve. At the limit they request one byte to distinguish EOF from excess. The loop invariant bounds subtraction and request arithmetic. Native metadata indices use checked get rather than untrusted indexing. No new unchecked model access or length-derived allocation is introduced.
- OOXML: zero findings. Embedding/removal/settings call their native ownership providers at `crates/rdocx-cli/src/commands.rs:3061`, `crates/rdocx-cli/src/commands.rs:3088`, `crates/rpptx-cli/src/commands.rs:1859` and `crates/rpptx-cli/src/commands.rs:1881`. Word metadata is immutable and adds no XML writer. Existing reviewed source-preserving native transactions remain unchanged. Serialization completes before the existing staged publishers at `crates/rdocx-cli/src/commands.rs:2247` and `crates/rpptx-cli/src/commands.rs:1197`. Output checks protect document/font aliases and existing destinations before preparation, and publication rechecks the existing collision policy. Failure leaves input bytes and existing output bytes intact.
- Tests: zero findings in this phase. Actual controls at `crates/rdocx-cli/tests/integration.rs:5955`, `crates/rdocx-cli/tests/integration.rs:6110`, `crates/rdocx-cli/tests/integration.rs:6163` and corresponding PPT controls at `crates/rpptx-cli/tests/integration.rs:3077`, `crates/rpptx-cli/tests/integration.rs:3201`, `crates/rpptx-cli/tests/integration.rs:3251` exercise actual binary parsing, all four kinds, saved/reopened face lists, real archive lengths, removals, tri-state readback, explicit/derived Word keys, source equality, invalid authorization/key/kind/font, oversized input, directory refusal, symbolic/hard aliases, output collisions, temporary-file absence and help. Metadata controls include out-of-range face and maximal record indices. Missing CLI behavior would fail the binary assertions. There is no manufactured Base RED or native Office claim.
- Structure: zero findings. Concrete multi-field options stay in existing command carriers. Mapping functions directly serve the actual family APIs. The additive native metadata method supplies real ownership facts that the CLI needs, rather than reconstructing relationships in a wrapper. No new product module/file/crate, dependency, trait, generic, feature flag or forwarding-only abstraction is introduced.

## Authentication and prior continuity

Checkpoint SHA-256 is `f8ead55d73db410ebccbdecb189a7e93b9075188dd6aa79faf02c5c09c5b7dcb`. Current six-carrier CLI patch matches `d0671c3bad2dffc5e3daf36872019982e16e5fcd10304dc3a2709b5a735b8fc9`. The separately retained native metadata delta matches `d528223b7b32a4eb670926bcc267933ddaf5f4b5ccf053e682dccb7b1471de50`.

Independently compared the clean public pass2 inventory against current sources. Exactly the six CLI carriers and Word document carrier changed. Removing only the new metadata method reproduces the prior Word hash `80404a2bb98d7c909e631ef5acb499793cec7026dc88696adb58e720276f9c5e`. The other seventeen native and twelve public carriers retain their reviewed state. Prior reviews remain bounded to their original acceptance, including explicit historical evidence loss and pending Office requirements.

Authenticated all3,205 tracked hashes,22 checkpoint evidence hashes, complete current CLI patch, all five actual qualified row logs and original50 corpus input hashes. Qualification receipt `38b90991ac62ef41bb08f7ee8d6aee3030c58c93e57362021d8b6ddee1ce3ed7` records current all-target/all-feature check, six actual CLI tests, strict all-target/all-feature Clippy, formatting and diff checking, each terminal zero with no timeout, reaped leader and absent owned process group. Raw logs show three actual Word tests and three actual PPT tests, no ignored controls. The driver removes RUST_MIN_STACK and the final receipt records null, so this is ordinary-stack qualification.

Original qualification1 and2 genuine PPT fixture construction stack aborts and qualification3 items-after-test-module Clippy failure remain retained and unqualified. Final PPT tests use the immutable existing bundled template as input and perform actual CLI mutation/save/reopen. No larger stack or product bypass conceals those originals. Checking unchanged50 corpus hashes is preservation evidence only, not font-bearing corpus qualification.

## Remaining boundary

Required native Word/PowerPoint producer and real uninstalled face-use evidence, required font-bearing corpus controls, full render/physical font qualification, final paired installed composition, HLD and README recipes, downstream skill patch boundary, affected archive measurements/policy, final hash/scoped acceptance and whole-feature microscope remain due. This clean CLI review waives none of them and grants no full-feature, handoff, release or native Office credit.

## Review integrity

Read AGENTS.md, CLAUDE.md, canonical workflow/microscope, approved plan, cited native ownership/packaging/binding/model contracts, progress and prior clean reviews. Complete tracked source, HEAD, branch, full binary diff and physical/logical index remain exact before and after report. Full diff remains `4de5330fb95f15e744e523338ff1afdcb9c90e2fb40b66142f4f976a296c7020`, physical index `0de54845b89f71bb5f690d1247df7eb853a1fae5090625b670c03bea1d5c482d`, logical index `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`.

Only this review was written. No source, plan, progress, HLD, test, ledger, formatting, Git mutation, Cargo, CP, UI, Office, global cache or escalation action occurred. This pass ends and returns control and the lease to ROOT.
