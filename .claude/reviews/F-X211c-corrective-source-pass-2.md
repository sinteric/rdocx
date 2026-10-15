# F-X211c, corrective PowerPoint source, pass 2

**Reviewed**: Frozen3346-file source map `9dc60b0b97227eb1c11b6fb525ded94e4c9a447bf2034dc336802d5bf103691c`, worker on HEAD5e05028a and its producer source capsule. Independently authenticated all mapped bytes, sizes and modes in both trees. Exactly two carriers differ from the failed corrective predecessor: native rpptx lib.rs and the existing Python documented-examples test entrypoint.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this corrective source review. Both pass1 findings are resolved. Installed controls and full compatibility acceptance remain separate gates.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D1 resolution: replay successful owned contributions

`crates/rpptx/src/lib.rs:25029` first obtains the bounded captured measurement events and checks the actual trial result at line25038. The corrected path opens a new bounded contribution scope at line25042, replays those exact events through the existing provider at line25048, retains its generated approximation suffix at line25049 and finishes the transactional scope at line25050. Replay and finish errors are checked before publishing any diagnostic result. Failed or overflowed measurements do not become approximation facts.

The provider replay validates every current physical font ID and trace bound before mutation at `crates/oxml-layout/src/font.rs:1428`. It calls approximation recording only for Contribution events at line1524. Resolution attempts alone cannot establish an approximation. The existing requested/actual physical-family check and exact message formatter at line1682 remain the sole authority. Genuine requested faces suppress approximation there. The corrective path does not create a new font resolver, choose a substitute, alter glyph resources or infer identity from generic family labels.

The scope finish truncates its diagnostic suffix using the existing mechanism at `crates/oxml-layout/src/font.rs:1309`. Each successful owner retains its own facts, while the deck-level refresh and fit-report projections apply stable first-encounter request deduplication at `crates/rpptx/src/lib.rs:2994` and line3024. Their local request keys remain separate for Aptos and Aptos Display. Non-approximation owner diagnostics remain intact. The former missing post-measurement suffix is regenerated from actual successful physical contributions rather than from loaded-font history.

No additional current-face, failure-boundary, trace-safety, deduplication or font-selection defect was found in this bounded repair. The seven installed controls have not been credited by this source verdict.

## D2 resolution: bind the actual authored nonvisual owner

`crates/rpptx-py/tests/test_documented_examples.py:10177` now selects the direct shape nonvisual path and matches the actual authored shape_id. Line10178 requires exactly one owner. Description and title are asserted on that owner at line10180, and line10182 independently verifies the slide-tree owner remains undescribed. Existing highlight, reopen, decorative, invalid-value rollback and clearing assertions remain present.

This corrects the qualification owner, without removing a saved-source assertion or moving shape metadata onto the slide tree. It changes the worker's new regression control only. The frozen P-AREA-14 body and its original missing-facade failure remain separate unchanged evidence. Neither receives retrospective success credit.

## Unchanged reviewed composition and boundary

The prior corrective owner survival, Row.cells, arrangement aliases and foreign refusal, exact theme-name edits, alt fields, highlight alias, optional core author and shared stale formatter are byte-identical to the first corrective review. Their bounded no-finding dispositions stand. The original two-failure installed receipt and review `2b89888d6442cf0a49dac47dd5f2ab41ddd0df2261f77ca65fd1dbf8f1392a96` remain retained as failed predecessor evidence.

This review does not infer runtime success from source, Clippy or a wheel build. Fresh installed controls, typing, exact paired provenance, declared native/render obligations and the complete frozen100 correct-file metric require their actual separate gates. No corpus expectation, digest or denominator changed.

No source, test, installed runtime, CP environment, output artifact, dependency, Git state or native UI was modified. No build, test, installation or runtime probe was executed. Only this canonical review file was written under the exclusive review-file lease. Review ends and returns control to orchestration.
