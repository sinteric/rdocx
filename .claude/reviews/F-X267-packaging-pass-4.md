# F-X267, packaging, pass 4

**Reviewed**: Prepared F-X267 at SourceHead df59c6731c3a461c38331bb92c9c8d50ab0405a9, tip 58c3e0cf252ad31ffc5c77723b21ca70b72a669e, Base 4998e398107e204c7cd81b938af2d3610e58b88b. Targeted integration acceptance reconciliation against F-X264's shared writer. This is a newly identified contract interaction, not a confirmation pass after the clean pass 3.
**Reviewer**: Codex root independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D3, The reread shortcut bypasses the shared ZIP local-name limit

`crates/oxml-opc/src/package.rs:933`
`crates/oxml-opc/src/package.rs:328`
`crates/oxml-opc/src/package.rs:515`
`crates/oxml-opc/src/package.rs:1402`

written_as_is admits a key shorter than u16::MAX minus the relationship suffix allowance. F-X264's shared writer also reserves the 30-byte local header and rejects an actual entry name of 65505 bytes. For example, a candidate edited from a valid immutable read baseline can add a binary part with a leading-slash stored key of 65506 bytes, producing an entry name of 65505 bytes. This passes the shortcut's length guard. With otherwise valid metadata and payload, reread can return Ok even though write_to rejects the name. Relationship entry names have the same mismatch.

This violates A9's exact save/read success and error equivalence. The existing shared-writer boundary regression is not exercised by the new fast path. The debug saves_as comparison can also expose the disagreement, while release builds lack that guard.

Add a focused regression on the current prepared source before repair. Compare write/read with reread at both sides of the actual ordinary and relationship entry-name boundary, using a valid unmodified read baseline. Include an invalid XML payload combined with a long name so the writer's error precedence remains authoritative. Compilation or harness failure is not RED.

Route unproved or writer-rejected entry names through the existing ZIP fallback. Reuse the shared local-name validator with the actual serialized name mapping rather than adding a separate guessed constant. A conservative fallback for unusually long otherwise valid names is permitted. Do not return a different early name error ahead of the writer's existing validation order.

## Smells

None.

## Nitpicks

None.

## Not found

No new public signature, opaque-byte, child-order or owner-lifetime issue established in this targeted check. The three Q13 changes remain separately labelled and the original M21 expectation remains unmeasured. Prior clean review and local evidence retain their actual scope, but do not establish this newly examined boundary.

## Acceptance boundary

Do not integrate or consume the prepared handoff until D3 is repaired, locally verified and independently reviewed. Update SourceHead and replace the stale handoff after the repair. This finding needs a bounded OPC regression and affected scoped checks, not another full suite, performance run, oracle or baseline update. Root changed no source during this review. This pass ends and returns to orchestration.
