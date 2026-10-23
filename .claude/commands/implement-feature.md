---
description: Implement a started feature against its design plan and test stubs.
---

# /implement-feature F-XXX

Turn the failing stubs from `/start-feature` into a passing implementation,
against the design plan as the contract.

## Steps

1. **Re-read the contract.** `.claude/plans/F-XXX-design.md` and its cited
   `docs/hld/` sections. Then `.claude/scratch/F-XXX-progress.md` if resuming.
   Use the exact approved signatures, numeric conversions and XML placements.
   If source inspection contradicts a compatibility claim or exposes an invalid
   content model, stop the affected change, revise the contract and resolve
   material decisions before proceeding. Do not code around a flawed plan or
   silently weaken its acceptance gate.

   On resume, compare the local plan and applicable canonical commands/skills
   with the current canonical sprint revisions. Read the authoritative files
   from the canonical worktree or Git objects. Record their commit or content
   identity, the material differences and applicability to existing work in
   the existing progress file. A claim Base alone does not identify a later
   approved contract. An unresolved semantic difference pauses only affected
   work. Do not reset, force-checkout or rebase a dirty worker, copy sprint
   trackers into it, or edit another owner's worktree. Its owner reconciles
   source safely through the normal workflow. Reuse unaffected local evidence.

2. **Confirm the state.** The F-ID is `in-progress`, the branch is `sprint/sNN`,
   and the stubs exist and fail.

3. **Work the checklist in order.** Implement the numbered acceptance rows,
   preserving the consumer map and test matrix. Inspect every caller of shared
   policy: defaults, context, owner selection and scope must agree where the
   contract says so. Compare actual relevant outputs, including positioned
   rendering, before/after update and save/reopen. Add a missing discriminating
   control when a new interaction is found. Keep unchanged-input consumers and
   refusal/atomicity controls valid. Use scoped `cargo check` between edits.
   For cached read facts, inspect direct native assignments and callback bodies,
   including publishers that keep owner identity or revision unchanged. Prepare
   updated facts or invalidate them before publishing changed source. Exercise a
   real value-changing operation of that kind and compare held, fresh, native
   and saved/reopened reads without weakening the promised handle lifetime.
   Batch related focused regressions sharing a test binary when practical,
   rather than compiling the same large binary separately for each added case.
   Retain individual outcomes and rerun the cases affected by each source fix.
   Run the narrowest useful test target, not the workspace. Never defer a check
   needed to resolve an actual implementation uncertainty merely to batch it.

   Follow the operation invariants in `/design`, including destination checks
   on move/clone and byte-identical refusal. For promised bindings, implement
   and invoke each exact public method and update its stub. Align native module
   registration, Python re-exports and __all__, stub declarations and a small
   typed use of each changed API family. Exercise factory class methods and
   changed property unions through the public import. A private constructor
   must not erase the factory class's type or static methods. Check observable
   default signatures against the approved omission/clearing contract, including
   PyO3 defaults displayed as Ellipsis. Use the existing text-signature mechanism
   when the actual approved default is not represented correctly. Preserve the
   implementation's default and refusal behavior. Do not hide drift with Any,
   a broader return type or a stubtest exclusion. Whole installed mypy/stubtest
   remains a final rider, separate from this small local typed discriminator.
   For raw XML, inspect
   the validator's actual attribute and applicable namespace checks rather than assuming
   the event reader or save/reopen performs them. If optimizing representations,
   inspect all affected constructors/callers and earn the necessary scoped
   compile checks before rebuilding the measured release artifact. Follow
   `/verify`'s build/artifact/semantic outcome rules when reporting evidence.

4. **Keep the progress file current.** Update
   `.claude/scratch/F-XXX-progress.md` whenever you stop, using the shape below.
   This is what makes a handoff or a resume cheap.

5. **Stay inside the plan.** If the implementation needs something the plan did
   not anticipate:
   - A small clarification: proceed and record it under "Deviations" for the
     AS_BUILT entry.
   - A change of approach, a new dependency, or a new public type: **stop and
     revise the plan** through `/design F-XXX --revise`. Do not let the code and
     the contract diverge silently.

6. **Update the HLD before freezing.** Execute exactly the approved plan's
   `## HLD impact` list during implementation. Describe implemented intent,
   without claiming pending oracle qualification passed. Source and HLD edits
   must be present for microscope and the final sprint gate.

