# F-X223, Qualify native Word embedded multi-face interoperability

**Status**: completed
**Sprint**: S95.11
**Size**: M
**Depends on**: F-X220

## Problem

`crates/rdocx-oxml/src/font_table.rs:17` already represents all four schema roles, and `crates/rdocx/src/document.rs:32680` validates licensed bounded font bytes before source-owned publication. Existing native observations establish genuine regular use with synthetic bold and possibly empty saved bold parts. They do not establish a writer defect or genuine four-face consumption. F-X220 is done in BACKLOG. No existing F-X223 plan, progress or review was found.

## Spec reference

- HLD14, F-X223, exact prepared face proof and the documented native limit.
- HLD04, Embedded font package transactions, ODTTF source graph, schema slots and native normalization distinction.
- HLD08, Embedded face selection across exports and Native Office font consumption, genuine selection versus synthesis and fallback.
- HLD10, Licensed embedded fonts in Python and CLI, unchanged authorization and four-kind API contract.
- HLD12, Embedded font acceptance, actual mapped glyphs, advances, native save and normal reopen.
- HLD15, Embedded font inputs and consumer recipes, prepared licensed inputs, absent host installation and app/tool pins.
- Microsoft MS-OE376, Part4 section2.8 Fonts, https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/22da0d03-7202-4731-9518-b96bfbe4de61 , one embedded font per logical name implementation boundary. Rechecked16October2026.

## Approach

This is bounded interoperability investigation with a faithful correction only if supported by evidence. Preserve the existing four API kinds, licensed authorization, GUID obfuscation, graph ownership, source preservation and deterministic caller-first/embedded-second renderer policy. No new public surface or font assets are required.

Prepare a fresh licensed uninstalled family with authentic regular, bold, italic and bold-italic bodies whose unchanged outlines and metrics distinguish each face. Authenticate preserved source, recipe and exact license rather than following removed worker paths. Add the still-missing discriminator: real bold bytes solely in embedBold with no embedRegular, selected by an ordinary bold run. Distinguish this from the historical bold-body-in-regular-slot control. Compare paired and four-role controls with fresh keys and name identities. Final native comparison inspects actual mapped font outlines and advances, text paint operations, source graphs and native normalization before save, after save and after normal reopened disk copy.

No product change is justified by a synthetic-bold PDF alone. If a concrete source representation defect is identified, revise this plan to name the minimal correction in existing document/font-table carriers before editing it. Retain every original negative result. Do not rename families, rewrite run styles, suppress warnings or equate synthesis with genuine face selection. If the documented one-font-per-name limit holds for all admitted controls, preserve the public/renderer contracts and report the exact native limit. A negative finding does not complete genuine multi-face support without an explicit acceptance disposition.

## Rejected alternatives

- Renaming each face into a different logical family changes document semantics and is not approved.
- A heavier pixel or PDF resource name does not prove the actual embedded bold/italic body.
- Repeating the existing regular-slot bold control does not test embedBold role consumption.
- Patching font-table serialization before demonstrating a writer defect risks the already qualified graph.

## Test plan

**Test gate**: differential.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | Existing font-table/native regression carriers, exact four-role serialization and source graph controls | Authentic sfnt/ODTTF bytes, unique role relationships, schema order, shared reachability, no-op and refusal preserve source and authority |
| local scoped | regression | Any evidence-earned repair gets a named actual old-source RED/fixed-source GREEN in the existing carrier | Demonstrated source bug rather than assumed external application behavior. No redundant test is added if no source change is justified |
| final sprint | differential | Native Word proper embedBold-only discriminator, paired and four-authentic-face controls with fresh names/keys | Genuine exact glyph outlines/advances from prepared faces before native save, after save and normal reopened copy, actual app/build and absent host installation |
| final sprint | differential | Missing font, disabled embedding, raw stored format and changed-face controls | Sensitive fallback/synthetic distinction, actual warning/normalization, no relabelled historical producer |
| final sprint | golden | Deterministic own renderer, caller override and four-face selection controls | Genuine selected bodies remain correct across exports, bundled32 fonts and licenses unchanged |

