# F-X207, distinct shared contexts and current owner fixtures, pass 38

## Scope

ROOT reviewed the exact two-carrier external correction05d554e008031d1ffd8b39d7ef052267142edda11ee118bacb2228d5f6e3b819 against actual source54bccc473235f98f0a8651a4e41f01193a7e1518445397e1e9c63e5dcdb0fc06. Both current before images and proposed candidates authenticate. No source or runtime was changed during review.

## Defects

Zero findings.

## Smells

Zero findings.

## Nits

Zero findings.

## Evidence

crates/rpptx/src/lib.rs:1545 selects one shared pair before emitting context reports. select_autofit_pair at11745 requires Q and S containment in every context. The original different needed_scale assertion therefore contradicts the actual common result contract. The candidate at14175 instead measures the original first/tenth slide field through real Q/S trials at100000/0. Both trial fit predicates must differ for the one-digit/two-digit consumers. The common reported pair must agree and shrink. All ten final containment, one physical publication, slide preservation and repeat assertions remain. No identical artificial context substitutes for the deciding distinction.

crates/rpptx/src/lib.rs:1696 rejects a stale physical fingerprint. The rich test at crates/rpptx/tests/integration.rs:28889 now reacquires the actual reopened owner before fit_text_at. Its source text, wrapped/rich/vertical/edge-spacing matrix, preservation and reopened containment assertions are unchanged. This is valid setup for a second mutation, not a relaxation of stale-owner refusal.

## Verdict and limits

Zero defects and zero smells for the finite fixture correction. The original failed receipts remain failed. Both affected selectors require actual retry on the corrected source. Existing Q-only and pinned structural oracle passes remain their exact source-equivalent evidence. No production behavior, shared-owner policy, native9823EMU discrepancy, independent paint requirement or threshold changes. Review ends here.
