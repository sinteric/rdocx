# F-X205, path reference correction, pass 10

**Reviewed**: Two immutable existing-carrier corrections in `path-reference-source46-capsule`. Production guard diff plus new source-qualified path control. This is a bounded review, not full-feature acceptance.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Input identity

Capsule `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/path-reference-source46-capsule`, manifest SHA-256 `8350862405af9626ae1e4228a4a3e9cb8f9028983904f6e364e6b942b6852c89`. All manifest artifact hashes and sizes match. Production diff SHA-256 `45fe1be0ec5c0bfe8b6ea7b8a90f9e4089853472f8adfd63cc22f00a7eb0e117`. Control diff SHA-256 `81f6478930e56fd437eb0a855d7428d413cd9544043c35a3f5a189fd40f90d8a`. Candidate style.rs SHA-256 `420edc52563b654d5efdc29226e7e58a3dbe7f5647cdc44f8e5f7bdadc86a5b3`, candidate context.rs `0d1eeb131c4f036e473c658481a59ca1a9191841627e7437065f81262873119f`.

The reviewer did not author these corrections. Current source47 HLD/border additions and live runtime outcomes are outside this input. No test execution or source modification occurred in this review.

## Defects

### D1, the new deciding fixture does not supply the shape fill reference

`crates/rpptx-layout/src/context.rs:7981` passes the complete p:style element as shape_with_details' fourth operand. That operand is body_properties at `crates/rpptx-layout/src/context.rs:8939`, and the helper inserts it inside p:txBody at `crates/rpptx-layout/src/context.rs:8948`.

Consequently the red fillRef is not a direct shape style child. The fixture also constructs a text body without its required bodyPr. It cannot decide whether the corrected supported path substitutes the real red producing reference into phClr. Move the exact style into the qualified shape's schema-valid style position, preserving the red/blue endpoint, empty-diagnostic and source-equality assertions at `crates/rpptx-layout/src/context.rs:7990`. Retain the failed attempt separately. No GREEN qualification is warranted for this control as frozen.

## Production assessment

The minimal guard at `crates/rpptx-layout/src/style.rs:314` retains the previous flip and tile-rectangle deferral unchanged. It now defers a path only when its fill rectangle is not explicitly centred with all four source operands equal to 50,000. This is the same admitted centred predicate used by concrete paint at `crates/rpptx-layout/src/context.rs:2029`. Admitted paths therefore reach the existing qualified colour substitution at `crates/rpptx-layout/src/style.rs:326` before concrete colour resolution.

No arbitrary fallback colour is introduced. Missing or recursive phClr references still raise the original error at `crates/rpptx-layout/src/style.rs:358`. Existing transform order is retained at `crates/rpptx-layout/src/style.rs:364`. Unsupported flip/tile/non-centred source operands remain deferred to their existing refusal path. No corpus assertion or expected error count is altered by these two diffs. The old corpus phClr failures remain actual failures until a qualified successor executes.

## Six aspects

- **Correctness**: Zero additional production findings in the minimal centred-path substitution guard. D1 is the deciding fixture defect.
- **Contract**: Zero additional findings. Existing support/refusal predicates remain bounded and no guessed colour is supplied.
- **Panics**: Zero additional findings. Optional rectangle operands are checked and no new untrusted indexing is introduced.
- **OOXML**: D1 above, incorrectly placed style and incomplete text body. Production source remains cloned and unchanged through resolution.
- **Tests**: D1 above. The intended assertions are discriminating, but their source precondition is invalid.
- **Structure**: Zero findings. Two existing carriers, no new module, trait, dependency or public policy flag.

## Limits

There is no actual control acceptance receipt in this frozen capsule. Worker runtime is separate and may reveal this pre-deciding fixture failure. This pass does not grant corpus closure, whole layout success, native paint parity or completion. The production seam and its source-preservation intent receive only bounded static assessment.

Zero smells. Zero nitpicks. Return for distinct fixture remediation.
