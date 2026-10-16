# F-X247, all, pass 1

**Reviewed**: Frozen working tree in /private/tmp/rdocx-s94-x247 against claimed Base90165c94e582b21e510c0c3c00ab1b6375415ba0,15 tracked files,1672 insertions and97 deletions. Authenticated diff SHA256 f6ad91d2757db5e529b04fb105a3f277bc08bf8b047c0645799248e8c5932524 and all3517 tracked file hashes, with zero drift before and after inspection. Reviewed the approved revised plan, progress, repository workflow/microscope, cited conversion/rendering/preservation/qualification contracts, complete diff and actual evidence. Root-authored plan prose corrections are included. No source changes, builds, Cargo/cache mutation, GUI activity or remediation.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Terminal atom correction is applied again when attaching scripts

`crates/rdocx-layout/src/math.rs:1037`

The new mapped-run path treats every run end as an ordinary atom boundary at line1002, then adds the selected MATH italic correction to both glyph advance and measured width. layout_script obtains its base through that same path at line1345. layout_script_pair then independently reads that base glyph's italic correction at line1401 and adds it again as the superscript gap at line1403, positioning the script at base.width plus gap at line1419. Thus a mapped x or f base now puts a superscript after raw advance plus twice the correction. A subscript also inherits the extra terminal correction despite the existing zero-gap MATH attachment policy at line1405. Combined sub/sup attachment has the same interaction at lines1515 and1566.

This is not a zero-valued theoretical path. Independent read-only stdlib SFNT/cmap/MATH inspection of the actual pinned Cambria.ttc face1 found italic corrections x62, f60 and alpha62 font units with2048 units/em. For x at12pt the extra duplicate is0.36328125pt, small enough to remain hidden inside the aggregate1pt gate. The source-proven duplicate is distinct from a new native Word measurement. Preserve one application at the appropriate completed atom or script attachment boundary. Add meaningful positive-MATH-correction controls for superscript, subscript and combined scripts, alongside the existing pi and ordinary-identifier controls. The approved increment does not authorize double script correction merely because a nested base run ends.

### D2, Synthesized text boundary spaces bypass the conversion text cap

`crates/rdocx/src/math.rs:482`

Contextual MathML boundary normalization appends a logical space after mtext without charging the text budget. The leading-space insertion has the same omission. equation_from_mathml normalizes and returns the resulting tree at line129 without validating its final text size. The parser's input lexical text count therefore no longer bounds produced logical text.

Concrete in-memory installed-candidate reproduction: a MathML math root containing mtext with exactly524288 ASCII x bytes followed by `<mspace width="0.333em"/>` is accepted with empty diagnostics. Public Paragraph.add_equation plus DOCX serialization produces524289 math text bytes. The input remains below the1MiB byte cap. The frozen source confirms the unchecked synthesis, while MAX_TEXT_BYTES at line25 is524288 and validate_tree at line1509 would reject the returned tree. No probe output file was created. The approved resource/atomicity gate at `.claude/plans/F-X247-design.md:210` explicitly forbids added normalization from bypassing this cap. Charge synthesized text or validate the completed conversion before returning/publishing, with actual at-limit and over-limit boundary-marker controls.

## Smells

0 findings.

## Nitpicks

0 findings.

## Evidence and limits

- Original73 inputs exactly match the manifest and compact-list SHA2567288e75fd28f588e1af81625ea762e1a9d0a01362027293a63827f222f091eba. Independently executed only the stdlib saved-OMML observer over existing Python/native original and reopened73-equation packages, all four yielding73 matching source-authored rows. Inspected actual eight package mutation tests and finite symbol/role/script/binomial/fraction/space and reverse tests. These do not cover D1 or the synthesized-space limit in D2.
- Authenticated actual Pandoc3.11 binary hash and all per-row output hashes for146 inline/display records. Both contexts retain72 emitted rows and original degree row43 warning/raw fallback. This is not73 successful Pandoc conversions, and no forced dfrac/tfrac style fidelity is inferred.
- Authenticated `/private/tmp/S94-F-X247-work/frozen-native-attempt-01/receipt.json`, SHA256edb7441a344ba33fa5c99129d3f1bd801e58fdd66f18d2360bd25d5890a2f04e, fresh consumer binary4ef3802351eef7dee0bd7f155471f51330d46f4163c41ea2c0355aaaf2a93994, all284 artifact size/hash records and all command-log hashes/exits. Its source snapshot SHA256cd3691ab12a387910d93b6aaeafcb0e1bd367aa1afba4f257e7923c22d0bae83 equals the actual freeze hash inventory. All7 new controls pass PDF/derived-width/ink1pt plus150DPI/.99SSIM. All7 real1.01pt translated outputs fail the composite gate. All6 historical captured controls and their exact-expression checks pass. Historical translated outcomes remain5 failures with radical passing. Passing these bounded numerical gates does not refute D1.
- Current captures bind Word16.114 build16.114.26101123. Historical captured replay separately binds Word16.113.2 build16.113.26092012 and original producer/source/PDF/font/tool identities. The strict old-installed-application check still refuses the actual newer application. The old13-family manifest pin remains unchanged, without claiming fresh export from its historical Word16.104 producer. Greek's current PDF preceded the separate disk reopen, as root's receipt records. The prior independent capture audit's selected roles, spaces and normalization limits remain applicable.
- Earlier iterative private consumers reused product/mutation PDF names. Failed measurements/logs and initial five150DPI raster pairs remain, but not every earlier failed raw PDF. Original native/source/historical captures and the final complete immutable284-artifact attempt are retained. No overwritten predecessor PDF is reconstructed or claimed authenticated. The retained historical-final-positive.json is the pre-2mu nested failure, not the passing final receipt.
- Scoped gates and actual package/catalog reconciliation are still separate completion obligations. This pass does not label them complete. Later source or catalog changes need affected successor review before preparation.

## Not found

- Correctness: 0 additional findings in finite glyph mappings, logical phi variants, canonical fraction topology, direct nested-Bar2mu selection, NoBar/non-MATH fallback padding or the generic run-only nonnegative display adjustment. Mixed/tall frames retain their separate rule. D1 and D2 remain the actionable failures.
- Contract: 0 additional findings. New behavior stays finite and uses existing carriers/signatures. Explicit fraction-style normalization, native Unicode spacing approximations, role differences and unsupported-context diagnostics remain documented. Historical producer mismatch is not disguised as fresh old-Word success.
- Panics: 0 findings. New glyph indexing is guarded by mapped character/glyph counts, finite color-free lookups use matching arms, and literal groups preserve bounded depth/events and XML validity apart from D2's synthesized-text accounting.
- OOXML: 0 additional findings. Normal text/function/plain roles, xml:space and NoBar/delimiter structure survive actual native saves. Conversion/authoring reuses typed writers and opaque-source preservation. No new source parser or schema reordering route was added.
- Tests: 0 additional independent findings. Actual semantic and rendered mutations are distinct from expected-value edits. Missing script-attachment and exact synthesized-cap regressions belong to D1/D2 remediation. Raw unconditional reporter labeling, semantic observers, diagnostic outcomes and historical failed attempts remain separately classified.
- Structure: 0 findings. No new product file, module, crate, model field, variant, trait, generic, font asset or dependency was introduced. Bounded helpers transform real current conversion cases rather than adding forwarding abstractions.

End of pass. Return the tracked write lease for separate remediation and a numbered successor review. This diff is not ready for preparation.
