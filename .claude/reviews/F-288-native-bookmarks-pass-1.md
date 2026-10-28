# F-288 native bookmarks, pass 1

**Reviewer:** Independent Codex reviewer `/root/review_source_pieces`.
**Reviewed:** Frozen working diff against `2c8dc516`, nine files, 501 insertions and four deletions. Source, tests, approved plan clarification and HLD 03/04/10/12 were inspected. No builds, tests or product edits were performed by this reviewer.
**Verdict:** One test defect, zero product defects established, zero smells, zero nitpicks. This bounded remediation is not ready for a clean microscope while D1 remains.

## Defects

### D1, the alias regression reparses part-scoped output as a standalone paragraph

`crates/rdocx-oxml/src/text.rs:20219`

The new alias control calls `CT_P::from_xml_fragment` on `serialized_paragraph` output. The serializer invokes the ordinary paragraph writer (`crates/rdocx-oxml/src/text.rs:17942`), whose generated `w:p` expects the containing Word part's canonical binding (`crates/rdocx-oxml/src/text.rs:10331`). Root namespace declarations are not retained merely for their own sake (`crates/rdocx-oxml/src/text.rs:465`). The standalone parser instead resolves the root from its explicit declarations and rejects this output with `MissingElement("w:p root")` (`crates/rdocx-oxml/src/text.rs:9775`). The recorded affected OXML batch actually fails here, with 23 passed and one failed.

Reopen the untouched serialized paragraph through a real `w:document` and `w:body` with the ordinary canonical Word binding. Do not inject a blanket `q` binding that could hide a lost producer declaration. Assert the actual expanded names of the aliased start and end, one physical pair, reopened bookmark axes, current value and reset default. This tests the part-scoped writer's real contract and still discriminates alias preservation. The present failure occurs before those reopened assertions, so they are not qualified evidence yet. No product namespace repair is inferred from this test-context failure.

## Source audit

The admission helper requires a valid legacy owner, a nonempty matching name, no nested fields, pure begin/end runs and an unambiguous same-ID pair at the exact adjacent boundaries (`crates/rdocx-oxml/src/text.rs:6167`). It excludes run-root namespace sidecars from physical child counting, while retaining checks for actual extra children, content and alternate drawings. The existing boundary policy still rejects every other interior bookmark and unrelated typed boundary (`crates/rdocx-oxml/src/text.rs:6080`).

The internal start is captured in the existing field source before its paragraph-level raw slot is removed (`crates/rdocx-oxml/src/text.rs:6095`). The external end remains outside that source. The special semantic start moves before the virtual field run and subtracts exactly the pure begin's one accepted and tracked contribution before the ordinary trailing-boundary remap (`crates/rdocx-oxml/src/text.rs:6138`). Checked nonzero counters precede those subtractions. The wrapper and deleted-prefix control checks direct `[1,2]`, accepted `[2,3]` and tracked `[3,4]` axes, physical marker counts and two changed save/reopen cycles (`crates/rdocx-oxml/src/text.rs:20087`). Nine negative shapes retain untyped source (`crates/rdocx-oxml/src/text.rs:20238`). No blanket interior-marker exemption was added.

Authored canonicalization still derives its remap only from authored marker IDs (`crates/rdocx/src/document.rs:21244`). A changed authored marker in a legacy-bearing paragraph selects actual whole XML serialization, the existing namespace-aware bookmark patch and reparse before updating the authored set (`crates/rdocx/src/document.rs:21263`). This covers the real rich merge publisher, where an imported `MailMerge1` precedes an existing authored `Tail` and requires IDs `(0,1)` (`crates/rdocx/tests/integration_test.rs:14533`). The assertion checks both issue-free bookmarks and their text through repeated reopen. Ordinary preserved native IDs remain excluded. The new parser helper itself is a pure eligibility query and does not publish partial changes.

The facade control separately checks current/default separation, bookmark text, REF cache update, one physical pair and reopen (`crates/rdocx/tests/integration_test.rs:14486`). The Python additions check all three kinds and held frozen snapshots through repeated setter/save/reopen (`crates/rdocx-py/tests/test_core.py:9278`). Their installed execution is pending.

## Evidence and limits

Consumed existing logs, without rerunning them:

- `/private/tmp/s954-native-named-fields-semantic-red.log` records the original parser semantic RED.
- `/private/tmp/s954-native-named-fields-composition-red-2.log` records the actual parser-only rich merge RED with nonidentity remap and duplicate bookmark ID. The earlier direct-import and empty-text fixture failures are not product REDs.
- `/private/tmp/s954-native-named-fields-integration-final.log` records the two facade controls passing.
- `/private/tmp/s954-native-named-fields-native-affected.log` records all 54 affected native integration cases passing.
- `/private/tmp/s954-native-named-fields-clippy.log` records the scoped three-crate check passing in 47.47 seconds.
- `/private/tmp/s954-native-named-fields-oxml-affected.log` records 23 passing cases and the D1 failure. It is not a green batch.

Reviewed source SHA-256: `text.rs` is `91e1df8363be1161b3e91d9ee02f3c6bd963a7b33e57209459e9c783bc5f61ea`, and facade `document.rs` is `ce6ce6a067da7bc8fc1d87b24ac719c0080d60a511e83033f7f6858d4882a5b3`.

The independent Word-created partial package establishes the original discrepancy but does not discharge the full A6 producer comparison, all three authoring round trips or the dense-form visual gate. The plan and HLD correctly preserve those limits. Installed Python controls, joint rebuilt wheel qualification and final sprint riders remain pending. No release or whole-sprint acceptance is granted.

## Not found

Correctness: zero product findings. Contract: zero findings in this bounded clarification. Panics: zero findings. OOXML preservation and schema order: zero product findings, with alias reopen evidence blocked by D1. Structure: zero findings. Smells: zero. Nitpicks: zero.
