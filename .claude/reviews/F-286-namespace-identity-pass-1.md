# F-286, namespace identity, pass 1

**Reviewed**: Frozen `sprint/s95.3` at HEAD `73a855df3e950b12373d8cda33f33b99505a6a74`. The uncommitted working diff is 7 files, 285 insertions and 14 deletions, or 299 changed lines. Against requested base `1aa81b3b9c3cbae8f51197744e7f8543e20ab659`, the complete delta includes the separate process commit and is 12 files, 307 insertions and 16 deletions, or 323 changed lines.
**Reviewer**: Codex, this actual independent reviewer session. This session did not implement the repair or delegate the review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this focused local namespace-identity review. The examined B2 serialization failure is repaired. This is not final sprint pass 6, integrated acceptance or release approval.

Read AGENTS.md, CLAUDE.md, `.claude/WORKFLOW.md`, the canonical microscope command, the complete F-286 design including Q26, final sprint pass 5, the prior repeating-context microscope and existing progress record. Confirmed branch, HEAD and working status. CURRENT_SPRINT and BACKLOG retain F-286 as done. The explicit post-completion amendment request and Q26 govern this focused review without resetting historical completion or granting new delivery credit. All eight historical delivery records and final reviews 1 through 5 remain intact. B1/B3/B4 retain their bounded accepted dispositions. No concrete applicable new evidence reopens them.

## Defects

None, 0.

## Smells

None, 0.

## Nitpicks

None, 0.

## Diff and contract examination

The seven working files are `.claude/plans/F-286-design.md`, `CHANGELOG.md`, `crates/rdocx-oxml/src/content_control.rs`, `crates/rdocx-py/tests/test_core.py`, `crates/rdocx/tests/integration_test.rs`, `docs/hld/03-architecture.md` and `docs/hld/10-bindings-spec.md`. The five additional base-relative files are the workflow, implement-feature and microscope commands, and their two generated skill adapters. Those process changes are separate from the product repair.

Working diff SHA-256 is `99ef8287252192f6e912a9c3501015d6ebdf56f621555ab1e0f1c3f8f97c4b5f`. Base-relative diff SHA-256 is `a665818d9ea28275a26c8cf9345113b960f01238bd7dbd1286194aa2bb669902`. Both identify `git diff --binary` output before this report was added. `git diff --check` exited 0.

At `crates/rdocx-oxml/src/content_control.rs:708` and `crates/rdocx-oxml/src/content_control.rs:730`, both Start and Empty constructors now retain the imported marker QName for every recognized TypeElement kind. This is the complete production behavior change. Attribute capture, raw child capture, namespace snapshots and property-slot ordering are unchanged. The nonempty path retains its matching source closing name and complete child bytes.

At `crates/rdocx-oxml/src/content_control.rs:850`, `write_type` uses the retained name only while the retained kind matches the selected kind. Absent or changed retained kinds still take the existing canonical fallback. The public payload authoring path in `crates/rdocx/src/content_control.rs:3311` continues declaring extension bindings. This repair does not synthesize a canonical binding over opaque descendants or expand type recognition by local name.

Traced namespace scope through document-root retention, CT_Sdt attributes, CT_SdtPr attributes and marker attributes. `word_prefixes_at` at `crates/rdocx-oxml/src/numbering.rs:603` replaces shadowed bindings in its local copy. `capture_attributes` at `crates/rdocx-oxml/src/content_control.rs:2223` retains declarations at their physical owners. Preserving the original marker name consequently preserves its expanded identity under those declarations, including a foreign binding on the canonical prefix. Captured namespace bindings also continue supplying the existing value-child readers. They are not newly injected into the writer.

