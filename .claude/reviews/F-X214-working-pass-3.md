# F-X214, bounded empty condition-list correction, pass 3

**Reviewed**: immutable source11 to13, one existing timing carrier, 11 added and0 removed lines. Scope is pass2 D1 only, with its public regression test. No whole-feature replay.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input and execution

Capsule receipt `/private/tmp/rdocx-f-x214-codex/.claude/scratch/F-X214-start/immutable-empty-list-correction-3/receipt.json`, SHA256 `569251a551c1803fd750d5a1d0c9b504f2240529da0fbd14b15bee534380523a`. All9 records and3 external references independently match their recorded size and hash. Complete diff SHA256 `cee3f4bf3f7769275ff7d2109c9ef31975ee3654fa47d6fb41ba6d1791c2bb87`. Full carrier preimage matches reviewed source11, and postimage matches source13 map `57de2942a26f4b69c049f1ffb7b7ba26f1f7325b445fe739b59bdc8e132d6013`.

Source11 to12 adds only the empty-list public test in timing.rs and the completed pass2 report. Source12 to13 changes only timing.rs, with3179 other records identical. The external retained RED proposal independently matches source12's exact timing carrier hash. Its delta to13 is the single production Empty-root branch plus whitespace-only formatting of five test statements. The source11 to13 complete diff is exactly that branch and the new regression test. No other production carrier changes.

The approved F-X214 plan and canonical microscope/workflow rules read in the preceding pass still control this continuation. This is a bounded successor to the ended pass2, not remediation within an audit.

Actual RED receipt SHA256 `585fb8de3c389b296ec5e91cd381f73948f502cfeafe103dc4d6aea99ab91a94` binds source12 `5e289534f92e27405b0194b08a137f8d077f5a597c2226626de47d452f4cbd14`. The exact public empty-list selector runs1 test and genuinely panics at the retained root expectation, exit101, with0 PASS/1 FAIL. Source match and natural safe cleanup are recorded. It fails on the first loop spelling, so no separate original RED execution is claimed for endCondLst.

Standalone first-row receipt SHA256 `3c28ca145a865c7826b96514351b955f5c0f45d727e2976aee24afaa4f359b06` binds source13. Its exact timing module selector is terminal0,5 PASS/0 FAIL/0 IGNORE, with safe row cleanup. Authenticated raw log SHA256 `6044195f8097d00c879d9b4e188346f5662620384b64c7c6bf25ca0b14622b77` shows the empty-list control and all four retained opaque, target-presence, alias/sublist and selected-work controls pass. The global scoped queue remains live according to the admission snapshot. This report grants only that completed row, not a global terminal, source-lease release or full gate.

## Defects

None in this bounded correction. Pass2 D1 is closed.

## Smells

None.

## Nitpicks

None.

## Aspects without findings

- Correctness: `crates/rpptx-oxml/src/timing.rs:2721` recognizes an Empty root before the child arm that requires an initialized root. Returning an empty presence result matches the accepted empty native condition-vector model. Both stCondLst and endCondLst are exercised through public CT_Timing parsing, with absent start/end metadata and exact source round-trip.
- Contract: the helper receives already selected condition-list fragments. The new branch changes only its empty-root case and adds no new public surface or acceptance relaxation. Nonempty root initialization, condition selection and explicit-target semantics are byte-identical to the reviewed source11 implementation.
- Panics: the previously reachable root expectation is bypassed for a self-closing list root. Subsequent child expectations still follow initialization by the nonempty root arm. No new arithmetic, indexing or unchecked source operation is introduced.
- OOXML: no serializer, namespace selection, child order, source range or opaque preservation path changes. Public empty-list round-trip retains exact source for both spellings.
- Tests: the original actual panic is retained and the current public loop completes both spellings. Existing metadata-overwrite and namespace-work controls remain present and pass in the same completed row. The formatting-only test edits do not alter assertions or fixtures.
- Structure: one explicit root case in the existing helper, no new file, module, crate, trait, generic, dependency, wrapper or flag.

## Limits

This clean bounded pass combines with pass1 and pass2's closed original findings. It does not qualify unrelated later source or replace required scoped, release timing, corpus, CLI, installed binding, native PowerPoint, archives, HLD and integration evidence. No runtime, Cargo, Git mutation, CP, Office or source edits were performed by the reviewer. The review ends with this report.
