# F-X236, all aspects, pass 2

**Reviewed**: unchanged working source diff against dbce3b2d401f64f6eed8406dbc31a557b897cc9e, one file, six inserted inline annotations. Fresh-property semantic remediation and both native artifacts independently inspected read-only. Approved plan and prior pass rechecked.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found. Pass1 D1 is resolved. The independent consumer now exercises fresh
optional true, false and None plus fresh direct true and false, alongside the
five retained initialized-property cases. Separate read-only XML inspection of
both native original and reopened packages verified all ten cases, including
absence of w:rPr for fresh None, complex-script twin fields, expected half-point
sizes and unrelated color retention. The recorded semantic-source digest matches
the inspected source. Original observer error and pass1 finding remain recorded.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness: all six annotations retain the exact existing setter bodies. No behavior change found in property initialization, inheritance clearing, paired complex-script fields or truncating size conversion.
- Contract: actual external public consumer, explicit no-LTO profile, nine alternating measurement pairs and separately consumed semantic artifacts satisfy the bounded measured-setter contract. ParagraphMark remains unchanged because baseline assembly already inlines it.
- Performance: baseline and candidate caller assembly retain the measured loop and expose actual public-call elimination. Ratios of medians are0.716466851 for Run,1.002663346 for the unchanged mark control and1.029707557 for noisy authoring. No end-to-end speedup claim is supported. Binary size falls192bytes. Source, manifest and lock hashes match recorded provenance.
- Panics: no executable control flow or arithmetic changed. New semantic probes are test infrastructure only.
- OOXML: independently decoded saved and reopened XML retains booleans, sizes, complex-script twins, unrelated color and fresh-owner absence as specified.
- Tests: corrected independent artifacts close the specific missing fresh-state matrix. Remaining scoped tests, package and policy gates are pending and are not claimed by this review.
- Structure and compatibility: no new signature, trait, generic, wrapper, dependency, module, file or feature in the product. Optional-helper annotations expose the three actual measured setter chains across the existing crate boundary.

This review pass ends here. Return control to sprint orchestration for the
remaining acceptance gates and preparation. No source was edited in this pass.
