# S93 sprint review, pass 1

**Reviewed**: clean sprint/s93 HEADbd7a6f028374ead5e62a7563532a163efd91f40d against its main merge base516fc3dee63020f3acf8aa0c28ad3675a6a2cc26.95files,10392inserted and465removed lines. Crates: oxml-chart, rdocx, rdocx-cli, rdocx-layout, rdocx-oxml, rdocx-py, rpptx and rpptx-py. Approved seven-story contracts, current delivery records and final source receipts read before audit. Focused read-only native and binding subaudits supplement the integrated review. No source, ledger, baseline or git mutation occurred during this pass.
**Verdict**: 0 blocking,1 should-fix,0 nice-to-have

## Blocking

None found.

## Should-fix

### S1, The binding spec overstates dedicated SVG revision-style support

`docs/hld/10-bindings-spec.md:2644`
`crates/rdocx-py/src/document.rs:6852`
`crates/rdocx-py/src/document.rs:6874`
`crates/rdocx-py/python/rdocx/_rdocx.pyi:981`

Fix now. The new sentence says "The PDF, PNG, SVG and selected-page Python
render methods" accept revision_style, including diagnostic counterparts.
The dedicated render_page_to_svg method accepts neither revision_view nor
revision_style and invokes native rendering with default options. Its stub
correctly reflects that actual signature. A caller following the HLD and
passing either keyword to this method receives an unsupported-keyword error.

The style-capable selected-page render_pages route can emit SVG, so removing
all mention of SVG output would also be imprecise. State exactly which Python
methods accept the two revision operands, and distinguish the dedicated SVG
method from SVG selected through render_pages. Reconcile the prose with the
approved implemented surface. Do not add an unplanned API merely to satisfy
this accidental documentation claim.

## Nice-to-have

None found.

## Integrated evidence and source boundary

The source16f1fc1beb11b61d5f4e38723c173c2891a06346 full-gate receipt at
`/private/tmp/S93-full-final-4/receipt.json` has the ledger's exact SHA256
b8fc4d70663c362caf6ef9461a1d063529e30469e4b5a868d03e41f2f7a23858.
Its15commands have their required exits. The continuation retains and
identifies eight passing predecessor steps rather than claiming a second
execution. Independent log summation confirms5299default passes/59ignored
and6552all-feature workspace passes/72ignored. These overlapping configurations
must not be added as unique tests. Policy runs182tests with2expected skips,
no-default fonts149passes/1ignored, and26README examples pass. Both WASM
checks, denied-warning docs, Clippy and cargo deny pass. The clean publication
command uses neither allow-dirty nor no-verify, and its log verifies all22
current packages. Independent existing-archive measurement validates every
catalog tuple and the unchanged64byte allowance/exact member checks/10MiB
ceiling. Actual maximum compressed size is4900986bytes.

The fresh native aggregate at16f1fc1 has exact SHA256
aab4177f87e10e845536ae0ef12402b0ccd84db42cb9b066f4cb6d62c1fe2b4b.
Its four fresh receipt identities and3510tracked source snapshot entries
independently match their recorded hashes and frozen Git objects. Actual
border controls, unbordered Base equality, legal paint/scale measurements,
Pandoc conversion and math comparisons retain their declared provenance.
The original cache consumer now has three byte-equal cold/warm pairs.
Its earlier true/false/false outputs and the semantic regression RED remain
preserved. The repaired regression exercises actual direct/nested table-cache
hits, with and without provenance, after insertion before the table.

The Word installed receipt has exact SHA256
36590c7aba096b45907d0beb9cab8483647319afdbdbff26dfc3f7f71342c3a3.
Fresh806Word nodes pass with zero failures, errors or skips at16f1fc1,
plus strict typing and six-module stubtest. PowerPoint's860nodes and nine-module
stubtest remain actual12fa271results, accepted through its authenticated197
unchanged inputs and14workspace packages. Independent hash comparison confirms
all197inputs match current files and actual12fa271Git objects. This is not a
fresh combined1666-node run at one SHA. Fresh Word pytest/typing/stubtest log
hashes also match the receipt.

Only six delivery/planning documents differ between tested16f1fc1 and reviewed
bd7a6f0. Runtime, test, manifest, dependency, font and harness source remain
identical. The ledgers explicitly retain this boundary
(`docs/sprints/AS_BUILT.md:19812`,:19814). A fresh full Rust gate at the final
post-remediation documentation/review successor and explicit external-runtime
successor authentication remain required before close. This review does not
claim those future executions, a main merge, sprint tag or publication.

## Milestone gate

