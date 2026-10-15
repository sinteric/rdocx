# F-X220, timeline fallback, pass 2

**Reviewed**: Frozen D1 remediation against the preceding safe-final checkpoint. Exactly two existing carriers,30 additions and12 deletions. The other3203 tracked carriers are unchanged. This is a bounded correction audit, not whole-feature acceptance.
**Verdict**: 0 defects,0 smells,0 nitpicks.

The reviewer also implemented the preceding correction in a separate implementation phase. This pass changed no product source and independently reauthenticated the frozen evidence. Root retains its independent reading and assigns the whole-feature review separately.

## Defects

None found. Prior pass1 D1 is resolved in the reviewed scope.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness:0 findings. At `crates/rpptx/src/lib.rs:1662`, the detached single-sample path replaces its prepared manager with the existing fresh configured manager using its staged package, actual presentation owner and supplied caller fonts. The helper at `crates/rpptx/src/lib.rs:12379` loads supported enabled embedded faces and then caller faces. This retains baseline font identities without removing embedding or caller precedence. The unchanged poster expectations pass.
- Contract:0 findings. The change restores the approved existing no-caller behavior. Animation still passes its retained assembly at `crates/rpptx/src/animation.rs:171`. Neither its public interface nor its manager lifetime changes. The two actual private request callers no longer need a selection flag.
- Panics:0 findings. The new production call propagates its existing Result. No new untrusted indexing, arithmetic, unwrap or allocation assumption is introduced by this delta.
- OOXML:0 findings. This delta selects render state only. It introduces no package mutation, namespace handling, schema order or raw-source rewriting.
- Tests:0 findings. The deciding unchanged static poster provides a genuine regression control, with retained failing current-source original and separately passing claim Base. The existing native font control at `crates/rpptx/src/lib.rs:23605` additionally paints returned timeline pages through the ordinary export font snapshot and compares actual PNG bytes for embedded and caller cases at lines23655 and23681. The separate animation controls preserve one preparation and mixed-font/media identities. This evidence is not native Office authority.
- Structure:0 findings. The existing concrete configured helper is reused. The obsolete boolean and unreachable legacy false branch are removed at `crates/rpptx/src/lib.rs:8304`, reducing cases. The two-manager detached setup is constant and preserves the established baseline behavior. No wrapper, trait, generic, field, dependency, module or asset is added.

## Authenticated evidence

Checkpoint `.claude/scratch/F-X220-model/timeline-D1-final-review-checkpoint.json` has SHA `38dad1ee84ab7242f52588680fedce973f7b19395abb28a8f5333b37c04ece73`. Full3205 tracked sources authenticate to source map `e754f9c3ce62b0e715a4c4f0fc32942290d49c7c199a9150acd509df12765bec`. HEAD remains `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d` on `work/f-x220-codex`. Full patch is `dbd2c40ae178d4b99e9bd5657b0bdd2082bbab8b47baa341569825fe699051ec`. The exact correction delta is `838a7657067562e3cfa83ef4d09b0e3f3c631f8190bfc0a7b41c675cf3057d05`. Physical index, logical index, branch, HEAD, diff and every tracked byte remain exact before and after this audit.

All retained qualified receipts and rawlog hashes were read and authenticated. Each accepted row has actual exit0 and terminal reaped process-group cleanup. Final deciding four controls have receipt `96e16526f0bbdcfda2726d36b2f7d93baabcb621e22fec288038c447004abb3e`. Current PPT library has159 actual passes and5 ignored, with current fmt/diff checks, receipt `77b36a3efb0f78938a53931f3f2ca2c208b81bf38b34decc7355c7fa42b3c5f7`. Legacy native test stack remains explicitly8388608 bytes. No ordinary2MiB inference is made.

Both current WASM graphs, strict changed all-target/all-feature Clippy and49 unchanged hash outputs pass in receipt `dc2f41bcb6a2f42fec9e66f64d751cedb0e343d59d0ad194bf0a7f437d1199ac`. Baseline and unchanged poster source are byte-exact. The original315-pass/seven-failure broad PPT result is preserved as failed, not replaced by these focused qualifications.

Fresh fifth paired wheels have build receipt `c6eda1d6cbe2b84730cf468fcd0efb4ff8d63ddcf3bc5f49e65ce3dcadc34ca1` and authentication manifest `734db786bfeb18b1d17a051ecf5adf6be3d0e26ca754d6f5a0673faf4bf4ec58`. Every one of12 Word and15 PPT wheel members, RECORD digest/size, local source body and arm64 native binary was reauthenticated. Word native remains `deb14e52940b9ef17acee1c0bf14e555ef399570d3c62432737132ae2c749501`. Corrected PPT native is `9683a03432436205b797cbc8f72738307cecc8e3ef12d166ac9738a04e015b8f`.

Own installed receipt `619fec5288bf5e5c255b9fa3f1df56ec28e98084e14081f7024b7e84170eae9a` records exact25 collected and passed cases, twelve public font/reader controls and thirteen export/layout controls. Both strict mypy consumers and both full native module stubtests pass. Owned installed-prefix before/after manifests remain exact. The fourth pair remains historical, not relabelled current.

## Boundary

Native Word/PPT producer qualification and real uninstalled font-use authority, root managed composition, archives/policy, two UV setup successors, four absent historical timeline/SmartArt fixtures, downstream proposal review and full-feature microscope remain outside this pass. Missing native fixtures and early-return215 rows receive no oracle credit. No managed CP, Office, target process, source, baseline, ledger or commit mutation occurred. Only this report was written. Return to root for the next distinct review or qualification phase.
