# F-X197, PowerPoint cell tests, pass 6

**Reviewed**: Two test-only corrections in one existing carrier on
work/f-x197-codex, HEAD b96c7888b7d99bf1ab6c720ea5ba318c7103d611.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and authentication

Read the approved colour input and lifetime contract, HLD10 cell-text
invalidation, HLD05 colour preservation, the preceding incremental review
and the actual failed R12 scoped receipt. The author is independent of this
reviewer. No runtime, Cargo, source correction or formatting ran in this pass.

Authenticated receipt
a0549fcda50e32c79a872bbeb08897b4b52b142de4d348ae3880a6df4a051cbc.
Independently reconstructed the complete retained before/current diff,
matching e8edb4d4db591552eaf514d6ff6b4eb52c9bea5301e987ccd0217e41158b339f.
The current carrier hash is
112b4b347c1149cb24386c1b7a8897e283fdbcb0ef42f72f39a05cfe4c71c8c9.
All318 current input hashes, sizes and modes match the freeze. Comparison
against the R12 manifest identifies only this existing test carrier as
changed. Production, manifests, stubs and packaged Python are unchanged.

## Assessment

At crates/rpptx-py/tests/test_documented_examples.py:4981, the setup retains
the grid cell and fetches its fresh frame after frame.text replacement. HLD10
requires that replacement to retire nested cell text handles while keeping
the grid cell live. The setter and colour view are captured after setup,
so the accepted-input, error atomicity and deliberate invalidation controls
still exercise their intended operation.

At test_documented_examples.py:5431, the lifetime control still checks both
stale getter and stale setter refusal and unchanged saved source. Its fresh
and reopened run assertions now require text second and retained RGB123456.
The unchanged native CT_TextBody::set_text at
crates/oxml-drawing/src/text/mod.rs:261 delegates to paragraph set_text.
At crates/oxml-drawing/src/text/paragraph.rs:1561, that operation retains
the first existing regular run and its direct properties. The old None
expectation contradicted this existing behavior. The fresh getter also has
an unchanged-source assertion. No production lifetime rule is relaxed.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness and contract: valid setup and retained formatting match existing
semantics. Panics and OOXML: no production or serialization change.
Tests: stale refusal and atomicity remain, with stronger fresh and reopened
controls. Structure: only an existing test carrier changes, with no new
dependency, module, wrapper or API. Zero additional findings.

## Runtime qualification

R12 actually passed155 Word cases and failed60 of497 PowerPoint cases.
That failure remains immutable. This pass credits no corrected runtime
result. Renew scoped and full execution guards with all four reviewed
test-only deltas and authentic R10 compiled wheel and installation
equivalence. Remaining scoped, typing, stub, full-suite and risk gates still
decide completion. Only this report was written. This review ends here.