The M24 gate begins "the approved capability matrix has no unexplained partial
row" and requires corpus/public authoring, preservation, rendering and binding
qualification without Word repair (`docs/hld/14-development-backlog.md:2668`).
Its manual Word confirmation is separately required at:2674. S93 does not close
M24. CURRENT_SPRINT explicitly retains the S99 boundary at:9and:48, so no M24
completion or missing manual gate is inferred from this issue-repair wave.

For the S93 definition of done at `docs/sprints/CURRENT_SPRINT.md:40`, all seven
implemented contracts have feature reviews, meaningful gates and integrated
source evidence. The cache interaction found during preparation has been
remediated and independently reviewed in F-X233-cache-pass-2. The remaining
fix-now item is S1, followed by the declared final successor checks.

## Follow-up, refuted and human considerations

- The broader73input Issue365 conversion request remains pending F-X247 in S94, not claimed by S93. Its explicit contract retains the original script and unconditional-label diagnostic, then requires separate corrected-observer and semantic evidence (`docs/hld/14-development-backlog.md:8146`, `docs/sprints/BACKLOG.md:818`). This is an existing planned follow-up, not a new review finding or unrestricted TeX promise.
- Missing original Issue366 input is an approved reproduction limit, not evidence that the independently measured inferred repair failed. The exact-original and whole-page font-contour claims remain excluded (`.claude/plans/F-X233-design.md:95`).
- Math's actual1.01point mutation rejects five controls, with radical signed-residual cancellation explicitly retained. No universal per-case or SSIM-only mutation sensitivity is asserted (`docs/hld/12-testing-strategy.md:1565`).
- Legend qualification covers measured components and adjacent plot edge, not whole-chart Office equivalence. Native default-placement and font disagreements remain diagnostic. The legacy overlap observer's insensitivity is retained alongside the independent painted-bar observer and real overlay/old-renderer rejection (`docs/sprints/AS_BUILT.md:19926`).
- Chart comparison acceptance follows surviving drawing relationships and actual workbook cells. The reporter's fixed chart1.xml observer discrepancy is retained, with no canonical part-name promise (`.claude/plans/F-X245-design.md:55`).
- Global historical tracker residuals, including HLD-only F-269a/b/c and aggregate-parent coverage, predate this sprint. They are not silently repaired or counted as new S93 defects (`docs/hld/14-development-backlog.md:2899`,:2908,:2916, `docs/sprints/BACKLOG.md:570`). The scoped eleven-check audit reports zero S93 discrepancies. Seven unique completed S93 delivery entries/rows and pending F-X247 remain consistent.

## Not found

- Interaction: zero remaining actionable runtime findings. Legal style reaches reusable context, paragraph/table keys and field substitution. Border normalization uses transient clones and prices frame bands through table/note/math flow. Recursive cached table rebinding retains its group flag (`crates/rdocx-layout/src/engine.rs:2882`,:6078,:18840, `crates/rdocx-layout/src/block.rs:524`,:543). Complete chart closure identity includes legend edits before comparison alignment and uses the staged bounded copier.
- Duplication: zero findings. Shared underline geometry replaces prior copies, border grouping reuses one private mechanism and chart comparison reuses the existing resource copier. No second importer or binding owner registry is introduced.
- Layering: zero findings. No format-neutral crate acquires a dependency on either document family.
- Harness: zero findings. Independent JSON comparison finds exactly18net unique hash changes and four pixel buffers, contract/invoice/proposal/quote. The two underline PDF entries overlap the later border set. XML/resources and unrelated pixels remain exact. Pins, dimensions and thresholds are unchanged, matching the separately labelled behavioral commits and AS_BUILT attribution.
- Gate: zero additional findings. Actual source receipts support the bounded story contracts, with ignored/skipped, failed predecessor, historical benchmark and inherited PowerPoint evidence classified explicitly. Final successor verification remains an acknowledged close prerequisite.
- Documentation: zero additional findings beyond S1. Plan-listed HLD mechanism updates retain all seven story contracts. S93 effort is21to27days, S94 has five pending stories/14to19days and Issue365 remains open for its explicit future scope.
- Dependencies: zero findings. The only added edge is rdocx-layout to the already resolved format-neutral ttf-parser0.25.1, consumed by private math font reads. No new resolved package, feature or external production executable enters the graph.
- Surface: zero unapproved implementation findings. The four pre-1.0 Rust structure changes are explicitly approved and documented. Both Python legend APIs and revision-style operands match their intended entrypoints and stubs, apart from S1's prose overclaim. No unrelated CLI/WASM API is introduced.

This review pass ends here. Return to the orchestrator for separate S1
remediation and independent pass2. No fix was made during this audit.
