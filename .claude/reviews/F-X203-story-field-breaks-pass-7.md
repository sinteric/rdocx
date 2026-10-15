# F-X203 visible story field breaks, pass 7

**Reviewed**: Immutable story-field-breaks-7 correction, two existing before/after carriers and three frozen provider/plan authorities. Production is limited to story_break_text plus its explanatory comment. Added fixture representations and namespace/source controls are included. Other mixed-carrier APIs are excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input

Receipt SHA `8fef3954f3342b715a9d3cf02786dbf860fe825d11c6b1e263ece09738c73fea` and all21 artifacts authenticated. Incremental diff SHA `acab3f7cb526cac7eed6b09e680ff129270b99bf8948b5ac9ed43c893161290f`. Reviewed full-source map SHA `41a137c4bc9781dba9eb5f0c2a26bec481c7e412facdb8695a3dc74fcf90569f`, runtime map SHA `205b7172ed85719e7938d6615aa9bb7b2339bbbb23bc25768f0dc0abb12321e9`.

Independent before/after comparison proves every document.rs byte outside the story_break_text helper/comment is unchanged. Regression preimage equals the reviewed pass5 fixture candidate. Paragraph and typed text provider authorities equal their pass5 copies. The only runtime-to-review source-record delta is document.rs, disclosed as the precise comment-only successor. No production semantic change follows actual runtime.

Paths below are relative to `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-story-field-breaks-7/`.

## Qualification

`after/crates/rdocx/src/document.rs:9431` returns newline for an admitted visible CR in either ordinary text or a field result. Existing page/column resolution remains at9446 and resolves the actual Word-qualified type attribute. The unchanged callsite at11218 requires admitted Word namespace, nonopaque accepted visibility, visible complex result and the actual run-owner context before invoking the helper. The corresponding Empty path at11281 applies the same guards. No local-name-only foreign admission is added by this change.

`authority/crates/rdocx/src/paragraph.rs:2703` still delegates ParagraphRef.text to accepted_text. `authority/crates/rdocx-oxml/src/text.rs:2344` reads typed CR as newline. Direct/flattened simple-field Break controls use ordinary newline representation, while a closed complex cached field preserves page form-feed and column vertical-tab. The added simple expected strings therefore follow the unchanged typed provider rather than forcing the separate closed-complex representation onto it. Original closed-complex expected characters, including pass5's CR newline, remain unchanged.

`after/crates/rdocx/tests/regression_test.rs:400648` adds real simple-field and alias cases, instruction-only CR/break suppression, foreign/rebound element/type negatives, deleted/moveFrom and opaque wrapper suppression. It retains nested and incomplete complex-field cases and both Paragraph/story equality predicates. At400718 saved document XML is compared exactly before/after inspection, with explicit opaque source spellings retained. No malformed-source normalization, hidden text exposure or accepted input modification is introduced.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings in this bounded correction. The change fixes the concrete accepted Paragraph/StoryItem CR disagreement without changing the typed Paragraph provider, page/column representation policy, instructions or revision visibility. No new file, module, dependency, trait or public API is added.

## Actual evidence and limits

Actual-equation-and-story-2 receipt `4bd8694527a7d41a827b729f443d75a01e1b1cfb1fcfa4a072e5f477765852c2` retains the genuine StoryItem CR failure after Paragraph equality passed. Actual-story-field-breaks-3 receipt `14d13769e1f53b8525f8738bcfd83bd05836cc6b953d508f7196e032dc41eab7` retains the separate new simple-field fixture precondition failure, where unchanged Paragraph expected ordinary newlines rather than the proposed FF/VT. Neither failed observation is relabelled GREEN.

Actual-story-field-breaks-4 receipt `38546d3dc342db9c4d24bd8636bda387472ca1cae812413be5a9e0673379c416` records native/binding check0 and exactly one current deciding control1PASS/0FAIL/0IGNORE. Source-after and cleanup guards are true. Earlier seven native selectors passed in attempt3, and their recorded result remains prior evidence. They exercise fresh compatibility, Markdown, TextReport and validation inputs unaffected by the added simple-field test representation or the later comment. They are not freshly rerun or a complete story339 producer proof. The runtime status mentioning seven therefore denotes this composite qualification.

All raw evidence hashes authenticated. No installed, full story339, all-story consumer, corpus, oracle or whole-feature completion credit is granted by this review. No source or runtime changes were made by the reviewer.
