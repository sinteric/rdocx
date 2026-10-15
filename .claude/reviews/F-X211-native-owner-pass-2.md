# F-X211, native owner, pass 2

**Reviewed**: Frozen native Word owner foundation source90, including correction of native-owner-pass1 D1, finite known-profile qualification and reconciliation with completed F222. Current working diff against HEAD `00b0d4eaec2b157a1f2555badd5cbbe89e0eb787` has14 tracked carriers,5344 insertions and153 deletions. Prior clean resource/dispatch boundaries remain applicable. This is a native foundation review, not a whole F-X211 review.
**Verdict**: 0 additional defects,0 smells,0 nitpicks in this bounded pass. Prior D1 is resolved. Existing namespace83 and maximal transaction85 barriers remain unresolved. No completion or public acceptance claim.

## Defects

None newly found in this bounded pass. The known namespace83 failure remains a completion barrier below.

## Smells

None found.

## Nitpicks

None found.

## Prior D1 resolution

`crates/rdocx-oxml/src/text.rs:565`
`crates/rdocx-oxml/src/text.rs:605`
`crates/rdocx-oxml/src/text.rs:628`
`crates/rdocx-oxml/src/text.rs:675`

The retained-root writer detects a producer binding of `w`, selects a collision-free producer prefix and rewrites producer attribute names under that declaration. It does not append the foreign binding to generated Word roots. Namespace-valued MC/XSI attributes requiring the displaced foreign `w` binding are refused before publication rather than rewritten without semantic qualification. Other retained declarations and attribute values keep their namespace dependencies.

`crates/rdocx-oxml/src/table.rs:4527` checks both table and cell root expanded names, producer attribute namespace/value and a competing producer prefix. `crates/rdocx/src/document.rs:36264` exercises checked table and cell replacement, forced modeled serialization, normal reopen twice, preserved typed content and atomic namespace-valued refusal with a held owner. These controls address the actual prior collision rather than another foreign-prefix example. The retained failing-before and successful successor receipts remain separate.

## Aspects checked

- Correctness: no additional finding. Physical paragraph/run/table/cell owners are selected before staging, stale owner boundaries are checked, successful cells validate the enclosing physical table and only a reopened candidate is committed. See `crates/rdocx/src/document.rs:21243`, `crates/rdocx/src/document.rs:21366` and `crates/rdocx/src/document.rs:21422`.
- Contract: no additional finding within the native scope. The five checked operations remain crate-private. Known section ownership is compared before publication, changed or introduced nested section profiles are named unsupported, and direct section replacement checks owned header/footer targets and geometry. See `crates/rdocx/src/document.rs:21389` and `crates/rdocx/src/document.rs:21436`. This is finite modeled-profile admission, not complete XSD validation or arbitrary package mutation.
- Panics: no additional finding. Loaded recursive models are admitted before staging clones, counters use checked arithmetic and parsed marker IDs precede their subsequent internal conversions. Section access, physical cell indices and missing relationship targets return errors. See `crates/rdocx/src/document.rs:15267`, `crates/rdocx/src/document.rs:8464` and `crates/rdocx/src/document.rs:8653`. Maximal timedout85 is not treated as qualified by this inspection.
- OOXML: no additional finding in the reviewed corrections. Strict lexical/root admission, expanded namespace checks, modeled child ordering and leaf validation precede typed replacement. Opaque routes remain opaque. Known property values that would silently default or lose ownership are refused, while valid spacing/numbering/table composites retain their native representation. See `crates/rdocx-oxml/src/text.rs:37`, `crates/rdocx-oxml/src/table.rs:53` and `crates/rdocx-oxml/src/paragraph_properties.rs:1964`. The separate local drawing namespace failure remains open.
- Tests: no additional finding in the finite controls. Cross-owner bookmarks, current comment definitions, dirty physical occurrences, note/field dependencies, existing repeated drawing IDs, new collisions and nested section refusal have targeted positive and negative controls. See `crates/rdocx/src/document.rs:36538`, `crates/rdocx/src/document.rs:36559`, `crates/rdocx/src/document.rs:36652`, `crates/rdocx/src/document.rs:36751` and `crates/rdocx/src/document.rs:36780`. Actual current-source receipt91 qualifies the bounded ordinary-stack controls only.
- Structure: no additional finding. Existing owner types and providers are reused. No new product module, dependency, trait, generic mutator framework or forwarding wrapper is introduced. The comment source inventory becomes crate-visible for an actual checked-owner consumer at `crates/rdocx/src/comments.rs:2936`.

