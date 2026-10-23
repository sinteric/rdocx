---
description: Design a feature before implementing. Writes a design plan, no code changes.
---

# /design F-XXX [--revise] [--draft]

Write `.claude/plans/F-XXX-design.md`. **No code changes in this command.**

The plan is a machine-consumed contract: `/start-feature` reads its test plan
and refuses a vague spec reference, and `/complete-feature` executes against its
HLD impact list. Write it as something that will be acted on, not as prose.

`--draft` is the mode `/run-sprint` uses to plan a whole sprint before
implementing any of it. It runs every step except interactive questioning in 4
and approval in 7, always writes the plan in 6, records open
questions instead of asking them, and leaves the status at `draft`. A plan left
at `draft` cannot be started.

## Steps

1. **Read the story.** Find `### F-XXX` in `docs/hld/14-development-backlog.md`.
   Take its size, dependencies and **test gate** verbatim. If the F-ID is not
   there, stop and say so.

2. **Read the cited spec sections.** Follow the story into `docs/hld/` and read
   the specific sections it implements. Read the actual code it will touch. Do
   not design from the backlog entry alone.

3. **Check dependencies.** Record actual status from
   `docs/sprints/BACKLOG.md`. Drafting may use a pending in-sprint provider's
   proposed contract, with its assumptions and readiness explicitly pending.
   Approve dependent designs in topological order only after their provider
   contracts are approved. Approval is not implementation readiness.
   `/start-feature` and `/claim-feature` still require dependencies to be
   actually done, including any required dependency-prefix checkpoint.
   An unfinished external dependency remains a named blocker.

4. **Ask before writing.** Collect every open question and ask them in one
   round using AskUserQuestion. Questions where a wrong guess would mean
   rewriting the story are blocking. Everything else gets a stated assumption.

   Under `--draft`, skip this. Write the questions into `## Open questions` and
   carry on, so a sprint's worth of planning produces one consolidated round of
   questions rather than one per story.

5. **Route the risk.** Apply `.claude/skills/risk-routing.md` to the diff you
   are about to describe. Record the matched rows and the exact extra checks
   each one adds in `## Risk routing`. `none` is a valid and common answer.

6. **Write the plan** using the template below. First enumerate all actual
   consumers and each issue/PR acceptance row. Read the existing execution
   paths, not only API names. Include a concrete signature and compatibility
   decision for every public addition. Use the composition-check guidance in
   `.claude/skills/differential-testing.md`. An internal agreement test is not
   an external parity claim. Keep unavailable oracle evidence explicitly due.

7. **Set status to `approved`** once the questions are answered. A plan left at
   `draft` cannot be started. Under `--draft`, leave it at `draft`.

## Design validation before approval

Use the detail warranted by the change. Mark an inapplicable contract below
with a short reason rather than inventing APIs or broadening a small story.

- **Verify facts before asking for choices.** Pin a compatibility source's
  version and exact file/function or spec clause. Inspect each affected class
  and collection separately. A candidate patch and a recommendation are not
  proof of compatibility. Resolve answerable factual questions from primary
  sources. Ask the user about genuine product choices and intentional deviations.
- **Specify the public contract.** Name exact signatures, return types, units,
  defaults, omitted arguments, explicit `None`, empty values and no-op behavior.
  State valid bounds, error type, publication atomicity and live-handle or
  collection invalidation. Distinguish getters from setters and Rust from Python.
- **Specify numeric behavior.** State storage scale, conversion direction,
  truncation or rounding, tie rule and negative-value behavior for each field.
  Give boundary and tie examples that distinguish the proposed rule. Address
  non-finite values and Python booleans where accepted numeric inputs make them
  relevant. Do not generalize one field's conversion to unrelated units.
- **Specify XML placement.** For every supported parent context, identify the
  qualified element, schema type, permitted child payload, order and cardinality.
  Inline and block contexts need separate rows when their content models differ.
  State how unknown siblings and unsupported placements are preserved or refused.
  Apply the invariant to every operation that can introduce the structure,
  including insert, move, clone, replace, unwrap and repeat where supported.
  Name the check each operation reaches before publication. A valid source
  does not imply a valid destination, and a permissive reopen is not validation.
  For transparent wrappers, specify how the inherited parent grammar reaches
  the actual payload. Distinguish wrappers that preserve context from elements
  that introduce a different child grammar. Neither accepting the wrapper name
  alone nor applying the outer grammar indiscriminately to all descendants is
  sufficient. Include incompatible nested payloads and legal nested controls.
  Treat physical placement, payload grammar and enclosing-owner requirements
  as separate proofs. Empty contents may fit several grammars without making
  their control type legal in each slot. Type changes, marker retirement,
  unwrap and relocation can invalidate retained descendants or the previous
  owner. Include those inverse transitions and compatible positive controls.
  Resolve qualified transparent ancestors without skipping real paragraph,
  row, table or cell discriminators. A containing ancestor is not automatically
  an immediate parent. Pin the primary rule before imposing either condition.

