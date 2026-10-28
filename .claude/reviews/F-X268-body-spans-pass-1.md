# F-X268, body-span serializer, pass 1

**Reviewed**: Mechanism 6 serializer prerequisite against 6579e2e0dcc9625eaead146d7c324117d43c3bca, one file, 304 insertions and one deletion. Root inspected the entire diff and the existing conditional w14 declaration helper.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None in this bounded prerequisite. Both entry points use the same writer and admission order. Marks follow the body start and each modeled child, with the final body section excluded from the content range. The existing w14 helper only inserts a declaration, so its length delta correctly shifts the recorded offsets. Successful writing always records at least the body-start mark. Empty raw children retain empty ranges and introduce no extra admission rule.

## Smells

None in this bounded scope.

## Nitpicks

None.

## Evidence and limits

`/tmp/fx268-m6-check.log` records the affected all-target check, exit 0. `/tmp/fx268-m6-span.log` records three passing tests, exit 0, including the new multi-fixture span case and the existing section and foreign-background controls. Assertions cover zero, one and many children, paragraph and final body sections, table and controlled content, a text box, raw and empty slices, entities, backgrounds, a Word alias, conditional w14 offsets, and the same InvalidValue on the existing table nesting limit. Earlier fixture substring-length mistakes were corrected and are not product RED evidence.

This does not complete mechanism 6. Native address-cache consumers, counted work, invalidation, full lookup equivalence, both reopen routes and Python verification remain pending. HLD adoption waits for that complete layer. Mechanisms 7 and 8 and every final integrated rider remain pending. The hash harness has not run and no output delta is accepted here.
