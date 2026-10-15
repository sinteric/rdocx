# F-X211c, affine native and LibreOffice disposition, pass 1

**Reviewed**: Actual original P-AREA-15/P-AREA-19 input-bound LO paths, P15 returned source geometry, native unit/scaled controls and current affine policy. Read-only diagnosis, not final clause admission.
**Verdict**: 0 demonstrated product defects, 0 smells, 0 nitpicks. The strict ad-hoc PDF equality comparison remains failed, with residual exporter differences retained explicitly.

## Defects

No demonstrated product defect in the inspected affine order.

## Smells

None.

## Nitpicks

None.

## Contract and disposition

The governing plan requires a real combined-transform oracle and a rendering
semantics decision at `.claude/plans/F-X206-design.md:86`. Its Issue323
acceptance at line442 requires actual connector/rectangle positions compared
with pinned LO and authenticated PowerPoint, including unequal extents,
rotation/flips and resolvable siblings. It does not specify two EMU as LO PDF
export precision. No new numerical tolerance or source acceptance amendment
is introduced by this review.

The actual quantitative LO comparisons are meaningful evidence for transform
order. They retain independently attributed colored vector paths rather than
nonblank screenshots. Correct-order source vertices and wrong-order inverse
predictions are both recorded. The actual wrong-order errors are88 to124
points at external `lo-actual-oracle-source-differential-facts-3.json:555` and
line829. Actual correct-order differences are separately quantified in the
same case records. The native coefficient/order proof remains independent:
current group_affine applies child map/scale, then center flip, then rotation,
with Transform.then computing next times self. The actual four function bodies
match the sealed bridge at worker `crates/rpptx-layout/src/context.rs:4163`
and `crates/oxml-layout/src/transform.rs:31`.

This establishes the observed flip-before-rotate discrimination. It does not
establish byte-exact LO geometry equality, a generalized numerical error bound,
or complete final native/render clause acceptance.

## Actual discrepancies retained

P15's returned LO PPTX source replaces the original rotated/flipped group
with an identity group and child R210 times FlipV at external
`lo-actual-oracle-source-differential-facts-3.json:2`. That linear map is
algebraically the original R30 times FlipH. The returned offsets/extents are
multiples of360 EMU and both child extents shrink by360 EMU. These are actual
oracle source changes, not changes to the original library artifact.

The returned model explains only part of the PDF difference. The blue child's
PDF-versus-returned-model residual is0.059162059597097735 points at line175.
Its exact attribution remains unresolved. Decimal path serialization alone
must not be claimed to explain that full residual. The strict two-EMU
comparison remains false for both cases at lines554 and828. It is not silently
converted to a pass and no replacement epsilon is selected.

Original P19 library PDF vertices independently match their own saved source
calculation with maximum coordinate error0.000012400000002799061 points in
`repaired-affine-original-observer-vector-facts.json:4`. That supports the
library's source transform on this artifact. It is not a transplanted native
control coordinate or a new claim about every P15 output. The exporter/source
and PDF/model differences above do not demonstrate a library rotation-order
fault. They remain distinct observed oracle differences.

## Authentication

Facts SHA543962ff0498e3c25f2e1f32ca1a4bb03596641f3df1edf386331bf864aa27e1
and native policy bridge2fa2120100d01ca7e27a46ac834f38cf1ea632647e14f9e062d078f4f5e00cc0
independently authenticate. Original attempts and both original saved input
hashes match the LO configuration. Both actual LO PDFs, P15 returned PPTX
and its slide XML match their pinned hashes. Actual conversion receipt
a009cf749df3f86713c2f9b78f0e174b4595aed2da34bef46d8b0864d2ef3527
and its raw logs authenticate successful conversion and safe cleanup, not
strict geometry equality.

Under `/private/tmp/S91-F-X206-remaining-native-preparation-r1/native-controls/`,
scaled native capture2cc55faa, geometry83ac4de8, original input2bdeec3f and
native saved outpute9ed05f8 authenticate. Every capture-listed original,
before/reopened package and both native PDF bytes/sizes was independently
rehashed. Their unit/scaled coefficients and native source preservation remain
input-specific provider facts, distinct from literal corpus coordinates.
The original retained P19 observer PDF likewise matches its recorded hash.

## Remaining acceptance boundary

The final f3 actual output byte/provider bridge remains required. The relevant
resolvable-sibling/owner-specific unsupported-member, source/ID/z-order and
save/reopen facts remain separate obligations under the approved plan. This
review does not supply those facts or a score. Root's eventual explicit
admission must preserve the failed strict comparison and unresolved residual
while admitting only the actual source/native order and output facts proved.
No historical artifact is relabeled as a newly generated f3 output.

Only this review file was written. No conversion, rendering, native tool,
Office UI, source/test edit, CP access, ledger/metadata or Git mutation occurred.
The review ends and releases the canonical review-write lease.