- **Specify raw XML validation.** Distinguish well-formedness, namespace
  correctness, parent/schema validity and application behavior. Name which
  existing mechanism proves each promised level. Event parsing or root counts
  alone do not prove attribute syntax, uniqueness or prefix binding. Include
  valid source-preservation and malformed-attribute refusal controls when the
  API accepts raw XML. Do not add an unrelated general XML framework.
  Separate attribute separators/quoting/duplicates from legal XML names,
  literal-value syntax, escapes and entity handling under the approved policy.
  Name which layers the parser actually checks. XML lexical names may contain
  a colon. Namespace-well-formedness is a separate requirement, not permission
  to reject an explicitly supported opaque XML input.
- **Preserve the whole acceptance contract.** Map every issue/backlog promise
  to an A-ID and evidence, including performance budgets and unreduced inputs.
  Missing native evidence or a reduced reproduction does not narrow the promise.
  A scope reduction needs an explicit decision and corresponding backlog update.
- **Preserve qualification gates.** Follow the provenance rules in
  `.claude/skills/differential-testing.md`. A self-generated fixture cannot
  satisfy a promised native comparison. Record unavailable evidence as pending.

Before approval, check each bullet against the actual plan. Record evidence
and outstanding decisions in the sections below. Do not approve merely because
all template headings exist. Before implementation, recheck that source pins,
provider contracts and actual dependency completion still apply.

### Contract closure pass

After drafting or revising, read the whole plan as an implementer. This is a
semantic check, not a check that headings and answered Q-IDs exist.

1. Inventory every promised operation from the backlog, decisions, approach,
   acceptance rows and dependent plans. Each must name its existing or proposed
   callable entry point, concrete Rust/Python arguments and returns where
   exposed, and a proving A-ID. Do not promise repeat, remove, clone or unwrap
   behavior without saying how a caller invokes it.
2. Compare every supported kind against its payload variants and valid parent
   contexts. Explain any kind created only as a child of another operation.
   An umbrella statement such as "all supported kinds" is not coverage.
   For shared validation, extend the existing consumer map with every publisher
   that can introduce the prohibited state, its enforcement path and proving
   A-ID/test. Search by behavior as well as helper name. A separate admission
   path, such as type conversion or repeating-section construction, can enforce
   the same invariant without calling the repaired helper. Record exclusions
   and their reasons. No new tracker or second inventory is required.
3. Reconcile repeated signatures, parameter names, defaults and representations
   across the approach, tables, decisions and consumer plans. Keep one complete
   definition and refer back to it rather than maintaining divergent copies.
   "Same methods" does not define a new binding payload representation.
   For promised bindings, give each public native operation an explicit
   exposed/not-exposed disposition consistent with scope. Name the actual
   binding and stub plus a test invoking it with public arguments. A native
   test or similarly named generic binding does not prove that route exists.
4. For a new projection of existing data, inventory the states the parser
   already accepts. Specify reading, preservation without assignment, explicit
   assignment and clearing for applicable variants and absence. A narrower
   setter does not authorize dropping imported states. Distinguish a schema
   default from an application fallback. Give a concrete stored-to-public unit
   conversion example when their scales differ.
5. Recheck source claims against the current integrated Base. Label historical
   observations as historical. Name affected running workers when a contract
   changes and require the revision acknowledgment in `/implement-feature`
   before they continue. Retain applicable evidence and invalidate only the
   observations affected by the actual change.
6. For performance acceptance, pin the workload, build profile, measured
   boundary, sample/aggregation rule and threshold before measurement. Separate
   setup from the timed operation. Specify a deterministic work-bound control
   beside timing where unrelated-data scaling is the defect. Diagnostic sizes
   and counters do not replace correctness, binding or final qualification.
   Name the actual public consumer being timed. Cover its lookup, validation
   and publication costs, including bindings. A native-only benchmark cannot
   discharge a binding cost criterion. Instrument actual traversal work rather
   than placing a constant counter at entry to an unbounded loop.

