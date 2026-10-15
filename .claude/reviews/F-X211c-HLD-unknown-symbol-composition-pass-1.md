# F-X211c, HLD unknown-symbol composition, pass 1

**Reviewed**: Five approved HLD impact files, 31 added lines, composed with the earlier seven-file HLD reviews and actual sealed source 8cb3efa3c40b760a057a32820377f9e02ab0c6bdfec890bc9cde39f8cc16cd59.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Source custody and review boundary

The inspected font, PPT renderer text, Word engine, Word deciding Python tests
and both public binding carriers independently match the sealed source map.
The prior product review remains separate authority for its actual scoped
native evidence. This pass audits current-intent HLD composition only.
The five files are members of the approved c impact list. The prior seven-file
HLD review remains valid for unchanged prose.

## Not found

- Semantic/glyph conflation at `docs/hld/04-opc-and-packaging.md:1883` or `docs/hld/10-bindings-spec.md:3684`. The actual query at worker `crates/oxml-layout/src/font.rs:1782` reports unknown mapping while retaining the original family/code, without asserting absent fallback glyphs. Physical requested-family identity and original-code coverage at `font.rs:1768` cannot be established by caller labels alone. Known mapped replacements and genuine physical coverage suppress the report.
- Global-history or bound drift at `docs/hld/06-presentationml-model.md:509`. The query admits only one canonical family and low-byte/private-use original scalar domains, at most 512 per family. The two families therefore admit at most 1024 distinct unknown facts per local vector, not a new bound on all diagnostics. PPT local insertion at worker `crates/rpptx-render/src/text.rs:904` deduplicates in first-use order after accepted visible marker contribution at line885. Empty/discarded markers contribute no such facts. The pure query does not record FontManager history.
- Word consumer/source drift at `docs/hld/14-development-backlog.md:7543`. Numbering markers and explicit symbols call the same query after successful shaping at worker `crates/rdocx-layout/src/engine.rs:8858` and `engine.rs:9752`. Hidden runs exit before that branch at line9110. These paths retain modeled source codes and XML. Cache safety excludes numbering at line5621 and Symbol content at line5657. Fresh measurement and final diagnostic vectors at lines3512 and3897 prevent probe history from becoming final facts. Existing first-use local dedup is at line12675.
- Same-pass companion drift at `docs/hld/10-bindings-spec.md:3684`. Worker `crates/rdocx/src/document.rs:34248` and `crates/rpptx/src/lib.rs:2551` encode the successful layout whose owned diagnostics they return. Their Python companions at `crates/rdocx-py/src/document.rs:6493` and `crates/rpptx-py/src/presentation.rs:1038` return that payload and diagnostic tuple, without a second layout to collect diagnostics.
- Test/asset drift at `docs/hld/12-testing-strategy.md:4213`. The test-only font-name helper at worker `crates/oxml-layout/src/font.rs:4295` updates the name table and checksums. The deciding physical-name control at line5407 compares the original glyph and all other retained tables at line5440. It adds no font asset and does not turn a misleading caller label into physical-family proof.

No full100 outcome, installed binding success, Office/native-output acceptance
or completed feature is asserted by these additions or this review. Those
remain separate required observations. No source, HLD, ledger, runtime, UI
or remote mutation was performed. Only this review file was written.
