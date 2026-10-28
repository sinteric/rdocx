# F-X268, final performance remediation, pass 27

**Reviewed**: The frozen working diff against `f48e40448686ea4ab46ae6895f9832a333f999d0`, nine files, 395 insertions and 79 deletions. This is the bounded scanner batch and grouped active-plan/HLD evidence-intent reconciliation, not whole-sprint acceptance.

**Reviewer**: Codex, `/root/review_source_pieces`. I independently assessed the scanner design and identified allocation pitfalls before implementation. I did not author this scanner implementation, prototype, tests or canonical port. Earlier profiling infrastructure I prepared is supporting diagnostic evidence, not independent timing qualification.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Identity and scope

I read the canonical workflow and microscope command, approved F-X268 contract, current progress checkpoint and relevant HLD sections. F-X268 remains in progress. The branch is `sprint/s95.4`.

All nine current file hashes match `/private/tmp/s954-scanner-canonical-scoped-v4/freeze.json`, SHA-256 `788c2018fce2bf44c2d1706c8697f54e527b842975acc11a5f0b11d6b1cc978b`. The current nine-path diff exactly matches its `frozen.diff`. The sole product file is `crates/rdocx/src/document.rs`, SHA-256 `0cb8c6c84b36246b4075edee776de45a86267a884bf63bbc51ae0e36da35b301`. No review-time builds, tests, runtime probes or source changes were performed.

## Source assessment

- **Namespace values, errors and lifetime**: `crates/rdocx/src/document.rs:11927` retains every attribute iterator check, declaration prefix validation, namespace value decoding and ordered retain/append operation. Repeated identical declarations still perform that operation. Ordinary attribute values are not newly decoded. The ordinary element prefix stays borrowed until an actual missing prefix requires insertion. `Arc::make_mut` preserves parent and sibling values. The scanner-local frame stores a flat vector without a parent edge at `crates/rdocx/src/document.rs:11557`.
- **Complete consumer map**: I examined all eleven helper sites at `document.rs:15503`, `15733`, `15923`, `16902`, `17665`, `17777`, `17883`, `18141`, `18608`, `18973` and `19364`. Scope-derived roots retain the original values and order. Empty roots are traversal-local, not allocated per ordinary element. Ancestor movement uses `mem::replace` at `document.rs:17778`, avoiding an allocating default Arc. The alternate text-box empty frame shares its traversal root at `document.rs:15663`. The owned field-edit boundary explicitly copies vector values at `document.rs:19680`. No retained cache or cross-edit history root was added.
- **Read sharing and physical axes**: `crates/rdocx/src/document.rs:13478` borrows raw items and clones only selected direct spans. Section classification and item order remain unchanged. `document.rs:13544` performs a scan only for the Body owner, preserving the nonbody closing-reader path. Insertion shares the exact immutable pre-insert source at `document.rs:29402` and independently qualifies the reopened source at `document.rs:29487`. The extra `after=Some` validation scan remains at `document.rs:29418`. The inserted direct-slot check still precedes projected lookup at `document.rs:29498`. The returned index comes from raw projection, not the filtered direct list. The scoped replacement caller merely borrows its existing canonical inventory at `document.rs:44071`.
- **Private field schedule**: `crates/rdocx/src/document.rs:8973` conservatively detects literal `fldChar` and `fldSimple`, including opaque false positives. Actual simple or complex field admission necessarily contains those local-name bytes, regardless of namespace alias. The typed scanner can enter `CT_P::story_complex_field_sources` at `document.rs:17567`, and modeled wrapper parsing remains at `document.rs:15498`. Field-bearing sources therefore retain the original repeated-scan path independently before and after publication. No successful span comparison is promoted into permission to change private identity allocation order.
- **Panics, resources and XML publication**: No new untrusted indexing, unchecked arithmetic, parser admission, XML writer, schema ordering or public interface was introduced. The change reuses immutable observations inside the existing candidate transaction. Existing image validation, namespace closure, checked source publication and reopen refusal remain in place.

## Evidence assessment

I verified the frozen check, native-result, final policy and prototype receipt hashes, the fifteen selected native log hashes and final clippy log hash.

The actual canonical native receipt records fifteen unique passing tests, including the two new matrices at `crates/rdocx/src/document.rs:51618` and `document.rs:51794`. The prefix matrix compares exact legacy values and first errors, malformed attributes/entities, declaration ordering, default/alias/shadow/undeclaration cases, pointer sharing and sibling isolation. The physical matrix distinguishes a genuine cell owner, inline projected content, a later unequal direct/projected index, Body section placement and nonbody boundary behavior. Existing selected regressions cover namespace scope, controls, text boxes, bookmarks, links, field excerpts, picture refusal, AlternateContent pictures and canonical readback.

Those fifteen executions precede seven equivalent needless-borrow corrections and rustfmt. They are not represented as rerun on the final bytes. Final all-target/all-feature clippy compiled the final source successfully. The earlier all-target/all-feature check, final fmt, prose and diff checks are recorded separately. Preserved fixture/setup failures are not semantic RED evidence.

The temporary normal-cfg prototype receipt, SHA-256 `9251dc38d463b3f2fbce4b390db756e0184f85dc72ce196f86ff8b39574940c4`, records 48 public windows across ON, both-OFF, scan-OFF and prefix-OFF, exact output/returned-occurrence comparisons, fifteen extracted prefix controls and eight existing Python guards. Its real scan and copy counters distinguish both mechanisms. All sixteen observed source-ID gaps are one. They demonstrate equal observed gaps and scan schedules, not positive allocator activation. Static parser tracing plus conservative fallback establishes that boundary. No normal timing pass follows from this evidence.

## Plan and HLD assessment

The selected contract at `.claude/plans/F-X268-design.md:636` matches the implementation and preserves the unchanged timing gates. HLD architecture, package and testing intent describe ephemeral sharing without a new retained cache at `docs/hld/03-architecture.md:75`, `docs/hld/04-opc-and-packaging.md:1965` and `docs/hld/12-testing-strategy.md:4478`.

The grouped corrections preserve actual evidence attribution. `.claude/plans/F-288-design.md:14` binds the six-file setter result to ef048ffb. `.claude/plans/F-289-design.md:14` binds the complete eight-row native A7 comparison to c62e0a71 and retains current-artifact applicability. I inspected both cited actual receipts. Neither is relabelled as the final artifact. The F-288/F-290 impact lists include the edited existing backlog entries. `docs/hld/14-development-backlog.md:3139` describes implemented additive default queries without expanding the public metadata structs. Its F-290 entry describes stored/read metadata without executing connectors. Existing versions, shared-buffer migration, native criteria and final acceptance obligations remain.

## Not found and remaining scope

Correctness, contract, panics, OOXML, tests, structure and evidence attribution produced zero findings in this bounded diff. No trait, generic, module, source file, feature flag, public API or persistent cache was added.

The prior normal picture timing failures remain failures. This review does not qualify a new normal artifact, either picture timing gate, remaining Word performance gates, memory rider or final integrated union. Source-bound native evidence still requires exact final-artifact input/source applicability. No feature completion, sprint closure or publication approval is supplied here.
