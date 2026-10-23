# F-X259, toc-and-cache-contract, pass 1

**Reviewed**: independent `/microscope F-X259 --working --aspect toc-and-cache-contract`, branch `sprint/s95.3`, HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. The 19-file scope envelope has 504 insertions and 16 deletions, including excluded shared-file hunks described below.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Scope and evidence limits

Read repository guidance, canonical WORKFLOW/microscope, the approved F-X259 plan and progress, and applicable HLD sections. CURRENT_SPRINT and BACKLOG both retain in-progress status. All 29 entries in `/tmp/s95.3-fx259-final-review-manifest.json` match the frozen files. This authenticates supplied source identity, not execution provenance by itself.

The verdict covers A8's small Issue400 selector correction, its native regression, five Python parameter cases, HLD08 range paragraph and HLD12 paragraph beginning “TOC range-composition regressions”. It also covers A6's latest canonical cache/publication and prerequisite-build instructions, five generated adapters, and the mechanical archive measurement/date coupling. Other test_core.py and HLD12 hunks, F-X260's plan, cache implementation and publication machinery are excluded. Consumed `.claude/reviews/F-X260-issue398-pass-5.md` without repeating that audit.

Consumed earlier F-X259 all-pass-1 and workflow-takeover-pass-1 conclusions. Unchanged STYLEREF/template repairs and prior process commitments were not re-audited. The earlier draft-plan wording nit is corrected. This is neither sprint review nor sprint closure. No builds, tests, suites, benchmarks, agents, external oracles, commits or state/progress changes were performed. Only this report was written.

## Defects

Zero.

## Smells

Zero.

## Nitpicks

Zero.

## TOC contract analysis

`crates/rdocx/src/field.rs:9065` applies both inclusive endpoints only to the final outline fallback. `heading_levels=None` accepts the previously admitted outline levels. The parser preserves that absence for outline-only instructions at `crates/rdocx/src/field.rs:14670`, while explicit heading selection supplies a parsed range. This rejects upper-bound leakage without imposing a new default on outline-only selection.

Explicit custom-style matching returns before outline eligibility and range filtering at `crates/rdocx/src/field.rs:9040`. The existing heading-style path remains separate. Qualified outline suppression returns before that path when u is active, and `heading_style_level` still admits only levels1through9 at `crates/rdocx/src/field.rs:9087`. No Heading0 admission, body-outline9 resurrection, new cascade or XML writer is introduced.

The native regression at `crates/rdocx/tests/regression_test.rs:406332` combines Heading1/Heading2 style-derived levels with direct outline0/outline1 paragraphs. Seven instructions distinguish upper and lower endpoints, wider inclusion, h/z, u/no-u, outline-only and an independent explicit custom-style mapping outside the heading range. Exact entry text and count are checked after initial rebuild and after save/reopen followed by another rebuild. This rejects maximum-only filtering, unconditional default ranges and blanket filtering before custom selection. Existing direct/inherited body-outline9 controls remain at `crates/rdocx/tests/regression_test.rs:405483`.

The five Python cases at `crates/rdocx-py/tests/test_core.py:10085` exercise the public binding with actual heading styles and direct outlines. They inspect saved/reopened entries before the second rebuild as well as afterward. Python reaches native rebuild at `crates/rdocx-py/src/document.rs:11341`, and CLI reaches it at `crates/rdocx-cli/src/commands.rs:1951`. Neither supplies another range policy. HLD08 at `docs/hld/08-rendering-spec.md:2171` and HLD12 at `docs/hld/12-testing-strategy.md:3336` accurately describe the bounded composition and distinguish consumer evidence from Word parity.

The plan pins Issue400's body and PR401's head. Its explicit contract governs this correction. Candidate algorithms and contributor Word observations do not become authority or workspace-native evidence.

## Recorded execution evidence

Inspected `/tmp/s95.3-issue400-red.log`: the actual original selector produced four entries instead of two. The subsequent native log records 93 passed and one ignored. `/tmp/s95.3-issue400-python.log` records 14 passes, comprising five in-scope TOC cases and nine excluded F-X260 cases. The latest `/tmp/s95.3-opaque-section-final-green.log` records 75 passes, comprising the same five TOC cases and 70 excluded cases. Neither aggregate is presented as that many new TOC tests.

The CLI help/build log records successful source compilation. Five CLI probes record the executable path/hash, actual child exits, JSON counts, expected selected texts, input/output hashes and unchanged input bytes. These are constructed consumer controls. They establish no native Word parity. The earlier 470-pass Wordcore observation predates opaque-section changes and is not complete current-source qualification. The running final integrated native gate is not counted as passed here.

## Cache and process contract analysis

`commands/design.md:165` under `.claude/` now names source facts, preparation boundaries, all fact-changing publishers, unchanged scope/revision paths, mirrored owners and read/write physical-owner equivalence. `commands/implement-feature.md:40` requires preparation or invalidation before publication and native/held/fresh/reopened comparisons without weakening handle lifetime. These work with existing consumer maps and publisher inventories rather than creating another tracker.

`.claude/skills/differential-testing.md:84` adds mixed controlled/direct coordinates, unchanged neighbors and logical-owner coverage. The surrounding bounded matrix includes nested owners. `.claude/skills/risk-routing.md:35` adds obligations without replacing ordinary gates. The instructions are model-neutral. WORKFLOW explicitly attributes the trigger to Codex-authored cache defects at `.claude/WORKFLOW.md:154`.

`.claude/commands/verify.md:36` stops dependent batches after failed prerequisites and retains separate child-exit, artifact and semantic checks. Its 8MiB workspace test-thread stack matches HLD15 and CI, without changing runtime limits. Independent review, native evidence, strict ratios, normal scoped/full boundaries and the original three-pass sprint-review bound remain intact. No new waiver or operational contradiction was found. All five adapters contain the actual SHA-256 of their canonical source.

## Archive coupling and remaining qualification

README's row matches the script's tuple: 3,851,059 compressed bytes, 52,612,374 normalized member bytes and 37 members, dated 2026-10-22 on the same macOS26.6.2 platform. The supplied archive log supports those values. The 18,650-member-byte increase is exactly explained by current document.rs, field.rs, regression_test.rs and integration_test.rs byte deltas: 11,107 +196 +3,613 +3,734. README's byte length is unchanged. This is integrated-source inventory growth, including excluded F-X260 source, not an unexplained algorithm measurement. Other recorded rows and the 64-byte compression tolerance remain unchanged.

Correctness, contract, panics, OOXML, tests/evidence, structure and process consistency each produced zero findings in scope. The successful 190-policy-test observation has two skips and predates the final prose additions. Current sync/prose/diff checks are reported successful, while final metadata policy observation remains pending. Full README validation, final integrated gates, native Word and font qualification remain due. This clean focused review supplies no acceptance waiver.
