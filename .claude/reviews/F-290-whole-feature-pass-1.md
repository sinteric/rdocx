# F-290, whole feature, pass 1

**Reviewed**: Codex Root independently reviewed Grok's uncommitted implementation on `work/f-290-grok`, HEAD `d1f42516eb3747442fe4304a6331eab946dc9a0e`. The frozen diff has 16 tracked files, 6795 insertions and 27 deletions. Its SHA-256 is `9dfb5742a133a89bea71761527e2278ff1d9744f4b9910af0ba4a34ddfb5cab9`. The generated extension is excluded.
**Contract**: The approved F-290 plan, its Rust and Python signatures, A1 through A7, the five HLD impacts, and the canonical microscope and HLD discipline.
**Verdict**: 6 defects, 2 smells, 0 nitpicks.

## Defects

### D1, graph reads do not account for all recognized metadata

`crates/rdocx/src/field.rs:4352`

The final missing-source check considers only the thirteen modeled settings. A qualified ODSO mapping or recipient reference without those values, a source or the library DocVar falls through to `Ok(None)`. A recipient reference is not even resolved in that case because classification is gated on another source at `crates/rdocx/src/field.rs:4255`. The contract reserves None for no recognized configuration, including opaque-only residue. Known mapping or recipient metadata with a missing source must return an error. Preparation currently calls such a package absent and removal can return false while known metadata remains.

An active package's read path also never resolves `header_id`, although the mutation preflight resolves it at `crates/rdocx/src/field.rs:4381`. A missing or wrongly typed header edge therefore escapes the read and preparation malformed-graph checks.

The publisher still uses a global relationship-type predicate at `crates/rdocx/src/field.rs:4453`. That predicate decides whether an embedded package can be created over modeled settings. An unrelated unused edge can change that decision without changing any qualified owned source reference. Qualified reference ownership must govern each consumer. Preserve the valid opaque-only and retained-header residue cases.

### D2, replacement does not prove the old embedded payload

`crates/rdocx/src/field.rs:4397`

The original-store preflight checks the GUID and properties schema but never validates the existing item through the table decoder. The setter can then overwrite its bytes at `crates/rdocx/src/field.rs:4078`, or an external assignment can prune it through `crates/rdocx/src/field.rs:19625`.

A valid properties part declaring the merge URI is insufficient proof of the item's qualified root and versioned table. For example, an item with that declaration and a foreign root, or a mergeData root carrying an unrepresented producer attribute, is an error on query but can be replaced or deleted by full assignment. This masks malformed ownership and can discard unmodelled XML. Validate the existing payload before replacement or pruning, preserving or atomically refusing unproved data. Retain successful canonical unshared replacement and bound-store copy-on-write.

### D3, alternate recipient identity is inferred from content type alone

`crates/rdocx/src/field.rs:19273`

The alternate branch returns Alternate without examining its bytes. A well-formed unrelated root under the Microsoft recipient content type is therefore classified like a supported recognition of the opaque wne/hash format. Preparation returns a blocked or unsupported diagnostic rather than the required malformed-graph error.

