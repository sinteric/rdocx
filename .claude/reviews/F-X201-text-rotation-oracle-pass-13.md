# F-X201, text rotation oracle attribution, pass 13

**Reviewed**: One source-qualified Calibri text watermark DOCX, its WPS Choice and VML Fallback, the corresponding immutable authoring functions, and retained Word and LibreOffice captures. No new runtime.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. The defect is an unresolved declared renderer acceptance condition, not a demonstrated missing XML rotation operand.

## Authority

`/private/tmp/S91-F-X201-native-direct-root-2/native-and-LO-receipt.json` SHA256 `8dd0fd0dafb4cb36befeed0e4128c620f84c27de2b5aa9e2cebd6f0acf20058a`. All ten listed artifacts authenticate exactly in hash and size. The untouched source DOCX SHA256 is `551016c4851dd970692ca0e8f0790fe901e9aecc701e6f0fd560ea0f3518ebfc`. Retained Word 16.113.2 build 16.113.26092012 and LibreOffice 26.2.5.2 are separate qualified oracle authorities. This review does not substitute another version or infer pixel parity.

Code citations below refer to `/private/tmp/S91-F-X201-final-authoring-snapshot-r1/source/`, bound by immutable receipt `02f128f28dbaad001f9e9b751ed607e03f76cd4f9584c8e6919d0425c5fb4716`.

## Defects

### D1, the authored rotated text watermark does not satisfy the required LibreOffice render condition

`crates/rdocx/src/document.rs:22222`, `crates/rdocx/src/document.rs:943`, `crates/rdocx/src/document.rs:951`, and `/private/tmp/rdocx-s91/.claude/plans/F-X201-design.md:254` and `:256`.

The required real-text watermark requests 315-degree rotation. The actual header member has qualified WPS `a:xfrm rot="18900000"`, equal to 315 degrees, under `wps:spPr`. Its VML Fallback independently carries `v:shape style="position:absolute;width:360pt;height:90pt;rotation:315"`. Both representations retain CONFIDENTIAL as real text, declared Calibri, 48-point size, and the authored alpha. The generated source therefore refutes a missing or zero rotation operand.

Retained `text-native-view.png` visibly paints the text diagonally. Retained `unattended-LO/text-view.png` visibly paints the same text horizontally. The actual receipt explicitly records this mismatch at `native-and-LO-receipt.json:15`. The approved contract requires native Word and LibreOffice normal reopen/render and does not authorize silently dropping requested rotation. Consequently this declared gate remains unfulfilled even though the file opens and Word paints it correctly.

The evidence establishes an incompatibility between this concrete authored representation and the pinned LibreOffice renderer. It does not prove which AlternateContent branch LibreOffice chose, that the writer is schema-invalid, or that LibreOffice cannot rotate other legitimate real-text representations. Do not change the requested angle, convert the text to pixels, or waive this gate on the basis of Word success. A subsequent remedy or explicit authorized contract disposition needs independent qualification against this retained failure.

## Smells

Zero.

## Nitpicks

Zero.

## Not found and limits

Correctness: no omitted rotation operand demonstrated. OOXML: both requested rotation operands and their owning namespaces are present. Panics and structure: zero new findings in the inspected authoring seam. Tests: the retained oracle captures provide a concrete rotation discriminator, no raster tolerance or font metric parity is claimed. Contract: D1 remains as stated.

The separate direct-image defect from native pass 10 is resolved in the actual current receipt: Word and LibreOffice both paint the direct image with rotation and translucent appearance. That does not grant text rotation acceptance. No production edits, rendering runs, environment changes, or broad source re-review occurred in this pass.
