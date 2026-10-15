# F-X209, background reference guard correction, pass 11

**Reviewed**: Immutable source54-to55 correction, two existing carriers. The production delta changes only explicit background-reference replacement admission. The facade delta extends the existing atomic-refusal control. All ten receipt records and both complete before/after diffs were independently authenticated. Approved F209 preservation and transaction requirements apply.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass9 D1 is resolved in this bounded correction.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D1 resolution

The admitted root remains expanded-name PresentationML `bgRef`. A direct child must be the single qualified DrawingML color owner, and only direct qualified transform children of that color are admitted. Nested color owners, nested transform content and duplicate colors are refused at `background-guard-source55-capsule/slide_parts.after.rs:763`. This closes the previous blanket descendant admission.

Attribute admission is now specific to the actual qualified owner at `background-guard-source55-capsule/slide_parts.after.rs:816`. System color accepts only val/lastClr, scRGB accepts r/g/b, HSL accepts hue/sat/lum, parameterless transforms accept no non-namespace attribute, and the remaining admitted colors/parameterized transforms accept only val. Namespace declarations are retained as transport. Foreign or rebound owner names, namespaced opaque attributes and other non-whitespace content remain refusals. No ordinary parser or renderer behavior is changed.

The original unknown-attribute and unknown-child negatives remain. The three added source-authored inputs include tint.lastClr, srgbClr.hue and an illicit nested scheme color at `background-guard-source55-capsule/lib.after.rs:20965`. These exercise the prior union-attribute defect and descendant-owner boundary. Each fixture first opens and saves the retained source, invokes the actual master-background picture replacement, requires refusal, then compares the complete saved package to its pre-operation bytes at `:20983` and `:21000`. This proves the selected opaque source is not discarded by an explicit conversion and publication/allocation is atomic.

## Not found

Correctness: 0 findings. Contract: 0 findings. Panics: 0 findings in the changed admission and fixed fixture scope. OOXML: 0 findings. Tests: 0 findings. Structure: 0 findings. No new files, module, trait, generic, public API or broad parser restriction is introduced.

## Authenticated evidence and limits

- Capsule receipt: `ea67628e24e95f07301d39f47ce1f7ee176242b1309457404dcc5e629f93462a`.
- Production diff: `9f95627b37a0f52cc423ff7b00aca87ceb605fe0285d21a8ccaa20539b6b4d3b`.
- Test diff: `0831ae13f78db695c06c20fe4f5be695cdddb4d8a16919b1e380fa0597639d4c`.
- Source54 map: `6f596580723d3dfa660e9192da59aea3610d740c6e1e5fbc33143820748fe314`.
- Source55 map: `9abb51798695614d4fad55855041f8cbd67dff2646244c7499f6f48b11a344b5`.
- Actual deciding receipt: `1ff531aeb5c678f34afdf28c7af8325528860a8c70cb61b4e8af717e882b8ce9`.
- Exact deciding log: `0fadcb6a293e8c1bd0efffca56924dd12cba94ceb0472b3d7d632856a2e32939`.

The recorded exact native selector exited 0 with 1 passed, 0 failed and 0 ignored. Its source-after guard matched source55. The owned group naturally ended, leader reaped, signal0 ESRCH and no signals. These are retained worker results, not reviewer execution. Original pass9/source42 and source54 failures remain distinct evidence. This review does not qualify whole F209, successful background image paint, installed Python, theme import or unrelated lifecycle acceptance. The expected-style fixture proposal from pass10 is a separate unrun correction and is not credited by this selector. No source or runtime changes were made during this review.
