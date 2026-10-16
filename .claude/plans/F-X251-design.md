# F-X251, Refuse invalid Word values before publication

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X201, F-X220, F-X238

## Problem

Issue385's five tests at pinned test_docx_everyday.py:270-311 expose silent invalid calls. Current Python save, tab insertion, run break and language setters accept them. The authored font directory validator does not require a name table. Word font CLI preflight also recommends a nonexistent force option. Concrete providers are document.rs:6701, formatting.rs:1053 and :471, run.rs:336, native document.rs:38025, and CLI commands.rs:3464/:3497/:3537.

## Spec reference

- docs/hld/14-development-backlog.md, F-X251. Test gate: regression.
- docs/hld/03-architecture.md, Owned publication and causal paths.
- docs/hld/10-bindings-spec.md, Python Word formatting and checked list publication, Python Word document operations, and Licensed embedded fonts in Python and CLI.
- docs/hld/12-testing-strategy.md, Binding tests.

## Approach

Refuse a path whose final extension is case-insensitive .pdf with Python ValueError naming to_pdf, before any write. Keep existing document variants and binary streams, without a general extension whitelist. Compare a new Python tab position against current stored twips after the existing signed EMU conversion. Refuse duplicates before publication, preserving native try_add_tab_stop's intentional duplicate contract and imported duplicates.

Refuse a Python Page break whose actual physical owner is a header, including header-cell descendants. Preserve body Page and header Line. This does not establish a new footer or column policy. Admit language setter values only under a finite lexical rule: nonempty ASCII alphanumeric subtags separated by single hyphens, beginning with a letter. Preserve None clearing, en-US, de-DE, zh-Hans and x-example. Do not claim complete BCP47/IANA validation, change case, or alter separate East Asia/bidi properties.

Require the exact name table directory tag in the existing bounded authored sfnt validator before embedding/key derivation. Keep existing bounds, supported faces and licensed positive controls. Refuse through the existing RdocxError route without adding a font record or part. Do not normalize imported fonts or add an asset.

For Word font output preflight, compose existing identity/file-kind checks with truthful ensure_output_paths_available, then unchanged nonreplacing stage_and_publish. Using the existing identity helper with its force-capable switch must not authorize replacement. Cover every relevant document/font input and preserve alias, hardlink, symlink and existing-output refusal. Keep real force-capable commands' diagnostics unchanged.

These named invalid Python calls intentionally change from acceptance to refusal. Public Rust signatures/struct literals, unit truncation and valid streams remain unchanged. No new file, trait, generic, feature or dependency.

## Rejected alternatives

- A global filename or language registry policy exceeds the five cases.
- Changing native tab duplication breaks an explicit native contract.
- Searching font bytes for the word name does not validate the sfnt directory.
- Removing output identity checks to fix wording weakens safe publication.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| installed reporter | Five original tests from pinned test_docx_everyday.py:270-311 | Actual S94 RED and corrected exception family, unchanged bodies retained separately from strict xfail outcomes |
| atomicity | test_fx251_five_reporter_invalid_values_refuse_atomically | Bytes, held owners and existing/absent outputs remain unchanged on every refusal |
| compatibility | test_fx251_valid_paths_tabs_breaks_and_language_remain_supported | Streams, document variants, distinct stored positions, body Page/header Line and valid/None language remain usable |
| native font | fx251_authored_font_requires_name_table_before_key_or_part | Licensed valid font succeeds, renamed directory tag refuses before key/part creation |
| CLI | fx251_font_existing_output_diagnostic_has_no_unavailable_force | Existing output and source aliases refuse, no unavailable force hint, no staged residue |
| nonregression | Existing fx238 stream, fx239 unit, fx220 font graph and CLI refusal controls | Signed forms, licensed multi-face records and actual source/output protection survive |

Use existing Word Python parity/core tests, native in-source font tests and CLI integration.rs. Preserve original strict XFAIL/XPASS distinctions, pin the reporter commit and qualify saved values independently. Run scoped floor, actual installed typing/stub checks and affected genuine publication checks.

## HLD impact

- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Units: read HLD01 and deliberate truncation rules, preserve existing EMU/twip conversion and declare unchanged harness. Parser/serializer and fonts: read HLD04/HLD06/HLD15, retain directory bounds, licenses and opaque source. Public API: read HLD10, state the named Python refusal changes and unchanged Rust signatures, genuine affected dry runs and archive ceiling. Bindings/WASM: actual isolated installed suite, strict typing/stubtest and both WASM targets. Any differential control pins its oracle and labels intentional refusal differences from permissive python-docx.

## Hash harness

Expected unchanged across all 49 hashes and seven deterministic pixel buffers. No baseline authority or rounding correction.

## Implementation checklist

- [x] Establish exact five reporter and CLI diagnostic RED controls.
- [x] Add the five narrow pre-publication guards.
- [x] Compose truthful existing-output diagnostics with retained identity checks.
- [x] Qualify valid controls, refusal bytes/owners and signed stream nonregression.
- [x] Complete scoped riders, zero-finding microscope and handoff.

## Open questions

No material user decision. Finite path, language, font and stored-position rules are selected. Preserve the existing supported document/stream routes rather than expanding policy.
