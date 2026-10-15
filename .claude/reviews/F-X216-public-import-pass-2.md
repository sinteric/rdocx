# F-X216, public import precedence, pass 2

**Reviewed**: bounded current foreign-layout validation order in frozen worker slide.rs77569053b957492784767276048f2457709ad04ab2b59b72b86c537de59f771c. Related owner and inherited test paths are source observations only.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. No full public-story verdict.

## Defects

### D1, Stale destination collection masks the required foreign-layout error

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-py/src/slide.rs:1142`, `:1246` and `:1286`

import_options calls self.len, which validates destination collection retirement, before checking the supplied layout's presentation identity at :1146. Existing add_slide and add_slide_at do the same before identity checks at :1247 and :1287. A valid caller can retain destination.slides, append a slide, then pass a live layout from another presentation to that stale collection. Current code returns StaleElementError for the destination collection instead of the mandated foreign-layout ValueError.

This state is not hypothetical. The slides factory binds the actual slides inventory at /private/tmp/rdocx-f-x216-codex/crates/rpptx-py/src/presentation.rs:1941 and :2986 through :2992. The existing test at /private/tmp/rdocx-f-x216-codex/crates/rpptx-py/tests/test_documented_examples.py:262 through :273 proves append retires the retained collection while layouts and existing physical owners survive. The canonical contract at /private/tmp/rdocx-s91/docs/hld/10-bindings-spec.md:65 through :66 requires foreign presentation identity before stale lookup or mutation.

Check destination-layout identity before any destination collection or layout stale lookup. Add a defining stale-destination plus live foreign-layout control for single import, diagnostic companion, batch import and the two inherited add methods, with byte/state rollback. A valid local layout supplied to the same stale collection must still report the original first invalidating append cause. Cross-presentation source Slide remains intentionally admitted for import and must not become a foreign-owner refusal.

## Prior review boundary

Public import pass1 missed this ordering defect. Its preceding static zero verdict does not close the newly evidenced path. This pass reports the confirmed issue against the actual current frozen source rather than inferring safety from an earlier review or unrelated successful tests. No runtime failure or acceptance result is manufactured by this audit.

## Not found and source pins

Smells:0. Nitpicks:0. No broader import architecture, native graph, scope migration, artifact or full-story review was attempted. Source snapshots follow.

```json
{
  "crates/rpptx-py/src/slide.rs": "77569053b957492784767276048f2457709ad04ab2b59b72b86c537de59f771c",
  "crates/rpptx-py/src/presentation.rs": "cb9d189d9e9a22c837210be92b624b6cf619fac793e7b05abcb297f225795beb",
  "crates/rpptx-py/tests/test_documented_examples.py": "7b9d22af0cb2eaed24ad916c9849146edb75840dc4e12597444c3ad4ae2582bb"
}
```

Only this canonical review file was written. Frozen slide.rs hash remained unchanged at audit end. No source patch, test, probe, build, Git operation, CP or application ran. Review ended and returned control for distinct remediation.
