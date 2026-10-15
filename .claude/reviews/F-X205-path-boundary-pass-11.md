# F-X205, path boundary correction, pass 11

**Reviewed**: One existing carrier, immutable source47-to48 production boundary and deciding fixture corrections. Static incremental review only.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass10 D1 resolved in the inspected bytes.

## Identity

Capsule `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/path-boundary-source48-capsule`, manifest SHA-256 `35f549f4abc5272024239c54b3ce476fc2dd651a0919401c5e96c81443b6e00e`. All artifact hashes and sizes match. Correction diff SHA-256 `b037c4a01a1f4e213fff47925506a3e2fbae9d82d175649c3ca4317f5ce28161`. Before context.rs is `8c2d798d2247054574d61849215e2711269e07c3585a97808deab6cb663907b5`, after is `01b2a4880796064bdf9158b8806bbe9b6344a7ccb9cf71bbe451cc3ca83443b7`. Source47 and frozen source48 maps differ only in this carrier, with unchanged mode. Other source records remain exact.

## Remediation assessment

At `crates/rpptx-layout/src/context.rs:8008`, the helper now receives no text-body operand. The exact p:style is inserted as the final direct p:sp child, after the shape properties and before its closing tag. There is no malformed txBody. The actual red fillRef now belongs to the shape and can supply phClr substitution. Original red centre, blue boundary, empty diagnostics and complete unchanged-slide assertions remain. This resolves pass10 D1 without weakening the deciding result.

At `crates/rpptx-layout/src/context.rs:1969`, an explicitly non-centred path refuses before concrete colour collection. The admitted predicate remains all four fillToRect operands present and equal to 50,000, matching the existing supported centred path resolver. Existing rotation, flip and tile-rectangle checks retain their order before this new check. Supported paths still reach qualified colour resolution and the source47 producing-reference substitution seam. No colour fallback or unrelated support flag is introduced.

The test at `crates/rpptx-layout/src/context.rs:7893` makes unsupported path inputs contain unresolved phClr without a producing reference. It retains no-fill, exact unsupported reason and diagnostic-content assertions. It now detects the prior late-boundary bug because attempting colour resolution first would fail instead of returning the prescribed refusal. The three concrete circle/rect/shape support positives remain separately retained. Corpus operands and zero-error assertions are unchanged.

## Six aspects

- **Correctness**: Zero findings. The supported reference fixture is valid, and the unsupported geometry boundary precedes unavailable colour evaluation.
- **Contract**: Zero findings. The exact existing centred support boundary remains, with explicit refusal for non-centred operands.
- **Panics**: Zero findings. Optional source operands remain checked. String replacement affects a known source-built fixture only.
- **OOXML**: Zero findings. The style belongs to the qualified shape in schema order. Production resolves retained source without modifying it.
- **Tests**: Zero findings. The new unresolved-colour negative distinguishes boundary ordering, and the supported fixture retains source-derived endpoint and preservation predicates.
- **Structure**: Zero findings. One existing carrier, no new module, trait, dependency or public flag.

## Evidence limits

Source47's failed corpus outcomes and the invalid predecessor fixture remain historical failed evidence. The source48 full PPT library attempt is running separately and is not an actual acceptance result in this capsule. This report provides no runtime, corpus, full-suite, native paint or feature-completion claim. The reviewer did not implement this remediation and does not re-audit unchanged source or earlier provider authorities.

Zero smells. Zero nitpicks. Review ended without source or runtime mutation.
