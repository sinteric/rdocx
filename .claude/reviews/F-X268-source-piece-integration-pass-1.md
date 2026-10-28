# F-X268, source piece integration, pass 1

**Reviewed**: Working tree on `work/f-x268-codex`, parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding four-file diff contains 6542 added and 213 removed lines. This bounded pass reviews the Root-authored current serialization memo, canonical source reconstruction, retained-source parse comparisons and masked serialization reuse. The inspected `crates/rdocx/src/document.rs` SHA-256 is `963f7837b57a1c834d95e0d49496505626468e118b087707ac628801a95b6529`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement the inspected changes or modify source during this pass.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this scope.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML preservation, tests and structure produced no finding in the inspected integration.

The current serialization memo is separate from the address-scan records and the authoritative retained part. `serialized_main_part` stores the record before `canonical_source` attaches assembled bytes to its returned copy. A staging clone shares the bounded memo, but serialization checks the actual current model, excludes private field, tab and bookmark facts from value reuse, and refreshes proof locks on a frame change. The memo cannot supply an unread owner inventory. `main_story_layout` preserves the memo while rebuilding its separate address frame. Inspected paths are `crates/rdocx/src/document.rs:8015`, `crates/rdocx/src/document.rs:22992`, and `crates/rdocx/src/document.rs:24501`.

Source reconstruction starts from actual parsed child proofs rather than a staged model clone. Prefix and suffix matching preserve physical occurrence order. The prefix is capped at both old and new body ends. Checked arithmetic and checked slicing reject an unavailable partition before the model splice. The changed region must contain complete top-level elements. Each replacement child must parse and write back to its exact bytes under the current frame. The completed model receives the resource check and the resulting record receives the full byte-partition check. An unavailable proof returns to the whole parser. Publication remains after those decisions. Inspected paths are `crates/rdocx/src/document.rs:23049`, `crates/rdocx/src/document.rs:9271`, and `crates/rdocx/src/document.rs:13317`.

The retained parse comparisons use `kept_main_part`, whose gate binds the current part name and exact retained bytes. A dirty live model does not invalidate a parse of unchanged source bytes. Equality still decides the same unchanged-model question as the prior fresh parse. Public package-byte mutation makes that opening unavailable. This does not promote typed equality into a child serialization or parse proof. Inspected paths are `crates/rdocx/src/document.rs:20760`, `crates/rdocx/src/document.rs:23576`, and `crates/rdocx/src/document.rs:23734`.

Relationship masking uses the pieces belonging to the just-produced current serialization. The masked document still runs the normal writer with changed children and frame checks. The temporary masked record does not replace the retained part or persist another model history. Force-off prevents memo reuse and updates, source reconstruction and masked-neighbor reuse. Inspected paths are `crates/rdocx/src/document.rs:20975`, `crates/rdocx/src/document.rs:23163`, and `crates/rdocx/src/document.rs:6707`.

## Evidence examined

The new inline tests compare actual source bytes, normalized bytes, parsed state, whole-reference errors and unchanged state on refusal. The splice cases include head, middle and tail insertion, deletion of duplicate physical positions, an empty body, a one-to-two replacement, a changed root, undefined entities and malformed nesting. The memo case interleaves the original and a staging clone and verifies that package bytes remain unpublished. The retained parse case changes package bytes to exercise invalidation. The masked-source case changes a neighboring paragraph across two sizes and compares the whole-reference relationship result.

`/private/tmp/fx268-codex-masked-source-red.log` contains the actual pre-repair native failure, with 52 versus 202 serialized children. `/private/tmp/fx268-codex-masked-source-green.log` contains 5 passed tests in 1.16 seconds after repair. These logs were consumed without rerunning builds or tests. The assertions and inputs were inspected rather than treating the aggregate count as acceptance.

The original nine Python work checks and alternate-root control are still running against artifact `79ba306a0bf0f58689014283a273dcd1a7e3e913cafe8abdd445c80b506a9a86`. No result for that complete batch is supplied by this review. The earlier artifact's bullet-list check passed and Run.text still failed its child-write bound before the masked-source repair. Those earlier observations remain distinct.

## Scope and pending work

This is a bounded microscope, not mechanism 7 completion, a whole-feature microscope or sprint acceptance. Producer-retained source reconstruction and the remaining public work bounds are unfinished. Mechanism 8, A3 and A5 composition, release equality and error-order execution, HLD completion, the 79 timing gates, five linear gates, complete catalogue, memory, native Word, hash and full integrated verification remain pending. Existing parsed-child and canonical-frame reviews were read for unchanged helper context. Their findings and observations are not relabelled as this review's execution.
