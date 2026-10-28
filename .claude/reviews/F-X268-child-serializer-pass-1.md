# F-X268, child serializer, pass 1

**Reviewed**: Codex root reviewed the mechanism 7 serializer prerequisite on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The one-file diff in `crates/rdocx-oxml/src/document.rs` is 231 added lines.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None.

## Smells

### S1, frame serialization duplicates the full writer's emission policy

`crates/rdocx-oxml/src/document.rs:4236`
`crates/rdocx-oxml/src/document.rs:4194`
`crates/rdocx-oxml/src/document.rs:4251`
`crates/rdocx-oxml/src/document.rs:3954`

body_frame_xml adds another implementation of declaration, root, background, body start, final section and closing-tag emission. root_start and the section serializer are shared, but the surrounding emission order and optional-background policy are copied. Future frame metadata or ordering changes would need updates in two paths. The prerequisite was intended to retain one writer policy.

Share the frame emission with the existing full writer while omitting child writes and deferring the conditional w14 declaration for the frame result. Preserve validation and error order. Do not implement a frame by serializing or cloning all body children. A concrete two-mode writer is a possible implementation, provided it adds no speculative generic or trait abstraction.

## Nitpicks

None.

## Not found

No additional correctness, panic, namespace or raw-admission finding was established in the inspected helpers. body_item_xml uses the existing root binding scope and body-content writer. Raw bytes remain passthrough, including empty, multi-root and unparseable bytes. The item need not belong to the body.

The new assembly regression and the existing span regression passed, 2 cases in 0.01s, in `/tmp/fx268-m7-serializer-test.log`. The scoped check passed in `/tmp/fx268-m7-serializer-check.log`. The existing writer preserves U+0001 in text. The fixture correctly compares those bytes instead of adding a rejection. A 33-deep table has the same resource refusal.

There is no native MainPartBlocks consumer yet. That consumer, holds_field, private provenance guards, staging instrumentation, composition, memory and all final gates remain pending. This is not mechanism 7 or F-X268 completion.
