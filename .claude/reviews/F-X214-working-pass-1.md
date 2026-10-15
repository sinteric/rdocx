# F-X214, whole working implementation, pass 1

**Reviewed**: immutable production6, eight existing carriers, 1445 added and 396 removed lines against claim Base639d354eeee67592bb32da6fd62cb88b7d900530. This is the first whole working source pass, not completed acceptance.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Bound input and contract

Capsule: `/private/tmp/rdocx-f-x214-codex/.claude/scratch/F-X214-start/immutable-working-1/receipt.json`, SHA256 `08350090d73333c20d7c1911fd3aa3133eb0ecf00a8e94c571f7a2a316906a23`.

Complete diff SHA256 `8f10e784938f02ad2cb94b9642d225a87f9204132fef41893a5d8777c1dffa63`. Full3178-record production6 map SHA256 `8ccb48da2134c0f4cfcc1d3581e9e73a0c174b33494fcaf4069527c140a4d2ee`. Base map SHA256 `437c9848643c8448d068334d40bd0a60ae3049654f861089e534e3da32deea86`.

All21 capsule records and24 authority references match exact size/hash. All eight preimages match git objects at the claim HEAD. Full before/after map comparison contains exactly the eight declared paths with no extra source/mode carrier. Approved plan copy hash519fc7a7f5edcc218ba7c783be358d7bcd45d0bdea14c2acd309d0446b262172 is authenticated. Canonical microscope rules and the plan's depth32, URI ownership, private preservation, ordering, authoring/renderer admission and risk/spec obligations were read. Applicable HLD04/06/08/10/12/14 intent remains controlling. HLD updates and final gates remain due, as expressly disclosed by the capsule.

Every source citation below refers to the corresponding complete `.after` copy in that immutable capsule. Unchanged namespace.rs is also authenticated against both maps, SHA256 `e2bc41b8fe99d22058b7d263a58f5180a36f278cfea63db1d6a8d4df28181724`.

## Defects

### D1, foreign timing payload can overwrite real condition metadata

`crates/rpptx-oxml/src/timing.rs:2606`, `:2613`, `:2626`, `:2677`, `:2692`, `:419`.

`collect_condition_target_presence` enters every element, including foreign opaque owners. It assigns a modeled owner ID whenever a descendant expands to p:cTn, without requiring the legal timing-node/cTn ancestry used by the frame parser. Its subsequent condition list inserts into the same `(node_id, end_condition, index)` map as a real parsed node.

Concrete input: a genuine par with cTn id1 and a first start condition containing an explicit target, followed by a foreign f:opaque under tnLst. Inside that opaque payload place qualified p:cTn id1 with stCondLst containing an empty p:cond. The native node parser retains f:opaque as Unsupported, yet the condition-presence collector enters it and overwrites `(1,false,0)` with false. `condition_has_explicit_target(1,false,0)` then reports false for the real node despite its explicit target. The opaque producer payload can also inject metadata for otherwise absent owner IDs.

The original collector was permissive too. That does not qualify the newly rewritten owner walk against the approved foreign/opaque boundary. URI qualification of the descendant alone does not make the foreign ancestry an admitted schema owner. Worker confirmed no upstream ownership guard or contract exception exists. Required remediation must restrict the metadata projection to the same legal recognized owner paths while retaining all opaque bytes, rather than dropping source or treating arbitrary nested qualified nodes as modeled.

### D2, opaque namespace nesting remains an unbounded quadratic heap/work path

`crates/rpptx-oxml/src/timing.rs:1554`, `:1633`, `:1642`, `:2602`, `:2607`, `:2632`, and unchanged dependency `crates/rpptx-oxml/src/namespace.rs:45`.

The new cycle check bounds recognized timing frames to32. It does not bound the independent condition-presence `open` stack, which descends through every foreign/opaque element and retains a complete NamespaceBindings at each level. `with_start` clones the existing prefix HashMap before adding declarations. A single foreign subtree with N nested foreign elements, each adding one unused unique xmlns prefix, therefore retains roughly 1+2+...+N prefix entries and performs the same quadratic cloning work. No recognized timing node depth increases, so the32 boundary is never reached. A modest source input can require quadratic heap storage before open returns.

The frame scanner additionally pushes qualified unknown descendants into its `elements` scope stack rather than capturing their opaque boundary once, exposing the same growing-prefix behavior for retained qualified extension payload. The current breadth event/immediate-byte controls do not count these namespace clones or adversarial opaque-depth scopes. A bounded ZIP/input size does not make this work linear in that size.

This violates the approved single-capture opaque boundary and deterministic linear-work contract. An explicit scope-safe opaque skip or another source-preserving bounded implementation is needed. Do not promote foreign payload or reject ordinary aliases to compensate. Worker confirmed there is no extra upstream guard beyond the recognized-frame ancestry predicates.

