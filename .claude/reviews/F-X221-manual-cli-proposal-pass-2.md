# F-X221 manual CLI proposal, pass 2

## Verdict and scope

One defect in the proposed existing-entrypoint test snippet. Zero workflow-remediation defects, zero smells and zero nitpicks. This external proposal remains unadopted. No claim, tracked test adoption, workflow mutation, hosted dispatch, network operation, CLI execution or publication occurred.

Proposal SHA256 `7d13815143558305afd3ddf5ba469a336399a5d72950cc027949e9de6edef27b`, workflow `bedd837346177714e0965e2961f2606128a009ed411eb8142af9aefa5a8f8c1d`, patch `93aeb58c3cc428ff5b6641267db902a482e830aa7adcefd1ab6a931e04636ac2` and every referenced predecessor/control/snippet hash were independently authenticated. Bytes outside build-cli-assets and cli-assets exactly match the current canonical workflow.

## Duplicate remediation and actual semantic controls

The successor `/private/tmp/S91-F-X221-readiness-preparation-1/manual-cli-duplicate-remediation-proposal-2/wheels.yml:434` preserves ordered ZIP members, requires exactly three records and unique names before dictionary construction. Lines443 to447 do the same for TAR. The exact executable/README/LICENSE names, TAR regular type/executable mode, nonempty payload and normalized README/LICENSE source bytes remain required at lines448 to457.

Executed the exact inline archive validator in memory using real ZIP/TAR encoders and canonical README/LICENSE bytes. Both predecessor and successor accept the clean12 manual and6 push archive inventories. The predecessor accepts all six duplicate variants, binary/README/LICENSE in each format. The successor rejects every variant before payload collapse. Only synthetic nonempty executable bytes were inspected, no binary was executed. The actual control helper `in-memory-controls.py` retains every inline consumer assertion and scopes environmental/archive mocking to this in-memory call.

The earlier reviewed manual2-family by6-platform and tag1-family by6-platform behavior is unchanged by this narrow duplicate fix. Outside-two-job identity preserves publication, source/main-SHA, tag, allowlist and push-only release/attestation guards. This is semantic proposal evidence, not hosted rehearsal evidence.

## Defect D1, adoption snippet shadows the test receiver

At `/private/tmp/S91-F-X221-readiness-preparation-1/manual-cli-duplicate-remediation-proposal-2/existing-entrypoint-test-snippet.py.txt:67`, `self` belongs to the nested MemoryPath object, not SprintWorkflowTests. MemoryPath has no assertEqual method. Thus the exact proposed existing-entrypoint method fails on its first clean positive archive inventory before reaching any duplicate check.

Independently executed that exact snippet in an in-memory unittest class with the successor workflow and read-only canonical README/LICENSE sources. It raises `AttributeError: 'MemoryPath' object has no attribute 'assertEqual'`. The separate helper controls pass because their equivalent assertion uses the outer `test` receiver. Their success therefore cannot qualify this different snippet for adoption. Fix the captured test receiver in a separate normal proposal/remediation phase and execute the exact resulting existing-entrypoint method, retaining positive and all six duplicate rejection controls. No new module or dependency is needed.

## Remaining boundary

All28 repair prerequisites and the approved F-X221 claim barrier remain binding. This pass neither adopts the workflow nor completes F-X221. Required final source composition, existing-entrypoint adoption, scoped/full gates and reviewed-SHA hosted rehearsal remain pending. Review ends with this report and releases the canonical review-file writer lease.
