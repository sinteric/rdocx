# F-X268, final performance, pass 9

**Reviewed**: Frozen working diff against `461eee97e1aa58b460bce4658f5a58453eaf9351` on `sprint/s95.4`. Six files, 1450 insertions and 146 deletions. Read the canonical workflow, approved contract, HLD changes, progress record and pass 8. Product changes are unchanged from that pass. This pass focuses on the replacement A9 semantic oracles and cold isolation.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. These are acceptance-test defects. No product correctness defect was established.

## Defects

### D1, control postconditions use incompatible coordinate systems

`crates/rdocx/src/document.rs:49739`
`crates/rdocx/src/document.rs:49755`
`crates/rdocx/src/document.rs:27109`
`crates/rdocx/src/document.rs:16985`

Pass 8 D1 remains open for control destination proof. The new checks execute after each action, which fixes their placement, but they assume that projected story locations form a hierarchical path or correspond directly to modeled body-child indices. Neither assumption holds.

`story_items` enumerates all projected items and gives each a flat `vec![index]`. The scanner includes nested content controls and drawings in that enumeration. Therefore the moved inline control and its enclosing destination paragraph have different one-component paths. A successful inline move cannot satisfy `moved.index_path.starts_with(&target.index_path)`.

The block end check compares the moved control's flat projected index with `document.document.body.content.len() - 1`. The fixture has an inline control and a drawing in every 25-paragraph group, each projected separately from its enclosing body paragraph. Those extra projected items are absent from the modeled body's direct child count. A correctly appended block therefore also fails this check.

Resolve the actual source and destination physical spans or actual checked owners for the semantic oracle. Prove containment and paragraph-relative placement for inline controls and direct-body order for block controls using the corresponding coordinate space. Keep these checks outside the frozen counter window. The new identity/payload equality and per-step checks are useful, but these invalid coordinate assertions prevent the required seven-family test from qualifying a correct implementation.

This is a source-established failure, not a reviewer-executed test result.

### D3, picture insertion oracle confuses a media filename with drawing metadata

`crates/rdocx/src/document.rs:49672`
`crates/rdocx/src/document.rs:49682`
`crates/rdocx/src/document.rs:28092`
`crates/rdocx/src/document.rs:772`
`crates/rdocx-oxml/src/drawing.rs:240`

The measured call supplies `a9-insert.png` as `image_filename`, then the oracle selects inserted occurrences by `PictureOccurrence.name == Some("a9-insert.png")`. The filename names the imported media input. The called overload creates default `PictureOptions` with `name: None`, and `picture_drawing` copies that separate metadata field into the inline drawing. Its writer defaults the drawing name to `Picture`. Thus a correct insertion does not produce the name required by this oracle, and the first inserted-count assertion fails.

Identify the newly inserted occurrence through the returned location plus an independent physical inventory, or use another discriminator consistent with this API's actual contract. Preserve the measured public operation rather than changing it merely to satisfy the oracle. Also verify immediate insertion after the requested paragraph. The current comparison at `crates/rdocx/src/document.rs:49700` accepts any later paragraph, so appending to the story end could satisfy that placement check once the name issue is corrected.

This defect is in the acceptance fixture, not in filename handling by the product.

## Resolved from pass 8

D2 is closed by source inspection. `crates/rdocx/src/document.rs:49830` opens each family's documents independently from fixture bytes. Shared parsed-state and body-scan Arcs from preceding families are no longer cloned into later cold cases. Each family's own target acquisition stays outside counting, as intended.

D1 is partially repaired. Cell checks inspect every physical cell after every call, so the alternating wrong-cell counterexample is rejected. Resize checks verify every occurrence's dimensions and image bytes after each call. Comment checks verify both endpoints and run indices after each call. The counters are frozen before these oracles run. Control checks now run per action and compare identity and payload, but their destination assertions need the correction above. Picture insertion also needs D3 addressed.

## Smells

None.

## Nitpicks

None.

## Not found

No new findings in product correctness, contract scope, panics, OOXML preservation, error ordering or structural rules. The unchanged product batch retains the pass 8 audit of immutable namespace sharing, actual borrowed parsed children, complete frame and child admission, separate writer proof, resource checks, private-field fallbacks and optional physical-cell routing. No new public API, retained cache field, source file or module was added.

Running the native count gate in release profile retains `cfg(test)` instrumentation and production checks. It does not supply the separate debug agreement evidence or installed-package timing evidence. Those remain distinct gates.

## Source identity and evidence limits

- `crates/rdocx/src/document.rs`: `48cc77e85f22d4d318ad18f77b0564347b07d67872798ef7ae7d92eb8ef579f4`.
- `crates/rdocx/src/content_control.rs`: `baea0de088d90910c7e3e9a3525d8fd11fb7c86b135d386124c6f14f416dfc18`.

At inspection, `/private/tmp/s954-a9-release-count-pass8-fixed.log` was compiling and contained no completed test result. The earlier immutable-iterator compilation failure is setup evidence, not a semantic RED. No seven-family count pass, new release timing pass, debug proof pass or memory/native oracle qualification is inferred. The previous V5 timing misses remain unresolved evidence.

The reviewer ran no builds or tests and changed no source. Return to implementation for remediation.
