# F-X268, final performance, pass 13

**Reviewed**: Frozen working diff against clean `c71560d29d1969ce8079ee3b15dfd2303dddf1ae` on `sprint/s95.4`. One existing source file, 85 insertions and 36 deletions. Focused review of batched comment paragraph discovery, fallback error ordering and valid comment-companion fixtures under the approved F-X268 contract and canonical microscope workflow.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. The defect is incomplete counted-work coverage with a remaining quadratic membership pass. No new document-correctness defect was established.

## Defects

### D4, the touched marker consumer retains uncounted quadratic lookup work

`crates/rdocx/src/document.rs:26834`
`crates/rdocx/src/document.rs:26856`

After the new batched paragraph discovery, `story_range_marker_spans` still scans the source with `read_resolved_event_into` without recording actual read attempts or consumed bytes. It then calls `paragraphs.iter().any` for every Start or Empty event, before checking whether the event is even a relevant marker. Those examined paragraph candidates are also uninstrumented.

On the valid growing fixture, every later paragraph's events search increasingly long prefixes of the paragraph vector. The sum is quadratic in the number of paragraphs. Events outside the selected owner can also scan the entire vector. Replacing repeated paragraph discovery therefore does not remove this other quadratic lookup pass, and the required per-pass count gate cannot expose it because neither these candidates nor this reader are counted. A newly passing aggregate test would not establish A9 coverage for the repaired comment path.

Instrument the actual marker reader, including failed attempts, and every actual paragraph-membership candidate before qualification. Then either keep a monotonic physical-range lookup or another proven bounded search, preserving behavior for overlapping paragraph spans from direct and controlled projections, or report the real bound failure. A cache-off or old-path control must compare real counted work. Do not infer linearity from the newly instrumented rebase loop while this pass remains invisible.

This finding is established by source inspection. The reviewer did not run a new sensitivity test or claim a new runtime failure for this source.

## Smells

None.

## Nitpicks

None.

## Other aspects checked

The new qualified path at `crates/rdocx/src/document.rs:26529` uses the existing shared paragraph inventory. Its paths retain the old direct `[item]` and controlled `[item, paragraph]` axes, and its excerpt scanner rebases physical offsets back into the original source. The shared scanner preserves namespace-qualified control selection, inherited namespace closure, opaque descendants and control context. No alias, default-namespace, shadowed-prefix, inline-control or nested-owner correctness defect was established in these unchanged helper mechanisms.

Both new consumers use the qualified result only after the helper succeeds completely. On failure they execute the old scanner, rather than returning the optional qualification error. In particular, the rebase fallback preserves its original incremental ambiguity check and later scanner error order. The marker fallback retains its original collection order. Selected expected-offset matching, marker identity and ordinal matching, comment reference restoration and run-index adjustment remain unchanged.

The fixture now assigns each comment paragraph an eight-digit nonzero `w14:paraId` and creates the matching `w15:commentEx` row at `crates/rdocx/src/document.rs:49540`. It replaces the existing seed companion part through its preserved relationship. This repairs the mismatched seed metadata without changing the public move operation or work bounds. Reordering comment movement first retains all seven variants.

No new findings in panics, OOXML preservation, public API scope or structure. No new cache field, module or source file was introduced.

## Evidence and limits

`/private/tmp/s954-a9-native-count-valid-companions.log` records an actual failed gate after 65.91 seconds. The reported control-paragraph and prefix-scope growth is not a pass. The new batched-path run is separate at `/private/tmp/s954-a9-native-count-batched-comment-paths.log`, with no completed result consumed by this review. Its eventual outcome must also be assessed against D4's coverage gap.

The v6 installed release belongs to clean `c71560d29`. Its reported 74/79 flat gates, 2/5 linear gates and 12/12 original issue 407 gates remain distinct results. No flat or linear acceptance is inferred. Separate debug controls, memory, native oracles and final integrated gates remain required.

Exact reviewed `crates/rdocx/src/document.rs` SHA-256: `00f867aa47978aeef7fe667c8f47e278358a99dd435c787130a2287c3f18697f`.

The reviewer ran no builds or tests and changed no source. Return to implementation for remediation.
