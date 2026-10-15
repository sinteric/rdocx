# F-X201, installed and text correction, pass 14

**Reviewed**: Four changed existing carriers in immutable final-installed-text-correction-r2, plus qualified current Word and LibreOffice watermark artifacts. Bounded correction review, not a replay of whole pass 11.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. No new implementation defect demonstrated. D1 is an unresolved declared fidelity disposition for the pinned external importer.

## Bound authority

Capsule receipt SHA256 `6773ac19cfed84cafdf745461b102d7c478a74d21e33a691c3ef4f104c7b30a7`, source-map SHA256 `de757b40af4dacc07a027b55f214904195ca500491d6f50074a41a2a267556a8`, 3076 records. All four copied candidate hashes and sizes, and all six named evidence receipt hashes authenticated. The capsule correctly labels working.diff as selected full carrier diffs, not an incremental patch. Citations refer to `/private/tmp/S91-F-X201-final-installed-text-correction-r2/source/` unless absolute.

Current oracle receipt `/private/tmp/S91-F-X201-native-fontwork-root-3/native-and-LO-receipt.json` SHA256 `63d2d8ca502fc8bf97fb0889f0b7ad710520f103645101a9058f6bb0055441e4`. All five listed artifacts authenticated. Source DOCX `d9f077bd9e14338ebd43a366f7b05bc2b37d300898b17d6240896e7983406c7b` retains actual declared Calibri. Paint-attribution artifact SHA256 `81ab9ef1ec685c4040f0e11fadd1de981286f010ec58aa39f729b1bbac853447` identifies actual Word ca0.5 and actual LibreOffice opaque808080. No other oracle version or fresh runtime was substituted.

## Reviewed corrections

`crates/rdocx/src/document.rs:18291` rejects unrecognized image bytes and raster formats without a successful existing media probe before staging, media allocation, or owner mutation. Valid recognized vector input with two explicit positive dimensions remains admitted. Existing dimensions, owner lookup, namespace closure, staging and single publication remain intact. Regression `:399337` proves exact malformed error and unchanged save/reopen. The genuine explicit SVG control at `:400140` proves the retained vector path. This bounded change does not claim a full image decoder validates every image payload.

The selected Python control corrections use actual RdocxError or KeyError with precise messages, preserve failed-input atomicity, construct the requested localized or canonical style owner consistently, and inspect the actual qualified fldSimple instruction and cached-result nodes. They add substantive rich-footer picture, table and both note-family input. No field production rewrite or weaker failure predicate is introduced.

Regression `:400056` exercises both note families, explicit custom marks, separator, continuation separator and notice preservation, empty-mark refusal, saved reference ownership, deletion and byte-exact reopen. Actual referenced rows record one exact note control pass, not an invented multiple-test count.

Watermark authoring `crates/rdocx/src/document.rs:22254` adds standard plain text warp only to the new watermark's WPS body properties. It retains real styled text in both physical representations, 315-degree rotation, alpha and reserved identity. Generic textbox construction is unchanged. The original source rotation operands remain present. Native regression `:399718` discriminates textPlain plus actual deterministic text, alpha and affine paint. HLD08 `:2495` accurately records this bounded intent.

## Defects

### D1, the remaining LibreOffice opacity divergence needs explicit fidelity disposition

`crates/rdocx/src/document.rs:22265`, `/private/tmp/rdocx-s91/.claude/plans/F-X201-design.md:252`, `:254`, and `/private/tmp/S91-F-X201-native-fontwork-root-3/native-and-LO-receipt.json:9`.

The actual source contains qualified w14:alpha=50000 in both real-text representations. Library deterministic SVG paints alpha0.5, and actual Word PDF uses an ExtGState with ca0.5. Actual LibreOffice26.2.5.2 successfully opens and renders diagonal text, but paints opaque808080 and emits no corresponding alpha state. This establishes loss by the concrete external import/render path, not a missing authored alpha operand or a demonstrated library projection bug.

Pass13 rotation D1 is resolved by actual current output: both Word and LibreOffice now paint diagonal text with bounded matching geometry and no clipped glyphs. Normal reopen/render succeeds. However, the approved plan also requires named opacity fidelity and says a requested feature that cannot be faithfully authored/rendered must not silently fall back. A successful conversion is not half-alpha acceptance. The remaining importer divergence requires a qualified interoperability remedy or an explicit authorized contract disposition that records the limitation and preserves source, library and Word opacity assertions. This report does not silently waive that obligation, require pixel parity, or assert that LibreOffice universally lacks alpha support. Do not flatten the requested source alpha into a guessed colour or convert real text into pixels to hide the difference.

## Smells

Zero.

## Nitpicks

Zero.

## Not found and limits

Correctness: zero new implementation defects in the four-carrier correction. Contract: D1 above remains. Panics: zero new unchecked production panic paths. OOXML: selected qualified source, real-text representations and child ordering remain preserved. Tests: named current native rows are authentic, and original installed failures remain retained. Structure: no new module, file, trait, dependency, or generic framework.

Current stub __all__ and subsequent namespace-equality corrections were requested for coordination but are not sealed within this capsule. They are excluded from this verdict. No current installed GREEN, final strict typing, full suites, or feature completion is claimed. No runtime, source, index, environment or baseline changes occurred. Only this external review artifact was written.
