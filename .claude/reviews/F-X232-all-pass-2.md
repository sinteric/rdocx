# F-X232, all aspects, pass 2

**Reviewed**: frozen working diff against d4ef72c894b2b0b4926696a90e42f07c3ee4cc63 in work/f-x232-codex, 28 files, 1183 inserted and 111 removed lines. Read the approved plan, cited rendering and checked-owner contracts, prior pass, complete source diff, remediation tests and recorded oracle evidence. F-X232 remains in-progress. Only this review artifact was written.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Prior findings resolved

- D1: `crates/rdocx/tests/regression_test.rs:403968` now aggregates actual segmented primitives into unique y/width bands before asserting six bands. All three native sizes and the independent 0.36 point bounds remain. The positive remediation log shows this test passing. Recorded original-gap and donor-gap mutations fail, so the observer correction has not concealed the former geometry errors.
- D2: `crates/rdocx/tests/regression_test.rs:403902` supplies actual Selawik bytes through the Calibri alias, asserts those bytes enter the resulting layout, alternates styles against fresh layout and repeats after mutation. `crates/rdocx-layout/src/engine.rs:19902` adds revised headers, footers, footnotes and endnotes with an actual caller font. It compares warm and fresh layouts before and after related-story mutation, exercises both styles repeatedly and checks the expected bar on every visible split-note fragment. The note actually spans multiple pages. Both tests pass in the recorded targeted runs.
- S1: `.claude/plans/F-X232-design.md:37` and `crates/rdocx-layout/README.md:41` explicitly name the public NoteLayout revision_ranges tuple change alongside the other approved pre-1.0 source changes.

## Not found

- Correctness: transient legal paint preserves stored formatting and revision metadata. Nested wrappers retain the inner revision color. Change-bar selection follows visible projected content, paragraph marks and the property-only black fallback. Note pagination clips the colored owning ranges to each visible fragment. No new incorrect boundary or precedence case was found.
- Contract and cache identity: style is carried through reusable engine context, paragraph/table identities, field substitutions and caller-font routes. Accepted/plain retains its existing dispatch. Invalid legal/accepted requests fail before output. CLI convert/render and all eight Python render methods and stubs expose the intended style argument and defaults. The checked paragraph publication change remains within the approved namespace repair.
- OOXML: checked admission closes declarations lost with regenerated paragraph roots onto preserved drawing owners. Retained run declarations and nested foreign shadows remain respected. Tests preserve exact opaque interiors, exercise inline and anchor drawings with local and ancestor bindings, reopen to a fixed point and refuse malformed replacements atomically. Unchecked source-snapshot parsing retains its previous path. No schema-order or dependency inversion was found.
- Tests and oracle independence: all six remediation targets pass in the recorded layout and regression logs. Four intentional mutations fail with source-restoration hashes matching the frozen files. Native Word source and PDF identities are separately bound, with independently measured 12, 24 and 36 point geometry. The declared 0.36 point bound rejects both previous gap rules. Evidence supports colors, stroke topology and scale geometry. It does not claim full-page SSIM or native legal change-bar selection parity, and existing strike metric divergence remains documented.
- Panics: changed production code propagates checked XML and namespace errors. No newly unchecked indexing, untrusted slicing or conversion panic was found.
- Structure: the existing convert.rs helper replaces duplicated underline formulas. No new trait, generic parameter, forwarding wrapper, module, feature flag, dependency or alternate revision model was added. The four affected archive observations agree with two actual measurements within the unchanged 64 byte tolerance, with member sizes and counts stable.

## Verification boundary

This source review does not claim final scoped verification. Earlier equipped evidence records 636 facade unit and 380 integration passes, while the pre-remediation regression run had 2420 passes and D1's known failing observer. The fixed regression targets and related-story targets now pass. Installed Python evidence records 360 passes. The latest full native retry inspected during this pass stops with four integration failures because LibreOffice was absent from that command's PATH. The implementer has identified the omitted mounted executable directory and is rerunning with the equipped PATH. That result, final policy/publication checks, HLD preparation and the reviewed two-entry behavioral hash recording remain implementation and orchestration gates.

The review pass ends here. Return the source lease to the implementer and control to sprint orchestration. No remediation was performed during this pass.
