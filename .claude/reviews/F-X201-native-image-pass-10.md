# F-X201, native image acceptance, pass 10

**Reviewed**: immutable image input09752fc0d7bf04e0326808f2ee50fe0672f2caa5833d0206f4ecb0fc0b6153a0 from correction capsule c818f4f87897a3396e388dccecfdfe242f1f37ddbd46b825126c1082e7afcbaf, its picture authoring branch and actual native output.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, authored image watermark disappears in independent Writer output

`crates/rdocx/src/document.rs:22384`

The writer encloses the ordinary anchored DrawingML picture in a single mc:Choice requiring pic, with no fallback. The exact generated input opens normally in native Word and paints the declared30degree, half-opacity image. The same immutable input converted by pinned LibreOffice26.2.5.2 completes exit0, but its actual one-page PDF omits the entire image while retaining page background, four borders, unrelated header and body. The actual raster was inspected. This fails the approved Word/LibreOffice image reopening and rendering acceptance. The missing image cannot be credited as successful watermark output. No particular vendor namespace-support rule is inferred from this observation.

The actual unattended conversion receipt, raw log and rendered PDF are in /private/tmp/S91-F-X201-native-image-root-1/unattended-LO-2. Input bytes remain unchanged, the owned process group terminated naturally and was reaped, and no recovery or reopen prompt appeared. The first sandbox launch abort remains recorded separately, without acceptance credit. Native Word acceptance is retained independently in native-image-receipt.json, SHA273be034e7919328d5920a42990b50b97a234aff7a4b336de83c02591dccd129.

## Smells

None.

## Nitpicks

None.

## Not found

The actual Word capture showed no clipping or lost header/body content. Page-edge borders and declared image rotation/transparency are visible. No full-feature, installed binding, pixel-identity or ambient font-parity claim is made. This bounded native pass changes no source. The implementing worker investigates and repairs the compatibility gap separately, preserving strict API-owner fingerprint and external-edit refusal.
