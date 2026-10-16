# F-X240, Python and CLI signing and signature verification

**Status**: completed
**Sprint**: S94
**Size**: M
**Depends on**: none

## Problem

Native Word and PowerPoint signing and signature reports exist, but their ordinary Python and CLI dependency graphs do not enable digital-signatures. The missing consumers are crates/rdocx-py/Cargo.toml:34, crates/rpptx-py/Cargo.toml:33, crates/rdocx-cli/Cargo.toml:42 and crates/rpptx-cli/Cargo.toml:44. The ordinary musl CLI build explicitly uses no-default-features at .github/workflows/wheels.yml:401, so a default-only feature bridge would omit the promised capability there.

The shared signer accepts strict DER only at crates/oxml-opc/src/signature.rs:153. Word verify_signatures reads retained package state at crates/rdocx/src/document.rs:17249, whereas PowerPoint verifies staged current state at crates/rpptx/src/lib.rs:2446. A Python wrapper must verify current typed content rather than accidentally authenticating older package bytes. This is a testable prerequisite of the requested sign/edit/verify behavior, not a claim that a new failure has already been executed.

## Spec reference

- docs/hld/14-development-backlog.md, F-X240, Python and CLI signing and signature verification. Size M, no dependencies and regression gate are retained verbatim.
- docs/hld/04-opc-and-packaging.md, Package integrity, digital-signature discovery, authenticated coverage and staged RSA-SHA256 creation.
- docs/hld/10-bindings-spec.md, Python API shape, Packaging and prepared owner publication. Native signature reports and public tool parity.
- docs/hld/12-testing-strategy.md, Test taxonomy, digital-signature regression gate and Binding tests.
- docs/hld/15-build-and-toolchain.md, Feature flags, Packaging and Supply chain, including current default-off cryptographic edges and actual artifact measurements.
- docs/hld/03-architecture.md, Owned publication and causal paths, Three families, one workspace.
- Issue380 and mantissaman's accepted S94 comment6200485985, preserved under /private/tmp/S94-Issue380. DER and PEM are both accepted scope. This draft implements package signing, not PDF signing or certificate-chain trust.

## Approach

S94 is open from main3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190. This approved design claims no unexecuted evidence. Establish the exact feature Base at claim and retain actual source/producer identities.

### Approved ZIP supply-chain compatibility revision

The user approved the bounded ZIP9 migration after the actual current cargo-deny gate rejected yanked zip8.6.0. Use workspace zip9.0.3 with the existing disabled default features and deflate-flate2-zlib-rs selection, precise lock resolution and the fifteen original checked-name callsite adaptations, including four test getters identified by the actual compile check, and the existing ODT extra-data assertion adaptation. Do not add a dependency alias, wrapper error, codec, product file or module. Existing domain errors carry fallible decoded names. ODT keeps its independent raw UTF8-name precheck. OPC and ODP retain existing normalization, traversal, duplicate and expansion rules without a new strict raw-name policy.

The native method signatures remain, but OpcError::Zip exposes the upstream zip9 error type instead of zip8. This is an explicitly accepted pre-1.0 source-compatibility change, also propagated through native Word and PowerPoint error families. Python errors keep their existing mapped families. ZIP9 tries valid UTF8 independently of the flag and otherwise CP437. This can change decoded identities, lookup and alias collisions. It is not a promise to reject every invalid UTF8-flagged name.

Retained old8 native packages already use ordinary Deflate central-directory made-by version20, refuting the source-only prediction of a45-to20 output change. Retain actual old8 and new9 same-content packages and compare raw headers, ordered names, flags, timestamps, permissions, compression and inflated member bytes independently. Attribute any observed container delta separately from semantic or painted output. The49 hash and pixel baselines are expected unchanged and must not be rerecorded. Original ZIP8 wheels/native receipts remain predecessors. Fresh affected native, installed-wheel, CLI, archive and supply-chain qualifications are mandatory on ZIP9, including MSRV, WASM and no-default paths. Preserve all original RED artifacts and unchanged yanked-deny policy.

### Existing signer and one PEM normalization path

