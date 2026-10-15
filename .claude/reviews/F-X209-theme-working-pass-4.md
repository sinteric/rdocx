# F-X209, theme correction, pass 4

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: immutable theme-source21-capsule, one complete native carrier with a bounded absolute-RGB correction and its existing theme control. Layout lifecycle additions in the same carrier are excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Prior pass3 D1 is resolved within this slice.

## Authentication and scope

Receipt SHA `708a5d94dc2c881cd7d289bf18bac7b6d5710a63594ed39501257114fd02d9d5` authenticates all 13 declared records. Complete diff SHA `bd10bf776702dea3d8307b7e8d6444a793e51e7614854af2d14a902e4074a498`. Copied native before SHA `37034a3ca21983adb3ba3f70364d0a486d82c6167a74bf3b3645f5b41def4f6d`, after SHA `44640e0c7411d88d06ec55d71769eea42c872196a3ab66bc5bb10b38592a5759`. Source21 map SHA `a7ec1c9b971daccfbeab36b14679d2a51566128ea283dcf59e0adf1ef0f401a6`. The approved F-X209 theme writes and preservation contract, canonical microscope command and workflow govern this pass.

## Resolved finding

Pass3 D1 retained tint/shade transforms after assigning absolute RGB. The srgbClr branch now patches the selected value and clears qualified modeled transforms before the source-equality no-op check and candidate publication. See `theme-source21-capsule/lib.rs.after:3589` and `theme-source21-capsule/lib.rs.after:3621` under `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/`.

The removal helper walks the qualified theme/themeElements/clrScheme/selected-slot/srgbClr path, records only selected direct transform ranges, and copies all other ranges byte for byte. Foreign transform lookalikes, unknown attributes, child payload and non-whitespace body payload cause refusal before publication. See the same copied carrier at lines 7767, 7800, 7809, 7820 and 7848. This preserves prior pass2 namespace and lexical-preservation resolutions and does not normalize the whole theme.

The extended existing control imports an aliased theme with a tint and independently retained opaque child, assigns RGB 17/34/51, asserts the effective getter result and transform removal, preserves opaque lexical bytes, and saves/reopens exactly. Foreign rebound and opaque-attribute transforms must refuse with unchanged package bytes. See `theme-source21-capsule/lib.rs.after:19469`, `:19487`, `:19491`, `:19513` and `:19534`.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: 0 findings in the absolute-RGB correction.
- Contract: 0 findings in effective color assignment and source preservation.
- Panics: 0 findings. Range positions come from ordered reader events, and the pending transform expectation is guarded by its presence check.
- OOXML: 0 findings in expanded owner selection, transform removal and opaque refusal.
- Tests: 0 findings in the bounded transformed-source and refusal controls.
- Structure: 0 findings. No new module, trait, dependency or forwarding wrapper.

## Evidence and limits

The actual theme row in receipt SHA `124783583f405937ecbcde6f90b72328e6341a2bd2bb8cdb5193503429d58255` ran exactly one test, 1 PASS, 0 ignored. Its authenticated raw log SHA is `618fe227178c167f6f8f0b8363a8064c482539a3f54b175b5a3d4db1d568af34`. SourceAfterMatches and owned cleanup are true, with natural ESRCH and no signals. The same two-row attempt remains failed because its separate layout-lifecycle row exited 101. This review neither resolves nor credits that excluded failure. The source20 predecessor compilation failure remains retained.

No tests were run by this reviewer. No installed, renderer, native Office, lifecycle or whole-feature acceptance is awarded. This pass ends at this report.