Record the closure result and any unresolved items in the existing plan.
For a cached read projection, name its source facts, preparation boundary and
every publisher that can change those facts. Include direct candidate assignments
and callbacks that preserve owner scopes or leave the revision unchanged. Map a
real fact-changing operation of that kind to held, fresh, native and save/reopen
observations. State when no such operation exists. Trace mirrored or paired
outputs so the update covers every affected logical owner.
Prove that immutable and mutable accessors select the same physical owner.
A shared numeric ordinal is not proof when controlled or nested content differs.
Answered questions alone do not discharge these checks. Revisions preserve
approved user decisions. Resolve source facts directly, use routine judgment
for implementation detail, and ask only when a material product choice remains.

## Template

```markdown
# F-XXX, Short title

**Status**: draft | approved
**Sprint**: SNN
**Size**: S | M | L
**Depends on**: F-YYY, F-ZZZ | none

## Problem

What is wrong or missing today. Cite files and lines. One or two paragraphs.

## Spec reference

Specific sections, never a whole document. For example
`docs/hld/05-drawingml-model.md`, "Colour, the part everyone gets wrong".
`/start-feature` refuses "see docs/hld/".

## Approach

What will be built. Type signatures for anything new. Enough that a reader can
predict the diff.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Concrete rule | Exact functions, native/binding/CLI where affected | Who owns defaults/context, including copied policy | Evaluate, update, serialize, render, reopen as applicable |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Applicable API, numeric field or XML parent | Signature, types, omission/None, units/ties, lifetime or schema constraints | Pinned version/file/function or spec clause, distinguish fact from chosen policy | Named case and expected values, or explicit blocker |

## Dependency readiness

Actual provider status, approved contract relied on, and the completion or
dependency-prefix evidence required before implementation. Approval of this
plan does not mark its dependencies done.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Concrete discriminator | Exact value/order/ownership/atomicity or justified tolerance | Named paths | Named RED/GREEN test | Named command/case, or none |

Map every external issue row to an A-ID, including limitations a proposed PR
excludes. Do not replace an unreduced case with an unrelated reduced example.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Applicable zero/one/many, implicit/explicit, legacy/new, direct/nested and lifecycle axes | Observable values, actual rendering where promised | Explain how this fixture catches a plausible mistake |

Choose bounded risk-based combinations. Do not run every permutation or add
weak fixtures merely to increase the test count. State why each omitted
consumer or axis is unaffected. Same-consumer round-trip tests cannot stand
in for cross-consumer semantic agreement.

## Rejected alternatives

What else was considered and why not. One line each. If nothing was considered,
say so and justify it.

## Test plan

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `name_of_test` | Actual old-source RED and fixed-source GREEN |
| final sprint | oracle or risk rider | `<exact command/cases>` | ... |

The **test gate** from the backlog, named explicitly. Distinguish local
regressions from final sprint oracle/risk qualification. An oracle-only test
gate stays pending until final acceptance, with a real local regression named
for implementation readiness. No whole-suite or oracle execution per worker.

## HLD impact

The `docs/hld/` files this story changes, as a list. `/complete-feature` updates
exactly these. "None" is valid and common.

## Risk routing

The rows of `.claude/skills/risk-routing.md` this diff triggers, and the exact
extra check each one adds. `none` is valid. `/run-sprint` takes the union
across the sprint when it builds the consolidated gate.

## Hash harness

Expected to be unchanged, or the expected delta and its justification.
**Mandatory for M1 through M6.**

## Implementation checklist

- [ ] ...

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q-ID | All consumers of the decision | Actual product choice, with consequences | Open or answered, exact answer and provenance |

Use stable sprint-wide IDs. Reuse the same ID for a shared decision across
plans. Preserve answered decisions and their source. Every blocking row must
be answered before approval. A request to improve instructions is not approval
of the current draft recommendations.
```

## Refused situations

- **The F-ID is not in the backlog.** Add it there first, with a size and a test
  gate.
- **Implementation with an unfinished dependency.** Drafting and contract
  approval follow step 3, neither unblocks implementation.
- **The story has no test gate.** Every story has exactly one. Fix the backlog
  entry first.
- **`--revise` on a plan whose F-ID is already `done`.** Write a new F-ID
  instead. AS_BUILT entries are not rewritten.
- **Missing consumer map, numbered acceptance criteria or discriminating test matrix for new or revised work.** Fill them before approval. Historical completed plans are not rewritten to meet a new template.
- **Approving a plan with an unanswered blocking question.** Under `--draft`,
  the questions are deferred to `/run-sprint`, not waived.
