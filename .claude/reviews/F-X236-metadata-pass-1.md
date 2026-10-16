# F-X236, documentation and measured metadata, pass 1

**Reviewed**: documentation and metadata diff only in HLD12, HLD15, the approved plan, README.md and scripts/readme_doctests.py. Product source remains covered by independent all-aspects pass2 and was not re-reviewed.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

- HLD contract: the new paragraphs accurately describe selective annotations, the real external non-LTO profile, separate initialized and first-use observations, independent fresh/initialized save-reopen semantics, retained measurement provenance and the unchanged paragraph-mark control. They make no end-to-end speed guarantee.
- Plan contract: the release-profile citation now points to Cargo.toml147, the actual fat-LTO setting. Existing measured metadata carriers are explicitly named, with unchanged64byte compression tolerance and10MiB archive ceiling. The semantic qualification names the actual independently inspected external consumer rather than an absent test function.
- Footprint correctness: README and catalog contain the same rdocx tuple,3730281compressed bytes,51941819normalized member bytes and37members. Independent measurement of the current archive returned3730285/51941819/37. The actual repository validator passes, with a4byte compressed difference inside the unchanged tolerance. No unrelated row or validation rule changed.
- Artifact identity: the measured archive SHA-256 is1d0340f9f274e47222bb407189b332ee03429dff237245726d484b3fe4b5ffac. Its run.rs and README members exactly match the frozen worktree bytes. The22-package verification receipt records genuine local-source dry-run publication verification with no publication.
- Structure, compatibility, panics and OOXML: this follow-up adds prose and numeric measurement evidence only. It introduces no additional executable behavior, public signature or product structure.

This pass does not repeat the prior product-source review or claim to have
rerun the complete scoped gate. It ends here and returns control to sprint
orchestration for preparation and integration. No source was edited.
