# S92 sprint review, pass 1

**Reviewed**: `sprint/s92` at `fa9d261dc1c628d037ced02723ea2c1d21b6f1c1`
against merge base `80117123c1073b826392b06a6e6601311a929b44` with `main`.
58 files,9350 insertions and504 deletions. Crates touched: rdocx, rdocx-py,
rdocx-cli, rpptx and rpptx-py. The last has test changes only.
**Verdict**: 0 blocking,1 should-fix,0 nice-to-have.

## Blocking

Zero findings.

## Should-fix

### S1, current prospective references retain the superseded sprint map

`.claude/plans/F-X192-design.md:4`
`.claude/plans/F-283-design.md:4`
`docs/hld/08-rendering-spec.md:2526`
`docs/rxlsx-contribution-plan.md:113`
`docs/rxlsx-contribution-plan.md:199`

The approved Issue379 through381 insertion assigns bibliography, navigation and
watermark followups to S95 at `docs/sprints/SPRINT_PLAN.md:2683`, with the
specific stories listed at2693,2694 and2696. It moves the benchmark to S100 and
conditional Excel to S101 throughS110 at
`docs/sprints/SPRINT_PLAN.md:2653`. The edited active X192 and283 plans still
declare S94, including their prospective scheduling prose. The rendering spec
still assigns X219 to S94. The Excel guide describes the earlier S98 benchmark
and S99 start as canonical, then explicitly labels its S99 throughS108 table
the current canonical schedule. These are present-tense contracts rather than
historical execution snapshots.

Reconcile these current targets with the approved final map, including the
guide's corresponding decision and branch examples. Preserve historical
snapshots and their explicit supersession. This is a documentation correction
within the accepted planning change, requiring no new product implementation.
Fix it before closure or file an explicit followup citing this review, as the
sprint-review exit contract requires for should-fix findings.

## Nice-to-have

Zero findings.

## Milestone gate

M24's F-310 gate at `docs/hld/14-development-backlog.md:3310` is:

> The complete milestone matrix has no unexplained partial row, all required
> public surfaces pass their gates, and repeated output is byte-identical under
> the declared deterministic inputs.

That milestone-wide gate is not established by S92 and is not claimed complete.
The final roadmap places it at S99. S92 is the six-story repair boundary in
CURRENT_SPRINT, with seven additional future stories remaining pending. There
is no S92 implementation of Issue316, the later benchmark or the new parity
and fillable-PDF stories.

The repair evidence was inspected with its actual source identities:

- The full14-check local floor passed at clean
  `9cdd7980223527400a25a0ed2efa231d0713079a`. Its receipt is
  `/private/tmp/S92-integrated-full-2/receipt.json`, SHA256
  `1fa4d659f05db80fe69cbfaaeef874bb3639ff1207ff9bc8357aa00828364a61`.
  Every named step exits0, and all14 retained log hashes were independently
  checked. Commands include native default and workspace all-feature tests,
  unchanged49-entry hashes, policy, no-default-features, WASM, denied-warning
  docs, README, supply chain and the actual verified22-package patched-local
  publication dry run without allow-dirty. All22 recorded archives have the
  exact9cdd VCS identity and nonzero sizes below10MiB. The only subsequent
  tracked changes before this review are feature and sprint ledger records.
- Fresh guarded Word acceptance ran at f86f1cb7, with782 passed, all56 required
  font nodes and the three X237 plus X231 Word cases. The fresh installed PPT
  story passes1 with0 skips using actual native HTML output. Both receipts'
  command logs, wheel hashes and installed file hashes were independently
  checked. Their recorded crate, Cargo and toolchain source inputs match the
  reviewed source. The pure binding Rust receipt records3 passes and0 ignores
  at the same f86 source. Release timing does not replace guarded acceptance.
- `/private/tmp/S92-ledger-rider-closure.json`, SHA256
  `3b51207d8554051d28d0fbb189c2b1621852531b35741ee4c3ee1709ec0b9e92`,
  binds that installed epoch to current unchanged consumers across the approved
  planning and exact policy-assertion edit. This is source equivalence, not a
  claim that the installed suites ran at fa9d261d.
- Actual hosted run38755615219 remains bound to638cf3e4. All18 required jobs
  succeeded, and owner compatibility remained skipped. The narrower routing
  and native optimization retain the selected suites, pinned tools, explicit
  debug and overflow checks and the separate binding profile. Later product
  changes receive current local integrated evidence, not invented later
  hosted acceptance. Broad Word and PPT SSIM trends remain false. The hard
  Word multiscript gate passed5/5.
- Actual deterministic dev/release profile rendering remains epochb271, with
  50 decks,421 PNGs and the ordered manifests equal. The current166-input,
  Cargo metadata and binary closure is recorded in
  `/private/tmp/S92-ledger-profile-closure.json`, SHA256
  `3e3b5f685d8addd9f822b68c578b108703482c0deb7bf212d729881b0fecfc1a`.
  This is unchanged-producer evidence, not a final-source rerender claim.
- The original timing files retain candidate120/240 fill medians0.925389s and
  3.304061s against authenticated0.16.0 baseline medians0.274227s and0.893942s.
  The former3x fill comparisons fail and remain diagnostic under the explicit
  user acceptance. Strict open/edit limits, actual deterministic work counts,
  full preservation, stale-owner and atomic-refusal controls remain active.
  No future fill multiplier or further optimization is authorized.

The original f86 full-floor publication attempt was interrupted and remains
failed. It supplies no success credit. A fresh full floor at the final recorded
HEAD, current metadata bridges and state/preflight are still required after
review records and any remediation are committed. This review does not credit
that future closure step as already passed. Merge, tags and publication remain
separate authorized workflow actions.

## Not found

- **Interaction, zero findings.** X229 reconstruction and X230 grouping share
  comparison paths but preserve separate correctness and normalized-output
  contracts. The integrated regression matrix retains hyperlink shells,
  table-adjacent marks, empty and trailing paragraphs, ignored formatting,
  physical raw boundaries and both reopened resolutions. X178 setters and
  X231 HTML imports retain their own source ownership. X237 changes inventory
  construction and matching without changing comparison implementation,
  publication authority or cached source lifetime.
- **Duplication, zero findings.** No duplicate production helper introduced
  by separate stories was found. Original algorithms retained under cfg(test)
  provide explicit equivalence controls rather than alternate runtime providers.
- **Layering, zero findings.** No manifest or lockfile changes and no new crate
  dependency edges. No oxml crate gains a Word or presentation dependency.
- **Harness, zero findings.** All49 entries match, with no baseline change.
  Comparison correctness, Word/Character grouping and selected HTML whitespace
  changes are separately attributed in feature commits and AS_BUILT. Run/default
  comparison behavior and deterministic font inputs remain controlled.
- **Gate, zero additional findings.** Actual source-bound local, installed,
  hosted and profile evidence retains its distinct scope and limitations.
  The pending final record gate and uncompleted S99 milestone are explicit.
- **Docs, one should-fix finding.** S1 covers current prospective scheduling
  contradictions. Product HLD additions otherwise agree with the implemented
  setter, comparison, whitespace, inventory and CI contracts.
- **Deps, zero findings.** No new dependency or feature flag appears.
- **Surface, zero findings.** The two nullable native Run setters are approved
  by X178. The sole new hidden native story_owner_snapshots method has the
  existing Python registry consumer authorized by X237. No new Python HTML
  importer, comparison option or future-story API appears.

## Review boundary

Only this review file was written. No Cargo command, implementation edit,
remediation, commit or state mutation was performed. This pass ends here and
returns the review writer lease to the orchestrator.
