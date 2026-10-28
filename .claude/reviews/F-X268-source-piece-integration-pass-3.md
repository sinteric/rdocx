# F-X268, source piece integration, pass 3

**Reviewed**: Producer-frame increment after pass 2 in `work/f-x268-codex`, parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding four-file diff contains 6808 added and 214 removed lines. This pass covers retained-source selection, the narrow body-whitespace bootstrap, shared source reconstruction and the new producer regression. `crates/rdocx/src/document.rs` SHA-256 is `41d56c0e22d55152e951f9603cd823b0aaa3b471952e091b67ecbbeca1d943c4`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement this increment, modify source or run builds or tests.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this scope.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML preservation, tests and structure produced no finding in this increment.

The retained-record attempt requires the record's complete part bytes to equal the current package part. Reconstruction still obtains actual parsed children from the exact partition. It does not treat equality with the current typed model as parse provenance. Given explicit incoming source bytes, the old whole reader likewise replaces the model with that source's parse. An unflushed typed change is therefore not a reason to substitute model values into the source proof. Inspected path is `crates/rdocx/src/document.rs:23094`.

The bootstrap accepts only a nonempty run of XML ASCII whitespace immediately after the complete exact canonical head and body start. It does not search for a body tag or strip arbitrary producer content. A changed root prefix, attribute, binding or other preceding byte fails that prefix gate. The adapted empty shell is actually parsed and must normalize to the existing proved canonical shell, with no body children. The root range remains inside the unchanged head prefix. All child and frame proof locks are fresh for the adapted context. The strict canonical empty-frame helper is not weakened. Inspected path is `crates/rdocx/src/document.rs:23110`.

`source_blocks_in` is the common reconstruction implementation. Both callers require the exact frame and the actual partition proof. Checked prefix, suffix and child boundaries cover shrink, insertion and deletion. Changed source children retain the actual read proof introduced in pass 2, with separate exact-byte writer eligibility. Global resource validation and final partition checking still precede publication. Unsupported text, a malformed child, a changed tail or an unavailable frame declines to the unchanged whole parser. The extraction did not introduce a forwarding-only abstraction. Inspected path is `crates/rdocx/src/document.rs:23160`.

## Evidence examined

`retained_body_whitespace_keeps_source_edits_bounded_after_reopen` creates a real byte-opened package with a producer body newline. Each of four distinct source edits is prepared and reopened. The test compares exact retained source bytes and the complete debug model with a fresh whole parse. After two changed warmups it requires zero in all four byte slots and equal six-slot work at 50 and 200 paragraphs. This rejects normalizing away the producer bytes, dropping source-read facts on reopen and reparsing all unrelated children. Existing malformed-source, duplicate-splice, changed-root and writer-proof controls remain in the native batch.

`/private/tmp/fx268-codex-producer-frame-red.log` preserves the actual initial failure, 0 passed and 1 failed in 0.41 seconds. Its measured row was `[11208, 8406, 2802, 0, 1, 51]`. `/private/tmp/fx268-codex-producer-frame-check.log` records the affected native and Python all-target check finishing successfully in 25.77 seconds. `/private/tmp/fx268-codex-producer-frame-green.log` records 8 passed tests in 3.79 seconds. These observations were consumed without rerunning their commands.

The prior source-read Python batch recorded 3 passed checks and one missing-`lxml` environment failure in 76.61 seconds. The unchanged page-border semantic test then passed in 0.76 seconds after that dependency was installed. Those observations qualify the earlier increment, not the newly built producer artifact. Its authenticated original-nine work run and alternate-root control remain pending at this review boundary.

## Scope and pending work

This is a bounded producer-frame review, not complete mechanism 7 or whole-feature acceptance. Drawing-ID callers, HLD completion and the mechanism 7 checkpoint remain pending. Mechanism 8, A3 and A5 composition, release equality and error-order execution, the timing and memory riders, native Word, hash and full integrated verification remain pending. No final sprint or release pass is inferred from these local counted-work observations.
