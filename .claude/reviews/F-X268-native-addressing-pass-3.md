# F-X268, native addressing, pass 3

**Reviewed**: Codex root reviewed the D3 remediation and its cold and warm fallback cases on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. The native file diff is 1552 added and 25 removed lines. This is the bounded native addressing review, not the complete F-ID microscope.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

D1 stays resolved. The ordered coverage sweep runs once for owners and once for items. Per-child extraction does not repeat the complete coverage walk. Actual sweep steps at 250 and 1000 children meet the 5 times bound.

D2 stays resolved. Counter-only vectors are configuration-gated. The current release test target type-checks, and functional assertions remain present outside debug guards.

D3 is resolved. single_frame_body requires exactly one scanned body and requires it to cover the scanned range. Both independent block scans and whole-part extraction use that eligibility rule. A miss returns to the old reader. The raw writer and the old first-body direct lookup are unchanged.

The cold two-body discriminator that actually failed before repair now passes. Its second case warms 20 paragraphs before replacing a middle block with the same raw body boundary. That case also preserves the old owner list, order, fingerprints, item spans and direct locations. Ordinary singleton, mixed-owner and raw-boundary positive controls remain in the passing group.

## Evidence and limits

`/tmp/fx268-m6-d3-debug.log` records 7 passed and 0 failed in 33.14s. `/tmp/fx268-m6-d3-clippy.log` records a successful scoped lint run. `/tmp/fx268-m6-d3-release-check.log` records successful current-source release test type-checking.

The earlier release executable ran 6 cases successfully in 1.01s. It predates D3 and is not current-source behavioral proof of that repair. A release execution including the seventh case remains pending for the final union after mechanisms 7 and 8.

## Not found

No additional correctness, panic, namespace-preservation, test or structural finding was established within the inspected native addressing scope. Exact frame and child bytes guard carried XML facts. No model field cache or prior document object is added to that record.

Python consumer validation, HLD alignment, complete provenance and composition coverage, mechanisms 7 and 8, timing and memory measurements, Word qualification, hash harness and full sprint verification remain pending. This report does not complete F-X268 or the sprint.
