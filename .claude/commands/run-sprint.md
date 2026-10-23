---
description: Run every unfinished F-ID in the current sprint. Designs first, implements in safe parallel waves, completes scoped dependency-prefix checkpoints and verifies the final integrated result, then loops on review until clean.
---

# /run-sprint [--prepare] [--reviewer NAME] [--max-review-passes N] [--max-workers N]

Drive the whole active sprint. Design every story before implementing any of
them, run independent stories in parallel worktrees, complete scoped dependency-prefix
checkpoints and verify the final integrated result, then loop on `/sprint-review` until
it comes back clean.

Defaults are three review passes and as many workers as the wave allows.

Each worker keeps its implementation scoped to its approved F-ID, with focused
RED/GREEN regressions, F-ID-bound fast `/verify --scoped` and a zero-finding
`/microscope`. Reserve `/verify --full`, the union of sprint risk riders and
`/sprint-review` for the integrated final result. A dependency-prefix
checkpoint uses actual scoped local evidence and focused reconciliation checks.
External oracles, complete suites and risk riders run only in the final union.
Subcommands consume the same evidence instead of repeating its execution.

Workers batch related local cases sharing a test binary when practical, with
individual RED/GREEN outcomes retained. Compilation and linking cost matter
alongside runtime. Preserve checkpoints needed to resolve real uncertainty.

**`scripts/sprint_workflow.py` is the state authority.** Everything below is
resumable through `.claude/scratch/SNN-run.json`. Reuse it rather than starting
over.

**This command never merges to `main` or creates a release tag.** It ends by
telling you the exact `/close-sprint` invocation. Release preparation F-IDs
finish with local evidence. Publication follows sprint close through
`/release` from the reviewed `main` merge SHA.

## Prepare only and reviewer selection

`--prepare` performs read-only GitHub intake, state/dependency/source inspection,
then drafts the detailed designs and consolidated open questions in sections1
and2. It may write the existing planning, intake and run-state artifacts. It
stops before approving unresolved decisions, committing plans, claiming workers
or implementing code. Preserve already approved/running/reviewed work and its
observations rather than resetting it to draft. Report the exact later run
command and any material questions. This flag belongs to this orchestration
command, not `scripts/sprint_workflow.py init`. It is unrelated to the worker
handoff flag on `/complete-feature --prepare`.

`--reviewer NAME` records the user's requested independent final sprint reviewer
through `init --resume --reviewer NAME`. Omitting it on resume preserves the
existing selection. Each actual reviewer records its identity through
`record-review SNN N --reviewer NAME --blocking N`. The helper rejects a missing
or different identity when selection is configured and closure rejects a stale
identity. These are declared provenance checks, not model authentication.
The invoking host must actually run the named independent reviewer. If it cannot,
finish implementation and applicable integrated gates, then stop at the review
handoff with no clean review record, ready-to-close transition, push or close.
Never ask Grok to attest that it is Codex.

For the user's S95.3 sequence, use `/run-sprint --prepare --reviewer codex`,
resolve the consolidated design choices, then let Grok run
`/run-sprint --reviewer codex`. Codex later runs `/sprint-review S95.3` on the
actual integrated result. Feature microscopes remain required separately.

## 1. Initialise

1. Read `CLAUDE.md`, `AGENTS.md`, `.claude/WORKFLOW.md` and
   `docs/sprints/CURRENT_SPRINT.md`.
2. Confirm the canonical worktree is on the matching `sprint/sNN` branch.
3. Refuse unrelated uncommitted changes. Changes belonging to a previous
   interrupted run of this command are resumed from state, not discarded.
4. Initialise or resume:

   ```bash
   python3 scripts/sprint_workflow.py init SNN --resume \
     --max-review-passes {N} [--max-workers {N}] [--reviewer NAME]
   ```

   Resume refreshes each known F-ID's title and size from
   `CURRENT_SPRINT.md`. It preserves phase, feature state, owner, wave, worker,
   verification, and review facts already recorded in the run state.

5. Audit for leftovers from an interrupted run. `git worktree list` and
   `git branch --list 'work/*'` against the run state. Report anything the state
   does not know about. Remove only clean completed worker worktrees whose
   handoff and integration commit are recorded. Keep their branches until
   `/close-sprint` has pushed the integrated sprint.
