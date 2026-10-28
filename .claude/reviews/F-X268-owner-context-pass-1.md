# F-X268, owner context, pass 1

**Reviewed**: Focused contract follow-up on the frozen owner-piece working tree against `b1906e7cc8a8207c492c17877d35c758c2821079`. The broader prefix review leaves A3 context coverage incomplete.

**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, a changed context does not select the agreed full-reader fallback

`crates/rdocx-py/src/document.rs:5308`

`crates/rdocx-py/src/document.rs:5375`

When an existing cached body context differs from the current context, `assemble` stops reusing old blocks but still reads new pieces and returns `pieces.inventory(document)`. The caller can publish those addresses without invoking the old full reader. Mechanism 2 explicitly requires a context change to use the old full rebuild before publication. A fresh piece projection is not that fallback. The reviewed ordinary and splice cases have stable contexts and do not establish this branch.

Route a changed existing context to a `None` inventory so the caller executes the authoritative full path. Freshly read pieces may be retained for a later qualified warm call. Initial construction without a cached context remains a distinct cold state. Keep relationship checks, refusal semantics and the existing warm work bounds.

## Smells

None found in this focused pass.

## Nitpicks

None found in this focused pass.

## Boundary

No output corruption is inferred from the existing passing cases. This is a concrete control-flow mismatch with the approved fallback contract. The bounded owner-piece review and its actual warm-path evidence remain historical, rather than becoming whole-feature approval. Mechanisms 3 through 8, remaining A3 contexts and the final union stay pending.