## F222 composition

Six retained provider functions were independently compared as complete function text against current canonical HEAD: comment_story_range_paragraphs, scan_story_control_paragraphs_with_scope, story_range_paragraph_sources, story_range_paragraph_spans, story_range_paragraphs and text_box_cache_paragraphs. All are identical. Nine checked-owner/validation functions were independently compared against preserved worker stash `2654ecd801e8c20a99271a7b9ad35f21e61219c1` and remain identical. All12 unchanged worker carriers and canonical field.rs match the composition receipt. This preserves both provider authority and the checked-owner foundation rather than resolving the overlap by dropping either implementation.

Composition receipt SHA `3ea938c2cf543c0ebfb72524c0d3ae5b45ca8f5fb953fa8fc5258a34e3ec2ffe` is authenticated. Current15-row receipt91 SHA `c88b101d40ca2500254b9c304a187423b95af99e0c495eff2354432d1c98b95c` and every log hash were checked. It records all-target checking, paired release compilation and13 ordinary2097152-byte-stack controls, all actual exit0 with qualified/reaped/gone flags. Current Word executable SHA `e04db0bfeda3caa94e3a2fb1e4db9cfa8d62062310a4ff84529b53c7ed1c2b03` and oxml executable SHA `24d2ad858c145cc16d9be66d84577dfd2859fa22f02c2231ba7f6fed675b8bf5` were independently rehashed. No command or test was run by this reviewer.

## Retained unresolved barriers

- Original83: imported picture with a/wp declared locally on its drawing failed during modeled snapshot flush with unbound prefix a. Receipt SHA `cbffc3a58f4ee208159b6050e903cfe1150488c1d59c5b9ca0635f5ee04eb7fb` and original failure log are retained. Root-complete84/91 drawing controls qualify the ID provider, not this local namespace case. The named residual in native-profile-inventory-84.md remains open.
- Original85: release compilation succeeded but the selected table transaction at table32 with outside control256 timed out at30 seconds. Receipt SHA `ec72fbedf44b12d2814ff9ee4074f36e4268a02734215bed54f866ed3f3c263a` remains a timed-out original. Cleanup86 SHA `22aacaa06711b43323dd660b1ce292c523cdcf482e1e73a21dc1bcc774fbf80d` proves the group naturally disappeared. The old Word executable was overwritten at the same Cargo filename by91 and is not currently rehashable. Its historical build/hash/source/timeout records remain evidence of the attempt. Neither13-node87 nor13-node91 replays this maximal transaction.
- Public Word/PPT element surfaces, stable generations, exact frozen100-script corpus, binding/typing acceptance, deterministic renders, actual native Office and final HLD/archive/scoped gates remain outside this review. No evidence is relabeled as whole-feature completion.

## Source integrity

All3205 source90 entries matched bytes, size and mode before review and immediately before writing. Source-map SHA `4e8231f96e7d3b4ed7aa54c9c75a6301214fc7565a46e4ba3d10bf7e2409a654`. Branch is work/f-x211-codex. HEAD stayed `00b0d4eaec2b157a1f2555badd5cbbe89e0eb787`. Binary working diff SHA `6ff9816ad8e30960c00e747c711dff46391d5108d4a37ac874b89f3a57682b74`, logical index SHA `a8c103650a04517f4eb9c9216089c30817da8a2ecefcd5b5a8eb1802a885e183` and physical index SHA `03dc45d44ada1ad908a43ab2552901f3843357a21f96a8dc8d6bf5dd24596cab` authenticate the frozen boundary. Only this review file is written. Remediation and complete-feature preparation remain separate phases.