Keep RSA-SHA256 creation, graph validation, signature schema order and complete-coverage verification in the existing oxml-opc signer. Extend the private create_signature boundary in signature.rs:153 to normalize both inputs once before its existing DER parsers. The private PEM provider adds no module, product file, crate, trait, generic or feature. The separate CLI support report helper has the one explicitly justified default-off feature described below.

Use one concrete private helper in existing signature.rs:

```rust
fn signing_material_der(input: &[u8], expected_label: &str)
    -> std::result::Result<Vec<u8>, String>;
```

The two internal labels are PRIVATE KEY and CERTIFICATE. The helper does not inspect or accept a caller-selected arbitrary label. The existing key/certificate call sites map its errors to InvalidSigningKey or InvalidSigningCertificate without including material bytes. The four existing named tool consumers are rdocx-py, rpptx-py, rdocx-cli and rpptx-cli, all reaching this provider through their native facade and OpcPackage::sign. Do not copy a PEM parser into each tool.

Accept raw DER without trimming or alteration. For PEM, accept exactly one unencrypted PKCS#8 PRIVATE KEY block for the key and one X.509 CERTIFICATE block for the certificate. Allow ASCII whitespace outside the single block and ASCII whitespace within wrapped standard base64, including LF/CRLF. Require matching exact labels, strict padded base64 and no extra header fields or non-whitespace trailer. Bound each material input at1MiB before copying or decoding. Reject empty/truncated material, wrong labels, multiple blocks, encrypted/private-key legacy formats, malformed base64 and non-ASCII framing. Existing ASN.1/key checks retain strict DER, RSA algorithm admission and key/certificate match. Do not add passwords, PKCS#1 conversion, certificate chains, ambient trust or OpenSSL production execution.

Native sign signatures remain unchanged, with additive DER/PEM input acceptance documented in their existing docstrings. Existing DER callers retain behavior. Use existing base64 already enabled by digital-signatures. Signing alone adds no newly resolved package. The approved ZIP9 migration additionally resolves its required macOS/BSD nix0.31.3 edge with only fs/defaults-off and cfg_aliases0.2.2 build dependency. Preserve the actual target-specific graph and qualify their licenses/MSRV.

### Current-state verification without a new native verification method

Keep the existing public native Document::verify_signatures return/error signature, subject to the explicitly approved upstream ZIP error-type compatibility change. The Word Python and CLI verification adapter stages current state through existing public providers: clone_for_staging, serialize that clone with to_bytes, reopen those bytes with Document::from_bytes, then call native verify_signatures on the reopened current package. Original typed state and held Python owners are not published or mutated. This reuses the same canonical preparation/invalidation behavior as a real save at document.rs:17646 and:17786. It avoids coercing arbitrary facade preparation errors into a misleading OpcError or adding a new public verification method solely for a wrapper.

For PowerPoint, use the existing staged current-state verify_signatures at lib.rs:2446 directly. A real RED probe must first sign, make an unsaved typed Word edit and show the retained-package route's result compared with independently serialized current bytes. If the suspected stale-package behavior is refuted, retain that result and still test the current-state adapter. Never label source inspection as an executed RED.

Signing uses a native staged candidate in each binding. Build the complete returned Python report and prepare owner/revision publication before replacing live state. Reuse Word publication at document.rs:5272 and PowerPoint prepare_owners/publish_owners at presentation.rs:3392. Signing must preserve supported unchanged paragraph/slide/shape handles, and failures must preserve bytes and authority. Read-only verification does not refresh or retire owners.

### Signing publication refinement

Signing uses a distinct private Signing publication policy. It does not use FormValues, AppendBody or the generic conservative lane. Reuse the exact physical source and ancestor certificates for unchanged paragraph, table, cell and section owners. An unchanged run additionally requires its exact accepted run path and certified parent paragraph. An unchanged hyperlink requires the same unique physical paragraph source and local hyperlink index. Unproved owners retire. Use prepare_local_scopes to retain document authority for certified content. Retain story snapshot authority only when the complete ordered story identities and item snapshots are equal before and after staging. All report, inventory and revision preparation precedes token commit and the infallible native replacement. Other publication policies remain unchanged.