6. Refresh live issues and PRs assigned or requested for this sprint, pin body
   and PR-head identities, and map each acceptance row to an existing F-ID.
   Compare candidate patches and their disclosed omissions with the complete
   issue contract. Do not silently add implementation beyond user-authorized
   scope. Record capacity expansion, unresolved/reduced cases and final evidence
   obligations in the existing sprint/plan records. No external message, merge
   or closure is implied by intake.
   Classify changed PR files by their consumers. A README measurement commit
   or doctest-script change can invalidate documentation, archive or policy
   evidence even when product behavior is unchanged. Reuse only evidence whose
   source and input identities remain applicable, never the entire old gate
   solely because a contributor called the delta metadata-only.
7. Report every F-ID that is not `completed`, with its state, its dependencies,
   and the skills its diff will trigger.

### Validation-only route

When the state contains zero features, first confirm that `CURRENT_SPRINT.md`
declares `**Validation-only**: yes` and that `SPRINT_PLAN.md` explicitly defines
the sprint as a validation boundary. Refuse an accidentally empty or malformed
wave.

For a valid zero-feature sprint:

1. Commit the regenerated `CURRENT_SPRINT.md` as `SNN, open validation sprint`
   so every later evidence record can bind to a clean, stable HEAD.
2. Skip design, questions, waves, workers, integration and feature ledgers.
3. Set the phase directly to `verification` and run step 6, including every
   boundary-specific gate in the sprint definition of done.
4. Set the phase to `review` and run step 8. Commit review records before
   recording the clean review. Adopt only permitted delivery/review metadata via
   `record-verification SNN --scope metadata --reuse-from <full-head>`.
   Source changes invalidate affected checks. Never rerun all categories solely
   because the review file changed HEAD.
5. Continue at step 9. Report zero integrated F-IDs and no retained workers.

Do not manufacture an F-ID, design plan, handoff, tracker row or `AS_BUILT.md`
entry for a validation-only sprint.

## 2. Design everything first

1. Run `/design F-XXX --draft` for every unfinished story lacking an applicable
   approved design. Preserve resumed implementation rather than rewriting it.
   **No implementation in this phase.** Require the template's consumer map,
   numbered acceptance rows, discriminating matrix, concrete public signatures,
   risk routing and exact local/final evidence for each planned change.
2. Record ambiguities in each plan's `## Open questions` and keep going. Do not
   interrupt the batch to ask.
   Use stable sprint-wide Q-IDs and list every affected F-ID/A-ID. Resolve
   factual compatibility questions from pinned primary sources first. Keep
   genuine choices distinct from unavailable evidence and dependency readiness.
3. Apply `.claude/skills/risk-routing.md` to every plan. Record matched rows and
   their extra checks in `## Risk routing`.
4. Compare the drafts against each other and find:
   - Dependency order between F-IDs.
   - Files, crates and generated artefacts two stories both edit.
   - Stories that both expect to move the hash-harness baseline.
   - Crate-boundary work, where one story's extraction changes what another
     story is building on.
5. In `--prepare` mode, report the consolidated questions and stop here, without
   marking drafts approved or starting implementation. Otherwise **ask one consolidated round of questions** with AskUserQuestion. Group a
   shared decision once and name every F-ID it affects. If no material question
   exists, approve without pausing.
   Before reporting, reconcile the union of unresolved Q-IDs across every plan
   with the summary. Include referenced shared decisions and their affected
   stories. No unresolved ID may disappear through grouping or truncation.
   State any unavailable required oracle evidence separately. Each question
   includes the recommendation, consequences and factual evidence where needed.
6. Apply each answer to every affected plan, clear its open question, and set
   `**Status**: approved`.
   Preserve answered rows and answer provenance rather than deleting the
   decision history. Apply `/design`'s validation checklist, approve provider
   contracts before dependent contracts, and leave actual implementation
   readiness pending until dependencies are done. If answers or source
   inspection change a shared contract, reconcile all dependent plans first.
   Run the semantic contract closure pass, including operation/variant coverage
   and imported states, before treating answered questions as approval. If a
   revised contract affects a running worker, report the exact canonical
   revision and require its owner's acknowledgment before affected work resumes.
   Do not silently change that worker's files or reset its recorded Base.
7. Commit all approved plans together:

   ```
   SNN, approve sprint designs

   One paragraph: the shared decisions taken and which stories they settled.

   Tests, not applicable
   Harness, unchanged
   ```

   This restores a clean canonical worktree before any claim, and gives every
   worker the same immutable base. Do not push.