## Smells

None independently identified within the reviewed scope.

## Nitpicks

None.

## Aspects without additional findings

- Correctness: mixed group/AlternateContent depth propagation, root owner counting, empty-owner error paths, frame attachment and child order are consistent. Iterative lineage, geometry and lowering retain transform multiplication and source order.
- Contract: one additive depth constant and private source state, no new product file/module/crate/dependency/flag. Facade append/group admission is before tree publication, with local ID allocation and refusal byte controls. No new public fields or broken public struct-literal shape were found.
- Panics: private frame expect/unreachable uses inspected have owner-state invariants. Range boundaries are derived from the same reader/source. Recognized tree traversal and renderer admission refuse33 before clone/lowering. No additional reachable production panic was identified.
- OOXML: shared Arc plus private ranges retain compatibility source, private equality compares projected fields plus the same raw bytes. TimingUnsupported equality retains its local-name/raw contract. Group header comments/PI and ordered opaque siblings are retained. Foreign shape lookalikes remain opaque. D1/D2 are the remaining timing boundary defects.
- Tests: genuine Base and current narrow controls are present. Source-built32/33 and100000 hostile cycles, mixed groups/AC, alias/default/rebound/foreign source, ordered raw/comment retention, empty timing error, geometry and authoring refusal are meaningful. Deterministic counters supplement rather than replace release timing. D1/D2 are not covered by the current foreign/depth controls.
- Structure: existing carrier parser frames and private Arc/ranges reduce recursive capture rather than introduce a public registry, trait or generic framework. No second detached document/shape model is introduced. Retained Clippy failure is a gate limit below, not silently credited as success.

## Authenticated execution and limits

Actual Base receipt355216bfacd99475f1c4a6a35a721522a01f2186b4adba9ea5dfce93e0f0665b is bound to source9f1735d81bc7ef935eb85a7107b49efd480400962a8e0e33d865f08cdecf659c. Valid ordinary and unsupported-command controls pass. The four semantic reds are group/AC child aborts exit-6, past-limit timing assertion failure101, and empty-par panic101. These are actual distinct outcomes, not compiler or missing-input RED credit. Compiler JSON/binary preflight evidence and raw logs are retained. Original abort/failure evidence remains visible.

GREEN1 receiptd2568143dbc36e85fd47b03a9bca3f1a8188b811bd2855c58fce186e49849313 binds source4eba4dce57a77e0577313e00f1cd0308ae6667e8638affd426f3a217044c157d1, build0 and eight exact controls1PASS each. GREEN2 receiptc2bcba2b9ce3d76c5fa038ab17fdd8ec1fa3c12287da0164347901d681fec429 binds source5da18d03bbce4d4dca879f4838ff447a09004a87cdc2843dde9f5510e7d77c37a, four OXML unit controls and six facade integration controls PASS with zero ignore. Source5 timing/facade/integration carriers equal production6, while other changed carriers are explicitly not given fresh execution credit from that receipt. All raw logs, sourceAfterMatches and natural safe cleanup are authenticated.

Source6 GREEN3 receipt7af1748d781316ba3ce27816bf155e107ab3db05ed6c58d943cc1a629c27de90 failed101 because the new layout test called find_group_lineage without its parent-module qualifier at context.rs4913. This is retained compile setup failure, not behavior RED or successful manual admission.

A separately supplied immutable successor closes only that setup: source6-to7-test-qualification.diff SHA2510830865297149494520eeab1e788566753e2f86e145b13026d5cf6ae99677 and source7-layout-test-qualification.rs SHA59a2f2451b30c36ac48a277b84b81f6c9b5fb9b0ad5a63bd0c525eaf09c1aadd independently equal the exact6 copy with only `super::` added to that test call. Full map source7d6a14caca56ae239d01c1697ed614b0648549a5fc13cf84885db86fbd1ecc753 changes only that carrier. GREEN4 receipt5db28c6a625d98ea0bc880e38c2b04f6de8e87b16e26702e5443a1ad6cf51f86 records manual layout1PASS and renderer1PASS with safe cleanup. Its separate four-crate Clippy row remains failed101 on the private large enum variant. Later moving source9 lint fixes are outside this audit and receive no inferred credit.

Release timing, CLI/isolated Python, fresh installed wheel, corpus/full scoped, native Office, archives, HLD and preparation gates remain due. Existing ordinary49 Base hash is authenticated but no current final hash pass is awarded. No new runtime, CP, Cargo, native UI or source changes were performed by reviewer. This pass ends with the report. Both defects require a distinct remediation phase before a clean whole-feature result.