### Python API and report shape

Both public owners expose the same exact call shape:

```python
Document.sign(private_key: bytes, certificate: bytes) -> _SignatureReport
Document.verify_signatures() -> tuple[_SignatureReport, ...]
Presentation.sign(private_key: bytes, certificate: bytes) -> _SignatureReport
Presentation.verify_signatures() -> tuple[_SignatureReport, ...]
```

Inputs are byte strings containing DER or the admitted PEM envelopes. Paths and password arguments are not added. Add the following private type-check-only JSON-compatible shape descriptors to each existing family stub, importing TypedDict as _TypedDict. The leading underscore is part of each descriptor name. They are not runtime classes or importable public package exports. Return ordinary owned dicts at runtime, and document the dictionary fields rather than advertise `from rdocx import SignatureReport` or its PowerPoint counterpart. Report dictionaries contain no live handles and mutations of them cannot mutate a package.

```python
class _SignerCertificateIdentity(_TypedDict):
    subject: str
    issuer: str
    serial_number: str

class _CoveredRelationship(_TypedDict):
    source_part: str
    relationship_id: str

class _SignatureIssue(_TypedDict):
    kind: _Literal["missing_part", "changed_reference", "duplicate_reference", "missing_relationship", "duplicate_relationship", "external_relationship", "missing_relationship_target", "uncovered_part", "uncovered_relationship", "signature_value_mismatch", "orphan_signature_part"]
    details: dict[str, str]

class _SignatureReport(_TypedDict):
    signature_part: str
    signer: _SignerCertificateIdentity | None
    cryptographically_valid: bool
    coverage_complete: bool
    covered_parts: list[str]
    covered_relationships: list[_CoveredRelationship]
    issues: list[_SignatureIssue]
```

The issue kind is the exact snake_case native variant: missing_part, changed_reference, duplicate_reference, missing_relationship, duplicate_relationship, external_relationship, missing_relationship_target, uncovered_part, uncovered_relationship, signature_value_mismatch or orphan_signature_part. details contains every named string field from that native variant, including exact uri or part_name and digest values where present, and is empty only for the fieldless mismatch variant. Declare the finite kind Literal in the final stubs. Do not expose Debug strings as the structured contract. Existing native SignatureReport fields come from crates/oxml-opc/src/signature.rs:99, signer identity:39 and issue variants:54. Keep the identical private descriptors only in `_rdocx.pyi` and `_rpptx.pyi`, outside their `__all__`. Do not add these names to extension registration, either package `__init__.py` imports, or either package `__all__`. Existing extension export boundaries are crates/rdocx-py/python/rdocx/__init__.py:48/:128 and crates/rpptx-py/python/rpptx/__init__.py:40/:45. Existing stubs already use underscored static aliases at _rdocx.pyi:13 and _rpptx.pyi:38. This avoids phantom public runtime exports while preserving inferred exact dictionary types.

Unsigned Python packages return an empty tuple. Invalid/partial signatures return reports with false flags and precise issues rather than being converted into a successful empty tuple. Malformed unsupported signature XML or package errors retain native exceptions. Certificate identity and cryptographic validity do not assert chain trust.

### Shared concrete report transformations

Add one concrete `pub fn signature_report_to_py(py: Python<'_>, report: &oxml_opc::SignatureReport) -> PyResult<Py<PyDict>>` to existing `crates/oxml-py-support/src/lib.rs`. It builds the owned report, nested signer/relationship objects and exhaustive eleven-variant issue dictionary mapping. The two current consumers are crates/rdocx-py/Cargo.toml:32 and crates/rpptx-py/Cargo.toml:31. Both binding methods use it before prepared owner publication. Add `oxml-opc = { workspace = true, features = ["digital-signatures"] }` to the existing support dependencies at crates/oxml-py-support/Cargo.toml:20. This transforms data for two actual callers and adds no module, file, feature, generic, forwarding wrapper or trait. The support dependency graph and shipped artifacts join the existing feature/license/size riders. Native facade defaults remain unchanged.

