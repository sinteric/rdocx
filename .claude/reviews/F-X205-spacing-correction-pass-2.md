# F-X205, spacing correction, pass 2

**Reviewed**: Two complete immutable carriers and their narrow correction against source6. Original pass1 categories and unrelated implementation are not re-audited.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority

Capsule `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/review1-correction-capsule`, manifest SHA256 `619ed4a9de276b000d240b14f30baf9762f71a38def5b88c65bfb04462a8bcdd`, correction diff `8c6f0a68213fcff7632d4304dad5fb7f90824c5caf7b65b5b9fc68eaa45f160d`. Both after references authenticate against the original source6 preimages. Text source after `45258ad103a8a5e72072bfdb197e6537d3abc65501a3d6a89d963984ceb03ed4`, test after `1e7505cd92e59dcaf59a27345f7f8914957cca6faa61dc3532e15323f783757d`. Actual installed correction acceptance remains null.

## Resolution

Pass1 D1 source logic is corrected: UniversalMeasure now converts every admitted unit mm, cm, in, pt, pc and pi to checked EMU, rejects malformed or nonfinite/out-of-Length-range values, and truncates before constructing Length. Absence alone returns None. The parser's admitted signed decimal grammar excludes scientific notation, so the unit boundary matches admitted source forms. No source normalization or getter write is added.

Pass1 D2 is resolved: the bullet-size documentation now explains that None means no percentage value, including retained point-size values. The setter continues to clear only the selected explicit size owner.

## Defects

### D1, new universal-spacing fixture never creates the rPr it tries to replace

`crates/rpptx-py/tests/test_documented_examples.py:8943`, `:8946` and `:8947` inside the correction capsule.

The source-built helper creates a fresh regular run without direct formatting. It does not create an a:rPr. Native `crates/oxml-drawing/src/text/paragraph.rs:3296` constructs that run with properties=None, and its writer at `:3434` emits a:rPr only for Some(properties). Python add_run at `crates/rpptx-py/src/text.rs:875` only calls this native constructor. The fixture then replaces a self-closing a:rPr and asserts exactly one match. The fresh blank-slide textbox input has no such direct property owner, so the seven cases stop at the substitution-count assertion before testing the corrected getter.

Create the selected real run's direct property owner before packaging, or insert a qualified rPr into that actual run. Keep the imported lexical spacing, no-write getter, unrelated edit and reopen assertions. Do not broaden the search to a different paragraph or inherited default owner. This finding is source-backed, no installed test was run by the reviewer.

## Smells

Zero.

## Nitpicks

Zero.

## Not found and limits

Correctness, contract and panics: zero additional findings in the correction logic. OOXML: source read remains nonmutating, selected units and unsupported errors explicit. Tests: D1 is the sole pre-deciding fixture gap. Structure: no new module, dependency, trait or speculative helper. No current installed, strict typing, rendering or whole-feature pass is claimed.

Only this external review artifact was written. No source, runtime, index or environment mutation occurred. Worker may copy the exact report under its source lease and remediate in a distinct phase.
