# F-X221 CI bootstrap remediation, pass 1

Reviewed the six existing working carriers against HEAD `342207c9`. This is the bounded implementation review following S91 sprint review B1, not sprint review pass 2. The reviewer performed no builds, tests, environment changes or source edits. Existing unchanged provider and acceptance reviews remain authoritative.

**Verdict: 2 defects, 0 additional structural smells, 0 nits.** Return to separate normal remediation. Neither full104 acceptance nor the final sprint gate is qualified by this pass.

## D1. Standalone reference qualification omits compiler and transitive input custody

`crates/rdocx-py/tests/test_rendering_threads.py:2312` selects a PATH rustc and checks its version text. Line2318 hashes the source, Cargo.lock and only three selected rlibs. Line2319 links through the shared target dependency directory, without isolating or hashing the other transitive artifacts or compiler sysroot inputs. Line2325 records a compiler path and version but no physical compiler hash. Nevertheless, line2328 declares `actual_transitive_abi_qualified: True`.

This does not meet `.claude/plans/F-X195-design.md:282`, which requires ROOT authentication of compiler, isolated transitive artifact closure, source and binary before use. The outer bootstrap controller guards tracked repository source and Cargo.lock, not this compiler and link closure. Successful compilation and the three pinned package versions cannot establish the omitted custody.

Resolve the actual pinned toolchain compiler, authenticate its physical bytes, and capture and guard the actual Cargo-emitted transitive dependency and sysroot inputs before and after the reference compile. Link through an isolated authenticated dependency set, or an equivalently complete guarded exclusive closure. Retain the original successful build as execution evidence without granting the missing qualification. The affected reference stage requires a genuine successor after this fix.

## D2. Generated variable receipts do not satisfy the unchanged advance contract

`crates/rdocx-py/tests/test_rendering_threads.py:2285` through line2292 emit each variable control without `total_x_advance`. The unchanged Word independent SVG consumer requires that field at `crates/rdocx-py/tests/test_rendering_threads.py:985`. The PowerPoint independent PDF consumers also require it at `crates/rpptx-py/tests/test_documented_examples.py:6681` and line6767.

The actual104 attempt confirms the consequence. `/private/tmp/S91-F-X221-CI-bootstrap-B1-1/actual104-1-receipt.json` records exit1. Its retained log, SHA-256 `42d6c73bbd09181663d0451a1b3ba7116934df59964e1d81524a0359b2c276a7`, has98 passes and six failures: Word SVG400/700, PowerPoint slide PDF400/700 and notes PDF400/700, all missing this field. Those six consumers did not complete their positive paint assertions.

Emit the total advance derived from the actual independent HarfBuzz static shape advances, retaining the original selected-source/static positioning equality. Do not guess a value, weaken the consumer, or change its geometry or paint thresholds. Preserve this failed attempt and run the affected acceptance successor with the corrected generated receipt.

## Coverage and evidence limits

The clean CI route now explicitly prepares inputs before full pytest using the existing test carrier. The audited scope includes the pinned HarfBuzz source and independent synthetic/static controls, authenticated portable Poppler source-build receipt and consumers, CLI discovery, reference compilation, workflow dependencies and ordering, existing workflow policy tests and HLD15 wording. No further material findings were found in those changes. No new tracked file, asset, module or dependency boundary is introduced by this preparation.

The author reports Poppler38960 and bootstrap94095 terminal0 with source guards released. These actual tool builds do not override D1 or the actual104 failure. Retained HarfBuzz and Poppler products may be reused only where their producer inputs and physical identities remain authenticated. All104 cases and their existing0.99 comparisons remain mandatory. The native-eight check remains a separate ROOT serial obligation after104 qualification, rather than an inferred new CI aggregate step. No native-eight, full Python, hosted, publication or sprint-completion credit is granted here.

This pass ends on writing this report and releases the exclusive review-file-only lease. No remediation was performed within the review.
