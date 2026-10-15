# F-X195, SVG comparison normalizer, pass45

**Reviewed**: Independent external R30 tests-only proposal, one existing
carrier,27 added lines and1 removed line. **Verdict**: 0 defects,
0 smells,0 nitpicks. Unapplied and unexecuted by this review.

## Identity

Read workflow/microscope, current approved F195 plan, its rendering/testing
contract, current progress and independently clean pass37. Both worker
ledgers retain in-progress. Branch is work/f-x195-codex at
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. Existing tracked implementation
changes remain present. Only this canonical review artifact is written.

Authenticated receipt
d847d31856154884c214d734eb7eef213513900e1a9c3d424baef49c069480aa,
source.diff
02f7b4975560d4e837d4ac0c78291cdbc2cbb2055482ae2256e8cab050b8e638
and complete candidate
206c8ee8d6220d7889c54637450a1186f7c1eba3bfa348bdca95ef38076dff46.
All2679 tracked source rows and317 runtime inputs match their declared
current maps. Independently reconstructing the patch reproduces the entire
candidate. Its AST parses. The only modified existing function is
_fx195_variable_svg_geometry. The sole added test is the focused pure
normalization control. Every other existing function and decorator AST,
including full104 paint cases and physical-source checks, remains identical.

## Actual failure and causal scope

Authenticated retained Word-installed.log SHA
0e50112514fdad0f5ebb9776bb8ecd6650d4457e52bacf10ed571c8e78e6c49d.
It records55 passed and1 failed, specifically the400 render_page_to_svg
variable case. The original terminal failure remains immutable. No success
is inferred for the pending PPT or mandatory native SVG stages.

Authenticated causal-svg-pair.json
 d66c855ea3a55061c4abf4bc1f6a541d6d80865b1aadbcc38fa98c0e412f65d3.
Independent standard-library parsing of both retained strings finds exact
expanded tag names, sorted attribute pairs, text, tails and recursive child
order. Direct standard C14N with prefix rewriting agrees. This is XML
metadata inspection of retained evidence, not candidate helper execution,
product import or new geometric acceptance.

## Comparison assessment

Candidate test_rendering_threads.py:924 still invokes the complete physical
source/effective-axis/owner validator before removing approved font-definition
and selector representation differences. Its original checks remain exact:
actual physical font hash, authenticated400 defaults or explicit700 axes,
all five Thai owner texts and partitions, positions, font size/weight/style,
and synthetic-operation refusal. Original raw SVG artifacts remain deciding
font provenance, rather than being replaced with normalized text.

At candidate test_rendering_threads.py:942, local C14N with rewrite_prefixes
replaces lexical prefix-dependent serialization. Strip-text is not enabled.
Expanded element and attribute names, attribute values, remaining text/tails
and child order remain in the comparison. Existing style removal is unchanged.
The correction does not modify SVG generation, font selection, contours,
Q/C images, static-owner positions, ROI, negatives or0.99 thresholds.
This bounded generated-SVG geometry comparator is not an arbitrary XML
roundtrip serializer or a new general opaque-XML preservation policy.

At candidate test_rendering_threads.py:1927, the pure test stubs only the
physical-owner validator to isolate normalization. It does not bypass that
validator in any real installed case. Default and prefixed equivalents
compare equal. Changed x/y, viewBox, text, tail, xml:space, another attribute,
namespace URI and child order remain unequal. monkeypatch restores the
existing module function after the test. Standard sys and XML APIs are
available in the current module/runtime. No product package import is added.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Aspects

- Correctness: zero findings. Local canonical prefix assignment addresses the retained lexical mismatch without coordinate or rich owner changes.
- Contract: zero findings. Original raw physical-font proofs, all104 outcomes, both Q/C comparisons and mandatory native eight-owner gate remain strict.
- Panics: zero findings. Parsing/normalization failures remain test failures, not passing fallbacks.
- OOXML: zero findings. No package or production mutation occurs. The change is confined to a qualified generated SVG comparison projection.
- Tests: zero findings. Negative semantic mutations remain deciding and all existing acceptance function ASTs are preserved.
- Structure: zero findings. One existing carrier and one focused test, without a new product file, module, API, dependency or wrapper.

## Limits and handback

No compiler, Cargo, product test/import, painter, native application, cache,
mtime, baseline or source adoption ran. AST and retained XML metadata checks
are not an actual pure-control or installed-case pass. ROOT must execute the
new pure control and failed installed case under refreshed current-source
admission, preserve original55/1 history, then complete full104 and mandatory
native1passed0ignored with both Q/C references and remaining feature gates.
No feature completion or acceptance waiver is recorded. Review phase ends.