Prove the declared alternate root and namespace, and XML integrity, before that classification. Keep legitimate opaque alternate data preserved and unsupported. Do not implement a hash conversion or a general Office validator. The approved recipient contract and [Microsoft's part definition](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/af3dc913-8c08-4843-ab40-495a92170b96) distinguish the root identities.

### D4, file URI validation confuses path with query and fragment

`crates/rdocx/src/field.rs:19089`

The file arm searches the whole remainder for a slash. A slash inside a query or fragment can stand in for the required absolute path. For example, `file://host?query=/data` is accepted although its path before the query is empty. Conversely, the explicit `path != "/"` test refuses `file:///`, whose path is absolute and nonempty under the approved rule.

Separate path from query and fragment before applying the bounded declaration checks. This is the existing literal metadata contract, not a request for a complete URI resolver. The Rust setter and Python external factory both use this helper.

### D5, HLD statements contradict the approved implementation

`docs/hld/04-opc-and-packaging.md:521`

The new paragraph says source and recipient relationships must both be internal and that an external mode refuses. The source relationship must be External, while recipientData is Internal.

At `docs/hld/03-architecture.md:625`, replacement is described as always allocating a new GUID, the one-part limit is scoped to ownership, and shared writes are described as always refused. Actual rules are reuse for identical data, in-place unshared replacement, copy-on-write for a bound or shared store, a package-wide recipient maximum, and refusal of a changed shared recipient part.

At `docs/hld/10-bindings-spec.md:2551`, Rust carriers are called immutable even though their public fields are mutable. The writer emits exactly thirty slots, including null padding, rather than merely at most thirty. Distinguish a missing owned source error from missing caller records on a valid external source.

At `docs/hld/12-testing-strategy.md:1101`, the text ties capability closure to the final union and mentions a Word session without distinguishing this story's selected round-trip gate. F-290 adds no own Word oracle. The union's F-288 and F-289 native riders remain mandatory under their contracts. Keep bounded capability readiness and final qualification in the appropriate records.

### D6, valid dynamicAddress on/off forms are lost or rejected

`crates/rdocx/src/field.rs:20869`

The field-map reader accepts only true, false, 1 and 0. It rejects the valid on and off spellings. It also stores a missing val as None, so a present empty dynamicAddress element becomes indistinguishable from an omitted element. Presence without val means true, while the Option snapshot must still preserve actual omission.

The [pinned SDK DynamicAddress definition](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.dynamicaddress?view=openxml-3.0.1) uses OnOffType and explicitly illustrates off. The recipient reader already admits the on/off spellings at `crates/rdocx/src/field.rs:19452`. Use a consistent concrete on/off reader and track element presence. Exercise absent, present-without-val, on, off and canonical booleans, while preserving imported XML until assignment.

## Smells

### S1, forwarding wrappers and an unused public helper add indirection

`crates/rdocx-py/src/document.rs:8195`

settings_from_native only clones a value. PyMailMergePackageOptions::from_native at `crates/rdocx-py/src/document.rs:8605` only constructs Self. Both violate the repository's forwarding-wrapper rule.

`crates/rdocx-oxml/src/settings.rs:2798` exposes RECIPIENT_DATA_ORDER without an actual consumer. The recipient writer instead hardcodes that order. Remove unnecessary surface or use the existing table in the actual writer. Do not invent a future consumer.

### S2, several promised acceptance discriminators are absent

`crates/rdocx/tests/integration_test.rs:33336`

A1 promises noncanonical settings and recipient part names, but this test authors and inspects the normal word paths. It does not distinguish a correct relationship-owner implementation from one hardcoded to those paths.

At `crates/rdocx-py/tests/test_core.py:13044`, assigning kind on carriers that have no such property does not prove their actual readonly fields are frozen. Test an existing property for each relevant carrier.

At `crates/rdocx-py/tests/test_core.py:13235`, changing textFile to spreadsheet does not exercise an imported data_type wider than the new authoring vocabulary. Use a genuinely unknown imported value and verify snapshot preservation plus new-assignment refusal. The raw ftp URI control is useful and should remain.

## Nitpicks

None.

## Evidence and limits

The review consumed the six native public groups, the codec group, the four Python groups, the focused typing command and the scoped lint evidence. Their assertions and relevant input paths were inspected. The archived Python extension is `/private/tmp/s95.4-f290-verified-_rdocx.abi3.so`, SHA-256 `6e6be2f74ec77c4f7c0b2227228f1a7cab1985db31fc5d5627e923eea5b60a95`.

That extension predates the final clippy-only source edits. The attempted post-clippy native invocation accepted no tests because Cargo rejected its arguments. The pre-review receipt records both limits honestly. Refresh affected source-bound observations during remediation before completion. A passing wrapper is not evidence that those tests ran.

The existing flat and rich merge engines are not changed by this diff. Public exports, callable factories, tuple defaults, native ownership staging, exact table budgeting, copied snapshots, no-op publication and the retained source/recipient distinctions were examined. No additional panic, dependency-direction or new module finding was identified. The two closure instantiations of splice_children are actual current consumers.

The full hash harness, installed full suites, strict package typing, stubtest, WASM, packaging and the sprint's native riders remain pending. No authentic Word pass is claimed. This is not a clean microscope or a completion record.
