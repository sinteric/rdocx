# F-X220, archive metadata, pass 1

**Reviewed**: Supplemental metadata delta after whole-feature pass2. Nine existing carriers, eight README archive rows and eight matching measurement tuples/dates in `scripts/readme_doctests.py`. The delta contains24 added and24 removed lines. All other3,196 tracked carriers remain exact to the reviewed timeline checkpoint. HEAD is `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`, branch `work/f-x220-codex`.

**Verdict**: Zero defects, zero smells, zero nitpicks. This accepts the bounded archive metadata maintenance, not full feature completion or native application authority.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero findings. Independently opened all sixteen retained archives across both measurement generations, authenticated every member hash/length, and recomputed compressed bytes, normalized member bytes and member counts. Final rows and tuples agree exactly on normalized payload and count. Compression differences remain within the unchanged64-byte allowance at `scripts/readme_doctests.py:394`.
- Contract: zero findings. The measurement work satisfies the affected-archive obligation at `.claude/plans/F-X220-design.md:99` without changing versions, API recipes or acceptance requirements. README rows retain their existing reproducible measurement command and platform fields. Only observed values and dates change.
- Panics: zero findings. No production execution, parser, allocation or indexing code changes. The script delta is existing dictionary values only at `scripts/readme_doctests.py:378` and `scripts/readme_doctests.py:400`.
- OOXML: zero findings. All model, serializer, font codec, package graph and preservation sources remain byte-identical to the preceding reviewed checkpoint. This delta cannot alter XML ownership or schema order.
- Tests: zero findings. The archive normalization at `scripts/readme_doctests.py:1292` and validator at `scripts/readme_doctests.py:1337` are unchanged. Exact member/count comparisons, the10MiB ceiling and pinned-toolchain compression tolerance are retained. Actual dry-run logs authenticate eight packaged and eight verified crates in each generation. No runtime, hash baseline or expected rendering output changed.
- Structure: zero findings. No product file, module, dependency, abstraction or second measurement mechanism was introduced. Existing README rows and their existing canonical dictionaries are maintained together.

## Independent archive observations

The final actual retained archives have these recomputed measurements. Recorded compressed observations come from the first verified generation. All final normalized totals and counts match their recorded values exactly.

| Crate | Final compressed bytes | Normalized member bytes | Members | Final compressed delta |
|---|---:|---:|---:|---:|
| oxml-layout | 4,899,757 | 9,820,916 | 60 | +11 |
| rdocx | 3,663,655 | 51,535,281 | 37 | +3 |
| rdocx-cli | 95,728 | 441,448 | 8 | -1 |
| rdocx-layout | 322,245 | 1,777,332 | 15 | +4 |
| rdocx-oxml | 489,655 | 3,031,505 | 32 | +2 |
| rpptx | 609,494 | 3,213,158 | 16 | 0 |
| rpptx-cli | 59,016 | 270,563 | 8 | +1 |
| rpptx-oxml | 200,562 | 1,286,086 | 20 | 0 |

Every archive is below10MiB. Every ordinary member matches the appropriate frozen source body for its generation, including `Cargo.toml.orig` and the Word root README mapping. Member names are unique, regular files, within the exact package prefix and without parent traversal. Each final retained copy matches both actual tmp-registry and tmp-crate candidates byte-for-byte.

Generated manifests have the actual package name/version, normalized local dependency versions and no published local path. Generated locks parse as valid package inventories. Generated VCS records contain the actual reviewed HEAD, dirty status and correct crate path. Normalized totals independently remove the dirty marker and replace the revision with40 zeroes using the existing encoding convention. No generated carrier was mistaken for an ordinary source file.

## Authentication and boundaries

Frozen checkpoint SHA-256 is `9ff57c10dd72d39b0686ef4c8e21f68ef8eb30d98ede28583532320cc0f49562`. Metadata delta SHA-256 is `a034daac7c775df1f3d885eaba91825e1e4ae1b2870efa48d18eabc3d66157b6`. Current full diff SHA-256 is `3a86aff59c9e1e51a5a27298cf44c14f5d09aa7c7d05b3ae0ab26e7597451f38`. Current source map SHA-256 is `a12e60f953c9953c899e37a69d8c6ca84d54509133f1815df0dcd7c8822d4af2`.

Authenticated all3,205 tracked bodies, sizes and modes, exact nine changed carrier hashes, unchanged HEAD/branch and physical/logical index. The prior whole-feature pass2 and timeline source checkpoint retain their reviewed implementation scope. The metadata supplement introduces no product behavior change.

Both actual dry-run receipts, raw log hashes, retained copies and complete inventories authenticate. Final inventory SHA-256 is `c28ae582c297bf28c8d362bc9f4fdb024fc491230ad6e8413b09c55dbe0abe48`. Final validator receipt SHA-256 is `21542eb44a5b5c647e6657a6ee0abd29b6d73cb4bc7f7bf00151a70df5edecd0`. Final actual dry-run receipt SHA-256 is `f9d7e72968ca85305c553a87b1a572aab36deea4df76eb3536b0acdbb4727fa0`. Its command explicitly retains archive verification and uses `publish --dry-run`, without upload. All recorded owned process groups are gone and leaders reaped.

The original safe-risk command failure remains retained as exit101 before compilation. Its distinct successor receipt records the corrected ten-crate MSRV checks, no-default tests and owned advisory qualification. This review neither relabels that original nor reruns those gates. The eight-archive phase does not qualify the other fourteen published crates or replace the final integrated policy gate.

Native Office producer, uninstalled-face use and authentic font-bearing corpus authorities, final integrated scoped acceptance, structured handoff and integration remain separate requirements. Missing historical goldens retain no oracle credit. Root's later LO and managed Python successors are separate evidence and were not repeated here. No feature completion or publication is granted.

Only this review file was written. No source, progress, plan, HLD, ledger, target, runtime, CP, UI, baseline or Git state was changed. No tests, Cargo commands or policy gates were launched. This review ends here and returns control to root.
