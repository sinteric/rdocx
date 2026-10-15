# F-X206, list transport equality, pass 81

**Reviewed**: External R29 receipt4c8c259193562504a97ecf802eb531690785d6277a36659c862613537fe40cae, two-carrier difffc470e551a6b7e4c04fa071b8773d23dade89e4932faf7f14a2d52ed57454ae7 against frozen source3f0782564efb5d84f21985984d94012fca1130ced7bafe8515629dd48321729f.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

Candidate `/private/tmp/S91-F-X206-aptia-list-transport-r29-prepared/candidate/crates/oxml-drawing/src/text/paragraph.rs:2718` and candidate list_style.rs:51 exclude only the exact canonical relationship declaration from equality. Paragraph attribute sorting and list-style ordering retain existing behavior. Foreign declarations, ordinary attributes, typed values and raw children remain deciding. Parsing and serialization are unchanged.

The source-built control at candidate paragraph.rs:4488 checks inherited picture-bullet ownership, save/reopen equality, repeat bytes, changed relationship ID, canonical and foreign declaration discrimination, opaque attribute and child discrimination, and typed level inequality. The captured Aptia failure attribution points to these two transport declaration operands. No fixture threshold or old coverage is removed.

This permits guarded two-carrier adoption and only the new unit plus failed corpus selector. It grants no runtime or whole feature acceptance. Historical Git writer sources are prepared authority only. Both archive SHA expectations remain unmodified and unresolved.
