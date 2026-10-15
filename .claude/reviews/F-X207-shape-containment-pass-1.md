# F-X207, Shape containment, pass 1

**Reviewed**: Targeted correctness and contract review of Shape refresh planning, shared measurement and publication in the frozen sixteen-carrier working diff. This is not the whole-feature microscope.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authentication

Read the approved fixed-width physical measurement and atomic publication contract and current Shape capture evidence. Independently authenticated all2943 current source records by size, mode and SHA against source81b9a8694e8c774a76747b5ff3fcf7bd2c35ecf4cd7ee40ae4fc7818a2bbb3ee. Working diff SHA f40172a6f0927328c90ab127f70bf7e96f0af06263a0f0a8e4e1d984be0db7fc and worker HEAD8f57b6ac5e6c9e2d8ac40cce1224201744a964d1 remain exact.

## Defects

### D1, Shape refresh publishes bounds while fixed-width text still overflows

`crates/rpptx/src/lib.rs:11780`

shape_autofit_bounds uses the trial's physical_height while ignoring its
available-width failure. The Shape branch at crates/rpptx/src/lib.rs:1420
plans and publishes those bounds without checking containment in the resized
context. An unwrapped20-point Arial line in a50-point-wide text box therefore
returns a successful updated result despite continuing horizontal overflow.

The actual source-built diagnostic at
`/private/tmp/S91-F-X207-shape-nowrap-overflow-diagnostic-r4/attempt/genuine-nowrap-width-overflow.log`
reports overflow=true before refresh, successful height change457200 to396240EMU
with width635000EMU unchanged, then overflow=true after refresh. The exact
input, saved output and harness are retained beside the actual receipt. Both
compile and execution exit0, their logs authenticate, source guards remain
exact and both owned groups are independently absent and reaped. Diagnostic
exit0 means the observation succeeded, not that product acceptance passed.

The selected contract keeps usable width fixed and requires real containment.
Refuse the entire staged refresh when width cannot fit. Recheck the proposed
resized bounds with the actual shared provider and every owner context before
publication. Add meaningful unwrapped width-overflow and late-owner atomic
controls, retaining successful growth, shrink and anchored cases. Do not
silently grow width or reinterpret this as an approved native-policy change.

## Smells

None in the targeted scope.

## Nitpicks

None.

## Not found and handback

No additional finding in the inspected anchor arithmetic, short-circuit
planning or selected-face measurement paths. The separate native height
allocation difference remains qualified evidence, not an invented fixed
multiplier. Whole-feature structural, OOXML, Python and acceptance review is
still required after remediation.

Only this report was written during review. No worker source, fixture,
environment, target, baseline, index or ledger changed. Review ends and
returns control for a distinct remediation phase.