Traced actual callers from imported story lookup through modeled staging and `flush_to_package`, then move/clone admission and staged publication. Nested move and clone at `crates/rdocx/src/content_control.rs:1647` and `crates/rdocx/src/content_control.rs:1709` inspect staged source XML before publication. `index_elements` at `crates/rdocx/src/content_control.rs:2435` resolves namespaces. `sdt_has_direct_type` at `crates/rdocx/src/content_control.rs:3631` requires the qualified marker identity. Retained aliases now reach those predicates with their original meaning. Direct-body insertion remains the supported boundary route and provides a distinct compatible control.

Examined existing date metadata writes at `crates/rdocx-oxml/src/content_control.rs:275`, their qualified attribute selection and fresh-prefix repair, plus checkbox child reads and writes at `crates/rdocx-oxml/src/content_control.rs:514`. Existing default Word namespace behavior remains bounded as before. No general imported namespace validator is introduced. The preexisting ordering assertion now expects retained `x:text` at `crates/rdocx-oxml/src/content_control.rs:2848`, while preserving the same ordering assertion. Its earlier failed aggregate remains historical, not a new semantic RED or a passing full-suite run.

## Tests and independent challenges

Read the actual assertions and inputs, rather than inferring coverage from pass counts.

- B2-N1: `crates/rdocx-oxml/src/content_control.rs:2416` directly serializes imported date, checkbox, section and item markers in Empty and Start forms. Its namespace-aware reader checks the bound URI, not merely lexical XML parseability. Canonical-prefix shadows, qualified custom attributes, opaque nonempty children and reopened type identity discriminate the original fault.
- B2-N2: `crates/rdocx-py/tests/test_core.py:6641` covers canonical and alias bindings at document root, outer control, properties and marker, plus alias with canonical `w15` bound foreign. Both nested move and clone into a paragraph must refuse with complete package-byte equality. The Rust integration addition at `crates/rdocx/tests/integration_test.rs:26642` independently checks alias/canonical refusal and atomic bytes.
- B2-N3: The Python test separately moves and clones a direct-body source to a supported body boundary. Expanded section/item identities, raw customXml metadata and exact move/fresh clone ids survive. Reopen followed by an alias edit exercises another modeled publication, then checks saved qualified markers, ids and metadata. Raw passthrough is explicitly only a control.
- B2-N4 local portion: Native and installed Python observations below exercise the consumer chain. Whole affected qualification remains outside this pass.

This reviewer freshly invoked the focused Python test directly from the current test file with Python `-B`, without pytest cache creation. Both Python 3.9 and 3.12 invocations exited 0. Each covers 18 incompatible move/clone refusals and 18 compatible relocations with subsequent metadata edits.

An additional 120 in-memory public alias-edit cases passed with child exit 0. They combine all 15 retained TypeElement kinds, Start/Empty forms and root/control/properties/marker declarations, with the corresponding canonical prefix locally bound to `urn:shadow`. Assertions check the exact expanded marker identity, qualified custom attribute, retained opaque child bytes for nonempty markers, and retained id/alias after reopen. Repeating-item probes retain an enclosing section. Repeating-section probes retain an item. These test namespace preservation, not general imported payload validity or authoring support for the five read-only kinds.

An additional existing-default-namespace date control passed metadata mutation and value replacement. Its unprefixed Word date marker and foreign local `w` binding retain the Word expanded type, and its new `fullDate` attribute resolves to the Word namespace. Source examination also confirms the existing checkbox source-name handling is preserved.

## Source, artifact and exit authentication

Independently recomputed the source hashes in `/tmp/s95.3-namespace-local-artifacts.json`. Current facade source is `cffcddc4d2043860e2ffd27914b6328177a3119473540deaa9ddb1eceacb00d7`. Current OXML source is `9906e0cfd8c5ff469b6aa5b8774ccec67fbac35a9a8e2c94f4f71bf94cc8fd80`. Both match the receipt. Its source note explicitly identifies working source over `1aa81b3b`, not a clean Git build.

