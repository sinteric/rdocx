# F-X268, block namespace proof, pass 2

**Reviewed**: Codex root reviewed D1 remediation on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 1524 added and 50 removed lines, including the earlier reviewed prerequisites. This is the bounded mechanism 7B2 parse and namespace review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved finding

D1 is resolved. The empty-owner proof scans the complete head and tail as an empty-body shell. It uses the same namespace_owners_from_spans construction as the authoritative full-part wrapper. The body-start substring heuristic is removed. A nonempty owner list, scan error or unavailable frame proof leaves the empty fact unavailable and keeps the checked whole-part fallback.

The confirmed diagnostic is now a strict regression. The old writer and block writer must agree, the old parser must accept the bytes, and the old scanner must find one owner in both the opaque background case and a different opaque section-tail case. The retained result must be unavailable or equal to that result. A normal frame requires an actual empty fact. Functional assertions remain active in release.

The full-part wrapper still records actual whole-input scan bytes. Partial frame scans have a separate private debug counter, with a reader compiled only for debug tests. They do not change the six approved units. The regression requires nonzero disclosed frame work and zero whole-part scan bytes for that proof.

## Evidence and limits

`/tmp/fx268-m7-namespace-shell-probe.log` retains the actual pre-fix RED, with one old owner and zero retained owners. `/tmp/fx268-m7-namespace-shell-remediation-test.log` records 14 passed and 0 failed in 18.10s. The ordinary measured edits still record zero whole-part parse and namespace bytes and one child write and read. The independent reference forces the whole main parser and retains growing parse and scan work. Canonicalization and identifier input bytes still grow on the optimized path.

The scoped check and clippy pass. Release test targets type-check in 46.77s with the earlier addressing code's unused child_count warning. This is not release behavior execution. The temporary extraction compile error E0277 was corrected and is not a behavioral RED claim. The installed Python extension remains the mechanism 6 build.

## Not found

No additional namespace policy, current-part identity, model-selection, raw admission, field reread, counter-unit, panic or structural finding was established in this remediation inspection. The extracted policy has two actual callers, the full-part wrapper and partial-frame proof, and adds no trait, generic or stable API.

Canonicalization, identifier optimization, complete native A10, section and list coverage, Python rebuild, HLD, whole-feature composition, release behavior, timing, memory, Word and final gates remain pending. This is not mechanism 7 acceptance or a whole-feature microscope.
