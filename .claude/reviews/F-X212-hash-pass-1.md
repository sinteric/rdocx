# F-X212, hash attribution, pass 1

**Reviewed**: The approved three-key intentional audit delta, authenticated
accepted R5 source, all49 measured entries, complete retained FeatureShowcase
packages and PDFs, and the bounded audit-method reversal with its explicit
terminal supplement. This is an independent attribution review, not a full
feature or sprint gate.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Authentication

Accepted worker document SHA-256 is
9f531eccc7bf4918c2c080fe636f425c83e72359713c7db308cb9d189be0eaa8.
All seven R5 source guards and the eighth harness guard match after actual
restoration. The baseline and sample generator remain unchanged. Worker
extraction HEAD is ea803be1e59ba588d4c8da3f1231d84314cd847a.
Prior full source and supplemental test reviews remain applicable to this
unchanged production prefix.

The accepted measurement receipt is
92e1ffef5c5b3397817b4a8577568358d2b0978950d06e11f8e8aa728702c56c.
Its actual child hash-check exit is1. Independently recomputed comparison of
its recorded49 actual entries with the unchanged baseline gives exactly:

- feature_showcase:word/document.xml
- feature_showcase:pdf/pages
- feature_showcase:pdf/bytes

All46 other entries match, including FeatureShowcase page1 PNG, PDF resource
streams, numbering and styles. Accepted artifact size and SHA-256 receipts
match actual retained bytes. This expected comparison failure is measured
intentional change evidence, not a passing updated-baseline check.

## Causal proof

The bounded reversal proposal SHA-256 is
2250fe4b6cddc91727fe81af9a73ecc1a662475ab0f6f1a9cc1b69301169fa0c.
Its executable helper SHA-256 is
5d9282ab34ca3d691fcf4a90b194996b06df19acc7084a948bb1fcdf2def88dc.
The proposal replaces only audit_accessibility with the method from the
mechanical extraction commit. New parser, public query and CLI semantics stay
in place. The reversed document is independently bound to
f1ebd3a4ba6817267f3e19bad53ba6518178c224b6e956fea972e6316b1417d2.

The actual hash child exited0. Its log SHA-256 is
249212f964241c3026b14f32f73c74747bfa7a78e74ae4fc091034dff6dbae4c.
The retained original partial execution receipt SHA-256 is
d670aa61ef97bb63eecc6d9ca308704542075466aabc973ae5e26ea2f5d641ae.
It records exact equality of all49 original entries. Independently collected
hashes from the actual generated old-audit samples equal that recorded map
and the unchanged baseline. Independently recomputed fingerprints of both
retained PDFs agree with their respective measured maps.

The enclosing helper exited1 after the successful child because its
post-generation assertion incorrectly expected only a count replacement.
The failed helper and its partial status remain preserved. The finally block
restored accepted document bytes and all guarded source. Terminal supplement
SHA-256
7632949a25da72ed507489da0d6342cbdcb4acb65f9d9b02f59bd50ce4ea4762
states this distinction explicitly. No source or Cargo retry was needed.

Independently compared the complete old and accepted DOCX entry inventories.
They are identical. Only word/document.xml differs. Direct byte reconstruction
proves exactly two effects and no additional XML change:

1. Accessibility issues: 4 becomes Accessibility issues: 3.
2. One complete numbered paragraph containing Info: 4 consecutive empty
   paragraphs (consider using spacing instead) is removed.

The exact removed paragraph SHA-256 is
a8472dacb2ae69e1e61c045e456077d3604cf4dfa9021c99fe56f181cd3f2ef2.
The authenticated XML diff SHA-256 is
b1e611c36b6b3d2eb53e01ef977876ffb58e5bee9483901614370ea0268d6305.
At crates/rdocx/examples/generate_all_samples.rs:655 the generator calls the
audit, at :656 it emits the count and at :657 it emits every finding as a
list paragraph. Both observed XML effects therefore follow the same removed
false finding. The audit's accepted DirectBody visibility at
crates/rdocx/src/document.rs:29685 governs its empty-paragraph streak at
:29694. The three declared keys are causally attributed to this method's
intentional behavior, including downstream PDF content and byte changes.

## Aspect results

- Correctness: zero findings. Independent complete package-byte comparison
  and the exact original49 reversal establish the declared causal boundary.
- Contract: zero findings. The observed three keys match the approved hash
  section exactly. The generator emits both count and finding content, so
  preserving the helper assertion failure does not leave an unexplained delta.
- Panics: zero findings in reviewed production attribution. Diagnostic helper
  assertions are bounded proof checks. Its failed assertion and finally
  restoration are explicit evidence, not a production pass.
- OOXML: zero findings. Exact removal of one complete numbered paragraph
  leaves all other XML bytes and package entries unchanged.
- Tests: zero findings in this attribution proof. Retained runtime receipts
  separately establish check0, OXML8, native audit6, public query2,
  malformed1, the53-case CLI matrix and the corrected three legacy CLI
  controls. Original compile and fixture failures remain distinct. Their
  reviewed corrections do not relabel earlier attempts as passes.
- Structure: zero findings. The proof substitutes one existing audit method,
  restores it and introduces no production module, dependency or API.

## Limits and handback

The accepted current-source baseline comparison still exits1 against the old
baseline. This review authorizes attribution of the exact expected delta,
not a baseline write performed by the reviewer. The exclusive owner must
record any authorized baseline change in its labelled behavior commit and
run the resulting check. Broader native suites, remaining complexity and
publication riders, HLD work and the final scoped and integrated gates retain
their own obligations. No Cargo, source, scratch, formatting or baseline
write occurred during this review. Only this review record was written.
This ends the review pass and returns control to the integrator.
