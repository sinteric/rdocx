# F-X209, physical owner working slice, pass 1

**Reviewer**: fonts_resume, independent of this implementation.
**Reviewed**: one existing native carrier, `crates/rpptx/src/lib.rs`, 605 additions and 264 deletions against claim `fcbab9c3c1210c071d31471cb1913cc09ee70e7e`. This is a bounded owner/provider review, not the whole F-X209 feature.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input

The immutable input is `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/owner-production7-capsule/receipt.json`, SHA-256 `deee58d870843d1bdccd0188b4899e99f8eeaa219138ebfd53ac8a91ff76c5fb`. Its complete copied `lib.rs` is `a07a0228a147af72e6a4c99eca6c6e47440c0eacf2f4c429f357d42c0d72813c`. The source-production-7 map is `88a97365ef479fcdeb06d1a22731da06b6ce5113f4f485f86e162090e267e46f`, with 3131 records and the exact copied native carrier. The retained before carrier is `d2fa4b3f3826040a4db62f19d316bf70e7c49540e5fe4e7ad44a2decfdeb59f9` and equals the claim Git blob byte for byte.

Read the approved F-X209 plan, repository workflow and microscope rules. The reviewed contract is physical slide/layout/master ownership, part-local relationship and ID allocation, dirty-part persistence, staged geometry/connector/group operations, and unchanged ordinary-open source preservation. Later worker source is outside this immutable input.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspects checked

- **Correctness, zero findings.** `ShapeOwner` selects the actual part record. Owner access checks indices before indexing. Master and layout changes reach `staged_package` and `current_part_xml`. Picture insertion allocates against the selected physical tree and records its relationship on that owner's part. Shared media does not change relationship ownership. See copied `lib.rs:3303`, `lib.rs:2392`, `lib.rs:2789` and `lib.rs:5085`.
- **Contract, zero findings within this slice.** The existing shape facade is generalized to the physical owner, including descendant paths. Geometry and grouping use the existing concrete provider and connector refresh rather than detached shape models or new generation registries. Unprojectable retained masters remain stored as parse results and are refused only when a typed operation needs them. See `lib.rs:785`, `lib.rs:9297`, `lib.rs:10801` and `lib.rs:24900`.
- **Panics, zero findings in the changed routes.** Checked owner/path access precedes the changed indexing and geometry materialization. Candidate edits are published only after serialization validation. Failed dimension checks and connector/group errors leave the original presentation in place. See `lib.rs:3334`, `lib.rs:3391`, `lib.rs:10212` and `lib.rs:24965`.
- **OOXML, zero findings in the changed routes.** Unchanged master/layout parts retain their package bytes. Dirty typed records are serialized through their existing schema writers. Relationships use actual owner-relative targets. The optional layout-name operation stages a package and reopens before publication. See `lib.rs:2392`, `lib.rs:3445`, `lib.rs:3457` and `lib.rs:5085`.
- **Tests, zero findings for the declared bounded control.** The owner test covers both master and layout insertion, actual relationship target existence, geometry persistence, exact negative-operation atomicity, save/reopen byte stability, unrelated part retention and media deduplication. See `lib.rs:18518`. Its deliberately minimal PNG-signature payload with explicit dimensions tests ownership and persistence, not image decoding or native logo paint.
- **Structure, zero findings.** This slice changes the existing carrier and concrete owner routes. It adds no module, dependency, detached public model, forwarding-only facade or new ownership-generation system.

## Authenticated execution and limits

The retained current receipt is `actual-native-owner-GREEN-3/receipt.json`, SHA-256 `6201cafcf94ae69c4ebc8988febab3b10185bd16ca610a8caacf4690f658048a`. Its exact command selects `write_tests::fx209_part_owned_shape_edits_persist_and_refuse_atomically` in `rpptx --lib` with all features. The authenticated log `466989cddfbcebb2a8a3b968c427e14d76af00b331a885910704fc8ad62d3d2f` reports 1 passed, 0 failed, 0 ignored. Source-after equality and natural owned-group cleanup are recorded, with ESRCH and no signals.

Predecessor receipts and log hashes also authenticate. GREEN-1 records a compile failure from the test's nonexistent `part_names` helper, not semantic execution. GREEN-2 records the real master image dangling-relationship save refusal. Their receipt status strings remain `running` despite terminal failed rows and safe cleanup, so they earn no success credit. GREEN-3 closes that deciding failure through current dirty master/layout XML projection. Source maps 6 to 7 differ only in this native carrier.

No tests or product imports were run by the reviewer. Group/connector routes received static review here, not new runtime credit. Master/layout lifecycle, theme import/editing, Python ownership, installed-wheel qualification, genuine image rendering, native application comparisons, HLD completion and whole-feature acceptance remain outside this pass. The result permits continued implementation, not F-X209 completion.
