# F-X211c, header cache ordinary failures, pass 1

**Reviewed**: Frozen f2 source, the two actual ordinary-stack failures, their shared fixture and current source-preservation/cache contracts. Read-only diagnosis.
**Verdict**: 1 test-fixture defect, 0 product defects, 0 smells, 0 nitpicks.

## Defects

### D1, the purported cacheable watermark fixture now retains an unsafe raw metadata record

`/private/tmp/rdocx-f-x211c-codex/crates/rdocx-layout/src/engine.rs:16024`

`image_watermark_header` declares VML and relationship namespaces at the
header root while its retained raw pict uses those inherited bindings.
The current source-preserving Run parser collects bindings used by raw
children at `crates/rdocx-oxml/src/text.rs:3385` and appends their synthetic
root-binding record to extra_xml at `text.rs:3429`. This correctly retains
namespace authority instead of silently losing it. The record is not a
qualified Word pict.

The cache deliberately admits a run only when every retained raw record is
Word pict at `crates/rdocx-layout/src/engine.rs:5813`. The additional binding
record therefore makes the default watermark header uncached. The other five
plain header/footer variants remain cacheable. The uncached branch at
`engine.rs:11779` bypasses the build counter, which increments only for an
admitted cache construction at `engine.rs:11858`.

The shared fixture at `engine.rs:16043` nevertheless promises six cacheable
variants. Its cold expectation at `engine.rs:16132` consequently observes
five instead of six, and the two-layout invalidation expectation at
`engine.rs:16115` observes ten instead of twelve. Both actual failures follow
from this same stale fixture classification. All six visible header/footer
text assertions at `engine.rs:16125` through line16131 completed before the
cold count failure. This is not an observed missing-header rendering defect.

Keep the deciding six-variant and twelve-rebuild expectations. A normal
remediation can make this fixture's retained pict namespace-self-contained,
so the parser does not need a separate inherited binding record. That retains
its image-watermark/cache coverage. Lowering the counters would reduce that
coverage and also misclassify the later deliberately unsafe variants at
`engine.rs:16312`. Broadening cache admission to arbitrary raw records is not
justified by these failures.

## Smells

None.

## Nitpicks

None.

## Contract and source comparison

The conservative boundary matches `docs/hld/08-rendering-spec.md:1434`, which
leaves rich paths uncached when they cannot satisfy cache safety. Preserving
source namespaces and raw XML remains mandatory. F-X201's watermark contract
and the d HeaderFooter publication route do not require caching an unsafe
retained record. This fixture constructs LayoutInput directly and does not
call the Python HeaderFooter publication route.

Independently compared the current cache predicate, raw-pict recognizer,
paragraph cache predicate, fixture constructors and both affected tests with
completed d feature commit171f398f. All seven function bodies are byte-exact.
The new unknown-symbol engine additions are confined to numbering/explicit
symbol diagnostics and their tests. They do not change these cache functions
or add symbols to this fixture. This report does not attribute the failures
to those unrelated warning additions.

## Actual evidence and limits

The original raw log is
`/private/tmp/S91-F-X211c-final-typing-paired-producer-1/actual-scoped-remainder-continuation-1/rdocx-layout-ordinary-lib-runtime.log`, SHA77d5ea9910e6f40cafe4348946a84cd3e907fc0763eef17272a82ce79e9ed624.
Its receipt SHAc6c6b24498a0313f9f1f72e5e6e404a14e47c5132bb74a8190b6f89d9f76c3e0
retains actual exit101,330 passes,two failures,zero ignores and two filtered
controls, with safe cleanup. No successor or unexecuted native control receives
credit. The source diagnosis above is static, not a newly executed parser or
cache instrumentation experiment. Original frozen100 outcomes are unchanged.

Only this review file was written. No source/test expectations, harness,
ledger, runtime, target, CP, UI, network or Git state was changed. The pass
ends and releases the canonical review-write lease before remediation.
