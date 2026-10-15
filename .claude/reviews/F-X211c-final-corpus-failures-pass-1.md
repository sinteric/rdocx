# F-X211c, final corpus failures, pass 1

**Scope**: Two genuine unexpected failures in the fresh composed candidate's unchanged100 run. Source89d8, fresh Word2967/PPTc44. The four existing bounded non-success dispositions remain separate and are not reclassified as new fixes.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. Both require distinct source remediation and deciding controls. No frozen body or scoring rule changes.

## Defects

### D1. Missing Paragraph.bullet_char compatibility surface

`/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-03.py:21` assigns bullet_char and the actual traceback stops there with AttributeError. The current binding offers its canonical bullet provider at `crates/rpptx-py/src/text.rs:1124` and setter at line1147, but no bullet_char facade. The stub lists bullet at `crates/rpptx-py/python/rpptx/_rpptx.pyi:1056` through line1058 and omits bullet_char. This is a missing compatibility surface under `.claude/plans/F-X211c-design.md:27`, not an accepted atomic refusal or a specimen conflict.

Inspected the whole original body to avoid successive one-attribute repairs. auto_number, auto_number_start, bullet_color, bullet_size and bullet_font at original lines12 through16 are already implemented and stubbed. The actual attempt reached line21 after those calls. The same bullet_font is used in the later loop. No additional missing bullet alias is required by this particular script.

Use the existing checked native character-choice provider, retaining source metadata, owner validation and current bullet semantics rather than inventing another resolver or altering the source PUA characters. Existing Paragraph.bullet character assignment preserves colour, size and font under `docs/hld/10-bindings-spec.md:906` through line911. Include an explicit public getter/setter and matching strict stub shape for the compatibility name, with invalid-value/clear/no-op behavior consistent with its selected contract.

The deciding regression must cover the complete numbered paragraph and all four original family/character pairs: Symbol F0B7, Wingdings F0A7, Arial F0A7 and Wingdings F0FF. Verify actual native colour/size/font/number-start values and all saved/reopened source characters, actual PDF/missing-or-unknown Wingdings diagnostic, rendering source immutability and meaningful invalid-value atomic refusal. These later rendering/save assertions at original lines24 through38 were not reached and remain unqualified. A single attribute-presence test is insufficient. Keep the original frozen body unchanged and preserve this failure.

### D2. Distribution cardinality refusal omits its actual required minimum

`/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-23.py:15` requests selection distribution with only two shapes and requires an actionable three/3/selection explanation. The native cardinality logic already derives three for distribution relative to selection at `crates/rpptx/src/lib.rs:37549` through line37552. It returns only too few selected shapes at line37555, omitting both the minimum and the actual reference form.

The observed actual public error is RpptxError: distribute shapes failed: too few selected shapes. `/private/tmp/S91-F-X211-frozen-corpus-r2/support.py:44` rejects that incomplete message before reaching the package-identity check at line46. This is an actionable-error reporting defect. The native refusal itself is expected. Preserve its public exception class, validation order and publication behavior while stating the actual required cardinality and selection/reference context. Do not edit the frozen accepted words or skip the refusal check.

A deciding regression should preserve the existing three-shape positioning positive, assert the precise two-shape minimum explanation and verify actual before/after bytes and held-owner validity on refusal. Keep alignment's separate minimum2 and slide-reference minimum1 intact if touching the shared native formatter. Invalid direction/reference/foreign/stale owner controls remain separate existing contracts. The final specimen's later save/reopen positioning at original lines17 through18 was not reached and receives no success credit from this diagnosis.

## Smells

None.

## Nitpicks

None.

## Actual candidate and reporting boundary

Independently authenticated attempts digest `c5f2b51397a43da0aef0144e1a37932f18bf3ab2748a0b7fb4675155a7f5a885`, summary `16270fb2b4848492226e712fe46bd488642c881e5f72fa86da6d46cb8cd2b566` and result inventory `a8e4107e5a5c1c8fbd57f1c2aaa3472c704b5fe6ab68dae2f3b60a3e579b313f` under `/private/tmp/S91-F-X211c-final-composed-frozen100-attempt-1`. All100 identities remain present. The actual94 exit-zero and six failures are execution facts, not full acceptance. The reported92 positives and full metricNULL retain their qualification boundary.

W-XML-02, W-AREA-19, P-AREA-02 and P-AREA-14 retain their original source-qualified reviewed conflicts. They remain in the denominator without success credit. Nothing here approves a whole100 result, parent native/render gates or sprint closure. Preserve the actual failed candidate and any separately qualified successor identities.

No source, test, corpus, runtime, CP environment, UI, remote state, dependency or Git state was changed. No tests or runtime probes were executed. Only this canonical review was written under the exclusive lease. Review ends and returns control for distinct remediation.
