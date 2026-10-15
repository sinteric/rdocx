# F-X211, resource boundary, pass 2

**Reviewed**: Frozen source17 D1 correction on `work/f-x211-codex`, HEAD `36cfafaa5999709a4899f74ad4ccef773981a876`. This is the bounded correction to pass 1's opaque inline ownership finding, not weighted quota or full-feature acceptance.

**Verdict**: Zero defects, zero smells, zero nitpicks within this correction. Pass 1 D1 is resolved.

## Resolved finding

### D1, opaque inline shape ownership

`crates/rdocx-oxml/src/document.rs:94` excludes inline drawing containers from recursive typed admission while retaining the actual anchor route. Once the inline node is opaque, parent admission remains false for all its descendants, so qualified-looking controls inside it do not consume the content-control budget.

The native inline parser at `crates/rdocx-oxml/src/drawing.rs:1435` still preserves shape text rather than constructing its Word text body. No parser/model expansion was used to justify the admission change. The native anchor parser's qualified text-body branch at `crates/rdocx-oxml/src/drawing.rs:1052` remains admitted even with an unknown graphicData URI, matching actual existing native construction rather than an invented URI gate.

`crates/rdocx-oxml/src/document.rs:3541` constructs 257 qualified controls under the real inline route. The actual control opens that document, serializes with the complete original inline subtree present, reopens it and proves deterministic output equality. Its anchor counterpart refuses. A smaller anchor with the same unknown producer URI exposes an actual typed text body and exact leaf text. The separate opaque/rebound control remains qualified.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero further findings in this correction. Opaque descendants remain excluded and the real anchor construction route remains counted.
- Contract: zero findings. D1 is corrected without new native shape support, URI reinterpretation, quota bypass or weighted policy implementation.
- Panics: zero findings in the correction. Removing the inline route introduces no indexing, recursive construction, unchecked arithmetic or allocation path.
- OOXML: zero findings. Actual namespace ownership and inherited/local rebound admission remain intact. Exact opaque inline source survives save and reopen. The unknown-URI anchor remains typed because the native parser actually constructs it.
- Tests: zero findings. The 257-control inline positive distinguishes the previous false refusal, while the real anchor negative and smaller typed anchor prove the correction did not simply disable all drawing admission. Retained runtime logs report the actual selected controls.
- Structure: zero findings. The correction stays in the existing admission predicate and existing inline tests. No new module, file, crate, dependency, ambient authority or alternate model is introduced.

## Evidence and integrity

Checkpoint SHA-256 is `2e529d7ed8ba6d7a3625cb19f4e0316806d6efaabf0bf09f2f53f16646f73bac`. Source17 inventory SHA-256 is `dab247b5680f47e210d10c0a61dfe805946392fb3def1a003a34b610d073e6ff`. Current document diff matches the retained artifact at `0fbda5b3129a311d39d855d4db405e052c6376cbdfc4057563ea48210e89db56`.

Actual16 receipt `abff04169ab22d6d806f7027829f7127e968f167b2414be65100e57232ae765b` and both actual row log hashes authenticate. Both rows have terminal zero exits, one passed control each, zero failures/ignored controls, no timeout, owned group gone and leader reaped. This review inspected those results without executing Cargo or the product.

Read canonical workflow and microscope instructions, the preceding resource-boundary review and the plan's resource amendment. Authenticated all 3203 tracked source hashes, sizes and permission modes before and after writing. HEAD, branch, tracked binary diff, physical index `5a949eb5421f440d730a533607236de39b84e42a012d523c6e0412701ed84532` and logical index `d1ac0aa6c587289eb6fc90884f73d985c585b9a037387546047402f222247b59` remained exact. Only this review record was written. No source fix, runtime, Git mutation, CP or Office operation occurred.

## Remaining boundary

The weighted inclusive path-budget candidate is not implemented or approved by this review. Its explicit design and independent acceptance, borrowed-model validation before serialization and staging clones, all actual owner routes, related-story entrypoints, alternating controls/tables, ordinary-stack CLI/Python behavior, atomic refusals and frozen100 compatibility metrics remain outside this pass. Neither the corrected source-input route nor its two passing controls establish complete resource safety or full F-X211 completion.

This pass stops here and returns control to the orchestrator.
