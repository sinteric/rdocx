# F-X209, native compatibility correction, pass 19

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable compatibility-source66-capsule, receipt bb0a12aba20cfe76319c4fa992210c9745bc66f345ea2cef1ef78fc41d592da6. Five complete before/after carriers and a 7,699-byte context-inclusive diff. This pass audits only the native slide-index error and stored click-address corrections. Four installed correction carriers were independently reviewed in pass18 and are not re-audited here.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation: 0. Tests: 0. Structure: 0.

## Correction qualification

owner_part_name now delegates Slide indices to the existing require_slide_index before looking up their part name. This restores UnknownSlideIndex with its existing index and slide_count payload. Layout and Master missing-owner handling remains the prior checked error route. No new error type, owner representation, publication path or mutation is introduced.

shape_hyperlink_address_at resolves the click relationship in the actual physical owner and returns its stored target. Removing the external-hyperlink-only filter restores internal slide-jump addresses and matches the established hyperlink_address contract. The lookup does not fetch an external target, alter a relationship or infer a target from a different owner's scope. The adjusted public comment accurately describes the stored address.

The integration carrier is byte-identical across source65 and source66: SHA256 f98ce35169046dc0fb91d49ba93e095374e7129f7c1a67ca3fa30040de3a54e3, size 1,155,626, mode 0644. Therefore all six original assertion bodies, including relationship reuse/pruning, grouped picture ownership, invalid input atomicity, draw order and effective geometry, remain unchanged. The additional shape.rs delta relative to pass18 is formatting only.

## Authentication and actual evidence

All 15 capsule records and 10 external references were authenticated by exact size and SHA256. The source65 and source66 maps, HEAD and index artifacts are retained. The original integration outcome remains 309 passed, 6 failed and 28 ignored. It is not rewritten as a successful whole-suite run.

The exact successors each passed 1 test, with 0 failed and 0 ignored:

- effective_geometry_follows_the_placeholder_chain_as_rendering_does
- invalid_picture_input_does_not_mutate_the_presentation
- moving_a_shape_changes_the_draw_order_and_keeps_every_shape_id
- picture_image_returns_the_embedded_bytes_and_content_type_of_grouped_pictures
- shape_click_action_parts_survive_and_relative_jumps_resolve
- shape_click_actions_link_jump_reuse_and_prune_their_relationships

The authenticated successor receipt is 5817914dc20a8776afa6f439fc3a28dabff757dc94471504de8da972cb4fe138. All six rows exit 0 and are qualified. sourceAfterMatches is true, source map SHA256 is 47ec9e8c165d0c862728e4451e430fb08a0b9d70c39c2b3e9deac3a86137a519, and ownedLeaseSafeToRelease is true with natural group cleanup.

This is a finite regression correction review, not a repeated whole-feature audit. Current check/Clippy, fresh installed correction, actual caller-font-controlled native Office outputs, archive/policy and final scoped acceptance remain separate gates. No new credit is assigned to those gates here. No source or runtime changes were made. This pass ends at this report.
