# F-X205, two-master fixture, pass 14

**Reviewed**: Independent bounded external proposal changing three hunks in the existing integration test carrier,9 additions and8 deletions. Product source, template asset and original production assertions outside this fixture are unchanged.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Source identity

Input: `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/integration-two-master-fixture-proposal-2`.
Manifest SHA `f1f0a61fb3154c2c7887562e1e51b0b14cd68f1d2db4e687ae912cf6469e5037` authenticates all four records without mismatch. Receipt SHA is `35a28d3e168a8343942c030f5e0a0011306d415243f86347e1f4f1f81de56568`. Before SHA `109c8860acd8e2a6a63d9b4703fed37ee03dfb502fd8e841f1ad508e513b97c9` exactly equals the reviewed pass13 candidate. Candidate SHA is `55ff07d3c3f84d41aad629c5269c478aa3e7a5113e0699f7e48a8f2463a6f3ff`, diff SHA `521b7a17918e5824931b37d36566db2b9c30e873d0ce40db16fd231ced92f2cd`.

The actual failed log independently matches SHA `57fc861c38c5f75dd998d2810f728dccdc7fcf9137efbf0b4f486716aa8145da`.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Six aspects

- **Correctness, zero findings.** `integration-candidate.rs:10191` constructs the replacement needle from the authenticated current MASTER_TITLE offset and explicitly requires exactly one occurrence. The old template has exactly one457200/274638 offset and no838200/365125 offset. The current template has exactly one838200/365125 offset and no old needle. Qualified XML inspection confirms this is the actual title placeholder, with current extent10515600/1325563. The distinct second-master authored457200/137319 offset remains unchanged.
- **Contract, zero findings.** `integration-candidate.rs:10325` checks that the retargeted title inherits the current native title extent, while keeping the deliberately different second-master offset. The source master is a copy of the actual current template, so expecting its previous8229600/1143000 extent was inconsistent with the fixture. The correction restores a deciding multi-master geometry test instead of removing or weakening it.
- **Panics, zero findings.** The exact-one assertion precedes lexical replacement, so a missing or ambiguous fixture operand fails explicitly. Existing checked production paths and test assertions are unchanged.
- **OOXML, zero findings.** Only the actual owned title offset is replaced. Relationship retargeting, schema order and master/layout construction are unchanged. The fixture does not alter opaque siblings, saved metadata or production serialization. Native template provenance remains the already authenticated pass5/pass13 authority.
- **Tests, zero findings.** Existing layout index, effective geometry for title and body, validation and export assertions remain. The new uniqueness assertion proves the intended title was actually altered. The fixture retains the first master's current geometry and the second master's distinct authored origin, rather than comparing the two against the same result.
- **Structure, zero findings.** One existing test carrier, no new helper/module/dependency or product API. The source-owned constant is reused instead of introducing another stale template literal.

## Actual failure and limits

The retained actual-integration-correction-GREEN-1 receipt remains failed. Its first exact geometry selector reaches the two-master assertion with actual838200/365125/10515600/1325563, proving the previous old-template needle matched nothing. Expected457200/137319/8229600/1143000 is retained in that failed log. The selector has0 passes/1 failure/0 ignored. Compilation succeeded, source51 stayed guarded, and both owned groups terminated naturally with safe ESRCH cleanup.

No candidate runtime or adoption is credited. The six later rows did not execute in that failed attempt and remain unrun. Earlier306 passes, archive attribution, hash expectations and native paint evidence are not replayed or relabelled by this test-only review. No product source or baseline changes were performed. This pass ends with this report.
