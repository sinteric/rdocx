# F-X268, in-memory reopen, pass 2

**Reviewed**: Mechanism 5 working diff against 7fc95c22b68d1761cb2c4c042eb55b5033b5913a, five files, 1040 insertions and 79 deletions. Root reviewed S1 remediation and retained the unchanged native review and applicable evidence from pass 1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None in this bounded mechanism.

## Smells

None. S1 is closed. The Python regression now runs both workloads inside one test with local records. It completes both functional workloads when debug counters are absent. With counters, each warm sample must satisfy its own work checks before the cross-size identifier-byte comparison. Child counts are compared across sizes rather than required to be zero.

## Nitpicks

None.

## Evidence and limits

The isolated selected node passed in 67.17 seconds, `/tmp/fx268-m5-s1.log`. Both sizes record four reopen samples [0, 0, 645] and four staging samples [0, 0, 0, 0, 0, 0]. The changed header bytes are real work. The immutable header snapshot stays old and the tested stale body handles match the prechange extension. This test-only repair used the existing fresh extension, SHA-256 8d3b0c03b401d2f22ec385a2b86d45bfafac0bfc42427ede460aaeb307c2d42b, without an unnecessary native rebuild.

The native review covers exact eligibility and frame checks, current identifier categories and ordered error replay, live related-part parsing, the retained namespace policy, and a new opening without a history chain. Actual ZIP and namespace-work REDs, native fallback and cache-off controls, scoped lint and the six-case Python batch remain applicable. The stronger retype test checks a new comment identifier even without debug counters.

HLD and actual test-name evidence adoption are pending before the local checkpoint. This is not the full F-ID microscope or final acceptance. Full forms, settings and mail-merge composition, A3 and A5, mechanisms 6 through 8, release fallback execution, memory, timing, hash, Word fidelity and the final integrated gates remain pending. The work counters do not claim zero byte comparison, model serialization, copying or other validation work.
