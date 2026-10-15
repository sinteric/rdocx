# F-X220, font settings correction, pass 1

**Reviewed**: Separate bounded correction on `work/f-x220-codex`, HEAD `e918fa29195153a6d6cbda179a269366033fe6f5`. Against the earlier clean Word native pass, only the existing settings root-attribute helper, two settings controls and one native Word control changed. All fifteen other accepted carriers remain byte-identical.

**Verdict**: Zero defects, zero smells, zero nitpicks in this correction pass. Full-feature acceptance remains pending.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero findings. `crates/rdocx-oxml/src/settings.rs:2861` selects an actual nonempty Word prefix and excludes internal NUL namespace records. `crates/rdocx-oxml/src/settings.rs:2869` considers all effective inherited and local bindings before declaring a generated attribute prefix. The generated name no longer inserts a marker into the QName or rebinds producer descendants.
- Contract: zero findings. The same helper serves font flags at `crates/rdocx-oxml/src/settings.rs:625`, background display at `crates/rdocx-oxml/src/settings.rs:688` and document background color at `crates/rdocx-oxml/src/document.rs:2960`. Existing checked candidates and source publication boundaries remain unchanged. No additional rewrite engine or public policy is introduced.
- Panics: zero findings. The correction adds only filtered prefix selection and finite collision avoidance over validated namespace declarations. Existing source slices and checked attribute locations are unchanged. Parsing and owned-value errors still return before candidate publication.
- OOXML: zero findings. Real Word attribute aliases, default element namespaces and local foreign rebinding retain their actual expanded-name ownership. Generated attributes require a nonempty Word prefix. Inherited foreign names are reserved during declaration selection, preserving opaque descendants' namespace meanings and original lexical payload. Existing source child order and selective scalar rewriting remain unchanged.
- Tests: zero findings. `crates/rdocx-oxml/src/settings.rs:3795` distinguishes the real multi-flag NUL QName failure through strict XML validation and reopen. `crates/rdocx-oxml/src/settings.rs:3811` covers actual font flags, background display and document background color with alias/default contexts, foreign local w binding and inherited/generated-prefix collision. `crates/rdocx/src/document.rs:47821` covers regular/bold native font embedding, subsequent true-to-false settings, strict serialization and document reopen with both faces retained.
- Structure: zero findings. The correction is confined to two existing carriers and reuses the existing namespace context representation. No trait, generic, crate, module, file, dependency or asset was introduced. The previously accepted Word/render implementation and all fifteen other carriers remain unchanged.

## Authenticated evidence

Checkpoint SHA-256 is `127d7cdebf8abf9728f73524ab9a6c59d62db2353661b79fea83abb15d2b2d63`. Frozen two-carrier patch SHA-256 is `eecf4f38bed37cc6fdc74426c82ae1cb36198b54da10943f69552488cb539eb0`. The prior clean Word review remains SHA-256 `7a845ca7f85f65280b2a30f452d982fe9b10941610e32ad718857f462c02e81d`.

Reconstructed the exact earlier reviewed settings and document postimages in memory from immutable HEAD and the retained earlier patch. Their hashes match the checkpoint's prior hashes. The resulting delta is precisely the one existing helper correction and three inline controls. Current settings SHA-256 is `74b388ffe73f4c698c07996f3c190375c79e9bdd209f073f12e129863617b9ae`. Current document SHA-256 is `1ef0dab72a4e13636edf027eb9999d81e04a836ada6226e046ef1bc27d733fa8`.

The genuine RED receipt `3a0424d9ff5098cdd0cbf8811244986a686e89e828c8e6725add6c8124ed96e1` and its actual log authenticate. It records terminal exit 101 after a runtime strict-XML assertion observes NUL bytes in the serialized attribute QName. This is not a compiler or missing-method failure.

Successor receipt `19ea71e3b801369bebd394715243c49845e3f3689587bd6e3816bf072f029f5d` authenticates 32 settings and 26 ODTTF controls passed, including the new native control. Actual log counts match those numbers with zero failures and zero ignored tests. Check/lint receipt `37d339e985952ec1f77f9868d5c984be61daff90e43bb5628ed28e8a6771919b` authenticates all-target/all-feature checks and strict Clippy for the affected crates, workspace formatting and diff checks. All qualified rows have terminal zero exits and no timeout. Every retained row has its owned process group gone and leader reaped. No runtime commands were rerun during this audit.

## Full-feature boundary

This pass accepts only the separate settings correction. Genuine producer ODTTF and native embedded-face use acceptance, public Python/CLI integration, fresh installed wheels and typing, verified archives, final HLD changes and final full-feature verification remain required. Later physical-oracle and Office evidence is not newly qualified by this review. The earlier native/render review remains authoritative for its unchanged scope. No completion or handoff is claimed.

## Review integrity

Canonical workflow, approved F-X220 contract, progress and prior Word-native review retain the preceding read context. Inspected the existing namespace context producer and all three actual helper consumers. Complete 3,199 tracked-file hashes, sizes and modes, HEAD, branch, binary tracked diff, logical index and physical index match the final command source inventory before and after this review. Source inventory SHA-256 remains `4eaea33a5699350069bfa99db8dd76e5906b966a7f14d927c19b3feb1466082e`. Physical index remains `9fb296d122bd2a99880625492750520bdf6c3aaefd633dbe429344b7d2dee883`. Logical index remains `7716a0d1d333c910d853bb81f64ca51e2df0e5ce8e79b0dab78504e2b7790c1e`.

Only this review file was written. No source, plan, progress, HLD, ledger or formatting edit occurred. No Cargo, product execution, Git mutation, CP or Office operation occurred. This review stops here and returns the writer lease to the orchestrator.
