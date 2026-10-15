# F-X220, model, pass 1

**Reviewed**: The uncommitted low-level model diff on `work/f-x220-codex`, HEAD `e918fa29195153a6d6cbda179a269366033fe6f5`. Exactly two carriers, `crates/rpptx-oxml/src/presentation.rs` and `crates/rpptx-oxml/tests/integration.rs`, with 1085 additions and 2 deletions.
**Verdict**: 2 defects, 0 smells, 0 nitpicks. The bounded model phase needs remediation and another independent pass.

## Boundary and contract

This is a model-phase audit, not full F-X220 acceptance. The approved design, AGENTS.md, CLAUDE.md, canonical WORKFLOW.md and microscope command were read before implementation review. F-X220 remains in-progress in both sprint records. The contract sections reviewed include HLD03 concrete font ownership and staged facade conventions, HLD04 relationships, media and package integrity, HLD06 `presentation.xml` and preservation, plus the cited rendering, binding, deterministic-font, packaging and backlog context.

The leased phase covers lexical and opaque embedded-font-list preservation, four typed face slots, schema order, tri-state root flags, qualified relationship reachability and ambiguous-source refusal. Native font decoding and authoring, Word mutations, Python and CLI surfaces, render paths, fresh wheels, native Office font-use controls, affected archives, final HLD updates and full feature completion are outside this checkpoint. Their absence is not a finding against this bounded phase and remains required for the complete approved story.

## Defects

### D1, transferred entries are serialized without their inherited namespace context

`crates/rpptx-oxml/src/presentation.rs:965`

`push_embedded_font` accepts any public `CT_EmbeddedFontListEntry`, including a clone from `embedded_fonts()` or an entry returned by `remove_embedded_font`, then stores it without checking the destination namespace context. `EmbeddedFontList::xml` emits that entry's raw bytes directly at `crates/rpptx-oxml/src/presentation.rs:417`. Its inherited bindings are retained only in the internal projection at `crates/rpptx-oxml/src/presentation.rs:618`.

For example, parse a source root declaring `xmlns:q=P_NS` and `xmlns:rel=R_NS`, with `<q:embeddedFont><q:font typeface="Alpha"/><q:regular rel:id="rId1"/></q:embeddedFont>` inheriting both aliases. Clone or remove that entry and push it into a new presentation. The destination writer declares its fixed prefixes but does not declare `q` or `rel`, so the emitted entry contains undeclared prefixes. If the destination already binds those aliases or its default namespace differently, the entry can instead silently change namespace ownership. The live typed entry still exposes the old face, while reparsing or qualified reference counting no longer sees that same owner.

The operation must preserve the source bindings in the destination or refuse the transfer before mutation. Add controls for inherited aliases, inherited default namespace and conflicting destination bindings, including reopened projection and reference counts. The current new-entry insertion test at `crates/rpptx-oxml/tests/integration.rs:6124` uses `CT_EmbeddedFontListEntry::new`, whose local declaration avoids this case.

Aspects: correctness, contract, ooxml and tests.

### D2, accepted relationship IDs can change during projection and reachability counting

`crates/rpptx-oxml/src/presentation.rs:557`

The face parser obtains relationship values through the generic `all_attributes` reader and stores that value at `crates/rpptx-oxml/src/presentation.rs:605`. That reader replaces decoded carriage returns, line feeds and tabs with spaces. Unlike `font_attribute`, it therefore also loses whitespace supplied by character references.

`set_face` currently accepts a nonempty XML string such as `a\nb` at `crates/rpptx-oxml/src/presentation.rs:235`. Replacing an existing face uses the lexical editor, which emits `a&#10;b`, but reparsing the candidate projects `a b` and returns success. Creating a missing face at `crates/rpptx-oxml/src/presentation.rs:283` emits literal whitespace instead, which also normalizes. The supplied ID is silently changed rather than preserved or rejected. Qualified reference counting uses the same lossy values at `crates/rpptx-oxml/src/presentation.rs:662`, so it also cannot distinguish an encoded-whitespace source reference from its space-normalized spelling.

Apply the appropriate relationship-ID validation and reject unsupported input before mutation, or preserve its exact XML value consistently through reading, replacement, insertion and reference counting. Existing opaque source must stay lexical even when its face cannot be safely projected. Add encoded-whitespace source and failed-input atomicity controls. The character-reference regression at `crates/rpptx-oxml/tests/integration.rs:6387` currently exercises only the typeface, not relationship values.

Aspects: correctness, contract, ooxml and tests.

## Smells

None.

## Nitpicks

None.

## Not found

- Panics: zero findings. Enum indexing is bounded by the concrete four-kind enum. The new `expect` and `unreachable` sites follow immediately established internal invariants. Source slicing follows successful parser positions. No runtime or hostile-input execution was performed in this review.
- Structure: zero findings. The implementation stays in the two existing carriers, adds concrete types and local helpers, and introduces no trait, generic parameter, forwarding wrapper, module, crate, dependency or feature flag.
- Tests: no separate findings beyond the missing distinguishing controls attached to D1 and D2. Existing controls exercise the actual producer list and faces, aliases/defaults/local rebindings/foreign lookalikes, opaque lexical attributes/events/payload, duplicates and atomic refusals, face insertion order, root presence/invalid values and typeface character references.
- Correctness, contract and ooxml: no additional findings beyond D1 and D2.

## Authenticated evidence

All evidence was read, not rerun. No cargo, target access, CP, rendering or Office command was executed.

- Source file SHA-256 values match the checkpoint: presentation `86a59d79c01fde7eb436d9335e34dab133c10ba8d1c8c5e95f33055f8ecf7383`, integration `e9d82b59e86a640044c132740821ac17644225782fdd89de628ed66349c84822`.
- Checkpoint SHA-256 is `65d3d760cd10cff8db75fb97a69bced0d21c954a072a1008dbf4c14c8cb9a67d`. Final receipt SHA-256 is `1621a31126e07dc5a6d1224d3509c030a9ba3600e3bf915ab1cfc4993a5440b3`. Its source inventory SHA-256 is `9a52e4c07d5c14430a15f8e01c2455b330d16206582b39fc7cbfe52f50d3caef`.
- Every tracked source file matches that authenticated source inventory before and after the review write. HEAD, physical index `9fb296d122bd2a99880625492750520bdf6c3aaefd633dbe429344b7d2dee883` and logical index `7716a0d1d333c910d853bb81f64ca51e2df0e5ce8e79b0dab78504e2b7790c1e` remain exact. The review artifact is the only written file.
- All six final receipt log hashes match their actual files. Logs show all-target/all-feature check, seven targeted PASS controls, one complete 50-presentation structural corpus PASS control, all-target/all-feature Clippy, diff check and rustfmt check. All six receipt rows report exit 0, no timeout, reaped leaders, absent owned process groups and safe lease release. These are retained worker results, not independently executed review gates.
- All 50 original corpus hashes match the final receipt. Genuine producer `placeholder-layout-color.pptx` matches SHA-256 `b683af99cf4e71db112fe87db15840846c53dbc7a6839cbf84804980d21af9ce`. The owned graph receipt matches SHA-256 `4dcd55a5f8c3f8bd13f749a7b4131b681c7939bf7c82e6fa82c393689ee3c74b`. That retained Mac PowerPoint producer has three typefaces and six compressed MTX EOT parts. Model projection and exact list preservation do not prove supported decoding or current native font use.
- Retained earlier failures remain checkpointed. No new hash-harness execution or baseline movement, scoped/full feature verification, handoff or completion is claimed.
