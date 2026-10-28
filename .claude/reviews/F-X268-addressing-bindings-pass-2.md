# F-X268, addressing bindings, pass 2

**Reviewed**: Codex root reviewed S1 remediation in the mechanism 6 binding regression. The four-file binding diff is 117 added and 22 removed lines on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. Product source and the installed artifact are unchanged from pass 1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

S1 is resolved. At both 20 and 80 paragraphs, the regression requires exactly three same-text paragraphs after clone, table movement and reopen. It requires the table at direct index 0 and the duplicate paragraphs at direct indices 1, 2 and 3.

`/tmp/fx268-m6-py-s1.log` records the isolated paired node passing in 10.68s. Both sizes retain first_measured 0 and repeat 0. This is measured reuse, not a cold construction or total constant-time claim. Functional assertions remain active without the debug helper.

The artifact SHA-256 remains `b3c7ee223d9465698570a5c5e58526727d98cc6a6933d57862c79696cfa9f165`. The earlier five existing consumer passes remain applicable because only the new Python test changed.

## Not found

No additional correctness, panic, export, signature, test or structural finding was established in the inspected binding scope. Native addressing has its own clean pass 3. HLD review, complete forms and settings composition, the current native seven-case release execution, mechanisms 7 and 8, measurements, Word qualification, hash harness and full sprint verification remain pending. This report does not complete F-X268.
