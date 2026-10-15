# F-X221 integrated gate remediation, pass 4

**Reviewed**: The distinct `release_unmapped_jumps` repair at `crates/rpptx/src/lib.rs:16358` and strengthened existing regression at `crates/rpptx/tests/integration.rs:34652`. The complete frozen working diff is13 modified carriers,570 insertions and148 deletions on HEAD `c48624f58122d4c22026434acc1496accb061342`. Other source retains pass3 coverage. Only this new report was written by the reviewer.

**Verdict**: Zero defects, zero smells, zero nitpicks.

## Ownership and behavior

The filter at `crates/rpptx/src/lib.rs:16401` preserves the prior release predicate and current row order. It now builds the proposed kept rows without first changing the original current item inventory. The call at line16410 therefore proves known relationship retirement against its actual retained source, while still permitting genuinely authored rows without retained owners. The existing ambiguity refusal remains intact for unexplained renamed historical owners. Surviving opaque relationship payloads use the qualified rebind operation rather than generic serialization.

No jump classification, namespace-owned action release, diagnostic, sound/shared-reference retention or exact XML rewrite logic changes. The existing refusal at line16393 still prevents dropping a slide relationship that remains referenced outside the released hyperlink. This operation returns local XML and relationships into the already staged import, retaining source immutability and destination atomic publication. Removing the unused mutable argument does not change executable behavior.

## Regression and evidence

The existing test now reopens the authored jump deck before adding a genuinely distinct blue PNG relationship at `crates/rpptx/tests/integration.rs:34659`. Both existing-map and unmapped branches retain their exact target assertions, diagnostics, unchanged destination slide and unchanged source checks. The additional saved-package image-byte check at line34720, seven-shape assertion and validated reopen ensure the new current row survives rather than being silently discarded.

The actual predecessor log `current-image-with-unmapped-jump-original-red.log` retains the InvalidRelationship failure at the import. The actual successor log `current-image-with-unmapped-jump-successor-green.log` records1 passed in1.90 seconds. That successful run preceded only the unused-mut cleanup and retains its warning honestly. No runtime execution was performed by this reviewer. This static verdict does not claim fresh wheels, a new full gate, a rerun of the frozen100 or native appearance qualification.

Zero additional correctness, panic, contract, OOXML preservation, mapping, ordering, atomicity, test-strength or structure findings. The approved8 MiB corpus-test runtime remains unchanged. This review ends here and releases the review-file write lease.