8. Mark each plan `approved` in state, then
   `set-phase SNN implementation`.

Do not start implementing while any plan is `draft` or carries an unresolved
material question.

## 3. Build the waves

Build a dependency graph from the approved plans. Two F-IDs share a wave only if
they are dependency-independent **and** conflict-free.

Treat these as exclusive. One story per wave may hold each:

| Resource | Why |
|---|---|
| The same source or test file | Ordinary merge conflict |
| The same integration test binary | Adding a file there adds a link target. Stories add modules to the existing entrypoint, which is one file |
| The hash-harness baseline | There is one baseline. Two stories re-recording it in parallel produces a delta nobody can attribute |
| `CURRENT_SPRINT.md`, `BACKLOG.md`, `SPRINT_TRACKER.md`, `AS_BUILT.md` | The delivery record. Workers do not write the last two at all |
| The same `docs/hld/` section, when the edit is semantic | Two rewrites of one paragraph is a decision, not a merge |
| A crate's `Cargo.toml` dependency list | A dependency-direction violation is invisible in either half of the diff |

Record waves, dependencies, exclusive resources, branches and worktrees in the
state with `mark-feature --wave N`. **Report the wave plan before launching
anything.**

Mark a dependency-prefix checkpoint before any later wave when a formal
dependency is integrated and `reviewed` but not `completed`. Every formal edge
is a completion barrier, including ordinary A to B to C chains. A release F-ID
that is a dependency of an unfinished story adds the release extension below.
Neither checkpoint is a reason to put dependent work in the same wave.

## 4. Run each wave

Run ordinary waves only while their dependencies are completed. When the next
boundary is a dependency-prefix checkpoint, prepare its dependency prefix, then
follow the checkpoint route below instead of waiting for all later waves.

Per ordinary F-ID in the wave:

1. Claim it from the canonical worktree following
   `.claude/commands/claim-feature.md`. The orchestrator issues these claims
   without asking separately.
2. In the worker worktree, run `/implement-feature F-XXX`. The approved plan is
   reused. Do not design again.
3. Consume applicable focused local checks, running only missing or invalidated
   cases. Preserve every named final risk rider and oracle as pending.
4. Run `/microscope F-XXX --working` against the worker diff. Iterate in
   numbered passes until zero defects and zero smells. **Not skippable.**
5. Run `/complete-feature F-XXX --prepare`, which writes and validates
   `.claude/handoffs/F-XXX-ready.md` and commits feature-local work to the
   worker branch.
6. Record progress with `mark-feature`, including the head sha and the handoff
   path.

A worker failure blocks that F-ID and everything depending on it, and nothing
else. Mark it `blocked`, say why, and carry on with the independent waves.

### Dependency-prefix checkpoint

Use this route when a later F-ID formally depends on an integrated story that
is still `reviewed`. The prerequisite must become `completed` before the
consumer starts, but full sprint verification and `/sprint-review` stay at
sprint closure. Keep the same sprint run state.

1. Integrate the prepared prerequisite through `/integrate-feature ...
   --batch`. Review its incremental integration diff against its approved
   plan. A semantic conflict receives another `/microscope` pass after
   reconciliation.
2. Confirm the worker's scoped local RED/GREEN regressions and zero-finding
   microscope remain valid for the integrated prefix. Only an explicitly
   approved genuine LOCAL provider gate that actually passed can unblock its
   consumer. Confirm the integrated source commit is an ancestor of the
   consumer Base. A pending oracle-only dependency gate cannot qualify, and
   reviewed status alone never means completed. Run only affected focused
   reconciliation checks, never full suites or final oracles at this checkpoint.
3. Record provider implementation completion in the existing completed run
   state and done BACKLOG/CURRENT_SPRINT cells, clearing its owner. Keep its
   plan approved and named final rider debt in existing plan/progress. Commit
   these bounded records without AS_BUILT or SPRINT_TRACKER entries or final
   oracle claims. Section7 writes the actual final delivery entries once.
   Keep the worker branch for final review. Remove its clean integrated worktree
   after preserving all evidence outside it and recording the integration commit.
4. Return the phase to `implementation` and start the dependent wave. Record
   that full verification and sprint review are still due at final closure.

The final `/verify --full` and `/sprint-review` cover the complete integrated
sprint at its current HEAD. A dependency checkpoint never supplies closure
evidence for that final result.

