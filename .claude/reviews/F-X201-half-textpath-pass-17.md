# F-X201, half textpath correction, pass 17

**Reviewed**: Immutable half-textpath-correction-r2, thirteen complete existing source, test, plan and HLD copies, plus its one-carrier lint successor. This is the bounded new half-only admission, native WordArt fallback and selected-branch correction. Earlier rich owners, notes, images and entity-capture reviews retain their authority. Reviewer: fonts_resume, independent of this correction.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Bound evidence

R2 receipt SHA-256: `89b3e3eddfef48327217f72cf4e14a5d68137adc079c43e4fd468df3f0118f3b`. All thirteen copied sizes and hashes authenticated. Full source-map SHA-256: `2348b5bfdbc17e48b9997df2adca4f39bc5537185183c8ac163a3792561af173`.

The user-approved plan copied from canonical amendment `12dd18a5` has SHA-256 `2bb96123e17a86fc40c3d9a1e0eb042879e5eb2d0e9184ee977815185e4f31c3`. It restricts new DrawingML text authoring to opacity 0.5. Other values must fail atomically. Existing VML compatibility retains its prior range and F-X219 owns later full-range interoperability.

Lint successor receipt SHA-256: `8a807c85fbadb52e1caeab556034595032a49ff7e82eb3acf24b1bf3d1562c54`. Header source changes only the equivalent loop-to-while-let expression. Source-map SHA-256: `7323c97f6573b8eeca91c0fbdcdfd66bbd62dfb443bc8d5bdf3a2fd78b72cd27`. Its actual four-crate Clippy receipt `5dd99765cbbf7f9975307fb016c3907c7f8cfc08da8b6214332b0f2b4fb722c2` records exit 0 and safe natural cleanup. The preceding lint failure remains retained.

All six R2 authority hashes authenticated, including original failed assertion attempts. Actual corrected watermark receipt `5a365ccffa987f582347fa0daee2a1379cb6bcad679c2e9a22fab270abc5ada7` and its raw log qualify one exact native deciding control, not a whole-feature pass.

Current Word and unattended LibreOffice receipt SHA-256: `ac183cfb3b982dc039dbce9cc3e3233db4d1b42506d03283950329cc045fc0cf`. Every listed artifact hash authenticated. It binds actual source DOCX `d8ac9a300ea5c49770a588db913a8732783eed57726656a094240b70599831bb`, normal Word open/save/close/reopen without repair, and one diagonal real-text watermark at PDF alpha 0.5 in both applications. The retained endpoint failures are not relabelled and receive no full-range credit.

## Defects

### D1, VML and DrawingML disagree on which qualified branch was selected

`crates/rdocx-oxml/src/header_footer.rs:1050`
`crates/rdocx-oxml/src/header_footer.rs:1114`
`crates/rdocx-oxml/src/drawing.rs:368`
`crates/rdocx-oxml/src/drawing.rs:1850`
`crates/rdocx-oxml/src/text.rs:2676`
`crates/rdocx-oxml/src/header_footer.rs:1435`

The new VML projection requires a genuine MC Choice and a qualified WP anchor/inline before suppressing its fallback. Its negative fixture rebinds the Choice or WP namespace and correctly observes one VML watermark. However, CT_R independently calls `parse_alternate_content` on the same captured qualified outer MC block. That provider recognizes Choice and drawing by local name, and the DrawingML reader recognizes anchor by local name. It can therefore still publish an `alt_drawings` owner for the branch the new VML guard rejected. A real WPS watermark with the same rebinding can reach layout as both DrawingML text and VML fallback, or project foreign source as a genuine DrawingML owner.

The negative test checks only `watermarks()`, so it cannot detect simultaneous DrawingML projection. Reconcile the actual branch selection at the existing provider seam, preserving complete physical raw source. Add a complete paragraph/header or layout control proving that foreign/rebound Choice and WP owners are excluded from DrawingML projection, valid qualified aliases remain admitted, and exactly the selected representation paints once.

The unchanged provider files were authenticated against the R2 map before reading: drawing.rs SHA-256 `e160bf05bc3443e63b8eb7a024f3ed8e1d1a3a3633f4abb15d191a4a14de38c2`, text.rs SHA-256 `3dca7a0ee9e80223e80c5f7d789df7c7b8eecf40cbab8d65fbe05e006acd3056`. This finding binds those actual provider bytes, not an inferred alternate implementation.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: no additional defects. Panics: 0 findings. OOXML preservation/schema: no additional findings beyond D1. Tests: no additional findings beyond D1. Structure: 0 findings.

The early unsupported-opacity check precedes staging, header creation and identity allocation. The deciding control tests absent and existing owners for zero, quarter and full opacity with unchanged bytes. The primary WPS owner retains real styled text, page-relative behind-text geometry and qualified solid alpha. Its native type136 textpath fallback retains equivalent declared text, font size, colour, rotation and page centering. The selected canonical branch paints once. The fallback-only control traverses actual MarkedContent, groups and Text/MultilingualText to prove one paint with effective alpha 0.5. Whole-owner checksums include inactive physical source, repeated save/reopen and replacement/removal assertions remain strict, and abandoned gradient-specific implementation is removed.

## Limits

This review resolves the former full-range requirement only through the explicit user-approved half-only boundary. It does not claim the endpoint defects are fixed. The current valid authored Word/LibreOffice half case passes independently of D1's foreign/rebound projection gap. Earlier source slices and separate installed/typing evidence are not freshly re-reviewed. No code, runtime or environment was changed during this pass, and no whole-feature completion is granted.
