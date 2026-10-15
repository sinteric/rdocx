# F-X205, native assertion corrections, pass 9

**Reviewed**: One existing test carrier, frozen source43-to45 assertion corrections plus the external border-only successor. No production change and no new runtime execution.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the bounded proposed assertion corrections. Current full layout acceptance remains failed.

## Immutable identity

Input capsule: `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/native-fixtures-source45-capsule`. Final manifest SHA-256 `882055d1f05458c26d091fbd501ca289440354aa6f47c10f22ce4144d498afac`. Every artifact hash matches. The predecessor equals independently reviewed source38 context.rs, SHA-256 `a08a89aee45b1e1271b9497ac5c5bd555ecba6eeb7532b0c3bb643ff0238a5b6`. Source45 candidate SHA-256 is `212e7fb6bde2622765af37558fa9fa7e59561ac7870004498806ea0037bc1491`. The unadopted proposed border successor is `1b9dc2338d508b8633626004ad5e41f9f20e1dbb02ea1ee295f86de83942a332`, border-only diff `8b01c163eea3c4be257c51d45f2e5f47e37fa18f07c66df1146636229065becb`.

All three files have identical complete production prefixes before cfg(test). The review concerns the assertion changes, not a new independent approval of historical inherited source. No product bytes were changed by this pass.

## Assertion disposition

The three gradient expectations at `crates/rpptx-layout/src/context.rs:5565` and the two existing placed/background controls near the end of the same carrier now expect 259 resolved stops. This follows the independently reviewed resolver's 257 sampled positions plus two exact source endpoints. The first and last source colours remain asserted. The inherited-master case retains an empty-diagnostic requirement. The placed rotation control retains its supported unrotated branch and unsupported rotated branch. The background control retains member count, diagnostics and complete source equality. None changes source input colours or bypasses unresolved colour errors.

At `crates/rpptx-layout/src/context.rs:5820`, a formerly unsupported pattern background must now resolve to a 64 by 64 Grid, an 8-point tile and both declared red and white colours, with no unsupported diagnostic. The independent group-fill refusal in that same test remains unchanged.

At proposed `crates/rpptx-layout/src/context.rs:7623`, the direct table cell must retain concrete pattern paint and both declared colours. The first cell's style/corner precedence assertion remains exact. The external successor at proposed `crates/rpptx-layout/src/context.rs:7635` additionally requires an actual visible left border, a Grid stroke, 64 by 64 dimensions and both colours. This strengthens the original unsupported-stroke expectation to the approved newly supported paint behavior. The removed diagnostic must be absent at proposed `crates/rpptx-layout/src/context.rs:7642`. It does not substitute presence alone for concrete paint.

At `crates/rpptx-layout/src/context.rs:7853`, the unsupported circle, rectangle and shape path operands still omit a centred fill rectangle. Their labels now match the precise resolver boundary rather than claiming that all such path gradients are unsupported. The original no-fill, exact unsupported-reason and diagnostic-content assertions remain at `crates/rpptx-layout/src/context.rs:7877`. Gradient flips and tile-rectangle unsupported cases remain unchanged. Centred positives are separately covered by the previously reviewed deciding paint control.

## Nine failures and current limits

The native2 receipt SHA-256 is `b0f47ff9bf06bd867c918fd3a46a9a6d1eccf01577eb479e1c72be150c27b03f`. Its raw layout output records 146 passes and nine failures. Three corpus controls genuinely stopped at missing required corpus paths. Six assertion failures correspond to the changed supported gradient/pattern behavior and precise path boundary. The original failed receipt remains failed.

Native3 receipt SHA-256 is `8e93531ddf9fde87089be7e2c76644c15c47ffc2bf09a2499dfb5da9c583711c`. With the corpus supplied and source45 assertions applied, raw layout output records 152 passes and three failures. The remaining border assertion is the exact target of the unadopted external successor. The other two failures are genuine unresolved phClr errors, not missing corpus or obsolete lexical expectations. Their deciding zero-error assertions remain unchanged at `crates/rpptx-layout/src/context.rs:6166` and `crates/rpptx-layout/src/context.rs:8281`. Actual affected operands include GeomLec1 slide2/slide3 and themes.pptx slide3. Their source cause and repair require a distinct production attribution phase. This report does not declare them passed or soften their expected result.

## Six aspects

- **Correctness**: Zero findings in the proposed test corrections. The uncorrected corpus phClr failures remain explicit separate blockers.
- **Contract**: Zero findings. The new assertions require supported visible paint while preserving unsupported operands and their deciding refusals.
- **Panics**: Zero additional findings. Test-only fixed stop-count and fixture assumptions do not add a production panic path.
- **OOXML**: Zero findings. Source operands and qualified ownership assertions are unchanged. No serializer or parser change is proposed.
- **Tests**: Zero findings. Endpoint colours, rotation boundary, group-fill refusal, source equality, corner precedence and concrete Grid colour predicates survive. The border successor is unrun and receives no actual PASS claim.
- **Structure**: Zero findings. One existing inline test carrier only, with no new module, dependency or abstraction.

Zero smells. Zero nitpicks. Review ended. No full layout, native paint parity or whole-feature acceptance is granted.
