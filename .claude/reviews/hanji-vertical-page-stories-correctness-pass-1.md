# Hanji, vertical page stories, correctness, pass 1

**Reviewed**: working diff from 873df08, seven files, 946 inserted and 48 removed lines, plus the approved custom-task plan
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, keep-next lookahead still measures the conservative layout

`crates/rdocx-layout/src/paginator.rs:3629`

The current paragraph is rebroken at the selected vertical measure, but
`keep_next_chain_height` reads the following table through `table()` and the
following paragraphs through their initial lines. A shorter selected story can
therefore leave a keep-next chain measured at the narrower preparation width,
causing an avoidable page move. The lookahead must use `table_for_measure` and
rebreak unconsumed following paragraphs at the current measure before pricing
the chain.

## Smells

None.

## Nitpicks

None.

## Not found

No source dropping or duplication in the tested paragraph and plain-cell
continuations. Generated hyphens are excluded from logical consumption. Bidi
consumption precedes visual ordering. Public carrier fields are unchanged.
Existing overlap, merge, rotation, nested-cell, clipping and anchor guards
remain in place. Alternative numbering state is cloned before the primary
measurement. Formatting, strict scoped Clippy, changed-crate native tests,
WASM checks and the 49-entry deterministic hash check pass. Native Word
fidelity remains unverified. Complete final validation and a second review
are required after remediation.
