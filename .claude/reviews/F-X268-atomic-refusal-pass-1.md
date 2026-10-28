# F-X268, atomic refusal, pass 1

**Reviewed**: Phase 1 only, Q19, Q21 and A1, against claim Base 9d2878b0c8d7cb0e1b8d9d3d697b5117893ac1bc. Four source and test files, 553 insertions and 1 deletion. Frozen diff SHA-256 1d329c074d98a9e00bfb4c01416e6980f6bb330e15486db47259a7e7292fa8ef.
**Reviewer**: Codex root independently of Grok implementation. Grok transcribes this report.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None in the bounded behavior increment.

The four additive Rust methods check insertion bounds, paragraph XML characters, and namespace replay before mutation. The original infallible signatures and legacy behavior remain. Zero dimensions retain the existing builders. The namespace diagnostic matches the authoritative writer. None from the hidden guard is explicitly not general savability proof.

Python checks the same native policy before its existing splice and fallback publishers. Its paragraph preflight preserves XmlError ahead of namespace refusal, and insertion bounds preserve IndexError. The seven named entry points are exercised through public Python. The binding retains its source-preserving fast routes rather than replacing them with a different publication path.

The guard cache is cleared at invalidate_layout before mutable model references are returned, at replacement of the main source, and after a changed typed main part is flushed. The guard itself does not invalidate layout. Rust borrowing prevents a caller from filling the document cache through a document borrow while its mutable paragraph borrow remains live.

The direct-native regression demonstrates both directions: distinct namespace owners prime a savable result, changing their text into an ambiguous pair must refuse, and restoring their distinction permits a checked edit and save. The original stale-None implementation actually failed this case before the central invalidation fix.

## Smells

None. No new module, crate, trait, generic parameter, dependency, stable Python export or Python try-method was added. The doc-hidden paragraph preflight has two concrete callers: the Rust checked methods and Python character-error ordering. Root accepts that internal shared-policy helper and its exact signature as a bounded implementation clarification. Record it in the existing plan at the next implementation checkpoint:
preflight_checked_paragraph(&self, index: Option<usize>, text: &str) -> Result<()>.

## Nitpicks

None.

## Evidence

The old-source public Python baseline ran on authenticated Base artifact ae3879224176459797da677f26058757a5aa0592817921004c985ded1d8d17d6. Seven valid cases passed and fourteen alias/twin cases failed after the old edit changed model state and a later save failed. Insertion cases also retired held owners. This is semantic RED, not a missing-API compile failure.

The native group covers all four checked methods, already-unsavable state, available XML bytes and model preservation, index and character precedence, zero dimensions, legacy behavior and cache invalidation. The cache discriminator failed in /tmp/fx268-inv-red.log and passed in /tmp/fx268-inv-green2.log. The current native group passed 1, 473 filtered, in 0.07s.

Fresh maturin completed with child exit 0 in 13.25s. The current extension and archive SHA-256 is 529111325fb0628f9752d92ddd5b27c28b0710010aed146413a7eb1a0057a8ac. Python passed 43, 524 deselected, in 3.37s. The real root r shadow is asserted in the ZIP before opening. The unused foreign-prefix control remains editable. The earlier URI-replacement fixture was a no-op and is not proof of a real shadow. Scoped check, format and scoped clippy pass. The receipt is /private/tmp/s95.4-fx268-phase1-ready.txt.

## Not found and boundary

Correctness, contract, panics, OOXML preservation, tests and structure produced no additional findings in this increment. The complete source/test diff and shared mutation, insertion and source-replacement callers were examined.

This is not a clean whole-feature microscope and does not qualify F-X268 completion. The remaining qualified body-splice and proof work in mechanism 1, mechanisms 2 through 8, their local discriminators, all five HLD impact files, the whole-feature review and every named final sprint rider remain due. The cold guard clone and replay are not claimed constant-time. The final performance and memory gates remain unchanged. Preserve this behavior in its own labelled commit with the expected refusal delta stated. No confirmation pass is needed for this unchanged increment.

The review ends with this report and changes no implementation.
