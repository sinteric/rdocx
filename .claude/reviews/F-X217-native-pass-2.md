# F-X217 native review pass 2

## Reviewed state

Reviewed the frozen native remediation diff on `work/f-x217-codex`, Base and HEAD `cb3648db2edca69f310fc459eba63f37775dbcf2`. The diff comprises seven tracked files, 1,754 insertions and 91 deletions. The authenticated native remediation checkpoint has SHA-256 `971229e7c13aa6a57dd9ff9664132b19892d018eee27db10c7e161bc91556675`.

This pass covers the five earlier defects and the affected persisted container styles, quote ownership, save/reopen/reimport grouping, NBSP visibility and text-only escaping. It does not decide public binding, CLI, Office or whole-feature acceptance.

Verdict: **one defect, zero smells, zero nits**. The original five findings have been addressed, but nested quote grouping still has a native defect.

## Defects

### D1. Joining nested quote paragraphs drops the inner quote separator

`crates/rdocx-html/src/markdown.rs:38` inserts exactly one `> ` on every same-owner paragraph join. However, paragraph emission prefixes content with every outer quote level at `crates/rdocx-html/src/markdown.rs:392`. The owner is taken from the first, outermost blockquote ancestor at `crates/rdocx/src/html.rs:3057`, so paragraphs in an inner quote beneath a compound outer quote share that outer owner.

For example, the following source has one inner blockquote containing two paragraphs:

```markdown
> - First
>
> > Inner A
> >
> > Inner B
```

The current control flow emits the inner paragraph content with `> > `, but the separator between `Inner A` and `Inner B` has only `> `. Removing the outer quote level leaves `> Inner A`, a bare blank line, then `> Inner B`. That closes the first inner blockquote and creates a second one. The q/l path persists quote depth but its single outer-owner ordinal does not preserve this inner grouping.

This finding follows from the frozen source, without a new runtime execution. Add a focused saved/reopened Markdown event comparison for a compound outer quote containing a multi-paragraph nested quote. Remediation must preserve both same-inner-owner joins and distinct adjacent inner quote owners.

## Previous findings

- Pass 1 D1 is resolved. Text-only block-start escaping at `crates/rdocx-html/src/markdown.rs:435` protects literal block prefixes. Generated page-break syntax is separated before this escaping at `crates/rdocx-html/src/markdown.rs:598`. The focused native control begins at `crates/rdocx/src/html.rs:4792`.
- Pass 1 D2 is resolved. List-owned headings retain the list branch through the conversion at `crates/rdocx-html/src/markdown.rs:288`. The focused control begins at `crates/rdocx/src/html.rs:4804`.
- Pass 1 D3 is resolved for the reviewed outer quote/list ordering. Container paths are persisted at `crates/rdocx/src/html.rs:3043` and emitted around list markers at `crates/rdocx-html/src/markdown.rs:379`. The focused control begins at `crates/rdocx/src/html.rs:4809`. The additional nested-grouping defect is recorded separately above.
- Pass 1 D4 is resolved. Thematic-break role styles survive quote projection at `crates/rdocx/src/html.rs:2998`. Paragraph and table container ownership is retained at `crates/rdocx/src/html.rs:3079`. Whole style definitions are compared at `crates/rdocx/src/html.rs:3130`, including the actual base style. The focused rule/table control begins at `crates/rdocx/src/html.rs:4817` and the collision control at `crates/rdocx/src/html.rs:4851`.
- Pass 1 D5 is resolved. Visibility trims only ASCII whitespace at `crates/rdocx/src/html.rs:4312`. The NBSP-only HTML, bare-fragment and Markdown controls begin at `crates/rdocx/src/html.rs:4823`.

## Evidence and limits

Authenticated the retained actual receipts and their logs against the remediation checkpoint. The final native HTML selection passed 32 controls. The retained emitter selection passed 14 controls. Scoped Clippy, Rust 1.93 locked offline library checking and formatting exited zero. The final native and Clippy receipts qualify the current complete source snapshot. The emitter and preceding scoped-check receipts qualify their recorded earlier snapshot, with unchanged emitter carriers and the recorded successor qualification. They are not described as fresh execution over every current native carrier.

The genuine earlier RED receipts remain actual failed executions, including the five-defect, extended-container and quote-owner controls. They were not counted as passes. The retained coverage includes distinct outer quote grouping, numbering, save/reopen/reimport stability, style collisions and template preservation. Its nested quote fixture has only one inner paragraph and does not decide D1 above.

There are no additional findings in source-offset, alt-text, resource/template preservation, whitespace, code, table or dependency contracts within this bounded remediation review. There are zero smells and zero nits. Missing future-phase acceptance is not treated as a native-phase defect.

## Review integrity

HEAD, branch, physical index, logical index, binary tracked diff and SHA-256 mapping of all 3,197 tracked files were captured before review and checked immediately before writing this report. The complete tracked-source aggregate is `ae55fdf98f9349c5afdf5a6fc46912b930ddb2e60fe49cece07f4db9747a18c9`. The tracked binary diff SHA-256 is `8eae2052e8cf44d80c2718a519159993bfb75eca4742073cd2bf1c407e5f3213`.

Only this review file was written. No source, plan, progress, HLD, test or formatting changes were made. No new execution suite, Cargo, CP or Office operation was run. The same complete guard was checked after writing the report. F-X215 source remains frozen.
