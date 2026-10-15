# F-X217, native phase, pass 1

**Reviewed**: Working native-phase diff at Base cb3648db2edca69f310fc459eba63f37775dbcf2 on work/f-x217-codex. Seven files, 1293 insertions and 87 deletions. Approved plan, cited native import/package contracts, original PR359 assessment, incremental e974ea2240691c0de581efa172ba8c88a77c054b assessment, progress and native-phase-2-checkpoint.json SHA 9fcab0ada44b507813b60f5985ad2a311d3ec18822fbd24ba0b6a9e68866b159 were read.

**Verdict**: 5 defects, 0 smells, 0 nitpicks. Native phase only. This is not full-feature review or completion.

## Defects

### D1, escaped paragraph prefixes become block syntax on export

`crates/rdocx-html/src/markdown.rs:480`
`crates/rdocx-html/src/markdown.rs:274`

An ordinary imported paragraph such as CommonMark `\# literal heading` contains actual Word text `# literal heading` without a heading outline. Inline escaping does not escape `#`, `>` or the punctuation of list markers, and the ordinary paragraph branch writes that text directly at the start of a Markdown line. The next import therefore changes the paragraph into a heading. The same path turns `\> literal quote` into a block quote and `1\. literal item` into an ordered list. Canonical round-trip must preserve parsed block semantics, including escaped block-looking text, rather than only inline delimiters. Add context-sensitive escaping for paragraph and continuation line starts.

### D2, list-item headings are emitted outside their list

`crates/rdocx/src/html.rs:3178`
`crates/rdocx-html/src/markdown.rs:234`

For CommonMark `- # Heading`, the importer correctly retains both the heading outline and first-item numbering on the native paragraph. The emitter checks heading level before list information, calls leave_list and writes a root `# Heading` without a list marker. The list container is lost after DOCX save/reopen/export. Heading syntax must be emitted inside its actual list item, including numbered continuation headings.

### D3, a quote containing a list is exported as a list containing quotes

`crates/rdocx/src/html.rs:2983`
`crates/rdocx-html/src/markdown.rs:187`
`crates/rdocx-html/src/markdown.rs:243`

For `> - First\n> - Second`, the importer produces Quote paragraphs with native list numbering. The emitter first prefixes the item text with `> `, then its list branch prefixes that result with `- `. The resulting `- > First` and `- > Second` place separate quotes inside list items instead of retaining one quote containing the list. This also changes grouping of adjacent quote contents. The container order needs to follow the actual imported nesting. The existing code-and-quote-inside-list control covers the opposite nesting and does not decide this case.

### D4, quote projection drops thematic-break identity and table ownership

`crates/rdocx/src/html.rs:2969`
`crates/rdocx/src/html.rs:2983`
`crates/rdocx-html/src/markdown.rs:173`
`crates/rdocx-html/src/markdown.rs:28`

The quote post-pass handles only paragraphs and replaces every non-code paragraph style with Quote. For `> ---`, this overwrites HTMLThematicBreak, so the exporter no longer recognizes the rule and emits an empty quote instead of the thematic break. A GFM table inside a quote is skipped by the post-pass entirely. It has no retained quote owner, and the table emitter writes a root table. Preserve the distinct block role and quote nesting for these supported block contents instead of treating the quote as a paragraph-style overwrite.

### D5, nonbreaking-space-only inline content is discarded

`crates/rdocx/src/html.rs:4161`
`crates/rdocx/src/html.rs:3106`

The admitted formatted-whitespace amendment explicitly preserves nonbreaking spaces. The visibility predicate still uses Unicode trim, which removes U+00A0. A tight list item containing only a nonbreaking space, such as HTML `<ul><li>&nbsp;</li></ul>` or Markdown `- \u{00a0}`, fails the inline visibility check. Its pieces are cleared and the item becomes an empty paragraph. A bare `&nbsp;` fragment is similarly discarded by the container flush. Use the same ASCII HTML whitespace boundary for visibility as for collapse, so the content reaches the native run writer.

## Smells

None found in this bounded pass.

## Nitpicks

None found in this bounded pass.

## Evidence and limits

The four retained phase-2 command receipt hashes and their raw log hashes match the checkpoint. Each retained receipt has identical complete sourceBefore/sourceAfter snapshots. Actual current native HTML tests passed 25, emitter tests passed 14, scoped all-target Clippy exited 0 and the locked/offline Rust 1.93 native/HTML library check exited 0. These existing results do not cover the block combinations and nonbreaking-space-only cases above. Findings are derived from cited control flow. This review ran no Cargo command, new test suite, install, application or oracle.

The exact pinned pulldown-cmark 0.13.4 dependency, html-only renderer feature and parser TABLES/STRIKETHROUGH options agree with the approved contract. Incremental PR359 reuse stays Word-only, and the predicate includes mark_highlight rather than adopting the donor omission. Source offset ordering, raw HTML diagnostics, bounded projection, code-span spaces and fence selection, image alt/resource projection, template graph and named-style controls were inspected without further findings.

HEAD, branch, physical and logical index, complete tracked diff and all seven checkpoint carrier hashes were captured before the source review and checked unchanged immediately before this report. The same guard is checked after writing this review file. Only this review file is written. Native consumers, installed bindings/CLI, Word repair-free open/save/render, full grammar differential, remaining dependency/WASM/hash/archive gates, HLD and final full-feature microscope remain due in their admitted phases.

## Not found

- Correctness and contract: no additional findings beyond D1 through D5.
- Panics and input bounds: zero additional findings in changed native production code.
- OOXML and preservation: zero additional findings beyond the block-role loss in D2 through D4.
- Tests: zero separate findings. Coverage omissions are attached to the concrete defects above.
- Structure: zero findings. No new repository product file, module, crate, speculative trait or generic was introduced.

Return to the implementing worker for a separate remediation phase. This review does not modify the implementation or its acceptance contract.