Use one corresponding concrete `pub fn signature_report_json(report: &oxml_opc::SignatureReport) -> serde_json::Value` in existing `crates/oxml-cli-support/src/lib.rs`, gated with `#[cfg(feature = "digital-signatures")]`. Preserve the published support crate's existing default/base dependency graph. Add precisely this optional edge and default-off feature in its existing manifest:

```toml
[features]
default = []
digital-signatures = ["dep:oxml-opc", "oxml-opc/digital-signatures"]

[dependencies]
oxml-opc = { workspace = true, optional = true }
```

Its two consumers today are crates/rdocx-cli/Cargo.toml:47 and crates/rpptx-cli/Cargo.toml:41. Change both declarations to `oxml-cli-support = { workspace = true, features = ["digital-signatures"] }`. These two actual signature-report consumers justify the new support feature. This is a dependency selection, so CLI `--no-default-features` still includes the helper, matching .github/workflows/wheels.yml:401. There is no new wrapper feature, module or file. serde_json is already present at crates/oxml-cli-support/Cargo.toml:22. Keep family input staging, command status and publication in their existing CLI providers. The workspace OPC dependency is Cargo.toml:60 and its current defaults are empty at crates/oxml-opc/Cargo.toml:15. The support manifest currently has no features and only serde_json/thiserror at crates/oxml-cli-support/Cargo.toml:21/:22/:23. Standalone support default and no-default builds must retain the existing base graph, with no OPC/cryptographic dependency activation from this feature.

The unpublished Python support crate has exactly two current Python binding consumers and may keep its mandatory direct signature-enabled OPC edge. The resulting design has one mandatory Python-support edge and one optional, default-off CLI-support edge. This avoids four exhaustive issue matches while preserving the published helper's ordinary lightweight contract. No intermediate report crate, cross-dependency between Python and CLI support, serialization framework, generic or trait is justified. Keep the Python static shape declarations mirrored and exercise them from both family typing smoke files.

### CLI commands and exit contract

Add to both existing CLI command enums and dispatchers:

```text
rdocx sign INPUT --key KEY --cert CERT -o OUTPUT [--json]
rdocx verify-signatures INPUT [--json]
rpptx sign INPUT --key KEY --cert CERT -o OUTPUT [--json]
rpptx verify-signatures INPUT [--json]
```

KEY/CERT are bounded binary files admitting the same DER/PEM bytes. Reuse each CLI's existing input/output publication and distinct-output protections, including existing-file, alias, symlink/hard-link refusal. Sign only one candidate and publish once after the report is valid and complete. A failed read, normalization, key match, signing, serialization or write leaves input and existing destination intact.

Verification exit0 requires at least one report and every report cryptographically_valid and coverage_complete. Exit1 covers unsigned, cryptographically invalid, partial coverage, malformed/unsupported signatures and runtime I/O/parse failures. Clap usage/type errors retain exit2. Sign exits0 only after a valid complete signature is published, otherwise1, with ordinary usage2. Unsigned is explicitly reported as unsigned, never as valid. No network trust lookup occurs.

For --json, sign prints {"signature": report}. Verification prints {"status": "valid"|"invalid"|"partial"|"unsigned", "signatures": [reports]}. Invalid outranks partial when both occur. Default text reports signer identity, both validity/coverage flags and every issue with its exact part or relationship. No private-key/certificate material bytes appear in logs or diagnostics.

### Ordinary shipped feature wiring and measurements

Enable digital-signatures directly on the four wrapper facade dependencies' existing feature lists, and select the optional CLI-support report feature explicitly in both CLI dependency declarations, leaving native rdocx/rpptx and published CLI-support default feature sets unchanged. This makes ordinary wheels and CLI packages include signing even for the release musl --no-default-features path. Do not hide support behind a new wrapper feature that ordinary consumers do not select. Maturin's existing extension-module configuration continues to produce the ordinary abi3 wheels.