7. **When the implementation checklist is done**, consume existing applicable
   local observations and run only missing `/verify --scoped F-XXX` checks.
   Obtain `/microscope F-XXX --working`. Do not run a whole suite or external
   oracle here. Record the final sprint checks still due. Source changes after
   review require affected checks and a review of the new material.

## Remediation discipline

Before patching a confirmed finding, record its violated invariant and root
cause in the existing progress file. Distinguish the faulty acceptance rule
from the input that exposed it. Enumerate the shared helper's relevant callers
and the other ways the same invalid state can reach publication.

Complete the publisher coverage in the existing consumer map before declaring
the fault class repaired. Inspect equivalent predicates as well as callers of
one helper. Every applicable publisher needs its enforcement path and proving
case, or an evidence-backed exclusion. A helper-level fix is not evidence for
a publisher still using a copied predicate. Reuse a concrete policy owner when
its context and contract match rather than adding another partial validator.

Keep the reported reproduction and add a structurally different discriminator
where the defect admits one. For grammar checks, test direct payloads and
payloads hidden inside transparent wrappers, with valid nested controls beside
them. Enumerate admitted wrappers whose content model depends on their parent.
For example, Word customXml can carry rows, cells, blocks or runs under the
same element name. Carry the destination context through its payload and keep
optional properties separate. Pair incompatible relocation and conversion
refusals with compatible metadata-preserving controls. A tag allowlist alone
is not proof of destination validity. This does not authorize a general
validator or a blanket rejection of supported opaque wrappers.
Changing a type marker can invalidate its physical position, enclosing owner's
child requirements or retained descendants even when bytes do not move. Check
those separately from payload fit. Use an empty ambiguous payload to expose
that difference. Include marker retirement and unwrap, with a compatible
enclosing-owner case so the repair does not become a blanket refusal.
Trace transformations that run before the enforcement point. If a parser or
serializer supplies structural identity to a guard, prove that actual modeled
staging preserves the expanded name and required namespace scope. An unchanged
raw passthrough does not exercise that writer. Pair imported prefix aliases and
locally shadowed canonical prefixes with compatible mutation and atomic refusal
controls. XML parseability alone does not prove the same qualified type survived.
Retain opaque metadata without rebinding its names to repair modeled fields.
For XML validation, successful attribute iteration is not proof that names,
values or references are legal. Cover those independent dimensions according to
the approved contract. Preserve explicitly valid opaque inputs and raw bytes.

Run new failing discriminators against the actual pre-repair source before
claiming semantic RED, then the repaired source for GREEN. If the old source
cannot be executed, label source tracing as counterfactual evidence and keep
the execution limitation explicit. Do not rename it an observed RED.

Return each stable finding ID as addressed with evidence, disputed with
evidence, needs-design or unresolved. A fresh review decides whether it is
closed. Two unsuccessful repair attempts for the same invariant require
designer or more capable reviewer investigation before another patch. This
does not clear the finding or extend the sprint-review bound. Missing external
oracle evidence needs an acquisition path, not repeated speculative code fixes.

Escalation ends with a concrete handoff in the existing progress record:
current source and dirty-work identity, stable finding IDs, root cause,
remaining publishers, failing inputs, exact next checks and the responsible
recipient. State whether the recipient is investigating, implementing or
reviewing. A reviewer stays read-only. A reassigned implementer becomes the
sole writer only after the previous writer stops. If the requested recipient
cannot actually be invoked, preserve work and report the handoff as pending.
Do not claim delegation or another model's repair from a suggested prompt.
After repair, obtain fresh independent review and keep external blockers open.

## Progress file shape

```markdown
# F-XXX progress notes

## Current state
What works, what does not, what is half-done.

## Changed areas
Files touched so far.

## Last green check
The exact command that last passed, and when.

## Blockers
What is stopping progress, if anything.

## Next action
The single next thing to do. Specific enough to act on without re-deriving it.
```

## Refused situations

- **The F-ID is not `in-progress`.** Run `/start-feature` first.
- **No design plan.** Run `/design`.
- **Changing approach without revising the plan.** The plan is a contract.
- **Committing.** That is `/complete-feature`.
- **Marking a checklist item done while its test fails.**
- **Updating the hash baseline to silence a delta.** Explain it or revert it.
