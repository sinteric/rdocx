# F-X267, all, pass 2

**Reviewed**: Working diff on work/f-x267-grok at Base and HEAD 4998e398107e204c7cd81b938af2d3610e58b88b. 32 tracked files, 4265 insertions and 1036 deletions. Binary diff SHA-256 b5351bee02478403241f42596693adf831d91dcbf2cde9785c32f1bdf219a33c. Incremental remediation review consumes unchanged pass-1 source review.
**Reviewer**: Codex root independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None. D1 and D2 are closed. The M21 expectation is restored to the original 411f028c4b51c6aac0a0e2ae11bc63592a6d0947ab1a5f0d15116d19bfe766bc and the false receipt claim is corrected. It remains unmeasured. Both actual ZIP consumers now count their write attempts. The OPC debug hook is an additive counter refinement authorized by Root and documented in the plan, not a stable Python API. The callback reaches note_commit_work and respects the paused debug-comparison state. The embedded write is counted once and its following read is not counted. The two real old-instrumentation zero outcomes remain separate from stack overflow. Fast-path counter controls still pass.

## Smells

### S2, Acceptance observations still bypass the changed publishers and contexts

`crates/rpptx/src/lib.rs:34532`
`crates/rpptx/src/lib.rs:34539`
`crates/rpptx/src/lib.rs:34636`
`crates/rpptx/src/lib.rs:34737`
`crates/rpptx/src/lib.rs:34741`
`crates/rpptx/src/lib.rs:34474`

S2 remains with a narrower scope. The saved producer fixture is now stronger and checks both slide and notes bytes, but it still performs only a direct slide edit followed by to_bytes. The claim Base already used preserving serialization for to_bytes. The intentional changed route includes checked package commits that previously staged all modelled parts. Force one actual checked package commit, such as a changed core-property setter, between the unrelated edit and saved ZIP assertions. This distinguishes that route from direct-save preservation. Preserve honest provenance for the prior compatibility case.

The publisher test calls shapes_mut_at into an unused variable and makes no edit through that collection. An acquired mutable accessor alone does not establish a fact-changing publisher. Make an actual collection edit and assert its payload after the subsequent package operation.

The independently built deck is checked only against its own subject and start value. It ends at start 4 while the reopened case ends at 5 and includes additional edited facts. This is not the promised in-memory equivalent. Apply the same relevant publications to independently built and reopened inputs and compare the intended observable facts through the same consumers. This does not require identical whole package bytes or unrelated fixture reconstruction.

The fit addition correctly refuses a slide target whose own layout or master theme is broken. It still does not exercise a shared layout or master owner in two consuming slide contexts or a relevant notes target. Run and map suitable existing focused controls if they establish these exact observations, or add the bounded missing assertions. An equal layout index in two slides is not an observation that fit considered both contexts.

No additional runtime bug is claimed. This is the same incomplete-consumer-evidence invariant as pass 1. Escalate S2 to the actual Codex root design and independent review role before further repair. The four concrete observations above bound the remaining task. Do not add broad suites, external oracles, timing runs or another arbitrary test matrix.

## Nitpicks

None.

## Closed smells and not found

S1 is closed: standalone public documentation now requires an unmodified read-equivalent baseline and normalized read or reread comparisons, and explicitly distinguishes the trusted provenance flag from mutation tracking.

S3 is closed: pure forwarding test aliases were removed and existing signature evidence uses its actual test name.

Correctness, native and binding ownership, panics, OOXML and structure have no additional established finding in this remediation. The runtime changes are confined to debug instrumentation and documentation plus restoration of the test expectation. Previous unaffected scoped evidence retains its limited applicability.

## Evidence consumed

/private/tmp/s95.4-fx267-pass1-remediation-ready.txt records actual RED status 101 and zero ZIP counts for unnormalized_opaque_part_counts_reread_zip_fallback and embedded_replace_counts_commit_zip_write after their normalization and replacement assertions. Their fixed-source tests pass. The other affected selected native cases each execute one test and pass. Scoped oxml-opc and rpptx all-target/all-feature clippy and prose checks pass. Earlier Python evidence is explicitly reused for unchanged binding paths, not relabelled as an execution of the new fallback controls. No source or implementation documentation was modified by Root during this review.

## Acceptance boundary

Preparation remains blocked by S2. Final catalogue execution, release timings and fallback measurements, independent M21 measurement, hash, native oracles, installed typing, WASM and archive qualification remain pending. This review ends here and returns to orchestration before remediation.
