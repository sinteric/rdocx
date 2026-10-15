# F-X209, physical-owner images and hyperlinks, pass 12

**Reviewed**: Immutable common-owner source45-to49 capsule, complete native facade and Python shape carriers with their incremental diffs. Scope is the physical Slide/Layout/Master image read/replacement and click hyperlink routes. Approved F209 owner, preservation and transaction requirements apply. All ten snapshot records and ten referenced evidence records authenticated and remained exact at review end. Later remove/move/source56 and other implementation are excluded.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, native picture APIs silently select an ambiguous shape ID

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/common-owner-source49-capsule/lib.after.rs:6770`

`picture_image_at` selects the first matching picture. `replace_picture_image_at` uses the same first-match lookup at `:6829`, then mutates that selected picture. The recursive helper returns the first matching ID at `:8490`, including group and selected alternate descendants. Neither public native operation first verifies that the ID identifies exactly one shape in the physical owner.

An imported master or layout containing two pictures with the same nonvisual ID and different image relationships therefore reads or replaces one arbitrary member instead of reporting ambiguity. Even a picture and a different shape sharing an ID makes the ID operand ambiguous. Ordinary imports retain these IDs. `owner_data` only verifies the physical owner at `:3324`, and `commit_owned_parts` serializes and reopens at `:3845` without an ID-uniqueness refusal. Reopen is not proof of unique identity. The worker confirmed there is no upstream native uniqueness guard.

The hyperlink route already supplies the exact-one precondition at `:7230`. Python picture calls do use `checked_shape_id` at `common-owner-source49-capsule/shape.after.rs:405`, but that protects Python calls only and cannot satisfy the new native public API contract. Add the existing per-owner exact-one check before native picture lookup or staging, plus duplicate master/layout deciding controls requiring named errors and unchanged complete package bytes. Preserve readable ordinary imports and existing successful ownership routes.

## Smells

None.

## Nitpicks

None.

## Other reviewed routes

Image resolution uses the actual owner's part and IMAGE relationship, not an assumed slide scope at `lib.after.rs:6773`. Replacement stages a clone, rejects non-image bytes and a second image relationship, isolates a shared relationship before changing one picture and preserves media shared across owner relationships at `:6816` through `:6939`. The unchanged position/crop model remains the replacement operand. Cleanup is limited to the old media candidate, followed by media inventory refresh and publication only after serialization/reopen.

Hyperlink lookup requires a unique physical shape ID, resolves qualified PresentationML cNvPr, DrawingML hlinkClick and actual relationship-namespace id, and interprets external HYPERLINK relationships in the owning part at `:7076`, `:7230` and `:9389`. Candidate source, relationships and abandoned target cleanup are staged before the live publication at `:7341` through `:7363`. Existing unrelated relationships remain if they were not removed source references. Invalid address control characters refuse before staging. Qualified existing hyperlink attributes/children use the existing source rewrite provider, with no new detached owner model.

Python image and hyperlink routes validate the live ContentPath, resolve its current Slide/Layout/Master discriminator and use checked shape IDs at `shape.after.rs:394`, `:1281` and `:1302`. The actual native operation publishes only after success. No new Python owner generation or public detached identity mechanism is introduced in this slice. Installed stale/foreign/opaque cases remain broader acceptance obligations rather than being inferred from the two positive native rows.

## Not found

Correctness and contract have the single D1 above. Panics: 0 additional findings in this bounded delta. OOXML: 0 additional findings. Tests: 0 additional findings. Structure: 0 findings. This is a bounded source audit, not whole F209 acceptance or proof of every imported opaque case.

## Evidence and runtime limits

- Capsule receipt: `e64d73709905b9eb307a693b89a0e2340052d1531908aa4b420ba321bc8c0c62`.
- Native after: `72f4ca21403d4a944ddd2d321d9b80dc6625d0fe7611ccfc24d1dab25113703d`.
- Python after: `d0dbe473e0ea7198a0e5dddc241e1c19995913b4ba0cf4ef9532fdb550ab216b`.
- Source49 map: `05b687a9174f1fd909a0ac5538b5dc9b9b91c6004c1eb84eda2e1831fb687ce5`.
- Source49 actual GREEN6 receipt: `00291aa4dc5aa6a43f42eeeb71132d63b107d5f78d2cb6096564a1d3e0e1c21e`.
- Original insertion selector log: `a3f87b7bb8506dce484800c069c7eb73559417dc98e78592f6d19b0d23678526`.
- Additive picture/hyperlink selector log: `7e8620e2c99689d2ef90fc1d2ca2f18374bc52f356a653df055ca00685a60bb8`.

GREEN6 ran the original insertion/dedup selector and the separate replacement/hyperlink selector. Each recorded 1 passed, 0 failed and 0 ignored. Both rows exited 0 with source-after match, naturally ended owned groups, reaped leaders, ESRCH and no signals. GREEN4 original insertion failure and GREEN5 insertion failure with separate replacement pass remain retained. The original insertion control was restored rather than relabelled as replacement coverage. The additive fixture exercises master/layout replacement, physical hyperlink read/repeat/removal/reopen, unchanged geometry, invalid inputs and unrelated slide bytes. It does not test D1 duplicate imported identities, physical paint or installed bindings.

No Cargo, CP, native UI, source edits or runtime execution were performed by the reviewer. Whole-feature microscope, installed APIs and native paint acceptance remain due. Review ends at this report.
