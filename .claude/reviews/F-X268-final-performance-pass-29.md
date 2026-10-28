# F-X268, test configuration repair, pass 29

**Reviewed**: One-file working diff against `fc670a18d15c25038b568f681613073ddea79968`, one insertion and one deletion. Only the attachment of an existing test configuration attribute changes.

**Reviewer**: Codex, `/root/review_source_pieces`. I independently diagnosed and assessed the proposed conditional-exclusion repair. I did not edit or apply it.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Identity and source assessment

The canonical workflow/microscope contract and current progress remain applicable. I verified freeze SHA-256 `168bc3742ec79161fe3182fc0f9340f8e2bc65993290777c2260ae7a97c524b2`, its exact `frozen.diff` and current source SHA-256 `1cc84df142bd2fd4cb62a812603e7ee11fd427992092ec23daf114e1b7229bb1`.

`crates/rdocx/src/document.rs:51715` removes the debug-only condition from the pure namespace-prefix comparison test. Its body depends on local comparison code, the production namespace helper, standard types and XML types available in both configurations. It does not invoke a debug-only fixture or counter. No test body or parser behavior changes.

`crates/rdocx/src/document.rs:52291` attaches that same condition to `picture_plain_read_requires_existing_actual_source_and_exact_partition`. This restores the intended condition of its helper `plain_picture_read_record`, which remains debug-only at `document.rs:51707` and itself uses the debug fixture. The other helper callers at `document.rs:52347` and `document.rs:52419` retain their debug conditions. This corrects the stranded-attribute cause of the actual no-debug libtest compile failure without weakening a production guard or changing A9's named work gate.

Both anchors are inside the existing `#[cfg(test)] mod tests` at `crates/rdocx/src/document.rs:48992`. I independently reconstructed the exact two-anchor replacement from the immutable fc source and compared it with the working bytes. It matches completely. Total size remains3238312 bytes and79580 newline characters. All bytes before the removal anchor and after the insertion/declaration anchor remain identical at the same offsets. Thus production code before and after this excluded test region retains both bytes and line positions. No other source or attribute changes occur in this diff.

## Evidence and applicability boundary

The actual fc no-debug A9 libtest compile failure remains a failure. The supplied fmt/diff checks are successful policy checks, not compilation or counted-work evidence. No fixed-source A9 build or count pass has been established by this review, and I ran none.

Actual fc normal, CLI and debug-binding artifacts and both picture-gate results retain their original producer identities. Their applicability to a later integration source requires the separately checked strict two-anchor source-exclusion proof, actual per-artifact `cfg(test)=false`, unchanged dependency/tool/profile/module hashes and immutable receipt linkage. This report does not relabel those artifacts or grant a blanket documentation/source exception. The failed `cfg(test)=true` A9 category must be rebuilt and executed on the corrected source.

## Not found

Correctness, contract, conditional exclusion, attribute attachment, panic, OOXML, tests and structure produced zero findings in this tiny diff. No public interface, schema behavior, production allocation, writer/parser authority or performance bound changes. No new build, timing, memory, final-union or completion acceptance is supplied.
