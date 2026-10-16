# F-X253, correctness, pass 1

**Reviewed**: Frozen working diff on work/f-x253-codex against 4b1ef1342c600c481d42df9e32de97aed27225b6. Fourteen files, 582 insertions and 20 deletions. All file digests and patch SHA256 match /private/tmp/S95-F-X253-worker-verification-1/review-inputs.json before report publication.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Invalid selected chart metadata is reported as a valid unsupported type

`crates/oxml-chart/src/lib.rs:429`
`crates/oxml-chart/src/lib.rs:434`
`crates/oxml-chart/src/lib.rs:378`

The scatter and radar match guards compare their scalar directly with one supported lexical value. A qualified direct scatterStyle val="garbage" or radarStyle val="garbage" therefore reaches the final None arm. These are invalid selected enum values, not valid unsupported styles. The existing ScatterStyle::parse and RadarStyle::parse distinguish those cases, but this getter does not use them. Likewise an invalid direct marker symbol is treated as merely unequal to none and can yield a precise line65. The approved contract requires invalid selected metadata to raise an error. Validate the selected finite lexical domain before deciding known type versus valid unsupported None, and add paired invalid/valid-unsupported regressions without requiring dense caches or a workbook.

### D2, Repeated global node scans make sparse metadata reads quadratic

`crates/oxml-chart/src/lib.rs:372`
`crates/oxml-chart/src/lib.rs:375`
`crates/oxml-chart/src/lib.rs:380`
`crates/oxml-chart/src/lib.rs:446`

Each direct series performs marker and explosion lookup through chart_optional_child_node. That helper scans the complete chart node inventory to detect direct matches and duplicates. A marker with a symbol adds another complete scan. Thus even a sparse chart with no caches performs at least two whole-inventory scans per series. Ten thousand small series and roughly forty thousand nodes already require about eight hundred million predicate visits, while the existing scanner permits up to one million nodes. The node cap does not bound this quadratic work to a practical input budget. Index direct children once or otherwise perform metadata and duplicate detection in a bounded traversal. Add a many-series control for the chosen traversal or explicit work budget, while preserving direct qualified ownership and duplicate errors. No full timing benchmark is required to establish the algorithmic issue.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Contract and correctness outside D1/D2: no additional finding. The four required exact enum cases and seven authored forms follow qualified direct metadata. The physical chart-source resolver retains unique frame, relationship and content-type checks while separating workbook mutation requirements. The getter is read-only and does not publish revisions.
- Panics: no finding. The new source-span reads use scanner-owned spans. Root indexing follows short-circuit root admission. Production additions introduce no unchecked unwrap or expect on caller input.
- OOXML: no additional finding. Foreign or nested marker/explosion lookalikes do not supply selected direct metadata. Source and schema write paths remain unchanged. The visibility conjunction matches the independent native truth table and leaves layout gating, placeholders, background and inheritance resolution separate.
- Tests: no additional finding beyond missing malformed-enum and work-boundary coverage identified in D1/D2. Existing controls include sparse metadata, grouped/retired owners, repeated byte-preserving reads, explicit/absent flags and occupied/uninstantiated placeholders. The stub and strict typed consumer account for int or None.
- Structure: no finding. The private chart-source owner helper has two actual consumers and retains the stronger existing chart_owner behavior. No new module, dependency, trait, public data field or speculative authoring API appears.

## Evidence inspected and limits

The native receipt hash matches 409537c130c69e1a914b2e4ae6df754f33735d3f92b23fda51ecd71bc6135c40. The actual four-route receipt matches f4a211182c74810f092c0b3b6e88c20c847bae21e3d8a6e1bf6cec9e573d1f45 and all 67 retained inventory file hashes authenticate. Its Python PDF/PNG and CLI convert/render observations match the native four-flag vector. The retained old S94 renderer fails the false-slide/true-layout master region. The actual compiled over-suppression mutant fails the true-slide/false-layout layout region, with occupied colors surviving. The mutation closure records exact restoration of the frozen context source. These are concrete semantic sensitivity records, not a claim that all scoped riders have passed.

No build, test, install, cache or source mutation was performed by the reviewer. Remaining scoped riders, genuine publication, installed suite and typing/stub results must be completed by the implementing phase. Only this review file was written. Return to the orchestrator for a separate remediation phase and the next independent review pass.
