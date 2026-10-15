# F-X220, timeline fallback, pass 1

**Reviewed**: Frozen safe-final checkpoint, specifically the public deterministic timeline manager selection and its media fallback consumer. Full working diff is50 files,7885 additions and421 deletions. This is a bounded regression review, not full-feature acceptance.
**Verdict**: 1 defect,0 smells,0 nitpicks.

## Defects

### D1, the prepared manager changes existing media placeholder paint

`crates/rpptx/src/lib.rs:1671`
`crates/rpptx/src/lib.rs:8315`
`crates/rpptx/tests/integration.rs:23636`

The public single-sample timeline now selects the prepared font manager instead of the previous fresh deterministic manager. That changes glyph/font identities for the existing no-embedded-font media fallback client. Its Audio and Video placeholder paints both become decoded hash `d0aca345b8b81fc82b9a179fdc49a2f829493cd70da156ce18039106158694e7`, instead of the two distinct existing expected paints. This violates the approved requirement to preserve existing no-caller behavior. The current failure is a genuine introduced regression, not the unrelated F-X218 publication manifest expectation.

The exact unchanged test passes on the canonical product tree authenticated byte-identical to the F-X220 claim Base. It fails on the frozen F-X220 product. The test uses the existing public timeline frame and ordinary deterministic font output, so changing its expected hashes would conceal the changed public behavior. Preserve the existing media fallback paint while retaining actual embedded-face use and caller precedence. A correction must pass the unchanged poster control together with the distinguishing F-X220 timeline font control and relevant font consumers. Do not replace the golden hashes.

## Smells

None found in this bounded pass.

## Nitpicks

None found.

## Not found

- Correctness outside D1: no additional finding in the selected public timeline routing.
- Contract outside D1: no additional finding. Native application font use and producer resave acceptance remain unqualified.
- Panics: no new finding in the inspected manager selection.
- OOXML: no new finding in this render-only selection. No package serialization change was audited here.
- Tests outside D1: no additional finding. The original315-pass/seven-failure PPT run remains failed, and the exact passing Base control is separate evidence.
- Structure: no additional finding in the existing concrete manager/assembly path.

## Evidence

Current safe-final checkpoint SHA is `5b21ab0d5f4b077616cca7489584ecfebf150b9c66e06e200c74bd77dc928590`. All3205 tracked byte hashes and HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d` were independently checked. Full patch SHA is `e44594582774dd3bfe3870fb3193a9fcc0a93bb33b49de567fe66e2c337a3494`. Original PPT failing log SHA is `babe174b81b24724c6aa1354698a85e08a11b5bdaebe694643bc03b690d3dbd8`.

Actual unchanged Base control receipt is `/private/tmp/S91-F-X220-poster-Base-1/actual-Base-control/receipt.json`, SHA `f7e17e3529c8dcec3e0131f4a868d350ea296422f1e672d2de58935bcd3299f4`. It records one exact passing test at the established8388608-byte legacy test stack, full source/HEAD/index guards and terminal reaped process group. No full-feature or native Office authority follows from that control.

Only this report is written during the review phase. Remediation follows separately.