Before any feature edit, freeze matched-profile baseline wheel, installed module, release CLI executable/archive and current crates.io archives for both families. Record exact source SHA, toolchain, target, build flags, dependency closure, artifact size/hash and commands. Repeat the same profiles after the feature. Compare absolute bytes and deltas rather than mixing debug/release, local/hosted, stripped/unstripped or different targets. The crates.io10MiB ceiling and existing exact-member/64byte archive policy remain mandatory. There is no invented universal10MiB limit for wheels or CLI binaries. Any material unexpected artifact growth requires explanation in the design evidence, not a weakened existing bound.

Actual locked graph/licence/advisory checks must cover the activated consumers and supported release targets. Ring, SHA and X.509 already resolve, but newly shipped reachability still needs evidence. Local manifest licence observations are preparation, not a cargo-deny pass. Leave existing policy and native defaults intact. If real required external CI needs reviewed source, use the existing source-availability checkpoint rather than claiming unexecuted target builds or broadening publication authority.

## Rejected alternatives

- Four independent PEM decoders, which would permit divergent key admission and diagnostics.
- Enabling signing only through wrapper default features, which fails the ordinary musl no-default-features release path.
- Returning cryptographic success as certificate-chain trust or treating an unsigned package as a valid CLI result.
- Verifying Word's old retained package after a typed edit, or silently changing the native public error signature when a current-state wrapper can reuse existing save/reopen providers.
- Adding new product files, public proxy types, a production OpenSSL command or a general key-management subsystem.

## Test plan

The **test gate** is **regression**, as the backlog requires. Use existing source modules, both native integration entrypoints, both CLI integration entrypoints, existing Word/PowerPoint Python suites and typing smoke files. No new tests directory binary.

| Category | Test | Asserts |
|---|---|---|
| regression | fx240_both_python_families_sign_verify_and_name_tampered_parts | Fresh ordinary installed wheels sign DOCX/PPTX, save/reopen, verify full reports, then name a changed Word main part or selected PPT slide part |
| regression | fx240_private_report_shapes_match_actual_exports | Fresh installed-family strict mypy infers all report fields and issue-kind literals, and stubtest checks actual methods with no newly missing public exports or new ignore/allowlist entries. Runtime results are plain dicts, private descriptors remain unexported, and ordinary public import lists remain accurate |
| regression | fx240_shared_report_mappings_cover_every_issue_variant | Both Python consumers and both CLI consumers preserve all eleven issue variants and native payload fields, including fieldless mismatch, absent signer and multiple signatures |
| regression | fx240_both_clis_reject_invalid_partial_and_unsigned_signatures | Actual ordinary release CLI artifacts give exact JSON/text and0/1/2 exit profile for valid, invalid, incomplete, unsigned and usage cases |
| regression | fx240_pem_and_der_share_one_strict_signer | Equivalent DER/PEM produce valid complete signatures. Wrong labels, duplicate blocks, encrypted/PKCS#1 forms, invalid padding, size caps and mismatched certificate refuse without mutation |
| regression | fx240_word_verification_observes_unsaved_typed_edits | Actual sign/edit before-save RED is retained. Current-state adapter agrees with independently serialized/reopened package and names the changed part. Read-only verification preserves original bytes and handles |
| round-trip | fx240_sign_publication_preserves_held_owners_and_opaque_package_parts | Held Word/PPT owners survive supported signing, signed bytes remain valid after no-op save/reopen, unrelated opaque parts remain exact and late failures publish nothing |
| integration | fx240_cli_support_base_graph_stays_lightweight | In isolated selected-package default and no-default feature graphs/builds, published oxml-cli-support has no active OPC or crypto edge. Explicit support digital-signatures enables the helper. Both actual CLIs select it even with their no-default-features route. Avoid workspace all-features unification as evidence of the base graph |
| integration | fx240_ordinary_distribution_graphs_include_crypto | Fresh ordinary wheels and both release CLIs, including no-default-features musl route, execute signing without special test feature flags |
| regression | fx240_fill_then_sign_and_sign_then_fill_report_current_integrity | With F-X241 integration, filled values are fully covered when signed, later fills invalidate signatures and name changed parts |

