# F-X221, paint oracle remediation, pass 1

**Reviewed**: The working diff on HEAD `b2f5cd87f7475a9322eaa23620933028a2b29ebe`, one existing test file with seven insertions and no deletions. Its reviewed postimage is `37933f80832de8e15ebe95c00f8d6feb8e6be21fca7d00097072d1963fdf87db`.

**Verdict**: Zero defects, zero smells, zero nitpicks.

## Contract and fixture checks

`docs/hld/08-rendering-spec.md:2554` requires PowerPoint endpoint and mirrored three-stop gradients to retain channel-directed gamma1.875, with other stop lists linear. The resolver and its independent formula assertions remain unchanged at `crates/rpptx-layout/src/context.rs:4381` and `crates/rpptx-layout/src/context.rs:8043`.

The explicit solid fill at `crates/rpptx-py/tests/test_documented_examples.py:4375` removes inherited endpoint-gradient interpolation from the chevron geometry comparison. It changes neither the preset, geometry, text, import, relationship, native or oracle save/reopen assertions. The seven comparison windows and the exact24-level,97percent and mean3 limits remain at `crates/rpptx-py/tests/test_documented_examples.py:4453` and `crates/rpptx-py/tests/test_documented_examples.py:4474`. The retained diagnostic `0d4d291c7a4a357eee572812d3d62b0836f1f4474a3c0a283c5f9a00ece5ffcf` shows identical dark-pixel bounds, a passing geometry-window close fraction and a broad interior gradient difference. The fixture correction isolates the stated geometry obligation without weakening it.

The added midpoint at `crates/rpptx-py/tests/test_documented_examples.py:4529` makes the synthetic preservation sentinel a non-mirrored three-stop gradient. Its distinct original endpoints keep orientation observable. The original omitted angle, scaled flag, exact saved gradient bytes, other preservation sentinel, original reporter input and all editing checks remain. The existing gradient window and its99percent and mean1 limits remain at `crates/rpptx-py/tests/test_documented_examples.py:4727` and `crates/rpptx-py/tests/test_documented_examples.py:4743`. Diagnostic `edc0771186d859a48966025bce88da06d95f81797d9ba9e0579d3a9a08c7b6af` distinguishes matching horizontal orientation from the approved native gamma versus LibreOffice linear interpolation. This changes the synthetic common comparison input, not the provider or the reporter's original fixture.

## Evidence boundary

The prospective candidate's complete file hash equals the reviewed postimage. Receipt `f0dc4496765ba9d7ad2b1c885c780e94df7f51fc37db5ec5ab9c4e5a5e9085a1` records both complete existing functions returning successfully with safe process cleanup and authenticated logs. That evidence remains prospective execution, not canonical pytest or full-suite credit. The original two failed comparisons remain retained. Canonical successor execution and the final full gate remain required.

The active delivery authority remains in progress at `docs/sprints/CURRENT_SPRINT.md:58` and `docs/sprints/BACKLOG.md:786`. The plan's completed local preparation does not claim the pending post-integration gates passed.

## Findings

Defects: zero. Smells: zero. Nitpicks: zero.

Correctness, contract, panic and bounds exposure, OOXML ordering and preservation, test strength and structure produced no findings in this bounded diff. No product code, baseline, threshold, comparison region, dependency or new abstraction changed. This review executed no tests, renders, builds or environment changes and wrote only this report. The review pass ends here.
