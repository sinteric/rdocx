# F-X268 final performance remediation, pass 6

**Reviewer:** Independent Codex reviewer `/root/review_source_pieces`.
**Scope:** Frozen working diff against `65e71f6e5df563519ba0028f4350e35eaf4f35b3`, four files, 58 insertions and 18 deletions. The batch changes address-scan carry across reopen, direct paragraph addressing, one existing regression, the approved plan and HLD 03/12. Earlier reviews supply unchanged feature context.
**Method:** Read-only source, callgraph, assertion and existing evidence inspection. No source edits, tests, builds or delegation.
**Verdict:** One defect, zero smells, zero nitpicks. No new release timing acceptance is granted.

## Defects

### D1, direct paragraph addressing bypasses the previous related-story validation

`crates/rdocx/src/document.rs:33183`

The new direct paragraph branch returns through `direct_body_item_location` before the existing `stories()` call. The old path enumerated all story sources and owners before returning the body paragraph address. That enumeration validates header/footer references and target presence (`crates/rdocx/src/document.rs:25359`), other related target presence (`crates/rdocx/src/document.rs:25399`), and the related owners themselves through `stories`. The new helper only serializes and scans the main body (`crates/rdocx/src/document.rs:27187`).

Consequently a valid direct paragraph with an unresolved header/footer reference, missing related story target or malformed related owner can now return an address instead of the prior related-story error. The controlled paragraph path still performs the old validation, so the error surface also differs according to physical paragraph placement. The existing branch is used by Python `Paragraph.replace_text` before mutation (`crates/rdocx-py/src/paragraph.rs:921`). Later mutation preflight is not a substitute for preserving the callable address lookup's result and failure order.

Preserve the old source and owner validation before consuming the direct-slot result, using the existing qualified layout reuse where possible. Add a discriminator comparing direct and controlled paragraphs against the old lookup for an invalid related reference or owner, alongside a valid related-story positive control. Do not weaken the existing refusal to obtain the fast result. This finding is source-established, not a newly executed runtime RED.

## Address-scan carry audit

No defect found in the changed carry mechanism. `install_carried_body_scans` obtains the current partition through `main_part_with_body_spans`, whose writer record is qualified against the current model and whose fallback remains the ordinary writer (`crates/rdocx/src/document.rs:23712`, `crates/rdocx/src/document.rs:23826`). It requires exact current head and tail before carrying the read frame. Changed child count or bytes clear the old whole-body fingerprint instead of discarding all neighboring read facts.

The later layout consumer still compares each aligned child with actual current bytes and requires item coverage for item queries (`crates/rdocx/src/document.rs:25555`). Prefix and suffix alignment never reuse an unaligned interior child. Duplicate equal child bytes remain relative scan facts under the same frame, while current offsets and owner ordinals are composed anew. A changed frame drops these facts. Fields, tabs and bookmarks remain part of the exact byte comparison. No typed model or writer permission is inferred from a read scan.

The install path releases its first lock before obtaining the current partition. At final installation it takes the newly qualified serialized memo from the current cache rather than overwriting it with the older carried memo (`crates/rdocx/src/document.rs:23735`). This keeps writer provenance separate from address scan carry. A failed optional carry leaves the ordinary subsequent qualification and scan paths authoritative. The existing producer-frame and unsupported-source fallback policies are unchanged.

## Evidence and scope limits

The extended existing test changes a previously unread child before reopening, then compares the actual fast address with the whole reader and requires exactly one fresh scan among both 50 and 200 children through both reopen routes (`crates/rdocx/src/document.rs:51508`). This rejects the previous all-neighbor discard. `/private/tmp/s954-changed-child-address-carry-red-2.log` is the actual old-source RED. The earlier variable-shadow compile failure is not a semantic RED.

`/private/tmp/s954-address-carry-native.log` records seven `main_body_layout` cases passed in 20.92 seconds. The additional scoped native batch was still running when this report consumed its log, so no completion is claimed. Existing tests exercise frame changes, raw blocks, mixed owners, aliases, default namespaces and full-reader agreement. They do not cover D1's newly bypassed related-story failures.

Reviewed source SHA-256 for `crates/rdocx/src/document.rs`: `40e70e473d21d5657bf3a1c54b8ab8776c9671a5278620cb95bc8a371aeb4169`.

The plan and HLD retain timing and final qualification as pending. Previous v4 targeted measurements remain failures. This local counted-work regression does not prove the 79 flat gates, five linear gates, complete catalogue, memory rider, installed Python behavior or final sprint acceptance.

## Not found

Correctness of exact-frame carry and child alignment: zero findings. Namespace and opaque XML preservation: zero findings. Panics, locking and resource checks: zero findings. Structure: zero findings. Documentation scope: zero findings. Tests: no additional finding beyond D1's missing failure discriminator. Smells: zero. Nitpicks: zero.
