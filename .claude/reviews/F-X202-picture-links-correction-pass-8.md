# F-X202, picture and link correction, pass 8

**Reviewed**: Immutable r8 correction capsule ac7680d599dfacfa86a9835dae4b5017acbc764d338fabb1bef749251890a217. Seven complete before/after carriers, 307 insertions and 12 deletions. Bounded follow-up to pass7 D1 to D3, with two finite Clippy corrections and deciding native controls. Earlier table, paired-text and complete picture implementation are not re-reviewed.

**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. Pass7 D1 to D3 are resolved in this bounded correction.

## Smells

None.

## Nitpicks

None.

## Authentication and contract

All31 manifest records match their recorded bytes and SHA-256. Complete preimages match source maps32 or33. The integration-test preimage is map32, because the correction includes the independently executed map33 anchored-picture control. The other native production and stub after-copies match map34 exactly. The Python test after-copy adds explicitly unrun cases after map34. No installed credit is inferred from those additions.

The approved plan's Bookmarks and internal links section permits supported related source owners while requiring checked document ownership, native name grammar and one candidate publication. HLD04 Story-scoped picture and hyperlink authoring and Checked related-story publication retain physical owner and transactional authority. HLD10 Python API shape and Native Word facade stability retain source ownership and checked API boundaries.

## Not found

Correctness and contract: `crates/rdocx/src/document.rs:19607` uses the existing authored bookmark-name validator before target inventory. `crates/rdocx/src/comments.rs:1889` changes only crate-local visibility. The source paragraph still passes checked physical snapshot admission, and refreshed story lookup plus `edit_story_paragraph` retains its actual related owner. Removing the blanket main-story source refusal does not allocate an external hyperlink relationship.

Atomicity and source ownership: `crates/rdocx/src/document.rs:19651` retains cloning, checked staged append, prepare/reopen and one commit. Heading source admission and the existing target contract remain unchanged at `crates/rdocx/src/document.rs:19691`. Suitable bookmark reuse additionally applies the same name validator at `crates/rdocx/src/document.rs:19736`. Missing, ambiguous, incomplete or invalid-name targets refuse before live publication. No new parser, serializer or opaque-owner rewrite is introduced by this correction.

Bindings and structure: `crates/rdocx-py/python/rdocx/_rdocx.pyi:67` declares the Picture factory-only type final with a nonreturning constructor. This matches the frozen runtime class without a Python constructor at `crates/rdocx-py/src/document.rs:244`. The local explicit-operand Clippy allowance at `crates/rdocx-py/src/document.rs:6679` preserves the approved checked public signature. No new trait, generic, dependency, module or product file is introduced.

Tests and panics: the new related-owner control checks header, footer, cell and text-box source links, bookmark reuse, saved/reopened output, absent external hyperlink relationships and invalid imported-name byte atomicity. The anchored picture control retains namespace aliases, crop, opaque metadata, placement, media and reopened physical extent. The regression byte-slice change preserves the same source interval and parsed assertions.

## Actual evidence and limits

Copied receipts and every copied runtime log hash are authenticated. Batch32 retains combined native/binding check0, heading-link1PASS and configured-picture-owner1PASS. Batch33 retains anchored alias/crop/opaque1PASS and its original Clippy failure. Batch34 receipt3dbb4efaf6dc6244fccbc09e2cc298ec4b5a071237dd8a879aa1ba1da5482631 proves combined check0, related-source/invalid-name1PASS and scoped Clippy0. All three batches record exact source-after guards and naturally safe process cleanup.

This pass did not execute commands. Public test additions, fresh installed Word runtime, typing/stubtest, actual Word/LibreOffice acceptance and remaining whole-feature scoped gates are pending. No full-feature completion or sprint closure credit is granted. The review pass ends here.