Independently verified local wheel SHA-256 `79f784fe074d4edda80b797d68374b6f5d9d3ace072d20cac52340b0586a81bd`, its RECORD digests and member sizes, and embedded native SHA-256 `867b26f5bb74d0ef9a7a6533661d6529e9bbbc518d7eb123a8c6ca7690246260`. Fresh imports on Python 3.9.25 and 3.12.13 resolve to the dedicated installed environments and independently hash to that same native. Both authentication subprocesses exited 0. The retained wheel build log records actual compilation of the changed OXML crate and dependent Word crates, successful optimized completion and the matching wheel destination.

Consumed `/tmp/s95.3-namespace-matrix-39.log` and `/tmp/s95.3-namespace-matrix-312.log`. Each reports three passing matrices. The qualification driver names the namespace, repeating-context and customXml tests, authenticates installed native identity, and records both actual matrix child exits as 0 in `/tmp/s95.3-namespace-qualification.log`. The fresh focused invocations above independently confirm the namespace matrix, not all three retained matrices.

Consumed `/tmp/s95.3-namespace-type-native-red.log`. It records the date marker becoming `<w:date xmlns:w="urn:opaque">`, an expanded-identity assertion failure and Cargo test failure. Historical Cargo exit 101 is supplied in the task. This reviewer did not rebuild the old native or independently recover that historical process status from a separate receipt.

Consumed `/tmp/s95.3-namespace-type-python-red.log`, which records unexpected acceptance of the alias nested move and one failed test. Independently hashed the actual historical hosted native to `abe560ee91d5d52ee54b0bdf3ed4162aa8f64d00bc793b8e75b2d094ff726cde` and reran the current focused test against it. The fresh subprocess actually exited 1 with `DID NOT RAISE RdocxError`. This is a semantic RED, distinct from the earlier unsupported placement-keyword TypeError or unsupported positive-route diagnostics.

Consumed the selected Rust GREEN logs `/tmp/s95.3-namespace-oxml-green.log` and `/tmp/s95.3-namespace-facade-green.log`. Each records one selected test passing. This reviewer also executed those exact selected cases from the existing binaries, without Cargo or compilation. Both subprocesses actually exited 0. The observed OXML binary SHA-256 is `131340d552c7181f0993a8acab95869b8ac9e9b2d6d2e4130d9961b63629acc4`. The facade integration binary SHA-256 is `366f065b0d3fc70277f6819a1f7ebc2e26146ec47d16b790b3452654e4345ef1`. These are selected-case observations, not fresh full-suite or clean-build qualification.

## Examined paths, exclusions and limits

Examined all seven working-diff files and the base-relative process changes. Supporting source examination covered `crates/rdocx/src/content_control.rs`, namespace helpers in `crates/rdocx-oxml/src/numbering.rs`, and relevant root namespace retention/admission in `crates/rdocx-oxml/src/document.rs`. Consumed the named design, workflow, progress, sprint-status, prior-review and local artifact/log records. No source, test, plan, status or historical review was changed.

Correctness, namespace identity, raw child/attribute retention, schema-slot ordering, both parser event paths, canonical fallback, existing metadata/value behavior, supported positive controls and atomic negative controls produced zero findings within this amendment. No new trait, generic, dependency, module or public API is introduced.

General imported namespace/schema validation, unrelated payload semantics, rendering/native Word parity, and previously bounded B1/B3/B4 acceptance are excluded. No whole external issue is closed. Affected whole native/Word suites, priorities/typing, lint, both WASM configurations, hashes/pixels, README, genuine affected archive verification and current hosted Word qualification remain the final qualification work specified by Q26. Other files or logs appearing during that work receive no acceptance credit here. Unaffected evidence may be reused only with actual source/input correspondence.

This focused report consumes no final sprint pass. Q26 permits final pass 6 after affected qualification and pass 7 only if needed, preserving passes 1 through 5. No delivery ledger, story count, sprint state, release authorization or publication decision changes.

Only this report was written. No build, broad suite, scratch file, subagent, remediation, commit, push, main merge, tag or release action was performed. Review ends here.