Use the existing in-source test key/certificate constants at oxml-opc/src/signature.rs:1636/:1638 and source-built packages. Genuine partial-coverage controls must produce cryptographic validity distinct from incomplete coverage, not merely malformed XML. Real mutations remove coverage or change a selected signed part and must fail the same public observer. Do not replace an external producer or trust check with a self-recorded claim.

## HLD impact

- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/15-build-and-toolchain.md

### Existing publication and README carriers

Update existing `crates/oxml-cli-support/README.md` to state the report helper is optional and default-off, name its two current CLI consumers, and retain all existing non-signing behavior/examples. Its actual archive observation is currently the row at README.md:26 and the existing catalog/date carriers are `scripts/readme_doctests.py:373/:397`. The worker records feature-local genuine verified archive values/date under its exclusive numeric-catalog lease, and the integrator remeasures the combined result after integration, using unchanged exact-member, compression tolerance and 10MiB policy, without speculative size numbers. Existing `crates/oxml-py-support/README.md` should describe its concrete shared report converter, without implying that its unpublished crate is a new public distribution. HLD03:128/:134 must reflect the ordinary tool signing edges while keeping WASM/native defaults and unrelated agile-encryption claims precise. HLD15:302/:326/:1145 must distinguish default-off native/CLI-support features from explicit ordinary wrapper selections. No new README, module or product file is introduced.

## Risk routing

- ZIP supply-chain compatibility: HLD03/04/10/12/15. Qualify all fifteen original checked-name sites, an actual downstream zip9 error identity, UTF8/CP437 raw-name/flag fixtures, normalized aliases, limits, opaque data and ODT raw UTF8 refusal. Exercise ODT/ODP imports and ODT/EPUB exports, signatures/encryption, deterministic raw-header attribution, fresh affected installed/native/CLI artifacts, MSRV1.93, WASM and no-default graphs with unchanged codec and yank policy.
- Parser/serializer: Package integrity in HLD04 and Parts/schema sequencing in HLD06. Strict PEM admission and native XML signature ordering/preservation controls, exact opaque payload round trips, malformed-input atomicity and bounded bytes are required.
- Dependency graph and shipped feature activation: HLD03 and HLD15. Inspect actual locked target graphs, keep four named wrapper consumers, the two current support-feature consumers, and unchanged native/CLI-support defaults explicit. Isolated support default/no-default checks must prove the optional dependency remains absent, while explicit feature and both musl CLI dependency selections prove it present. Do not use all-workspace feature unification to claim a default-off graph, run cargo deny with unchanged policy, both ordinary WASM target checks and oxml-layout no-default-features tests.
- Published API: HLD10 and structural rules. State additive native PEM acceptance and owned binding report shape, run denied-warning rustdoc/Clippy, actual local-patched verified publication dry run and genuine affected archive inventories under unchanged size/member policy.
- PyO3: Fresh complete installed family suites with equipped oracle inputs, exact test membership, strict typing and stubtest. The actual shipped route is .github/workflows/wheels.yml:190/:191/:195/:197. Private descriptors must not leak into `__all__` or be advertised as runtime imports, and no new stubtest ignore/allowlist may hide a missing public symbol. Final workspace tests exclude both extension crates. Prepared Python allocation and owner publication, no-op and refusal semantics require real controls.
- External comparison/artifact builds: Pin actual compiler, target, signing fixture and any independent verifier. Capture matched-profile artifact before/after sizes and provenance. Any accepted external CI results bind exact reviewed source. No native GUI signature-trust or public CA trust claim is added.
- Release scripting: Any wheel/CLI build invocation edit needs HLD15/release-contract inspection and actual family preflight/policy tests. This F-ID performs no tag or publication.

## Gate environment clarification

The immutable historical rdocx-layout0.10.1 policy proves its checksum-authenticated published locked graph with unchanged original manifests and lock, registry-only normal dependencies and the original oxml-layout0.6.0 selection. Fresh unlocked consumer resolution is currently unavailable because every admissible zip8 release is yanked. This does not alter current workspace supply-chain policy, synthesize a consumer lock or claim historical fresh resolution succeeds. The actual unlocked failure and independent published-lock probe are retained in private evidence.

