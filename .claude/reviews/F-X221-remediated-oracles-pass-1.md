# F-X221 remediated oracles, pass 1

**Scope**: Only the additions after the clean whole-source review: the exact stale-message prefix, historical S90 notes versus current carriers, and manual CLI build versus tag-only publication policy tests. Worker source was held unchanged. The prior58-file review remains authoritative for unchanged implementation.
**Verdict**: Zero defects, zero smells and zero nitpicks in this bounded remediation.

## Stale message

The entire diff in `crates/rpptx-py/tests/test_documented_examples.py:253` adds only the literal `stale ` to the existing expected first string. The exact equality at line259, StaleElementError type, captured/current/committed revisions, first invalidating operation, recovery text and every caller body remain unchanged. This resolves diagnosis020165f6 against the already approved shared formatter. No product or owner/atomicity policy is altered.

Both actual successor logs under `/private/tmp/S91-F-X221-implementation-1/clean39-successor-union-runtime.log` and `clean312-successor-union-runtime.log` end with78 passed and no skip/failure count. The selector union retains every original priority/compatibility selector and adds the other exact helper callers. These are genuine new installed successors, not relabelled original70/1 failures. No test was executed by this reviewer.

## Historical notes and current carriers

`scripts/test_sprint_workflow.py:174` derives the current family tag from the actual current workspace/PowerPoint manifest, then checks current family inventory and lock membership. The historical tag variable still selects the S90 release notes at line184, preserving their original contribution links, credits, known boundaries and note assertions. Unpublished carrier checks at line209 retain publish=false and the independent rpptx-wasm0.12.1 version, while comparing present carrier/lock/CI consistency rather than requiring obsolete0.16.0/0.14.0 values.

This does not replace the explicit current-version gate. `test_s91_release_families_match_reviewed_versions` at line126 separately requires exact0.16.1/0.14.1, seven/fifteen selected crates and current release identities. Consequently an arbitrary coordinated version change cannot pass merely because the historical test follows current manifests. Historical notes and their tags have not been retargeted to S91.

## Workflow publication boundaries

The corrected shared consumer at `scripts/test_sprint_workflow.py:2999` requires the exact manual-or-push condition for only build-cli-assets and cli-assets. The release-assets aggregate, crates publication and GitHub release remain exactly push-only at line3004. PyPI additionally requires the expected tag-family predicate at line3009. This matches the independently approved two-job build-only rehearsal and does not grant manual publication authority.

The negative controls at line3047 mutate each of the six job conditions in its own block and require the real workflow consumer to reject the mutation. Existing matrix, artifact inventory, provenance/checksum, release dependency, environment, pinned-action, registry verification and manual-publication mutations at lines3032-3066 remain. Publication tests are not weakened to a substring that would accept push-or-manual publishing. The four focused policy successors actually report Ran4/OK in policy-focused-successor-3.log, separately from the retained failed predecessors.

## Execution boundary

The full-policy successor was still in progress when assigned and receives no terminal/full-suite credit from this source review. Root must inspect its actual final result separately. This clean bounded review does not replace final scoped verification, integration, hosted rehearsal, sprint gates or publication approval. No whole-feature, native or corpus audit was repeated.

Only this worker review file was written. The pass ends here and releases the review-write lease without source fixes, tests, builds, runtime, Git, CP or remote mutations.