#### Release preparation dependencies

A release preparation F-ID is an ordinary dependency until its reviewed local
workflow, artifact and version preflights pass. Complete its delivery records
through the dependency-prefix checkpoint before starting any dependent story.
Do not create or push a release tag, publish a registry package, or call
`/release` inside `/run-sprint`. Publication follows `/close-sprint` from the
reviewed `main` merge SHA under separate approval for each family tag.

Every verification and review record remains bound to its exact current HEAD.
Never use checkpoint evidence for final closure after later waves or delivery
records change HEAD. The ordinary final integration, verification, review,
`close-preflight SNN`, and sprint push still run after all waves finish.

## 5. Integrate

From the canonical worktree, integrate every remaining prepared branch that a
dependency-prefix checkpoint did not already consume. Process them in
dependency order, one at a time:

```text
/integrate-feature F-XXX work/<fid-lower>-<agent> --batch
```

Set the phase to `integration` before the first one. Never resolve a semantic
conflict automatically. A conflict between two sprint features is reconciled
against both approved plans and re-reviewed. Any other conflict stops for a
human.

## 6. Verify once, over the integrated result

### External CI source availability checkpoint

Use this checkpoint only when an approved feature's new workflow must be on
GitHub before its named actual external CI gate can run. A validated
`external-ci-source-availability` handoff must already be consumed by batch
integration. Every implementation item, named focused local check and independent
microscope must pass. Hash, oracle and final risk riders remain explicitly
pending for the integrated union. The plan remains
approved and the feature in-progress with its named external gates pending.
Do not write completion ledgers or unblock dependencies.

Review the integrated source and authenticate the clean canonical sprint
branch, expected remote, exact ref and SHA, and ancestry. Record the local
evidence, pending gates and intended CI routing in the existing plan, progress
record and run state. The orchestrator may push this reviewed sprint ref
without force solely to make the exact source available for acceptance.
Never push a worker branch, merge main, tag or publish at this checkpoint.

Run only the named reviewed CI jobs. Bind actual run and job identities,
source SHA and outcomes to the existing records. Failed or incomplete gates
remain failed or pending, keeping the feature in-progress and closure
forbidden. Any source change invalidates affected CI evidence. After actual
external gates pass, resume ordinary final verification, feature completion,
sprint review and close preflight below. A source availability push is not a
claim that the final sprint gate passed.

For the ordinary final gate, only after every branch is integrated or consumed
by a dependency-prefix checkpoint:

1. Finish all source and HLD intent edits, then run `/verify --full` once at
   clean source H. Not per worker, not `--fast`, and not `--scoped`. Reuse
   categories already covered by the same actual suite/configuration.
2. Add the union of every `## Risk routing` rider the sprint's plans declared.
   A rider one story earned runs once here for the whole sprint.
3. **The hash harness is the step that matters most.** Every delta must trace to
   a story that declared it in its `## Hash harness` section, and the totals
   must reconcile. A delta no plan predicted stops the sprint. It is not a
   prompt to re-record the baseline.
4. Record the exact commands, outcomes and anything skipped:

   ```bash
   python3 scripts/sprint_workflow.py record-verification SNN \
     --scope full --passed --harness "unchanged | <delta>"
   ```

5. Fix failures and re-run the affected commands. Do not push while the sprint
   is red. Record an incomplete gate as a failure, never as a pass.

## 7. Finalise the record

After verification passes:

For every integrated F-ID, including release preparation and locally completed
provider-prefix stories, write actual final delivery records exactly once:

1. Confirm HLD intent was updated before the full gate. Append the actual
   `AS_BUILT.md` evidence at producer H and the `SPRINT_TRACKER.md` row. Prepare
   the close-sprint summary and velocity rows here so final review covers them
   and close can consume them without changing the reviewed tree. Do not insert
   source or HLD edits in this completion-only phase.
2. Set `done` in `BACKLOG.md` and `CURRENT_SPRINT.md`, clear its `Owner` cell,
   and regenerate the AUTOGEN counts.
3. Set its design plan to `**Status**: completed`.
4. Delete its `.claude/scratch/F-XXX-progress.md`. Its durable facts are in
   AS_BUILT now.
5. `mark-feature SNN F-XXX completed --clear-owner`.
6. Commit the ledgers as one `SNN, sprint ledgers` commit. Do not push.
7. `set-phase SNN review`.

