# F-X236, all aspects, pass 1

**Reviewed**: working diff against dbce3b2d401f64f6eed8406dbc31a557b897cc9e, one source file, six inserted lines. Approved design, binding setter contract, performance provenance, publication riders, external consumer, assembly, all eighteen timing samples and native semantic artifacts inspected read-only.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, Fresh optional-property branches lack the promised semantic control

`crates/rdocx/src/run.rs:548`
`/private/tmp/F-X236-consumer/src/bin/semantics.rs:7`
`/private/tmp/F-X236-consumer/src/bin/semantics.rs:18`
`.claude/plans/F-X236-design.md:47`

The approved semantic matrix includes fresh and already initialized properties.
Every optional semantic probe first sets color and three direct properties, so
None only clears an existing property block. Every direct probe also sets color
first. The published ten-case result therefore does not exercise the newly
annotated optional setters' absent-property early return or their construction
from fresh None. Existing direct optional references inspected in the repository
do not supply that missing matrix. Add independent source-built controls for
fresh true, false and None, including the requirement that fresh None leaves
w:rPr absent, and inspect the saved and reopened packages. Keep the existing
unrelated-color controls. This is missing acceptance evidence, not an observed
setter implementation error.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness: the source diff consists solely of six ordinary inline annotations. Setter bodies, inheritance behavior, size conversion and complex-script twins are unchanged.
- Performance: the real external crate explicitly disables LTO. Baseline assembly calls three public Run setters, while candidate assembly exposes stores around the existing ensure_rpr calls. The measured loop still calls format_run on every iteration. ParagraphMark is already fully inlined in both assemblies and is correctly left unchanged.
- Measurement: all nine alternating pairs favor the Run candidate. Ratio of medians is0.716466851. The unchanged mark control ratio is1.002663346. Authoring ratio1.029707557 is noisy and supports no end-to-end speedup claim. Candidate binary is192bytes smaller. Input, manifest and lock digests match recorded provenance.
- Native artifacts: independent XML inspection confirms the existing five initialized cases in both original and reopened packages retain expected booleans, sizes, complex-script twins and unrelated color. Correcting the observer's numeric-boolean assumption is recorded honestly.
- Panics and OOXML: no executable logic or serializer behavior changes. The semantic artifact gap is recorded above.
- Structure and compatibility: no new trait, generic, public signature, module, dependency or feature flag. Optional helper annotations are needed to expose the three measured setter chains across the actual crate boundary.

Scoped tests, packaging and policy evidence are still pending and are not claimed
by this review. The review pass ends here. Return control to sprint orchestration
for separate remediation and a subsequent independent pass.
