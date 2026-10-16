# F-X241, all, pass 1

**Reviewed**: Frozen working diff in `/private/tmp/rdocx-s94-x241` against Base `105860260bd778faf73aca0019d34050ba1c69e5`, 15 files, 1,599 insertions and 45 deletions. Diff SHA-256 `63cfae76b2b23d133b982f7a80fb9b25b0fa0209a03f911328311ca9fd98e209`. All 3,517 tracked file hashes match `/private/tmp/F-X241-final-source-freeze/tracked-source.json`, snapshot `d9240f086596ae56e35822ee7426e0d2d3e984479f6526810e9a4a7e12900d4d`, before and after inspection. Read the approved clarified design, progress, repository guidance, workflow/microscope and cited ownership, preservation, binding, test and packaging contracts. Inspected the complete production, tests, exports, stubs, HLD and archive-carrier diff. No builds, cache changes, GUI actions, source edits or remediation.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Microscope exit condition met. This is feature review, not integrated sprint verification or public signing acceptance.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: 0 findings. `crates/rdocx-py/src/document.rs:6645` validates the entire bind dictionary before staging and reuses native tag precedence and ignored unmatched keys. Legacy inventory at `:6663` produces frozen owned snapshots. The setter at `:6676` resolves the current unique name or exact part/ordinal pair, refuses partial/stale/ambiguous identities and boolean ordinals, checks the selected native value kind, and allocates the actual reopened result before publication. Native constraints remain authoritative.
- **Contract**: 0 findings. Existing callable content-control snapshots and all-match setters remain unchanged. New runtime registration, package exports and strict stubs describe the same LegacyFormField class and methods. No public constructor or mutable snapshot authority is added. Native main-body/nested SDT scope, related-story legacy reach and existing date/drop-down display/bound-value semantics are preserved. The CLI keeps old --tag behavior distinct from strict --values selection.
- **Panics**: 0 findings. Finite JSON admission at `crates/rdocx-cli/src/commands.rs:2735` bounds input before mutation, rejects duplicate decoded keys, unsupported whitespace/scalars, trailing data and excess assignments. Untrusted target indices are derived from checked immutable inventories. No new production unwrap or unchecked untrusted index route was found. Owner traversal uses internally generated acyclic parent inventories.
- **OOXML**: 0 findings. Native form writers remain unchanged and authoritative for checkbox state/glyph/font, bound custom XML, typed legacy source patches, schema order and preserved producer metadata. CLI `fill_values` at `crates/rdocx-cli/src/commands.rs:2912` stages SDT edits first, checks the full pre-edit legacy inventory before legacy writes, then allows only planned value substitutions. Selected SDTs are re-resolved through their unique selectors and frozen metadata before mutation, after the phase and after final reopen. Actual nested SDT retention is admitted in both input orders. Final reopen precedes existing atomic output publication.
- **Tests**: 0 findings. Reviewed meaningful old-surface and untouched-owner RED evidence, actual five-kind expanded XML/reopen controls, typed legacy kinds, qualified unnamed and duplicate-name selection, refusal/no-op bytes and held owners, related-part edits, bound-XML late failure, changed-table descendant retirement and retired-handle non-revival. CLI tests at `crates/rdocx-cli/tests/integration.rs:6652`, `:6716`, `:6778`, `:6813` and `:6870` cover output/input atomicity, JSON/selector collisions, nested controls, shifted legacy ordinals and late native failure. Semantic observer negatives independently lose checkbox state or disagree with legacy default/cache while retaining superficially plausible text.
- **Structure**: 0 findings. New private source keys and certificates solve the two current form adapters' publication requirement in the existing binding carrier. No product file, module, trait, public model field, dependency or feature flag is introduced. Existing conservative publishers retain their behavior. Tests use existing entrypoints. Only the affected CLI archive row/date and matching README measurement change.

## Owner certification assessment

The earlier consultation risks are addressed in the frozen implementation. `word_owner_locations` at `crates/rdocx-py/src/document.rs:4035` collects form source proof from the same native batched inventory as its owner addresses. Paragraph/table source maps mark duplicate routes ambiguous rather than retaining a first entry. Raw cell-route and cell-story multiplicity are captured before the existing collapsed parent lookup.

`form_owner_certificates` at `:4257` requires one exact old and candidate route, agreement with the existing indexed matcher, unchanged owning source and the exact certified old/new ancestor pair. The matcher alone is not treated as uniqueness proof. Cell certificates require unique physical table/row/cell/story identity and a certified unchanged table ancestor. Changed tables therefore retire the whole descendant branch even if a descendant paragraph itself is unchanged. TextBox/Comment roots, runs and hyperlinks remain conservative. Existing sections use checked section-properties XML at the same ordinal.

`publish_form_candidate` at `:5568` flushes both serialized states before collecting proof and returns on exact source equality before token retirement. Both new methods allocate returned Python objects before prepared publication. Actual mutations retire global run/snapshot epochs while preserving only certified current owner states. Already retired handles are not reintroduced. The certificate regression at `:11529` deliberately supplies an ambiguous duplicate route and confirms refusal despite the indexed matcher's first result. No general structural-survival guarantee is inferred beyond the unchanged native scalar form providers.

## Actual evidence and limits

Independently authenticated every indexed receipt in `/private/tmp/F-X241-review-evidence/index.json`. Checked actual final successful command-log hashes for the complete installed835-test Word suite, strict mypy/stubtest, binding-host and CLI coverage, clippy, rustdoc, hash49, format, prose, adapters and policy182. The negative mypy command intentionally exits1 with four admission errors. Both WASM consumers pass. Earlier owner RED, fixture errors, warning and release2 strict source-guard failure remain retained rather than presented as final passes.

Using `/private/tmp/F-X241-final-authentication-1/receipt.json`, independently authenticated all22 retained archive hashes and actual normalized measurements with the existing read-only archive helper. Exact member totals/counts, unchanged64-byte compression tolerance and10MiB ceilings hold. CLI actual104,540 compressed bytes differs from recorded104,539 by one byte, with483,886 normalized member bytes and8 members. The genuine verified publication evidence remains distinct from measurement-only packaging.

Checked final wheel hash, wheel members and installed package/native/stub identities. Authenticated permanent baseline/current wheel and release CLI hashes/sizes. CLI grows99,232 bytes and wheel45,434 bytes in the matched ordinary release profile. No wheel or binary ceiling is invented. Independently inspected the actual saved release CLI package's expanded checkbox state, retained producer date metadata and legacy default, with unchanged input and the expected output hash. The broader observer and tracked semantic mutations cover the remaining five-kind details.

The native fill/sign receipt `/private/tmp/S94-F-X241-native-qualification-01/receipt.json` and all28 retained artifact hashes authenticate, and all14 relevant native source inputs match this worker. Fill then serialized sign is valid/complete. Signed then fill is invalid/incomplete on final current serialization and names the main part. The retained intermediate native view is explicitly different: SDT-only reports valid/complete and legacy staged reopen invalid/complete. This evidence qualifies existing native providers, not a repair of retained native verification and not F-X240's later installed public signing acceptance. Integrated cross-story and full sprint gates remain the orchestrator's obligations.

End of review. Return the tracked write lease to the worker and orchestrator. No remediation or confirmation review is initiated.