When the sprint is validation-only, skip this section and use the route in
section 1.

## 8. Review and remediate

Run `/sprint-review SNN --pass N` through the actual requested reviewer when
configured. If unavailable, stop at the handoff described above. Record the
implementer, actual reviewer, source and A-ID/evidence coverage. Do not allow
the implementing model to substitute a self-review. Classify every finding:

| Class | Meaning |
|---|---|
| `fix-now` | An actionable defect, smell or in-scope documentation gap |
| `tracked-follow-up` | Real but not blocking. Needs a backlog home, created now |
| `human-action` | Something an agent cannot safely do, such as opening a corpus deck in PowerPoint |
| `refuted` | Contradicted by concrete evidence in the repository, cited |

A pass with no `fix-now` findings is clean. **Stop there.** Do not run a
confirmation pass after a clean pass.

Otherwise: fix every safe `fix-now` finding, re-run the impacted checks, commit
the remediation separately, and start a fresh independent pass. Reuse finding
IDs across passes so a reader can follow one defect through the sprint. Record
each pass with `record-review`. Continue this loop inside the current
`/run-sprint` invocation. A review pass remains read-only, but returning from it
to a distinct remediation phase does not require another user invocation.

Apply `/implement-feature`'s remediation discipline before each repair. When
a finding survives on a different input, retain its ID and reassess the shared
invariant. Do not count more example-specific patches as closure. Two failed
repair attempts require design/reviewer escalation within the existing bound.
Separate code defects from externally missing acceptance evidence. Do not
spend another code-fix cycle pretending to obtain an unavailable native oracle.

Use the concrete escalation handoff in `/implement-feature`. Report the actual
recipient and role, preserve the current dirty diff and wait for the previous
writer to stop before reassignment. Review delegation is not implementation
delegation. Resume unaffected work only when leases and dependencies permit.
After a focused repair review is clean, refresh invalidated final evidence
before the next sprint review. A known external acceptance blocker remains
blocking, and an unavailable named reviewer requires an explicit handoff.

At the bound, if actionable findings remain, `set-phase SNN blocked`, do not
push, and report what is outstanding. Closure stays forbidden.

After the clean review and completion records are committed at R, run the
cheap metadata applicability check, including affected delivery/review Markdown
and commit-message prose validation, plus affected final-state policy checks.
Completion can change inputs read by capability/status assertions even when
product source is unchanged:

```
python3 scripts/sprint_workflow.py record-verification SNN \
  --scope metadata --reuse-from <actual-full-head-H> --passed
```

The helper retains the actual full producer H and validates only the bounded
completion/review/plan-status/checkmark delta. Source, HLD, README, command,
script, workflow, manifest or test changes block metadata adoption. The latest
full attempt must pass. A later failure cannot reuse an older green attempt.
This check never reruns product suites or oracles. Remediation reruns affected
checks and records the actual successor evidence before any adoption.

## 9. Finish

When the latest pass is clean:

1. Run `close-preflight SNN`. It refuses on an unconsumed handoff, a feature
   that is neither completed nor carried, a blocking review finding, a missing
   full verify, or a tracker that disagrees with the run state.
2. Push `sprint/sNN` once, carrying every F-ID, ledger, review and remediation
   commit.
3. Report:
   - The sprint base and head.
   - Integrated F-IDs, and anything blocked or carried.
   - The verification evidence, especially the harness result.
   - Review passes and their verdicts.
   - **Retained worker branches and any remaining worktrees**, which
     `/close-sprint` will clean after the sprint merge and tag are pushed.
   - The prepared family tags to release from `main` after sprint close.
   - The exact next command:

     ```text
     /close-sprint SNN --next SMM
     ```

## Refused situations

- **Implementing while any plan is `draft`.**
- **Verifying per worker instead of over an integrated checkpoint or the final
  integrated result.** That is precisely the failure `/sprint-review` exists to
  catch.
- **Re-recording the hash baseline to make step 6 pass.**
- **Deleting a worker branch before `/close-sprint`, or removing a dirty,
  carried or unrelated worktree.** Only clean integrated worker worktrees may
  be removed early.
- **Running a confirmation pass after a clean review pass.**
- **Asking the user to rerun `/run-sprint` solely to cross from a completed
  review pass into its remediation phase.**
- **Merging to `main` or creating a tag directly.** `/close-sprint` owns the
  merge and sprint tag. `/release` owns release tags.
