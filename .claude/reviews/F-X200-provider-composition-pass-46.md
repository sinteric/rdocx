# F-X200, provider composition, pass 46

**Reviewed**: external provider-propagation.diff in `/private/tmp/S91-F-X200-next-adoption-prepared-r1`, two existing carriers, 14,397 bytes, SHA-256 `90d71174e92785daca1994a49f67adf4266c1015545ed5ab4b28d97636f94a25`. The counter-production.diff is excluded from this pass.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. The proposal remains unadopted and runtime is unexecuted.

## Authority and authentication

Read canonical CLAUDE.md, WORKFLOW.md and microscope, the approved F-X200 and F-X195 contracts, current F-X200 branch/status/progress, prior EPUB counter review44 and the accepted default-axis, whole-feature and borrow-lint review authority. Worker HEAD is `7c9b46a37a5c40ff3d7a21953dec06571b1e5964`, branch `work/f-x200-codex`. Current F-X200 remains in progress in the sprint records. Historical introductory plan wording is not a new runtime assertion.

Receipt SHA-256 is `0ba49297cf7e26d48b82c41ae6eafe2c7a2445b01763ba1c601b98630a973001`. Independently authenticated every artifact declared in that receipt. All 2,714 current source records match manifest `b341537058a23c5c91480f0ca5c19170ddb02ade8934ddf16196950cc82071bf` by bytes, size and mode. The NUL-delimited staged-index digest is the declared `34a791385771b53d4288429b4b573cdecb5274ec9c9d93db41a4ba86c6f95cc4`.

Applied every provider hunk in memory against its actual current preimage. Both complete candidates are exactly reconstructed. SVG before is `c7c1bd5266fed23d836c9f9331c77f3b3ac029a9b13e4d33743f096c235494a0`, after is `2985b98e2411e3ddc7be437bf892d3ac8190d538f58e12c6bb8eaeb0d525fe24`. The complete SVG candidate is byte-identical to the accepted F-X195 source317 authority `24a19e78fea73012f0c56f5cf6190e26b0da65aa465367e865d8b0f9a13edd59`.

Font provider before is `e2eaf446541941993d4072a48ff72925bbc1ffaa51bf189a2fbe0fb6a3b3a348`, after is `e26599ac52ab2ba568bb67bc9b6c9401d7193d0045fc12944393c10109238aa1`. It deliberately does not replace the full provider with the F-X195 carrier. Its single expression change preserves the current F-X200/F-X206 contribution, language, caps and trace code.

Historical F-X195 source/test participation by this reviewer is not newly independently audited here. This pass checks the concrete propagation and its preserved preimages. Prior independent whole-feature and accepted runtime authorities remain separate.

## Six-aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Candidate `crates/rdocx/src/svg.rs:927` reads the actual selected face's registered defaults for wght, wdth, ital, slnt and opsz, then overlays selected coordinates once. Both definitions and emitted text use the same effective settings. Static declarations and source FontData are unchanged. Candidate `crates/oxml-layout/src/font.rs:1162` replaces the same staged slice byte-size term with size_of_val. The preceding checked multiplication and byte-budget validation remain exact, so allocation accounting, overflow refusal, event order and ownership are preserved. |
| contract | 0 findings. Reuses accepted F-X195 selected-instance behavior rather than introducing a new selector or F-X200 API. Current contribution and shaping semantics survive because the provider is patched locally. No counter behavior is admitted by this review. |
| panics | 0 findings. SVG default lookup reuses validated_font_face, retaining bounded invalid-face behavior. No new unchecked untrusted slice or face-index operation is added. Inline fixed-font precondition unwraps retain their deciding test purpose. The size expression adds no new arithmetic or unwrap. |
| ooxml | 0 findings. No source XML parsing, serialization, namespace, sequence or owner mutation is changed. Searchable SVG text and geometric emission remain the accepted provider implementation. |
| tests | 0 findings. Candidate `crates/rdocx/src/svg.rs:1472` retains the accepted physical default100/default400/selected700 discriminator, unique declarations, emitted text/definition agreement, exact static declarations, synthesis distinctions and source immutability. The five borrowed-slice test changes retain all font inputs and assertions. The eight-owner Q/C gate, original oracle paths, ROI and0.99 thresholds are not altered. |
| structure | 0 findings. Two existing carriers, no new file, module, dependency, package identity, public API, trait, generic or forwarding framework. |

## Findings and limits

0 defects. 0 smells. 0 nitpicks.

The exact provider selector remains `svg::tests::fx195_svg_freezes_css_registered_defaults_and_selected_overrides`. It is prepared coverage, not an executed F-X200 GREEN. The full native/installed188, typing, factory, provider, publication, hash, HLD and other approved risk gates remain required after actual adoption and current producer handover.

The separately reviewed EPUB counter production cannot be adopted on the strength of this provider review. Review44's actual genuine Base requirement remains binding. Native table preconditions or compiler failures do not earn semantic RED, and all twelve contexts require subsequent actual GREEN. Original failed evidence and existing test-only adoption remain unchanged.

Only this canonical review file was written. No source, packet, Cargo, environment, cache or runtime changes occurred. The pass ends and returns its writer lease.
