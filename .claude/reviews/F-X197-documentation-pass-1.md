# F-X197, documentation, pass 1

**Reviewed**: Four documentation files, 139 added and 39 removed lines,
against the R15 accepted source snapshot. Exact incremental patch is
`/private/tmp/S91-F-X197-doc-remediation-r16/doc-only.diff`, SHA-256
`e8d2d3e05f8a83b00ed67935bad59e98e94da780a0ef8b2de4bd118a93d16c4b`.
The ignored target link is inspected as environment setup, not a source edit.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: the exact approved HLD patch is applied only to
03, 10 and 14, matching the plan's HLD impact. The shared extractor ownership
at `docs/hld/03-architecture.md:72` agrees with the existing PyO3 dependency
without support extension-module and the concrete helper at
`crates/oxml-py-support/src/lib.rs:13`. Neither facade nor WASM graph gains
a Python support consumer.

The live getter and scalar migration at `docs/hld/10-bindings-spec.md:240`
agree with Word `crates/rdocx-py/src/formatting.rs:427` and PowerPoint
`crates/rpptx-py/src/text.rs:1440`. Font-only None clearing, auto, cached
Word literal and theme modifiers remain explicit. The supported enum tokens,
same-kind transform and opaque-child preservation, and shadow alpha exception
agree with `crates/rpptx-py/src/dml.rs:523`. Finite brightness and deliberate
bool/NaN oracle divergence at `docs/hld/10-bindings-spec.md:831` match
`crates/rpptx-py/src/dml.rs:665`, including round-half-even and non-luminance
preservation. No new behavioral claim exceeds the approved implemented scope.

Tests: the S91 table now uses the existing three-column placement grammar
at `scripts/test_sprint_workflow.py:7943`. All 22 ordered IDs, titles and sizes
remain unchanged, as does all worker text outside that table. Historical
worker counts are not copied into the canonical current ledger. No test or
assertion changed. The first failed whole-policy attempt remains retained,
with 141 tests, two failures and two expected skips. Named roadmap, prose and
diff checks passed in the implementation phase. Whole-policy retry is pending,
so this review does not grant policy or completion credit.

The previously absent ignored `target` link resolves to the existing shared
cache. It satisfies the real archive lookup at
`scripts/readme_doctests.py:1281` without creating a duplicate cache or changing
the validator. The earlier external claim that this link was unnecessary was
incorrect and is superseded by the observed failure and this bounded repair.

ROOT independently compared all 2691 tracked inputs against the R15 snapshot:
only these four documentation paths changed. All 318 accepted runtime input
hashes remain identical. The original full Python, native, rendering-hash and
verified-archive evidence remains source-qualified. No executable package
payload changed. Canonical integration must retain the F-X194 and F-X212 HLD
union, carrying the approved hunks rather than replacing whole older worker
files.

Panics, OOXML and structure: zero findings. This incremental change adds no
runtime logic, parser, serialization, type, module, crate or feature flag.
The original production and test reviews remain applicable to their unchanged
inputs. No new scope or network intake occurred.
