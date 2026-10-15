# F-X195, incremental provider tests, pass 9

**Reviewed**: One existing PowerPoint test function, 64 additions and four
deletions. This is the bounded runtime fixture correction, not a full
provider or feature confirmation pass.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Identity and authentication

Worker work/f-x195-codex, HEAD
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. F-X195 remains in-progress.
Read canonical workflow and microscope, approved provider contract, retained
progress, prior provider pass6 and pass8, actual FontManager resolution and
FontData construction, complete current test and exact remediation diff.

Proposal /private/tmp/S91-F-X195-ppt-cache-test-proposal/receipt.json SHA256
9f187135b9f855054d1654786007d78225e7ebb13596e93618ea85af0f939075.
Applied receipt SHA256
d58d460beb5c2a89e589b025e28529b89eeecd5bf22e5cce86132dbf7d401557.
Exact test-only.diff SHA256
6c40a2b16e8f16e8e5de3ef247bbcd6cdb8c455c44c80ad273eda70212176582.
Current text.rs equals candidate SHA256
b0e44fc21ec99cb6f735b7fcd2a739598b15d240a77da7562623e576634ce949.
Independent inverse application in memory recovers the exact previous
0a9baa6746ca918912d33f8b3d062d11da00e80d5f38d9e33999b17071671ae7.
Production prefix and suffix outside this one function are byte-identical.
No source, scratch, runtime, formatting or mtime was changed by the review.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Assessment by aspect

Correctness: zero findings. FontManager keeps a loaded identity per original
request key at crates/oxml-layout/src/font.rs:1674 and creates each new ID
at :1696. font_data at :2252 returns that loaded ID with its physical face
and instance data. Distinct Aptos, Calibri and Aptos Display keys may select
identical fallback bytes while receiving different transient IDs. The revised
cross-request comparison therefore normalizes only cloned segment IDs after
requiring their membership in the actual ordered selected inventory at
crates/rpptx-render/src/text.rs:3866. It validates physical face data and
actual contributing-event membership at :3870 and :3875. It does not change
live spans, cache entries, traces, font metadata or resolver policy.

Contract: zero findings. Both request orders and all six cold/warm calls
remain. Original requested-family attribution is still asserted at
crates/rpptx-render/src/text.rs:3816. The physical comparison at :3912 retains
family, face index, full original font bytes, bold/italic flags, coordinates
and both synthesis flags through :3839. Ordered inventory comparison
prevents normalization from concealing another selected face or instance.
Distinct original request carriers remain required at :3918.

Tests: zero findings. The original unnormalized accepted span debug identity,
ordered face inventory including actual IDs and full contributions remain
in the warm comparison at crates/rpptx-render/src/text.rs:3896 and :3903.
The warm path still requires exactly three cache entries at :3898. Only the
cross-request comparison uses cloned IDs keyed by ordered inventory index.
The reconstructed multilingual span at :3879 copies every other field:
base glyph IDs, text, advances and style, logical index, language, script,
direction, bidi level, positioned advances and offsets, clusters and break
flag. The inspected MultilingualTextSegment constructor at
crates/oxml-layout/src/font.rs:173 retains those arguments and validates
position/cluster invariants. Full normalized span debug equality plus exact
physical inventory remains a geometry and instance discriminator. The test
no longer requires unrelated request keys to share transient IDs, while
same-request warm IDs remain exact.

Panics: zero findings. New panic and expect paths belong only to the
controlled nonempty text fixture. Selected inventory membership precedes
indexing at crates/rpptx-render/src/text.rs:3872. Fixed six captures precede
three-capture comparisons. No production panic path changes.

OOXML: zero findings. No namespace, child order, XML parsing, serialization,
unknown capture or document mutation changes.

Structure: zero findings. The local physical-face projection and cloned
normalization live inside one existing test. No new source file, module,
dependency, trait, generic, flag or public API is introduced.

## Runtime qualifications

Root reports R9 PowerPoint actual child101, four passing controls and one
failure, after the original warm comparisons passed and the cross-request
raw ID0 versus ID1 comparison failed. That retained failure is not a
production resolver defect or a passing result. Corrected test execution is
pending. Root separately reports both historical reversals with child101,
exact191 restoration and current one-test pass, with wrapper0. Those remain
separate qualified proofs and are not execution performed by this review.
No runtime waiver, full provider gate, entrypoint coverage or feature
completion is granted here. This ends the incremental review pass.
