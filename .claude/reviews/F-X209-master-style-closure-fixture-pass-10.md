# F-X209, master style expected closure, pass 10

**Reviewed**: Immutable `master-style-closure-expected-proposal`, one existing native test carrier, one hunk with seven added lines and one removed line. Production is unchanged. The receipt and all seven recorded artifacts were independently authenticated. The proposed diff was independently reconstructed from the complete before/after carriers and matched exactly.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. The bounded fixture proposal is admissible. Successor runtime remains due.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Admission and preservation

The expected value remains derived solely from the caller's independently requested properties. It uses their existing public serializer, supplies the conventional local DrawingML namespace on that standalone level, then parses the independently built list style. It does not read the actual getter or saved part to construct the expectation. See `master-style-closure-expected-proposal/lib.proposed.rs:20855` and `:20865`.

The local binding is an explicit existing setter transport behavior, not a guessed value. The immutable setter rejects a rebound local `a` and supplies the actual DrawingML binding when absent at `master-style-closure-expected-proposal/lib.before.rs:3464`. The fixed requested fixture serializes a newly changed level without an existing local declaration. Adding this closure therefore does not duplicate a namespace attribute in this operand.

The original mismatch is explained by the source-sensitive equality contract. `crates/oxml-drawing/src/text/paragraph.rs:432` retains the conventional prefix in owner closure when a local declaration exists, while `:2782` compares that closure. The original authenticated failed log shows the actual level has local `xmlns:a` and the independently serialized expected level lacks it, with the same requested 4000 size and Carlito face. The correction supplies the missing expected physical closure rather than changing equality or discarding preserved source checks.

The complete object comparison remains at `master-style-closure-expected-proposal/lib.proposed.rs:20869`. Repeat whole-package equality, saved/reopened size, opaque lexical extension, all unrelated part bytes, invalid style/level atomicity and all five body levels remain unchanged at `:20874` through `:20938`. No product, parser or serializer code changes are proposed.

## Not found

Correctness: 0 findings. Contract: 0 findings. Panics: 0 findings in this fixed test operand. OOXML: 0 findings. Tests: 0 findings. Structure: 0 findings. The small start-token insertion is bounded to the existing generated fixture and introduces no product helper or API.

## Evidence and limits

- Proposal receipt: `fe45e6fd51a46caec040ef8a9c9156947974c55e86f97ba329d2e357e8ea3231`.
- Correction diff: `9db342a5bd0b8aec4532eab291a22d5ceb0f800d9516eb0fd74aae4a550d21e0`.
- Immutable before carrier: `c9e1bbbf2c76d61d8f77c64199c4ccac150bd18e32f4f4078c516166be293634`.
- Proposed carrier: `175b8467bcb51d7b521f98610e37f3b89ab228cd9e670920e11855b53aaa6431`.
- Source54 map: `6f596580723d3dfa660e9192da59aea3610d740c6e1e5fbc33143820748fe314`.
- Original actual receipt: `3b985322bbb00fb499f2bf601ebe94506719d08c33ed08fb7d02cdff5b981bd1`.
- Original failed style log: `83f835f33d5530638581e8a1c7c8bb57899b677721b73d54478d9ae4a6d03923`.

The original exact style selector failed with 0 passed, 1 failed and 0 ignored. Its separate scoped Clippy row exited 0. Both owned groups naturally ended with ESRCH and no signals, and the original source-after guard matched. Those outcomes remain distinct. The six-line net fixture addition is unadopted and unrun, so this review awards no successor native, installed, paint, theme-import or whole-feature acceptance. No source edits or runtime execution were performed by the reviewer. All recorded artifacts remained hash-equal at the end of inspection.
