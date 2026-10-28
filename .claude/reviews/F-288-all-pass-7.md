# F-288, all, pass 7

**Reviewed**: Codex independent frozen working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 3906 insertions and 228 deletions. Diff SHA-256 f641fa313441341cc439f8dd64f00e869885e6700aaeefe0a3d71ce8b3bb1ac6.
**Verdict**: 2 defects, 1 smell, 0 nitpicks.

## Defects

### D5, namespace allowlisting is still broader than accepted owner grammar

`crates/rdocx/src/field.rs:1725`
`crates/rdocx/src/field.rs:1747`
`crates/rdocx/src/field.rs:1750`

The live-plus-deleted related paragraph now fills correctly. The raw selector still accepts every Word namespace container except del and moveFrom, and accepts unresolved or unqualified containers by returning false for an absent namespace. That is not the accepted-owner grammar. A preserved Word wrapper can contain a drawing that is not an accepted run.

Codex directed another read-only public-module probe, authenticated extension SHA-256 4c6acbbab9bbc9c5e5e5b446474bd719225520e30620baf4d4317d254b36e660. One header paragraph contained the live active drawing plus a distinct drawing inside Word bidi embedding w:dir with val rtl. Inventory contained active only and excluded directed. Filling active with new actually raised RdocxError, text box paragraph boundaries do not match the typed story owner. Bytes stayed equal, fresh and reopened current stayed old, and the directed drawing and wrapper remained. Transcript: /private/tmp/s95.4-f288-review7-opaque-word-probe.txt.

Replace the broad namespace whitelist with the actual recognized accepted owner/container policy. Recognize supported Word run wrappers and block containers positively, using the existing canonical classification, and keep other Word or unqualified wrappers opaque. Do not add only dir to another rejection list. Preserve ignored XML rather than deleting it, do not silently skip mismatched copies, and retain qualified offsets, duplicate occurrence identity and inherited namespace handling. Add this input to the existing mixed-owner regression and check the same rule for sibling ignored wrappers without creating new supported contexts. Retain D5 because this is the same physical read/write owner invariant.

### D9, effective nested projection repeats recursively and grows exponentially

`crates/rdocx-oxml/src/text.rs:2079`
`crates/rdocx-oxml/src/text.rs:2105`
`crates/rdocx-oxml/src/text.rs:2185`

Both effective_display and effective_display_field_segments first call observable_parent_cache_override. That helper recursively computes nested_effective_concatenation. When no override exists, the caller computes nested projection again. Each nested child segment method repeats the same work. An ordinary single-child typed Field chain therefore repeats recursive projection at each level instead of visiting each node once. The stored cache equaling its derived baseline does not avoid the first traversal.

A bounded native probe compiled from stdin against the exact already-built rdocx_oxml rlib confirmed the cost. Source text.rs SHA-256 0da2d3d0117b22889d7479f2145e09ce5386bc1a5159ab7b1e0614ec2768ab54. rlib target/debug/deps/librdocx_oxml-ec51ead2d96f03d4.rlib SHA-256 632c4a6d1781a4fd812195f8f1856937843e82f47fc57d84ef50194155184566. rustc 1.97.1, default codegen, edition 2024. Compiler child exit 0. The temporary review executable was removed and no product source changed.

Each node was a public complex QUOTE Field containing its child in checked cached runs, with physical and effective display x. Direct effective_display raw samples in ns:
- Depth 4: 32542, 22209, 27541.
- Depth 8: 240000, 227958, 227833.
- Depth 12: 1712541, 1672708, 1651958.
- Depth 16: 28616625, 27697750, 25678125.

Direct segment projection likewise rose from 20667 ns at depth 4 to 25577000 ns at depth 16. Construction was measured separately and excluded. Results stayed x. These debug observations diagnose the source recurrence. They are not a new release threshold or the sprint's final performance qualification. Transcript: /private/tmp/s95.4-f288-review7-native-projection-probe.txt. The earlier held Python paragraph-getter timings were cached and are explicitly inconclusive for this native path.

Compute nested segments or display once per node and share that result between override selection and returned projection. Keep one concrete override policy rather than copying predicates between scalar and segment methods. Preserve true observable parent overrides, derived baselines, effective defaults, child ownership, formatting and all borrowed physical APIs. Add a discriminating direct typed-chain regression with a structural evaluation-count control or equivalent non-flaky evidence. Do not add a public debug API, generic cache layer, new resource limit or arbitrary timing threshold.

## Smells

### S3, unreachable AlternateContent choice state remains in the new scanner

`crates/rdocx/src/field.rs:1607`
`crates/rdocx/src/field.rs:1691`
`crates/rdocx/src/field.rs:1738`

The selector unconditionally rejects AlternateContent because its projected drawing is layout-only rather than accepted run content. Descendants inherit rejection, so the choice and fallback selection state never participates in an accepted span. alternate, saw_choice and the associated choice-selection branches add impossible cases for readers. Remove this unreachable state while retaining the explicit existing layout-only boundary and preserved raw bytes.

## Nitpicks

None.

## Resolved and retained observations

D8 is resolved. Document structural insertion now calls publish_api_candidate, whose source requests Conservative. The binding regression covers refusal with unchanged bytes and live old handles, changed insertion through both routes with stale old live paragraph and run handles, and re-fetch. Owned story-item snapshot text remains readable under existing semantics.
The deleted, move-from, foreign opaque and layout-only AlternateContent portions of D5 have a fresh qualifying native regression. The Word namespace opaque-wrapper input above remains wrong.
D1, D2, D3, D4, D7, S1 and S2 remain resolved in the inspected paths.
The frozen implementation has actual 14-case native, 25-case Python, local mypy, fmt and scoped all-features clippy receipts. These do not discharge the new failed owner probe or direct native work regression.

## Not found

No additional panic, dependency direction, new trait, generic, module, child-order or Python result-allocation-order defect was found. Word, hash harness, complete installed suite and final union checks remain pending. DOCX-055 and Q4 remain incomplete.

## Review boundary

No product source was changed by this review. Mechanical probes used the authenticated built artifacts. The only temporary compiled probe lived under the review directory and was removed. Grok may transcribe this exact Codex report only. End the review pass when written. Remediation is a separate phase.
