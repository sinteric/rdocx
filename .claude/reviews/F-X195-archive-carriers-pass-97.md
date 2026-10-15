# F-X195, actual archive carriers, pass97

**Reviewed**: External nine-package measurement proposal, ten existing carriers comprising nine README rows and script constants. Canonical review-only lease. Current F195 worker includes separately adopted HLD08/10/15 and accepted baseline synchronization.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and measured evidence

Proposal receipt SHA256 `88d3eb99ffe349f16ecf1eb4bc576b282245a1b09c4e6533ac84058ac3d4a1ed`. Patch SHA256 `868a6499018a2fca42935382ea096af69702e64a505bfb6ffa72bc636c6c67ed`. Actual publish2 receipt SHA256 `3aa99b5f007ce51b4dbca66072e69ebdac3160c8be896647b628f3cf9d394873` records actual exit0, verified workspace dry-run without --no-verify, matching raw log and reaped/absent owned group. Original snapshot f7418cff remains immutable.

Independently authenticated all22 retained archive hashes, every recorded tar member hash/size/mode, VCS metadata and owned-source matches against that original published snapshot. Independently recomputed normalized measurements with the unchanged archive_measurement semantics. All22 archives remain below10MiB. The unchanged13 tuples have exact normalized member bytes/counts and gzip differences within the unchanged64-byte bound.

Authenticated current2679 worker records against post-doc-sync map `14cc4175ea2f248755c392aa622f6e1a058814fec32c903baf09229afa547773`. Its only four differences from the published source snapshot match the explicit doc-sync receipt. Source317 remains `4c67badc56194d029796b2426a17b1c79293f64f1ecde73c9d28da3758477d4d`. The proposal's ten actual preimages and candidate hashes match, and their independently reconstructed diff is exact.

The nine measured provisional tuples are compressed bytes, normalized member bytes and member count:

| Package | Tuple |
|---|---|
| oxml-layout | 4641873, 9312164, 51 |
| oxml-pdf | 83738, 390360, 14 |
| rdocx | 3557026, 50927097, 37 |
| rdocx-cli | 78258, 360252, 8 |
| rdocx-layout | 315013, 1706904, 15 |
| rdocx-oxml | 457660, 2842528, 32 |
| rpptx | 466850, 2421151, 16 |
| rpptx-cli | 42923, 189025, 8 |
| rpptx-render | 68902, 373391, 8 |

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings. Candidate `scripts/readme_doctests.py:400` through :418 changes exactly the nine measured tuple constants. Matching README rows include `README.md:42` and `crates/oxml-pdf/README.md:22`. No member-byte estimate replaces an actual retained measurement.
- Contract: zero findings. Candidate `scripts/readme_doctests.py:369` changes only the same nine archive dates to2026-10-11. The remaining13 tuples/dates and all performance values, version/platform fields and commands remain unchanged. No successful final README consumer or policy result is claimed.
- Panics: zero findings. Constants and measurement prose only. Existing archive validation and errors are unchanged.
- OOXML/package preservation: zero findings. No product or package-selection logic changes. Actual source/member/mode/VCS evidence remains bound to the original publish2 snapshot. The subsequent HLD/baseline sync does not silently relabel those archives as current final source. Final archives must retain exact newly edited README members before final correspondence is accepted.
- Tests: zero findings. Existing measurement function SHA256 `24c05f2fdb70a31a14a9c0032cd36226a706edc337a497669fcc7a330eb3059c` is unchanged. Candidate `scripts/readme_doctests.py:1340` retains exact normalized bytes/count and the64-byte compressed tolerance. FINAL-REMEASUREMENT.md requires actual per-package measurement commands and retained outputs, source/member/mode/VCS checks, current README equality and repeat only affected tuples if needed. Its --no-verify package commands are explicitly measurement-only, separate from actual verified22 publication evidence. It uses actual shared target paths rather than assuming the worker has a target symlink. No final fixed-point success or compiled-gate credit is invented.
- Structure: zero findings. Existing nine README rows and one script carrier, with no new API, file, module, dependency or runner mechanism in product source.

## Limits and handback

The final nine post-README archives have not been measured by this review. Exact final member/count correspondence, gzip tolerance, README consumers, policy and any workflow-required final verification remain due. Verified publish2 is preserved as actual prior evidence, not replaced by the future measurement recipe. Only this report was written. No source, Cargo, environment, cache or runtime action occurred. Review ends and returns the canonical review lease to ROOT.
