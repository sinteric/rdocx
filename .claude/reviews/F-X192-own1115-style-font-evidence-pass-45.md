# F-X192, own1115 style font evidence, pass 45

**Reviewed**: Independent read-only evidence audit of the actual single-factor native capture following clean preparation review 43. Native receipt `4401c6529bc1fe0b33693c8f427d998d50ee79c4a0402fc7ded63fe6ef7ac408` and comparison report `84c4d6d16670cb5d053b9be35a6d997befc4b465d865393b27365184d510db4d` in `/private/tmp/S91-F-X192-own1115-style-font-single-factor-r1-prepared/root-native-1`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None in the retained evidence or its diagnostic qualification.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: Independently recomputed the complete comparison using the existing analyzer definitions without running its main. The recomputation equals the retained report. Every saved stage's SHA and size matched. All 17 entries, their order and text, all 18 instructions, source operands, paragraph properties and scalar run properties remain equal to the unmodified actual native result `071bb55f8a8f229a3d9a82456394119eff7f22b34e5644ab405a8cd1e950c373`. No font-property differences were found.
- Contract: The explicit Bibliography style `w:rFonts/@w:cs="Arial Unicode MS"` survives before F9, after F9 and reopen. This demonstrates retained direct style metadata, not physical font selection, typography causality or a portable production policy.
- Panics: This audit performs bounded package and XML reads only. No native UI, Cargo, producer stylesheet or product operation was executed.
- OOXML: The report retains complete raw-tree differences instead of normalizing them away. F282K09 has 21 `w:rsidR` run-attribute changes from `00D4707A` to `00DD216D`. Its text, run count and run properties are unchanged. Therefore complete run trees are not literally identical, despite unchanged formatting. Package metadata changes are also retained.
- Tests: All three saved stages retain the explicit style intervention. After-F9 and no-F9-reopened files are byte-identical at `0b6dfe245da2a47194cac1d802cfae4d0e4df145ce943f3d64881a5f97f90116`. The preexisting unrelated saved document matches its authenticated SHA. These are diagnostic observations, not bibliography or full-catalogue acceptance.
- Structure: The evidence uses the existing package analyzer and canonical review record. No source, expectation, baseline, environment or helper was changed during this pass.

## Supported conclusion and limits

The explicit complex-script style attribute survived, while this measured regenerated bibliography retained the same text and formatting as the unmodified full-context native result. This does not establish why Word selected its recorded run fonts or whether a physical font was used.

The actual receipt records document-only style selection before F9, which can itself update field caches. Saving and Saved states were observed for that stage. After F9, Undo Update Fields and Saved state were observed, but a distinct Saving transition was not observed. Reopen byte identity authenticates the retained disk output without inventing that missing UI transition. Native application settings were not changed. No product acceptance, native expected-result replacement or cross-host inference follows from this experiment.
