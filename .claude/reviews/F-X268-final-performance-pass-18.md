# F-X268, final performance, pass 18

**Reviewed**: Frozen working diff against `1b71a322e0f27771098dffcf7aa1c836ddd43e47` on `sprint/s95.4`. Five existing files, 128 insertions and 4 deletions. Reviewed the normalized owned-output shortcut, two test groups and plan/HLD 03, 04 and 12 updates under the canonical workflow and microscope contract.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Current GREEN verification and final acceptance remain pending.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Qualification and provenance

The shortcut at `crates/rdocx/src/document.rs:24175` runs after the existing resource preflight. It requires enabled canonical block proofs, an existing serialized writer record, an unchanged opening main-part identity and a positive normalized serialization proof from that opening. `kept_main_part` checks the actual current package main bytes against the opening. A changed package main cannot supply this opening evidence.

The copied bytes are `Parsed.serialization_proof`, not the package's potentially unnormalized producer XML. Their reuse additionally requires `writer_bytes_match` to establish the complete current writer frame, explicit root declaration, current child count, current private-fact eligibility, field-token exclusion, staged values and exact expected bytes. That check is repeated on every call. An older opening proof cannot survive a changed child merely because the source package remains unchanged.

The existing helper recomputes current `body_frame_xml`, whose writer runs resource admission and current root binding setup. Its narrow explicit `w14` condition retains the actual serializer for conditional declaration cases. A failed or false qualification falls through to the exact previous `MainPartBlocks::serialize` path. Resource errors still occur before cache selection. Cache-off operation cannot select the shortcut.

The owned result is a fresh byte copy. The returned record is cloned from the qualified writer record and has `part` cleared at `crates/rdocx/src/document.rs:24187`. It therefore does not claim the copied normalized bytes were a retained source opening. Child and frame facts refer to the exact same qualified pieces and frame, while the independently checked source/read authority remains separate. No new field, retained payload or public API is introduced, and the shortcut does not replace the stored writer record with a new history entry.

`kept_main_part` does not acquire the writer mutex, so using it while the existing scan lock is held introduces no recursive lock acquisition. The lock is explicitly released before returning the owned result.

No additional findings in correctness, error ordering, panics, OOXML preservation, contract scope or structure. The helper qualification and private-field fallbacks remain as assessed in the earlier writer review.

## Discriminating tests

The new positive test at `crates/rdocx/src/document.rs:50240` establishes that the producer main differs from normalized writer output, then compares the complete result with the actual whole writer. It checks absent retained-part provenance, zero assembled-output bytes and an exact copied-byte count. The real force-off route must assemble and must not record normalized copying. These assertions distinguish output reuse from returning raw source bytes or merely hiding an assembly counter.

The second group at `crates/rdocx/src/document.rs:50286` retains actual serialization for an undeclared frame, fields and bookmarks. It also changes a child after acquiring the old output and requires the new whole-writer result rather than the stale opening bytes. A new document without an opening proof must take the old serializer. Existing shared-helper tests retain tab-stop, changed frame and conditional-declaration coverage. No test assertion was weakened in this diff.

The plan and HLD distinguish lookup read/candidate units from assembled and copied output units. The new hook measures the actual normalized output copy. It does not establish zero copying, constant-time output, every allocation or a memory result.

## Evidence limits and identity

`/private/tmp/s954-normalized-output-red.log` records one actual passing test and one failure in 0.09 seconds: the previous unchanged output path assembled 39,589 bytes instead of zero. This is historical RED evidence. The current GREEN batch has not run or been consumed by this reviewer. No count, clippy, hash, memory or timing acceptance is inferred for this source.

Prior measurements belong to their authenticated source and artifacts. Installed release timings, memory, native oracles and final integrated gates remain required. This review does not complete F-X268 or authorize release.

Exact reviewed `crates/rdocx/src/document.rs` SHA-256: `5b1835ce91ccdac042707e9bb91eebd375a938c88fc532084482cf83f12c53f5`.

The reviewer changed no source and ran no builds or tests.
