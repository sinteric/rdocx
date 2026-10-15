# F-X206 union adoption, pass 55

**Reviewed:** Incremental adoption proposal `/private/tmp/S91-F-X206-reviewed-union-adoption-prepared` against independently reviewed R16 union and actual current R15 source. Packet receipt SHA-256 `96790d7959fd29e7cebf8b506e16805c4fde3bd8e66b9f9249fa6508a8169acc`. Adoption script SHA-256 `eac0f948ea3f5bd2820e7e44f2d0d4401d054ac5b61a3b22f337688976e3faae`.

**Verdict:** 1 defect, 0 smells, 0 nitpicks. No adoption or runtime executed.

## Defects

### D1, adoption rollback omits the currently failing carrier

`/private/tmp/S91-F-X206-reviewed-union-adoption-prepared/adopt.py:35`

The script appends the carrier to `changed` only after `write_bytes` and `chmod` succeed. A partial write failure or a chmod failure after successful byte replacement enters the exception handler with that carrier absent from `changed`. The restoration loop at `/private/tmp/S91-F-X206-reviewed-union-adoption-prepared/adopt.py:38` consequently restores earlier carriers but leaves the failing carrier changed. The before-map guard then raises while the worktree remains modified. This violates the stated restoration guarantee, even though it does not grant false runtime success.

Record each attempted carrier before its first mutation and restore its exact preimage and mode on the exception path. Preserve failure and restoration evidence if restoration itself cannot complete. No source was changed to demonstrate or fix this defect.

## Smells

0 findings.

## Nitpicks

0 findings.

## Authentication and six aspects

All packet artifact hashes and authority input hashes match. All 2714 actual records match current source `a9de0b3018cca10e5ac9b4a951de8542a3546060b5946917016cf516ba1884a4`, including size and mode. HEAD is `7b1e5ef29464e93ae30e68df1523ce09143578ff`. The semantic staged-entry digest is `c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c`. The manifest and prospective map differ from the current map at exactly the declared 22 existing carriers. Every candidate and preimage identity matches.

- **Correctness:** D1 above. No other finding in the incremental annotation or guarded adoption predicates.
- **Contract:** 0 additional findings. Adoption requires the actual ancestor-highlight semantic RED and two actual installed picture semantic failures, matching the declared baseline and restored whole-tree guards. Both prerequisite logs and all owned-group cleanup records are checked. Future picture receipt and adoption status remain unfilled in the prepared receipt. These prerequisites are discriminator evidence, not GREEN acceptance.
- **Panics:** D1 affects exceptional source restoration. No additional unchecked production operation is introduced by the annotation. Source mutation is blocked by current whole-tree, HEAD, index and candidate guards before it begins.
- **OOXML:** 0 findings in this incremental scope. Every R16 candidate is byte-identical except the one cfg annotation. Namespace, cell, naming, picture geometry, opaque payload, source order and existing provider semantics retain the independent pass 53 union authority. This pass does not claim a new independent audit of inherited self-authored production.
- **Tests:** 0 additional findings. The annotation at candidate `crates/rpptx-render/src/text.rs:279` applies only to the private cached wrapper. Its callers are cfg(test) inline_items and test-module controls. Production shaping still calls inline_items_with_caps. No expected value, oracle or deciding assertion changes. The command notes retain mandatory explicit execution of both ignored caps oracles with one passed and zero ignored each, current installed collection rather than arithmetic historical counts, fresh paired wheels and all broader plan barriers.
- **Structure:** 0 findings. The exact difference from reviewed R16 is one `#[cfg(test)]` attribute in the existing text carrier. No new repository file, module, dependency, public API, trait or forwarding wrapper is introduced.

## Limits and handback

Prior independently reviewed R16 semantics remain the parent authority. Whole working review, current compilation/GREEN, installed acceptance, native application/no-repair oracles, HLD delivery, hash attribution, archives and scoped policy remain required. No current runtime result or feature completion follows from this review. Only this report was written. Return the review-only lease to ROOT.
