# F-X220, model, pass 2

**Reviewed**: Remediation of pass 1 D1 and D2 and affected low-level model behavior on `work/f-x220-codex`, HEAD `e918fa29195153a6d6cbda179a269366033fe6f5`. The complete working diff has two source/test carriers, 1344 additions and 2 deletions, in `crates/rpptx-oxml/src/presentation.rs` and its existing `tests/integration.rs`.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass 1 D1 and D2 are resolved within the reviewed model phase.

## Boundary

AGENTS.md and canonical microscope rules were reread. CLAUDE.md, WORKFLOW.md, the approved F-X220 design and its cited HLD sections were read in the preceding pass and retain the same contract. The model covers source-preserving font-list projection, four face slots, namespace ownership, schema order, tri-state flags, reachability and safe edit refusal.

This result does not accept the complete F-X220 feature. Native EOT/sfnt decoding and authoring, Word mutations, both Python and CLI surfaces, renderer integration, fresh wheels, native Office font-use qualification, affected archives, final HLD updates, handoff and whole-feature verification remain outside this checkpoint and required by the approved full-story contract.

## Resolved findings

### D1, namespace transfer

`crates/rpptx-oxml/src/presentation.rs:335`

The candidate entry now carries missing or conflicting inherited namespace declarations onto its own start tag. It explicitly protects an absent source default namespace against a destination default binding. The complete element and attribute expanded-name sequence is compared before and after transfer, including opaque descendants. Destination context is resolved from the actual serialized root and list. Candidate preparation finishes before a missing list is created or any destination child is appended at `crates/rpptx-oxml/src/presentation.rs:1079`.

The distinguishing tests at `crates/rpptx-oxml/tests/integration.rs:6392` cover inherited aliases, default namespaces, foreign local rebindings, conflicting destination aliases/defaults, original opaque spellings, cloned and removed entries, same-context exact bytes, reopened face projection and qualified reference counts. The unsafe unbound-prefix control at `crates/rpptx-oxml/tests/integration.rs:6504` proves refusal without materializing a destination list. No remaining D1 finding.

### D2, relationship identity

`crates/rpptx-oxml/src/presentation.rs:237`

New IDs are validated before candidate edits and refuse whitespace rather than silently normalizing it. The face parser uses the exact XML-decoded attribute reader at `crates/rpptx-oxml/src/presentation.rs:666`, with invalid imported face identities retained as opaque source. Qualified reachability uses that same exact-value reader at `crates/rpptx-oxml/src/presentation.rs:772`.

The controls at `crates/rpptx-oxml/tests/integration.rs:6454` cover failed replacement and insertion with newline, carriage return, tab, interior and edge spaces. The source control at `crates/rpptx-oxml/tests/integration.rs:6485` distinguishes encoded newline, encoded tab and literal space counts, retains the original invalid entry exactly and refuses its mutation. No remaining D2 finding.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. Candidate preparation, inherited binding materialization and exact identity reads resolve the reviewed failures without partially publishing destination state.
- Contract: zero findings within this phase. Original producer bytes and unsupported entries remain preserved. Out-of-phase full-story requirements are neither waived nor counted as defects in this checkpoint.
- Panics: zero findings. The added internal invariant branches follow successful owned serialization or private entry construction. Namespace scope access is checked, and enum indexes remain bounded. No product runtime or hostile-input command was executed during review.
- OOXML: zero findings. The affected paths preserve actual namespace ownership, local shadows, opaque lexical payload and font-list/face schema position. Unsupported imported IDs remain exact source rather than being rewritten.
- Tests: zero findings. The four new controls distinguish the original transfer, authoring and reachability failures, cover reopened output and failed-mutation atomicity, and retain the existing producer and corpus controls.
- Structure: zero findings. Changes remain in the two existing carriers, with concrete types and local helpers. No new trait, generic, wrapper, crate, module, dependency or feature flag.

## Authenticated evidence

Evidence was inspected rather than rerun. No cargo, target access, CP, render or Office command was executed in this pass. Only this review file was written.

- Remediation checkpoint SHA-256: `5f396f6c4a7e22493f859489fc5bdf948792862d941b16ec4d9f777b27d56b30`.
- Final receipt SHA-256: `f3dad0e14209008a565f2420ebafe44258243234b6d64830348c5fb81744efdd`. Its authenticated source inventory SHA-256 is `f8cb6081408e6b2850a904e279af0534edc00d40edb2bb62e2aa123cb0845578`.
- Presentation source SHA-256: `3a96a07ecdbe12ae3854a36abd6bc3c10d438d53170edbdf78b6257761b8afe7`. Integration source SHA-256: `80d715961f548ec33264403f4433b4c1c34989f681d0a4ce0e2b3d1e8fac6a01`.
- All tracked source files match the authenticated inventory before and after the review write. HEAD remains exact. Physical index `9fb296d122bd2a99880625492750520bdf6c3aaefd633dbe429344b7d2dee883` and logical index `7716a0d1d333c910d853bb81f64ca51e2df0e5ce8e79b0dab78504e2b7790c1e` remain exact. Pass 1 remains unchanged at its checkpointed hash.
- All six final log hashes match their actual files. Retained logs show all-target/all-feature check, eleven targeted PASS controls, the complete 50-presentation structural corpus PASS control, all-target/all-feature Clippy, diff check and rustfmt check. Every row reports terminal exit 0, no timeout and safe owned-process cleanup. These are authenticated worker results, not newly executed review gates.
- All 50 original input hashes match the checkpoint, including the actual font-bearing producer. Earlier D1, D2 and D2-count failure receipts and all their log hashes authenticate, as do both intermediate remediation failures. They remain retained.
- The model checkpoint retains the original unchanged hash baseline and explicitly claims no new hash execution. This review makes no full-feature completion, fresh runtime, native font-use or package publication claim.
