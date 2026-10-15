# F-X221, manual CLI proposal, pass 1

**Scope**: External proposal only, before feature claim/adoption. Workflow SHA `ece5df95d609ecc803f145ed7eb453d6c1d29b9c470b4f6e47859b007d51af01` and patch SHA `f3de289c399fb321cee26041f215170a590d9b5d81232c5ef001c87c957ce121` under `/private/tmp/S91-F-X221-readiness-preparation-1/manual-cli-concrete-proposal-1`.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. The inherited archive validator must reject duplicate members before this proposal is adopted. No hosted rehearsal or release completion is claimed.

## Defect

### D1. Duplicate archive members collapse before exact inventory checking

`/private/tmp/S91-F-X221-readiness-preparation-1/manual-cli-concrete-proposal-1/wheels.yml:434` turns ZIP members into a name-keyed dictionary before checking the expected three names at line435. The TAR branch does the same at line439 before checking names at line440. A fourth member repeating README.md, LICENSE or the executable is collapsed and can pass the exact-member check. Later payload checks inspect only the surviving dictionary entries.

Executed the proposal's exact extracted archive-validator body against real in-memory TAR/ZIP containers through a read-only in-memory path adapter. The normal manual twelve-archive positive passed. Removing one expected archive was rejected. Adding a fourth TAR member duplicating README.md with its expected bytes was accepted by the actual validator. No binary was executed and no file was written. This is a concrete inventory defect, not hypothetical missing hosted evidence.

The omission is inherited from the previous validator, rather than introduced by the matrix expansion. It is nevertheless within the required exact CLI artifact inventory audit. Check the original ordered member list's count and uniqueness before dictionary construction in both formats. Keep exact expected names, regular TAR members, executable mode, nonempty binary, matching README/LICENSE and source provenance assertions. Add meaningful duplicate-member negatives in the existing policy entrypoint during the separately authorized preparation implementation. Do not add a new release system or new tracked test file.

## Smells

None.

## Nitpicks

None.

## Expansion and read-only semantic controls

The external family matrix at `/private/tmp/S91-F-X221-readiness-preparation-1/manual-cli-concrete-proposal-1/wheels.yml:241` selects both families for manual dispatch and only the tag's selected family for push. The six platform records at line243 through line248 are unchanged, yielding manual2x6 and tag1x6 CLI producers. Aggregate validation at line395 selects the corresponding two or one family and requires exact twelve or six archive filenames at line414.

Manual version lookup at line274 reads the actual CLI manifest and resolves only the documented workspace version form. It validates a concrete numeric version, derives the family's tag spelling and invokes existing unified-release-family preflight at line289. Tag selection retains the original branch at line292 and requires matrix/tag family agreement at line297. Manual artifacts include both family and platform at line290, avoiding collisions. Tag artifact names remain compatible with the existing publication consumer. The manual aggregate artifact at line455 is SHA-named and distinct from the tag release artifact.

Independently checked all six changed-region Bash run blocks with bash -n, all exit0. Invoked the existing preflight read-only on actual current CLI manifest versions v0.16.0 and rpptx-v0.14.0, both passing their respective exact-family validation. Wrong999.0.0 tags and the legacy py-rdocx-v0.16.0 form were rejected. These controls qualify present lookup/preflight semantics only, not prepared0.16.1/0.14.1 artifacts.

README/LICENSE identity and nonempty executable checks remain at the external workflow line447 through line449. Unix executable mode and regular file checks remain at line441. The one inventory defect above does not invalidate the other preserved guards, and their presence does not excuse duplicates.

## Publication and source boundaries

Independently compared complete workflow bytes outside build-cli-assets and cli-assets with the canonical existing workflow. Every outside byte matches. CLI producer attestation at the external workflow line362 remains push-only. Registry publication, unified artifact verification, main-SHA/sprint-tag checks, selected allowlists, thirteen-subject checksum/attestation gates and GitHub release jobs remain byte-identical and push-only. Manual dispatch reaches builds and artifact uploads, not those publication jobs. This is consistent with `.claude/commands/release.md:72` and line102.

Hosted build-only rehearsal on the final reviewed/pushed SHA remains incomplete. Its real12 wheels/2 sdists/12 CLI archives, current versions, installed results and exact provenance need actual separate evidence after all repair prerequisites and feature claim. No missing-evidence defect is manufactured here, and no local/in-memory control is substituted for a hosted run.

No workflow, source, tests, external proposal, runtime, UI, remote state, dependency or Git state was changed. Only this canonical review file was written. The read-only semantic controls used in-memory artifacts and no tracked test additions. Review ends and returns control for separate remediation after the approved claim barrier.
