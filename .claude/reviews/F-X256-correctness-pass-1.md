# F-X256, correctness, pass 1

**Reviewed**: Working diff in work/f-x256-codex against d75f407b98128f611961c91617ce4cc3da45e905. Eight changed files, 439 insertions and 62 deletions. All six microscope aspects were checked.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and source identity

Read the approved design, current in-progress sprint/backlog rows, progress checkpoint, canonical workflow and microscope guidance, cited unit conventions, binding contracts and testing requirements. The scope is the eight changed Python runtime, stub, test and HLD files. No prior integrated S95 implementation was re-reviewed as a new diff.

All eight source/test/HLD hashes and the unchanged approved-plan hash matched frozen-source.json before and after inspection. The manifest SHA256 is62239f288d17dcb99dbf4c33c0d82b4d6c1ea4565a787474b6151aa6cdeac35c. The branch and claim-base HEAD matched at report creation.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: spacing admission distinguishes actual Length instances from non-bool integer EMU and applies the full signed twip quotient check before native narrowing. Nonnegative space_after and signed character spacing keep their distinct domains. Line spacing tests Length before numeric multiples, rejects unsupported or nonfinite values, and checks the same f64-times240 truncating conversion actually used by native set_line_spacing_multiple. Absolute spacing checks the full integer EMU quotient before forwarding through the existing native points conversion. Clearing still uses the existing value-and-rule removal path. No new native conversion behavior or generic unit engine is introduced.

Public section admission is confined to update_section. Its ten geometry inputs validate before existing mutation and rollback paths. The shared checked_section_length helper is unchanged. Live section geometry validates its existing raw EMU first, wraps only the internal forwarding value in the existing Length helper, and preserves its existing owner route. No public bypass is exposed. Non-geometry live delegations remain unaffected.

Panics: no new unchecked narrowing, arithmetic overflow, untrusted indexing or panic path was found. The changed live forwarding removes former Python conversion unwrap calls and propagates allocation errors before borrowing the document for mutation.

OOXML: the diff adds no parser or serializer and reuses current native setters and publication. Tests independently inspect saved spacing attributes and reopened values, section snapshots, signed package equality and held owners. Unrelated properties and existing native source preservation remain under their current providers.

Tests: the actual retained old-prefix log shows all four reported numeric controls fail as intended, rather than import/setup failure. New tests cover zero, None, signed values, typed sub-twip truncation, exact and adjacent signed32 bounds, integer/float multiples, nonfinite inputs, all ten section geometry keywords, multi-key refusal and live forwarding. The three old truncation positives retain their expected values with explicit Emu inputs and gain separate bare-input refusal coverage. Stubs preserve meaningful integer routes while public update_section uses Length or Literal0, and the existing typed consumer exercises both numeric and typed spacing.

The independent python-docx1.2.0 thirteen-row receipt and all six source pins were authenticated. Its receipt SHA256 is3eff11a8042f323eedc557eaeca7e17d49c78a2ef5341673014f1bceb147d7a7. The off-grid241 versus native240 observation is explicitly asserted as a deliberate divergence, and tiny bare spacing refusal is distinguished from oracle acceptance. No character-spacing oracle parity or new native Office coverage is claimed.

Structure: no new file, module, trait, generic parameter, dependency, public native API or owner model was introduced. The small spacing admission helper has two existing consumers with explicit signed/nonnegative policy.

## Verification boundary

This is independent source review, not scoped completion or final integrated acceptance. The retained four-row old-source RED and independent oracle observations were inspected, not rerun. The worker reports native check, binding clippy and prose success. Fresh ordinary installed Word, full suite, strict typing, stubtest and remaining scoped riders were still pending during review. The untouched paired-family Issue386 reproduction and final sprint union remain root-owned. No tests, builds, product imports, cache mutations, baseline changes or remediation were performed by this reviewer. Only this review file was written. Return the worktree write lease to the orchestrator when this report is delivered.
