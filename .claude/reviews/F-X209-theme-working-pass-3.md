# F-X209, theme correction, pass 3

**Reviewer**: fonts_resume, independent of the correction.
**Reviewed**: the source14 to source18 theme setter/source-selection correction in one existing native carrier. Added layout inventory readers in the same complete diff are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior pass2 D1 and D2 are resolved in the bounded correction, but a new absolute-RGB write regression remains.

## Bound input

Receipt `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/theme-source18-capsule/receipt.json` authenticates as `f9400062c3431d27c421fed919f241494facae0551253f7483bab9ea0e74726f`. All ten referenced records authenticate. Complete diff is `569ca991e0bbc14f426dbbe604b40ab3e8c187259798ebfa5b744ae58ba45c5a`. Native before is the exact pass2 after `eb67898c2b8e283021a284d98c14260914e4289bb2326758a5bd26c3cad951a0`. Copied native after is `37034a3ca21983adb3ba3f70364d0a486d82c6167a74bf3b3645f5b41def4f6d`, matching the 3131-record source18 map `4d7087902239c87f19d567b5798e8d8ea048cfc1d16d057502c0fca821b594b4`.

Source14 to18 also changes slide_parts.rs and Python slide.rs for other work. Those changes are excluded from this review. The unchanged color provider matches source18 SHA `de124822cca0f7ec266acae56926c3ea8a2c693362cbd156a5e75aa3e25e0375`, and the unchanged theme provider matches `2562096f5b724834be30540ecc149a44ff890b1f3bc207a61a258cc2399aec44`.

## Defects

### D1, absolute RGB writes retain transforms that change the requested color

`crates/rpptx/src/lib.rs:3587` in copied `lib.rs.after`.
Getter citation: `crates/rpptx/src/lib.rs:3698` in the same copy.
Qualified provider citation: `crates/oxml-drawing/src/color.rs:828`.

For an existing srgbClr choice, the correction patches only its val attribute and retains all child color transforms. A valid imported accent1 such as an srgbClr containing a qualified tint or shade therefore remains transformed after set_theme_color succeeds. Theme.colors resolves those transforms when returning the current RGB. Assigning RGB(17,34,51) can consequently return and render a tinted or shaded value rather than the requested RGB.

The previous ColorChoice::srgb replacement had no transforms, so this is a regression introduced by the preservation correction. There is no approved base-channel-only assignment contract. Absolute RGB replacement owns its understood transforms as well as the base value. It must not silently leave them active, and preservation of genuinely opaque content remains required. The implementing agent confirms the requested effective RGB contract and that no transform-clearing path exists in the frozen input. The canonical built-in test does not contain a transformed slot, so its current pass does not decide this case.

## Smells

None.

## Nitpicks

None.

## Prior findings and aspects

- **Prior pass2 D1 resolved.** theme_source_owner uses NsReader resolved namespaces for the complete direct owner chain, requires a DrawingML theme root and rejects an absent or duplicate selected owner. Foreign or rebound ancestors cannot satisfy the selected path. Copied `lib.rs:7479` is the shared source-selection route for colors and fonts. The deciding control includes q aliases and foreign root, scheme, slot and font-collection owners.
- **Prior pass2 D2 resolved.** The selected attribute patch retains the original bytes outside its exact raw value interval. Numeric-entity opaque root attributes and aliased wrapper names remain literal. Font writes retain supplemental fonts, attributes and unrelated source ranges. A non-sRGB type replacement refuses nonempty or opaque payload rather than dropping it. See copied `lib.rs:3587`, `lib.rs:3591`, `lib.rs:3659` and `lib.rs:7568`.
- **Correctness and contract**: the new D1 above remains. Native publication still stages the actual selected theme part, validates the resulting theme and reopens before replacing live state. Invalid slot and forbidden/control-character operands refuse before publication.
- **Panics, zero findings.** Source intervals come from the same resolved reader event and raw attribute value. Checked owner selection and duplicate refusal precede patching. Pending selected-owner extraction is depth guarded.
- **OOXML**: the prior namespace and opaque lexical findings close in this slice. D1 is the remaining understood color-transform semantic issue. Unknown source is not granted deletion authority by this report.
- **Tests, zero additional findings.** Existing canonical scheme/font, supplemental/minor/format-scheme, unrelated-part, repeat/reopen and atomic predicates remain. New deciding source controls genuinely cover the prior two defects, but not transformed RGB assignment.
- **Structure, zero findings.** Two actual setters share the concrete source-owner selector and attribute patch. No new module, dependency, ownership generation or detached editable model is added by this correction.

## Authenticated runtime and limits

Retained source17 attempt1 receipt `412fb84382442c286717adf8ce915b9c257373c272a6694efbac5aedd5a7feb3` and log `2fab35ba3f1aac12d487544258c0b4240a269f52727a10173b8a967532effe8e` authenticate a compiler failure, not semantic success.

Current source18 attempt2 receipt `3d4218c5a9e01897a4f8f2fd2e0ecca450e80be9a59d0438e7563312869573c5` and log `97d47b944cdd97caa07d8ca26e942ba1944ad4c098d52f5ee60bbdf40d00c362` authenticate the exact theme selector: 1 passed, 0 failed, 0 ignored. The receipt records source-after equality and natural owned-group cleanup, ESRCH and no signals. It is not a source18 all-target Clippy, installed, native paint or full-feature gate.

No reviewer runtime or source modification occurred. Layout lifecycle/import and later Python changes are excluded. End this review before distinct remediation.
