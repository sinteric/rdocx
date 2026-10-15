# F-X216, native composition, pass 2

**Reviewed**: Frozen successor against claim Base `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572` and native composition pass 1. The seal has 19 immutable current/Base carriers and the complete working patch of 20,792 lines, with 15,088 additions and 4,275 deletions. Remediation changes the existing notes admission and tree composition consumers plus defining controls. Previously reviewed code outside this delta remains source-qualified to its prior bounded scope.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D4, cross-boundary raw deduplication still discards source ordering

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3428`

Distinct schema-final extension owners now refuse and new raw children receive a mapped physical member boundary. However, before computing that boundary, the code searches every candidate boundary for byte-equal content under the same namespace context. A match anywhere skips the source occurrence entirely. Thus equality of bytes and namespace authority still substitutes for equality of physical placement.

A deciding valid witness has identical root bindings, including `xmlns:z="urn:meta"`. The local notes tree has a trailing retained `<z:marker/>`. The source master has `<z:marker/>` before one source decoration. Appending that decoration shifts the local trailing marker to the final boundary through `append_child` at `crates/rpptx-oxml/src/shape_tree.rs:3422`. The global deduplication then finds that marker after the decoration and skips the source marker that belongs before it. The successful output has only the trailing occurrence, losing the source-before-member opaque boundary. A comment or processing instruction at those positions provides the same valid boundary witness without an extension namespace.

The same early skip bypasses the new unrepresentable-order refusal when byte-equal content exists at an unrelated boundary. Compute and admit the source's target boundary before deciding whether an existing occurrence represents it. An occurrence at another boundary must not silently discharge the source occurrence's preservation obligation. Preserve the required source/local occurrences and order, or refuse the unsupported composition atomically. The current four tree controls exercise distinct raw children and do not cover this equal-content, different-boundary case.

## Prior findings resolved in the inspected scope

- **D1**: `crates/rpptx/src/lib.rs:14591` and the following notes-master admission now compare common extension bytes after complete namespace closure, prohibit resource-dependent metadata, and name unsupported owner metadata and events. `crates/rpptx/src/lib.rs:14895` compares source/destination effective header/footer flags before publication. `crates/rpptx-oxml/src/shape_tree.rs:3262` refuses incompatible source group/non-visual authority. The allowed common extension remains under an identical global-master owner rather than being moved to a different owner kind.
- **D2**: `crates/rpptx-oxml/src/shape_tree.rs:3289` compares complete effective contexts with absent/empty default namespace normalized. A nonempty local tree refuses context changes before added declarations can reinterpret its opaque payload. The new-tree exception has no preexisting opaque authority to reinterpret.
- **D3**: `crates/rpptx-oxml/src/shape_tree.rs:3302` uses namespace-expanded attribute identities. Equal aliases coalesce only with equal values and full context. Conflicting values or context refuse before publication.
- **D4, distinct final extensions only**: `crates/rpptx-oxml/src/shape_tree.rs:3447` rejects two distinct qualified schema-final extension owners. Target boundaries are computed at `crates/rpptx-oxml/src/shape_tree.rs:3467`. The earlier global deduplication remains the defect above.

The prior common-metadata predecessor inserted a duplicate common-slide extension list. Its source schema-validity credit is withdrawn. The current negative control changes a single existing extension-list owner by adding one extension and retains the source and destination byte sentinels. Root, header/footer, group and non-visual negative purposes remain separate. This review does not infer source validity from a successful permissive parser alone.

## Authentication and scope

Manifest SHA `69242f290f23ccc11ba91d7c561242186d4cc5b462a5152f005861414fb609f4` authenticates all 19 current/Base copies. Live carriers match. Source-map SHA `5103344bf73356bb04fc4a146f61a7b29862b7d4408fa458aeabfdeed3342c04` authenticates all 3,296 live mapped bodies, sizes and modes. HEAD and logical staged-entry SHA `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07` match. Patch SHA is `c516fc49efb56e46e03c243caab07f5976d247e49a5681ec30863395072866fd`.

Actual successor receipt `F-X216-start/actual-native-notes-composition-successor-4/receipt.json` matches SHA `b16498a28f811081b090a8d8b1b8aa864b9e10b4a7832578d2e9eec37e5a7edc`. All four raw log hashes match, with exit 0, qualified rows, group-gone, leader-reaped and safe lease release facts. The native log contains exactly the sealed 26 distinct control names, 26 passed, 0 failed and 0 ignored. The tree log contains the four named notes-tree controls, 4 passed, 0 failed and 0 ignored. All-target/all-feature check and strict four-crate Clippy pass. The retained integration binary matches SHA `8edd25042317d31a5fdaf41d79622a63870a7d76a40924e28dd7f8a248ac8641`. Runtime evidence uses the documented 8 MiB default debug test environment.

The retained genuine predecessor receipt `actual-native-notes-composition-RED-1/receipt.json` matches SHA `752f4112c37f43bcbdad3d888518d32490b7e173e303e1dd73f26c45820f01a8`. Its tree and master rows both exit 101 and their recorded raw log hashes match. Those failures remain failures, with the common-fixture validity limitation above. This review executes no builds, tests, binaries, UI or package operations.

The displaced Clippy annotation was restored to the existing nine-argument `capture_style_child` declaration. It was removed from the two-argument preparation helper. The restoration does not add a new suppression.

## Smells

0 found in this bounded pass.

## Nitpicks

0 found.

## Acceptance limits

No additional finding was established in the inspected unchanged source/theme/font/resource scopes, staged sparse line handling, prepared Python publication and CLI path serialization. The remaining D4 prevents this composition review from reaching zero defects. Current installed Python/types, native Office save/reopen, deterministic corpus comparison, broader applicable gates and formal handoff remain pending. No full-story or installed/native acceptance is claimed. The pass ends with this report and no remediation.
