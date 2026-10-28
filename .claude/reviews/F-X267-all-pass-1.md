# F-X267, all, pass 1

**Reviewed**: Frozen working diff on work/f-x267-grok at Base and HEAD 4998e398107e204c7cd81b938af2d3610e58b88b. 30 tracked files, 3959 insertions and 1027 deletions. Diff SHA-256 4a121982b977070e2fe055bdec2fb3ed1e100a62b4363879a33b1395ea5b43a9.
**Reviewer**: Codex root independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 2 defects, 3 smells, 0 nitpicks

## Defects

### D1, The unmeasured donor digest was copied

`crates/rpptx/tests/integration.rs:12394`

M21_CURRENT_MINIMAL_SOURCE_SHA256 changed from 411f028c4b51c6aac0a0e2ae11bc63592a6d0947ab1a5f0d15116d19bfe766bc to 4566e218cd5855162101dd6dec3101f68e90d3ae1323e5fe93cb63db00b0adc7. The latter is exactly the contributed PR416 value. The approved plan forbids copying this value and the ready receipt explicitly says the M21 digest has not been measured. Restore the original expectation for this local preparation. The intentional delta must be independently measured and reviewed in its own behaviour commit at the final gate. Do not run that final rider here or claim current measurement.

### D2, Real ZIP safety fallbacks escape the commit counter

`crates/oxml-opc/src/package.rs:338`
`crates/rpptx/src/lib.rs:3331`
`crates/rpptx/src/lib.rs:7112`
`crates/rpptx/src/embedded.rs:528`

The fourth take_commit_work slot promises actual ZIP work. saved_package_read_back and reopen delegate to OpcPackage::reread when a parsed baseline exists. reread may write and read a ZIP for normalization, malformed source or another unsafe case, but this branch does not update the PowerPoint counter. commit_embedded_candidate also writes a ZIP without a counter marker. The parsed-None test sees the separately instrumented branch and does not cover either omission.

Instrument the actual fallback writes, respecting the existing debug-comparison exclusion and avoiding duplicate counts. Add a discriminating unsafe-reread case with a parsed baseline, plus an actual embedded transaction control. Record the old instrumentation outcomes before repair. The successful fast-path zero must remain zero. No counter tuple change or constant-time promise for safety fallbacks.

## Smells

### S1, The public baseline preconditions remain incomplete

`crates/oxml-opc/src/package.rs:307`
`crates/oxml-opc/src/package.rs:443`

reread documents a package that was read, without explicitly requiring that read is still unmodified and read-equivalent. trusted_baseline records provenance and does not become false on public map edits. Therefore it cannot supply the omitted caller obligation. same_package_as similarly needs the approved normalized read or reread precondition rather than an unconditional saved-byte claim for arbitrary mutated packages. HLD 04 states the stronger rule, but the approved standalone public helper contract requires the rule at these methods. Document it explicitly. Do not invent general mutation tracking or weaken the supported caller contract.

### S2, The named local tests do not establish several promised acceptance observations

`crates/rpptx/src/lib.rs:34471`
`crates/rpptx/src/lib.rs:34514`
`crates/rpptx/src/lib.rs:34404`
`crates/rpptx/tests/integration.rs:32293`

untouched_slide_bytes_stay only inspects the existing backing package after a direct in-memory edit. It never saves or commits that edit, and its original slide-only version already passed Base. It cannot distinguish a writer that rewrites untouched producer bytes. Use a deliberately noncanonical but accepted producer slide and notes payload, edit another slide, then inspect the actual saved ZIP and reopen.

parsed_baseline_tracks_real_edits covers a shape position plus the checked core-properties setter. It does not establish the plan's direct core_properties_mut, notes and handout header/footer mutations, default_run_properties_mut, or a shared layout with two slide contexts. Map the listed publishers to concrete local assertions through a later package edit and current, fresh, saved and reopened observations. Reuse applicable existing tests only with exact observations and source identities. A cloned deck is not the promised independently built in-memory equivalent.

The fit test covers unrelated breakage and the global report, but the required relevant-context positive and refusal controls for a shared layout or master and notes remain unproven by the local receipt. The signature wrapper exercises an existing opaque root package-signature case, not the changed embedded transaction path. Add or consume the bounded affected controls, not a full suite or external oracle. No runtime failure for these omitted combinations is claimed in this review.

### S3, New test aliases only forward and duplicate executions

`crates/oxml-opc/src/package.rs:2874`
`crates/rpptx/src/lib.rs:34395`
`crates/rpptx/src/lib.rs:34442`
`crates/rpptx/tests/integration.rs:32293`

These new test functions invoke another test without adding an assertion or input. Full collection executes the same case twice. This violates the repository's no-forwarding-wrapper rule and obscures unique evidence. Rename the newly adopted actual tests to the planned names, or record an existing test's actual name in the plan and evidence when it already supplies the required control. Do not duplicate an existing case merely to satisfy a label.

## Nitpicks

None.

## Not found

- Shared-byte migration: get_part remains borrowed, both Vec and Arc set_part instantiations exist, and downstream changes preserve byte semantics. No additional public signature break found.
- Native ownership: Arc allocation-origin queries are conservative. Parsed holds an immutable internal package baseline. Direct native mutable access uses copy-on-write or dirty-state publication. No additional established stale-data defect.
- Binding lifetimes: part-local inventories retain explicit proof checks and prepared revision publication. Structural ambiguity checks remain for affected structural operations. No additional established generation revival or partial publication defect.
- OOXML and panics: no new established namespace, sequence, unchecked input indexing or opaque-byte defect in the examined changed paths.
- Structure: existing generic helpers have actual multiple callback or data instantiations. No new source file, module, trait or feature flag.
- Evidence: original fit semantic RED is distinguished from stack overflow and the passing producer compatibility control. The successful dev extension and real pytest outcome are distinct from the failed feature-selection build.

## Evidence consumed

/private/tmp/s95.4-fx267-local-ready.txt and the existing progress record identify selected OPC, native, package-signature and two Python controls. The authenticated dev abi3 extension SHA-256 is 1fe91d2dac0ae8fea4fed7bb97978b466d143e1f0bd0958c77a46c0700658c57, Python 3.12.13. Two selected Python cases passed, child 0, 13.56 seconds. Scoped clippy and formatting passed. These observations retain their actual limited coverage. No source test or metadata was modified by Root during review.

## Acceptance boundary

Preparation is blocked by the findings above. The trusted flag correctly routes a known hand-built baseline through ZIP, but its current equality test alone does not measure that work. A1 catalogue execution, A2 and A3 release timing, signed and embedded fallback timings, the independent M21 measurement and Word 49 hash result, full union, installed typing, native oracles and archives remain pending. This review ends here and returns to orchestration before any remediation.
