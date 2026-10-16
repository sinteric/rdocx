# F-X249, correctness, pass 2

**Reviewed**: Frozen nine-file working diff against 43c54fb01714f28d679085ff927d5994b143b525 after separate remediation of D1/D2/D3. 1199 insertions and 46 deletions. All nine digests match /private/tmp/S95-F-X249-worker-2/frozen-source.json, SHA2565a39450f5c988a17d52b6659a5ef1c3c81e5bba0b4778a70053fcade95367089, before report publication. Relative to pass1, only the OXML implementation/tests and native regression carrier changed. The approved plan and canonical review rules remain the contract.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found. All three pass1 defects are resolved.

## Smells

None found.

## Nitpicks

None found.

## Previous findings

D1 is resolved. The same local semantic-URI predicate now handles resolver results for metadata elements and attributes. It decodes one lexical entity layer and compares the resulting URI to Word main. It does not decode already-semantic scope again. New controls distinguish numeric-reference aliases from literal ampersand-reference lookalikes, cover local element/attribute/default namespace bindings and verify foreign raw payload survives alongside accepted metadata. Retained source is not globally normalized.

D2 is resolved. Conflicting retained property types use the same qualified supported-type predicate as the parser. Date and dropdown controls now refuse supported checkbox/repeating-section conflicts, while foreign same-name children remain accepted and byte-preserved. Native late-failure controls include extension conflicts after an earlier selected valid control, retaining atomic source publication.

D3 is resolved. Word-main QName correction excludes the actual checkbox and repeating-section extension kinds. Their marker namespace is no longer changed by selected metadata updates. OXML and native selected-text controls retain repeatingSection/repeatingSectionItem, remove the qualified placeholder flag and reopen with the same expanded control kind. Existing kind guards and publication rules remain unchanged.

## Not found

- Correctness: no new finding in the remediation. The finite civil-date, dropdown, display/metadata/bound-scalar preparation and late-failure staging logic remain within the approved behavior.
- Contract: no finding. Individual KeyError/local ValueError and distinct native/bulk counts and error families are unchanged by remediation. Native/schema divergences and producer-owned lastValue remain explicit.
- Panics: no finding. The new namespace predicate safely rejects failed decoding as unqualified. The remediation adds no unchecked indexing, arithmetic or new caller-controlled panic path.
- OOXML: no finding. Alias, shadow, default and one-decode decisions preserve actual namespace meaning and source-owned foreign bytes. Extension type identity is retained. No unselected metadata or namespace declaration rewrite was added.
- Tests: no finding. Each original defect has a sensitive failing predecessor control and a remediated positive/negative pair. Added native controls exercise public selected edits, extension identity and late-failure atomicity. No helper or assertion was weakened.
- Structure: no finding. The semantic comparison is one local closure with actual element and attribute consumers. The existing supported-type predicate is reused. No public field, API method beyond the two approved methods, dependency, module, trait or generic was added.

## Evidence inspected and limits

The bounded three-regression predecessor run actually fails all D1/D2/D3 cases. Its production-prefix SHA25653c76cfda71f41c975e38d374333b5081dbaeec1d599bea2c3d301a4a9ac7444 matches an independent in-memory reversal of the exact three remediation edits. No predecessor source file was overwritten by the reviewer. The red-source receipt and logs are retained in worker1, not worker2.

Authenticated inspected log hashes:

- review-red.log: 95055ad75c9215e0e6435f7fe9d9d6fe7f8b0e5b362e2bf5cdb793a8904bd70d, three actual failures.
- review-green.log: 4753e27832cc2b55c2100c74a6e2b5731d09f8238d45677cd68d72c9174def52, eight focused OXML passes.
- remediation-native-green.log: 5c233a2365eb358346421dc6ce3c28041bc9003d6ca45e2f84b57c1410bbcb38, three focused native passes.

These are worker executions inspected as evidence, not tests run by the reviewer. Successor whole-source wheel, installed/oracle/typing/stub and remaining scoped/publication closure were still pending when review began and are not declared passed here. The immutable native seal and approved normative differences retain their prior qualified meaning.

Only this review file was written. No source, private evidence, build, cache, progress or Git mutation was performed. Return the worktree lease to the orchestrator. No further correctness confirmation pass is needed without a new relevant change.
