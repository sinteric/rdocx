# F-X211c, output observer integration, pass 1

**Reviewed**: External runner derivative6c8bb5b65374a439a178d5dfb2edd91e88ab2180846642de1f19ef52cf0eec28 against retained reviewed predecessor7d72dfe876ea30a75442796831e96b12c3d2de09b26c4ef149597431b84d36d6. Exact patch6ce3d9a319218bfbabb966279ba9de423c401335ade27a69391782b2470550c5. The three changed functions are verify_corpus, run_case and main. Other existing function ASTs match.
**Verdict**:0 defects,0 smells,0 nitpicks.

## Defects

None found in the bounded integration.

## Smells

None found.

## Nitpicks

None recorded.

## Not found

Correctness and scope: Only the eight reviewed mapped cases use the fixed observer, only in the final stage. Original script command remains recorded separately. Other bodies retain direct invocation. The original observer2552ff46 and all frozen body inputs remain pinned. No body, input, product provider or in-place wrapper mutation is introduced.

Guards: The observer is authenticated with corpus input guards before and after each case and final. The generated per-case configuration pins actual script and support hashes, is hashed before spawn and rechecked after termination. Existing current-source, installed membership, external input, owned process and timeout guards remain intact. Output receipt pins case, script, support, observer and actual first attempt. Returned artifact paths must be unique absolute direct children, regular nonsymlink files with actual hash and size. Each artifact explicitly represents original returned bytes with no reencoding.

Result separation: Actual child exit, timeout, runner error and owned cleanup decide assertion credit exactly as before. Missing receipt, altered configuration, altered output bytes or extraction error leaves output observation unqualified without hiding the original body outcome or retrying it. The runner never sets full acceptance credit. Final render facts and native acceptance require separate artifact admission.

Tests: Checkpoint5f74a1c3 authenticates all declared sources and raw logs. Receipt465b11d2 contains an actual exit0 qualified safely reaped test-runner row with nine passing synthetic integration controls. Integration Popen and child payload are mocked. They do not prove actual corpus child or provider execution. The earlier exact selector fails against the retained pre-runner, and remains failed evidence. Separate observer review3b879ec8 covers actual synthetic-body invocation equivalence, with tracing, lifetime and timing differences stated. Anonymous P-AREA-12 buffers remain unavailable.

## Boundary

Draft remains unapproved for measurement, current seven product carriers remain frozen and the100 corpus has not executed. This review grants no CP, installed runtime, native, render, corpus or feature-completion credit. Only this canonical review file was written. No source, runtime, test, Cargo, Git or application mutation ran during the audit. The pass ends here and returns control for distinct orchestration.
