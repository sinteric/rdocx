# F-X211d, actual installed remediation, pass 1

**Reviewed**: Frozen3348-file source map `dc1dbafd420ccff9afaca0f2cde4b1a027c39dd6c0d17c961c466f3456363f70` and patch `d62bf315432de22abe62d2a8ca1ce3f8b3e3419cf17e70d82d8922f356f36bc5`. Independently authenticated every mapped worker body, size and mode. Exactly document.rs binding and the existing Python core-test entrypoint differ from source4007d483.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this bounded source remediation. Both actual-installed review findings are resolved in source. Fresh installed acceptance remains uncredited.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Header publication authority

`crates/rdocx-py/src/document.rs:2075` now carries the validated existing section index in HeaderFooterAppend. The concrete publication variant at line4134 maps to the selected Section owner at line4810. The existing physical-authority lookup and refusal at line4836 remain mandatory. The previous unit-variant fallthrough no longer rejects every header append, and no blanket owner-preflight exemption was added.

The method validates the actual HeaderFooter handle before staging at `crates/rdocx-py/src/document.rs:2004`. Native story creation/insertion stays confined to the staged candidate. Existing selected-owner publication retains the Section Owner state and renews its affected content authority. Header/footer paragraph/table physical routes remain distinct from the section furniture scopes. A shared physical part is not duplicated or assigned a new identity merely because one section's furniture authority changes.

The next HeaderFooter Content path is captured from the prepared Section at line2081. Returned paragraph binding translates the section furniture scope through the existing prepared mapping at `crates/rdocx-py/src/paragraph.rs:462` and `crates/rdocx-py/src/document.rs:4591`. Result allocation occurs at line2090 before revision commit at line2100, candidate publication at line2101 and updating the caller HeaderFooter path at line2102. Origin/serial validation, prepared births, overflow checks and allocation-failure rollback remain the existing implementation. No unchecked returned child or premature native publication was introduced.

The formerly failing shared-physical-header and corrective section/header controls still need actual successor execution. The source now reaches their prepared-capture paths, but this report does not infer that they pass from static inspection.

## Exact native exception contract

`crates/rdocx-py/tests/test_core.py:6537` imports the precise public RdocxError, and line6553 asserts that exact class with the unchanged opaque-cell-metadata message. This matches the existing Error::Other translation at `crates/rdocx-py/src/lib.rs:116` through line130 and the native topology/setter distinction at `docs/hld/10-bindings-spec.md:370` and line402.

The test retains same-object no-op, complete package identity, held first/second Cell use, cell count, exact opaque bytes and reopened source checks at `crates/rdocx-py/tests/test_core.py:6551` through line6560. No broad Exception catch, removed assertion, weakened metadata guard or changed frozen body is introduced. Actual failure evidence is preserved separately.

## Acceptance boundary

The genuine source4007 installed28-pass/4-fail log and actual diagnosis report `7b3824c694f7b87d615b15e01a39e9fed5e7cbe608882513f3d80d36d60892c7` remain historical failures. This review does not rescore them or treat a source check/build as installed success. Current paired origins, exact32 installed controls, typing, policy, routed risk gates and complete parent corpus/native obligations remain separate actual requirements. All other reviewed d source bodies are unchanged.

No source, test, runtime, CP environment, output artifact, dependency, Git state or native UI was changed. No build, test, installation or runtime diagnostic was executed. Only this untracked, unstaged worker review report was written under the delegated exclusive lease. Review ends and returns control to orchestration.
