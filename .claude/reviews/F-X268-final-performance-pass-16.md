# F-X268, final performance, pass 16

**Reviewed**: Frozen working diff against `72a7568ff8e5eb0925ac9b103b07a2ccd9cd6cc7` on `sprint/s95.4`. Five existing files, 244 insertions and 16 deletions. Reviewed the writer comparison implementation, tests, plan amendment and HLD 03, 04 and 12 under the canonical workflow and approved F-X268 contract.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Source review is clean. New verification remains pending.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Writer qualification

`crates/rdocx/src/document.rs:8841` recomputes the current writer frame using `body_frame_xml`. That method runs current resource admission through `write_part` before recursive work. Matching the complete head and tail covers current root attributes, namespaces, background data and section metadata. Child cardinality and every current child are checked before any boolean mismatch is returned. Private-fact admission is evaluated on the current item, field-bearing bytes decline, and the staged value must match exactly. A previously admissible record does not cache those current eligibility decisions.

The explicit `xmlns:w14` requirement is intentionally narrower than inferred prefix use. `CT_Document::root_start` emits those exact extra-namespace and root-attribute fields. The whole writer's declaration pass returns when it encounters that literal root declaration. Thus an admitted record with the exact current frame has no conditional declaration step depending on concatenated body bytes. Ordinary undeclared frames and local declarations retain the actual writer, avoiding assumptions about opaque bytes or cross-piece token patterns.

After admission, piece slices are compared against exact expected offsets with checked addition and total length. No joined result is constructed. A boolean mismatch is a writer-byte mismatch, not source eligibility. Any unavailable admission falls back through the existing actual serializer in `serialized_main_part_matches` at `crates/rdocx/src/document.rs:24151`.

The two consumers preserve their original namespace, parsed-model and unchanged-package conjunctions before using this comparison. The final unchanged-main comparison still requires exact expected bytes. Existing writer failures remain refusal or absence of eligibility at these boolean consumers. No new source/read authority is inferred.

Removing the second aligned-value comparison in `MainPartBlocks::serialize` is safe in this call: every offered block comes exclusively from the prefix or suffix that already matched the current item, and middle entries are `None`. The current private-fact and field guards still run for each offered item. This does not cache alignment across calls.

## Retention and lifetime

The opening retains only the successful writer result already produced for its serialization proof in the existing `MainBodyScans.serialized` slot at `crates/rdocx/src/document.rs:23631`. Failed proofs and cache-off operation do not establish that retained record. A newly qualified opening record takes precedence over an older carried record. Subsequent comparison checks current frame and children, so a stale memo cannot become unconditional writer authority.

No retained-history container, cache field, public API, module or source file is added. Earlier retention does increase the opening's live normalized bytes and staged values. The plan and HLD explicitly put that change under Q22 measurement. This review does not claim it is free or that its memory rider has passed.

## Tests and evidence limits

Inspected the three new groups. The whole-byte comparison cases cover zero, one and 25 children, a different expected length, and independently reset root, tail and final-child mutations. The no-assembly test observes actual assembled-output bytes and unchanged record storage, with a real cache-off execution required to violate the same zero-assembly condition. The fallback cases cover fields, bookmarks, tab stops, a local conditional declaration and an ordinary undeclared frame. These are meaningful discriminators for the optional comparison.

`/private/tmp/s954-proof-only-serialization-red.log` records the actual pre-repair failure: the unchanged comparison assembled 39,589 bytes, with one test passing and one failing. That is historical RED evidence, not a GREEN result for this frozen source. No new verification build or test was run by the reviewer. The assembled-output hook measures this particular writer construction, not all copying or memory use.

The previous comment-batch native count pass belongs to its earlier source. Affected tests, counters, clippy and hash need the planned batched rerun for this change. Installed v6 flat and linear failures, release memory, native oracles and final integrated gates remain unresolved by this source review.

Exact reviewed `crates/rdocx/src/document.rs` SHA-256: `e3be424920600d773f7007b6c69a90c215b8c52998fdb7e24e61ebb1a3243bc2`.

No additional findings in correctness, panics, OOXML preservation, error ordering, contract scope or structure. The reviewer changed no source and ran no builds or tests.