All prepared faces require authenticated original license and unchanged outline/metric/style lineage. Actual saved-part identities and font graphs are separate from native rendered use. Historical11receipts/16artifacts and seven preserved prepared fonts are inputs, never fresh acceptance. Final complete installed suites and actual package verification are required only if a product/binding change earns them, within the common sprint union.

## HLD impact

- docs/hld/04-opc-and-packaging.md
- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

Describe implemented contract and the exact remaining native qualification honestly before final freeze. Correct current scheduling references, preserving old append-only history. HLD03/15 become impact files only if an actual revised design changes architecture or preparation/toolchain intent.

## Risk routing

Parser/serializer: read HLD04 checked source ownership and HLD06 preservation. Final checks retain schema order, namespace scope, exact opaque branches and no-op/refusal byte identity. Layout/font changes: HLD08 and HLD15 deterministic font mode, unchanged bundled inputs and actual native app/build identity. Published API and binding changes, if earned: HLD10, actual affected verified archives below10MiB, both WASM graphs, fresh complete installed family suites and strict typing/stubtest in the final sprint union. External oracles: canonical differential-testing skill, pinned Word and LibreOffice26.2.5.2 with Poppler26.01.0. Native application PDFs are separate from deterministic product baselines. No new dependency, module, crate or tracked font input is planned.

## Hash harness

Expected unchanged for the49 hash and7 deterministic pixel cases. A discovered intentional paint delta must be separately labelled and independently reviewed before the sole baseline writer changes anything. Native outputs are never product baselines.

## Implementation checklist

- [x] Authenticate preserved prepared font lineage, exact licenses and source graph inputs.
- [x] Prepare proper-role single bold, paired and authentic four-face distinguishing controls.
- [x] Retain Base/local graph evidence and add focused regressions only for an actual source repair.
- [x] Preserve current package/API/renderer contracts and update HLD intent before review.
- [x] Obtain scoped local evidence and independent microscope, final native debt explicit.
- [x] Run actual native face-selection matrix once in the integrated sprint union, retaining unsupported observations separately.
- [x] Complete only after genuine face proof or a separately explicit native-limit disposition.

## Accepted native-limit disposition

On17October2026 the user chose to document the Word limitation after the
concrete carry-or-accept question. This accepts the bounded investigation
outcome. It does not claim genuine multi-face support or carry F-X223 to S96.

Word16.114 build26101123 uses the prepared Bold body in the proper Bold-only
control initially, after native save and after normal disk reopen. The paired
Italic control uses Regular outlines and advances with a shear transform in
all three phases. Native saving leaves its Italic font part empty. No source
representation defect or semantics-preserving writer correction was established.
The existing package/API/renderer contracts remain unchanged.

All27 actual native phase captures remain retained. Thirteen observations
completed, including those genuine-Bold and synthetic-Italic results. Fourteen
refused unsupported PDF stroke colour-space state, so they have no face verdict.
Missing-font and raw-storage controls matched no prepared face. This accepted
scope does not turn unsupported observations into passes.

Evidence: `/private/tmp/S95.11-F223-native-d65f2c06-1/observer4-matrix-triage.json`,
SHA256 `2778c91d6a885f52613ec99522ecad4fdf5202513d06874b7d87eec7005a017c`,
and the proper-Bold summary, SHA256
`78fb0fa53a2b6b2bde991f2819fd509daeb200689b572afc488e9c39cf784b44`.
These retain actual producer d65f2c068695ae53de0af9b1ca9fd4e88c1d1403.
Current source correspondence and the standard verification union are in
`/private/tmp/S95.11-final-evidence-applicability-audit-1/receipt.json`, SHA256
`9ab365b4dbd2d99fb291ff3d45570d7866595c543eff108800f1fe66bfb048e6`.
The former pending user decision is settled by this explicit disposition.

## Open questions

None. The documented native limitation is accepted. Family aliases and run
rewrites remain prohibited. Final completion records follow the integrated
verification and review workflow.
