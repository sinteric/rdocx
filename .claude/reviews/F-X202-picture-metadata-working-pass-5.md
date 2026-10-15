# F-X202, picture metadata, working pass 5

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable picture-metadata-capsule-r5, four complete carrier copies. Only native PictureOptions metadata admission, docPr title/decorative parsing and writing, and EPUB drawing projection were reviewed. The context-inclusive metadata.diff is 47,502 bytes. Unrelated topology and paired replacement hunks are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, decorative attributes are compared before XML entity decoding

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-metadata-capsule-r5/after/crates/rdocx-oxml/src/drawing.rs:165`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/picture-metadata-capsule-r5/after/crates/rdocx-oxml/src/drawing.rs:173`

The qualified extension reader compares raw Attribute.value bytes both for the extension URI and for decorative/@val. A valid marker with val="&#49;" has decoded value 1 but is reported false. Similarly, an extension URI whose opening brace is written as &#123; is not recognized. These spellings have the same XML attribute values as the supported plain forms. The surrounding docPr metadata parser already uses decoded_and_normalized_value, while this new branch does not. Decode these two scalar values before comparison and retain their original physical XML. A deciding imported numeric-reference control is needed alongside the plain authored round trip.

## Smells

0.

## Nitpicks

0.

## Aspect results

- Correctness: D1.
- Contract: D1 affects the requested decorative-state snapshot.
- Panics: 0 findings in this bounded slice.
- OOXML: D1. No additional namespace or child-order findings. The docPr parent is admitted by canonical_wp_element and extension descendants use expanded namespace names.
- Tests: 0 additional findings. The current control proves plain authored metadata and invalid XML text atomicity, not entity-spelled imported markers.
- Structure: 0 findings.

## Authentication and evidence limits

Receipt SHA256: 4078802e5b4031ab0496343b9639d1b2f9b454a4a40541f45a3ca1ea6e63be6d. Manifest SHA256: 5e160ed740c53d3eeb94e4647dbf05e68374268161229c28672685ddb3bec6f5. All 17 manifest artifacts were authenticated by exact size and SHA256. The four after carriers are bound to source-native-batch-20.

The retained batch20 check exited 0. The exact fx202_picture_options_title_decorative_and_invalid_metadata_are_atomic selector passed 1 test with 0 ignored. Its receipt records sourceAfterMatches and safe natural owned-group cleanup. The same batch includes a separate paired test failure, which is preserved and does not become metadata acceptance credit.

The setter validates metadata before staging and publishes a staged checked story edit. EPUB projection carries title and decorative state into its inline drawing projection. These source observations do not grant installed bindings, imported alias/foreign metadata controls, anchored-picture runtime coverage, native Office paint, scoped Clippy, inventory APIs, or whole F-X202 acceptance. No source or runtime changes were made by this reviewer. This pass ends at this report.
