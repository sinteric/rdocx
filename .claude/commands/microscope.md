---
description: Adversarially review one feature's diff. Records findings, changes no code.
---

# /microscope F-XXX [--working] [--aspect NAME]

Review the implementation diff for one F-ID. **This command changes no code.**
It is required before `/complete-feature` and it is not skippable.

`--working` reviews the uncommitted working tree, which is the normal mode.
Without it, review the F-ID's commits on the sprint branch.

## Steps

1. **Establish the diff.** `git diff` for `--working`, otherwise the F-ID's
   commit range against the sprint base. State the file and line count so the
   scope is on the record.

2. **Read the contract first.** `.claude/plans/F-XXX-design.md`, then the cited
   `docs/hld/` sections. A review that does not know the intent can only find
   typos.

3. **Review by aspect.** Default aspects, or one named by `--aspect`:

   | Aspect | Looks for |
   |---|---|
   | `correctness` | Wrong logic, off-by-one, unhandled cases, wrong operator precedence |
   | `contract` | Does it do what the design plan said, and nothing more |
   | `panics` | `unwrap`, `expect`, indexing, slicing, arithmetic overflow on untrusted input |
   | `ooxml` | Schema child order, namespace prefixes, whitespace preservation, unmodelled subtrees dropped rather than captured |
   | `tests` | Does the test gate actually prove the story. Would it fail if the code were reverted |
   | `structure` | The rules in `AGENTS.md`. New traits, generics, wrappers without a second implementer today |

4. **Challenge the acceptance evidence.** For every changed semantic rule,
   trace the consumer map through actual callers. Look for duplicated policy
   hidden behind a shared parser/formatter. Check a risk-based combination the
   existing tests might collapse: multilevel context, zero items with implicit
   lookup, fresh versus held handles, or final completion metadata. Name which
   plausible wrong behavior the fixture rejects. Rendering claims need actual
   positioned/render output. Native parity needs actual native evidence.
   Recorded tests may be consumed without rerunning them, but inspect their
   assertions and inputs rather than accepting aggregate pass counts.

   Verify A-ID coverage, excluded issue/PR scope and the exact source for
   evidence. Report unavailable criteria explicitly. Do not fix anything in
   this review or manufacture a defect to satisfy the challenge.
   Challenge compatibility claims against their pinned primary source and
   inspect context-specific XML content models. Check numeric tie examples,
   omission versus clearing, and collection/handle lifetime where applicable.
   A self-produced fixture cannot discharge native parity. Confirm no promised
   issue row or gate disappeared when a reduced example passed.
   Check `/design`'s contract closure as well as implementation conformance.
   Following a contradictory or incomplete plan is a contract finding, not a
   reason to approve. Verify callable operation and variant coverage, imported
   states and the worker's acknowledgment of applicable canonical revisions.
   Trace structure invariants through every applicable lifecycle operation,
   especially move/clone destination checks. Inspect public binding and stub
   coverage per promised method, with actual binding invocation evidence.
   Check the public export path and __all__, callable factory typing, changed
   property unions and visible defaults. A source-only mypy pass does not prove
   installed stub/runtime agreement, and matching stub names alone do not prove
   a factory remains callable through the public import.
   For raw XML input, verify attribute and applicable namespace validation mechanisms.
   For measured results, follow `/verify`'s separate child-exit, artifact and
   semantic-outcome checks. A successful wrapper or diagnostic size is not proof.
   For remediation, inspect the cause and shared-helper consumers, not only
   the newly passing reported examples. Challenge context-preserving wrappers
   whose payload changes grammar with the destination, not only nested instances
   of the initially reported element. Inspect each admitted context-dependent
   wrapper, its metadata and its repeating/conversion publishers. Require
   compatible positive controls alongside atomic refusal cases. Challenge
   physical type placement and owner membership independently of payload
   grammar. Empty wrappers are a discriminator. Inspect type/marker retirement
   and retained descendants on unwrap or relocation, including compatible
   enclosing ownership. Use namespace-qualified structural identity. Challenge
   transformations before the guard as well as the guard itself. Distinguish
   raw passthrough from actual modeled serialization. Compare expanded type
   identity across staging, not only XML parseability, using imported aliases
   and locally shadowed canonical prefixes with compatible metadata-preserving
   edits and atomic refusal controls. Challenge
   independent name/value validation where relevant. Preserve valid
   controls and distinguish actual historical RED from source counterfactuals.
   Retain finding IDs when the same invariant fails on a new input.
   Check publisher coverage, including behaviorally equivalent predicates that
   do not call the repaired helper. When parsing drives the invariant, challenge
   equivalent content represented by different event variants. State the paths
   examined and exclusions. Do not close a fault class solely because every
   supplied regression passed.

5. **Classify every finding.**

   - **Defect**, it is wrong. Must be fixed before completion.
   - **Smell**, it will be wrong later. Fix or file an F-ID.
   - **Nitpick**, taste. Record and move on.

6. **Cite everything.** Every finding carries `path:line`. **A finding that
   cannot be cited is deleted, not softened.**

7. **Write** `.claude/reviews/F-XXX-<aspect>-pass-N.md`, incrementing `N`.

8. **Hand back.** Return after the review file is written. The implementing
   phase fixes defects and smells, then `/microscope` runs again as pass `N+1`.
   An active orchestration command may perform those phases sequentially in the
   same user invocation. **The exit condition is zero defects and zero smells.**
   Nitpicks may remain.

## Template

```markdown
# F-XXX, <aspect>, pass N

**Reviewed**: <what diff, how many files, how many lines>
**Verdict**: N defects, N smells, N nitpicks

## Defects

### D1, one-line summary
`path/to/file.rs:123`

What is wrong. What input triggers it. What happens instead of the correct
behaviour.

## Smells

### S1, one-line summary
`path/to/file.rs:456`

## Nitpicks

- `path:line`, one line each.

## Not found

Aspects checked that produced nothing. Say so explicitly, by name.
```

## Refused situations

- **Modifying any file outside `.claude/reviews/`.** No source edits, no fixes,
  no formatting, no incidental cleanups. If you find a bug, record it and move
  on. A review that patched the code cannot report what state the code was in.
- **Manufacturing findings to fill a section.** Zero findings in a category is a
  valid and expected result. Report it as such.
- **Reviewing an F-ID that is not `in-progress`.**
- **Beginning remediation inside the review pass.** Return to the caller when
  the review file is written. An active orchestration command may then begin a
  distinct remediation phase without asking for another user invocation.
