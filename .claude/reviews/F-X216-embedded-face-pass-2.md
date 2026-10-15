# F-X216, embedded face helper, pass 2

**Reviewed**: frozen presentation.rs6f9df0313249be453920e8ca11d796f2db2f2942b367f2f16d19ece60a19d261 against the pass1 frozen predecessor766594e0, including the retained two helper controls. Manifest2e2162d046a8ca622c41718d3271ae96f6aae024f0bf43434abf72b08352df43 pins the exact source, control snapshot and receipt.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded helper review. Prior D1 is resolved.

## Defects

None found. Description comparison at crates/rpptx-oxml/src/presentation.rs:524 and existing-face reuse at :470 now compare complete decoded namespace contexts before removing root declarations for lexical comparison. The concrete helper at :797 records actual inherited and locally rebound scope at each Start or Empty element, restores parent scope on End, sorts binding entries and treats an absent default namespace as empty. This prevents equal mc:Ignorable prefix-list values from collapsing different namespace authority. It does not guess dependencies from arbitrary opaque attribute strings.

The fully modeled description branch at :516 and plain-face branch at :437 retain semantic equality for ordinary modeled aliases. Opaque values and child payload remain subject to exact lexical comparison after the namespace-context proof. Candidate face construction and namespace closure remain staged, with assignment only after successful candidate parsing at :490. Existing destination face overwrite refusal, relationship ID checks and schema face order remain intact.

## Smells

None found in scope.

## Nitpicks

None found in scope.

## Actual evidence

Authenticated actual debug receipt8890f042bc37da9aa3868dbc44adcb5651de0d409ea4116f592e5dfa50749d23 and all five relevant raw log hashes. Strict scoped Clippy, OPC query control and current debug compilation each have exit0, qualified and safe cleanup. Two exact font helper nodes each report one passed, zero failed and zero ignored with safe owned group cleanup. The retained debug binary matches d7268ceea404d4de3ab20d3c1e2312bb5c087d5b2bca6a4d7a8f50fc6519708b.

The new retained control at crates/rpptx/tests/integration.rs:33639 independently exercises both description-level and existing-face MC namespace-value conflicts. Both refuse while preserving source and destination serialized bytes. Its valid p-root and q-child fully modeled alias positive succeeds without changing destination bytes. The retained preceding control at :33585 covers actual source opaque namespace payload, unchanged destination face, repeated import, save/reopen and conflicting opaque values.

The outer receipt still has status running-classic-section-successors and contains a separate table control exit-6 with qualified=false. Only the completed qualified rows above are credited. The composite table abort, prior release image repeat exit101 and initial missing notesSz setup are not helper success evidence or whole-story acceptance.

## Not found and boundary

Correctness, namespace authority, opaque source retention, staged refusal, schema order, test meaning and structural rules were checked in the helper delta with no additional finding. No full lib.rs font pipeline, table style transfer, licensing, embedded-byte graph, public API, render or F216 completion verdict is inferred.

Frozen source and retained control hashes matched the manifest before and after review. The live presentation carrier remained exact. Evolving lib.rs and live integration tests were excluded from the freeze. No source fix, build, test, probe, Git, interpreter environment or application operation ran. Only this canonical review file was written. Review ended and returned control.
