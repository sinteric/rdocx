# F-X209, theme working slice, pass 2

**Reviewer**: fonts_resume, independent of the implementation.
**Reviewed**: two existing carriers, 389 additions and 40 deletions from source-production-12 to source-production-14. Native staged color/font setters, live Python ThemeColors/font collections and one native deciding control only.
**Verdict**: 2 defects, 0 smells, 0 nitpicks. This slice is not ready for preservation qualification.

## Bound input

Receipt `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/theme-production14-capsule/receipt.json` authenticates as `3536b69188e3b4c26882f2d5e86fc8de43f73cee2ba9271a2982a11ddf41c474`. All twelve referenced records match hashes and sizes. Both before copies equal source12 and both after copies equal source14.

Source maps12/13/14 each contain 3131 records, with hashes `f926abb44a1bec4d3fea23fdee040640a627d4f017fa2df02bf5a05630a3bb13`, `ee536ea8afa0c00084bf68d096ca90a6d187b6dd49adf5d513165028c1157231` and `41222840da7697b72ea2af80230f1672e8ecc75bb4345780333dd71da82f9c96`. Map12 to13 changes only native lib.rs and Python slide.rs. Map13 to14 changes only native lib.rs for the control. Complete diff hash is `ea3b28e89ae7091100ba35c6c61dbff35a09779cb75430cbab778a7f7bf3c4df`.

Copied native after SHA is `eb67898c2b8e283021a284d98c14260914e4289bb2326758a5bd26c3cad951a0`. Copied Python after SHA is `5cbfd943ba9b1e1c3e47cda6ec94eb5a7902098ee69cbfad21ab427fcc737334`. Provider `crates/oxml-drawing/src/theme.rs` and binding ownership provider `crates/rpptx-py/src/presentation.rs` match source14, respectively `2562096f5b724834be30540ecc149a44ff890b1f3bc207a61a258cc2399aec44` and `3efcd232edc5693fa0ee42b911d5cbdf5ba10a9d66c6fc7c0c244ecff0bdb183`. Their current files were used only after this immutable-map qualification.

## Defects

### D1, theme editing admits foreign or rebound owners and canonicalizes them

`crates/rpptx/src/lib.rs:3550` in the copied after carrier.
Provider citations: `crates/oxml-drawing/src/theme.rs:493`, `theme.rs:499`, `theme.rs:522`, `theme.rs:549` and `theme.rs:565`.

The new editable_theme route resolves the actual theme relationship but directly parses its bytes with CT_OfficeStyleSheet::from_xml. That provider uses a plain Reader and local-name matching for the root and modeled descendants. There is no expanded-name preflight in this slice. A retained theme-like document whose a prefix is bound to a foreign URI can pass the local-name parser. A changed color or typeface then serializes its root and modeled descendants using canonical DrawingML a tags. The root writer also discards the original xmlns:a binding and supplies the DrawingML URI. A foreign or rebound owner is therefore silently reclassified and changed instead of being preserved or atomically refused.

The same owner admission issue applies to local-name lookalikes in scheme/slot positions. The permissive existing reader is inherited, but the new write route activates the destructive conversion. Whole-package reopen does not reject it because it receives the newly canonicalized bytes. The canonical built-in theme control does not exercise this boundary. The implementing agent confirms no upstream namespace admission exists in the frozen slice.

### D2, a named theme edit rewrites opaque attribute lexical bytes

`crates/rpptx/src/lib.rs:3601` in the copied after carrier.
Provider citations: `crates/oxml-drawing/src/theme.rs:1334`, `theme.rs:1347`, `theme.rs:565` and `theme.rs:1374`.

Every successful theme setter commits the entire typed theme through its existing writer. The parser's raw_attributes function decodes and normalizes unmodeled attribute values, and the writer reconstructs those attributes. For example, an otherwise admitted theme carrying an unrelated root extension attribute `x:keep='&#65;'` is changed to an attribute whose value is literal A when accent1 or a font typeface is edited. This loses the original opaque lexical source even though that attribute was not the requested owner. Namespace admission alone does not repair this separate preservation defect.

The approved source contract preserves unmodeled XML verbatim and restricts named edits to their actual owners. The control checks typed format-scheme, minor-font and supplemental-font equality, plus unrelated package parts, but has no opaque attribute byte-retention assertion inside the edited theme. The implementing agent confirms there is no approved exception for this normalization. The setter must preserve those untouched source ranges or refuse unsupported source without publication.

## Smells

None.

## Nitpicks

None.

## Other reviewed aspects

- **Correctness and contract**: D1 and D2 above. For canonical source, concrete slot selection, font collection selection, control-character refusal, repeat no-op and staged reopening follow the approved edit semantics.
- **OOXML**: D1 and D2 above. Typeface updates mutate the existing modeled typeface field instead of replacing supplemental-font collections. No new format-scheme editing or copying policy is introduced by this slice.
- **Panics, zero findings.** Checked master identity and related internal part resolution precede native edits. Errors mutate candidates only. New theme vector indexing follows insertion or a successful existing-record search.
- **Tests, zero additional findings.** The deciding canonical control proves current color/font persistence, supplemental/minor/format-scheme typed retention, unrelated part bytes, invalid slot/control-character atomicity and exact repeat. It cannot qualify the opaque or namespace boundaries identified above.
- **Structure, zero findings.** Existing carriers and concrete owner proofs are reused. ThemeColors and font collections resolve the captured presentation/master identity for reads and writes. No new ownership generation, detached editable theme, module, trait or dependency is added.

## Execution qualification and limits

Actual all-target native/binding check10 receipt `093bf6d3c6dd7d4e606ec460ee2275319c1506eb06d9556a7f55d05080c558b8` and log `c1fea16b078280125c272e5aee65f9192f7e428cdbf908629a4ce70c1edcd88b` authenticate exit0 on source13. Its retained warnings are not a strict Clippy pass.

Actual native theme receipt `8831e94ab7af8a8a6baef53a92570b158c5078b67a9c57a1fcf5f1596634f905` and log `11c898c64eaf5a2f7a257cf4e63ea60923135f257a3cb7724285dc3610848c4b` authenticate the exact `write_tests::fx209_theme_slots_preserve_other_parts_and_refuse_atomically` selector: 1 passed, 0 failed, 0 ignored on source14. Both receipts record source-after equality and natural owned-group cleanup, ESRCH and no signals.

No reviewer execution or source modification occurred. Installed Python/stub qualification, shared-theme/multiple-master paint, lifecycle/import, native application rendering and full-feature acceptance remain outside this bounded pass. End the review before remediation.