## Hash harness

Expected unchanged, all49entries and the shared pixel manifest. Signing and verification fixtures are source-built and do not change shared sample rendering. F-X240 has no baseline write lease. Any unrelated delta stops completion. Numeric package observations use the existing README/catalog carriers and belong to the integrator's exclusive metadata lease.

## Implementation checklist

- [x] Adopt after S94 opens, confirm pending status/current branch/Base, approve the all-sprint plan and claim the isolated worktree.
- [x] Freeze authenticated prefeature ordinary wheel/CLI graphs, sizes and artifacts under matched profiles.
- [x] Execute current public-surface RED and the Word current-state verification probe, retaining actual outcomes.
- [x] Add the one strict existing-provider DER/PEM path, four ordinary facade consumers, and default-off CLI-support report feature explicitly selected by its two actual CLI callers. Preserve isolated support default graph.
- [x] Implement owned Python reports/stubs, prepared sign publication and current-state verification.
- [x] Implement both CLI pairs, precise exit/JSON/text profiles and atomic output behavior.
- [x] Complete the explicitly approved ZIP9 compatibility migration and fresh affected runtime/wheel/archive qualification, retaining old8 source/artifacts and attributed raw-container differences.
- [x] Execute both-family installed/release CLI tamper/partial/unsigned/PEM controls and meaningful negatives.
- [x] Complete graph/licence/size, WASM, docs, archive and policy riders, independent microscope and scoped verification.
- [x] Execute the actual Unified family release workflow at the reviewed remote SHA: twelve installed wheel signing members, twelve selected CLI signing members including both no-default musl routes, two sdist members and the CLI-assets verifier. Preserve manual build-only publication guards. External CI remains pending until all required jobs actually pass.
- [x] Reconcile HLD and package observations through the integrator and prepare structured handoff without sprint ledgers or worker push.

## Open questions

No unresolved blocking product question remains. The orchestrator approves the concrete approach and risk riders in this plan under the authorized S94 scope. Native and external controls are future gates until actually executed.
Strict single unencrypted PKCS#8/X.509 PEM, bytes-only inputs, owned dictionaries with private typing descriptors, no chain-trust inference, unsigned CLI exit1 and staged current-state Word verification are approved. The shared CLI-support signing feature stays optional and default-off, selected by its two current CLI consumers.

## Approved sprint waves and exclusive resources

Wave1 runs F-X238 and F-X247 in separate worktrees. F-X238 owns Word parity tests and the existing binding naming/stub lanes, plus its PowerPoint XML-divergence pointer. F-X247 owns native math/conversion, the existing OfficeMath oracle, Word core math tests and the sole published-package numeric catalog lease in this wave. Neither edits the other's test entrypoint or source files.

Wave2 runs F-X239. Wave3 runs F-X241. Wave4 runs F-X240. These stories normally serialize because their actual edits share binding carriers and the source file scripts/readme_doctests.py for mandatory genuine archive measurements. If F-X238 is blocked, its fully checkpointed, frozen worker may explicitly release its source, test, HLD and cache leases so independent F-X239 can start from clean canonical source without inheriting or integrating that blocked implementation. F-X247 may continue on its disjoint math lanes, retaining the exclusive numeric-catalog/archive lease until explicitly released. F-X239 defers that shared stage and remains unprepared until every rider passes. F-X241 also waits for actual Word core/test lease release. F-X240 follows validated integrated form source for public fill/sign integration tests. A later F-X238 resume requires explicit lease return, reconciliation against the current approved consumers, a fresh microscope and affected union checks. Operational source leases add no speculative backlog dependency and do not waive any completion gate.

Each worker updates exactly its listed HLD impact under feature-specific semantic section leases before its prepared handoff. The integrator coordinates shared HLD sections and numeric catalog ownership. Shared package observations use genuine archives, and combined observations run again after integration. Workers never update sprint delivery totals. The integrator alone writes AS_BUILT and SPRINT_TRACKER. Keep complete installed environments and target/build caches isolated or exclusively leased. The hash baseline remains exclusive, with no approved new baseline move.
