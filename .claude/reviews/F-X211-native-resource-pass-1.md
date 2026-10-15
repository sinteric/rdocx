# F-X211, native resource phase, pass 1

**Reviewed**: Frozen native resource source26 on `work/f-x211-codex`, HEAD `36cfafaa5999709a4899f74ad4ccef773981a876`. Eleven tracked carriers, 2,297 additions and eleven deletions. Scope is native source admission, borrowed stored model admission, serializers, save staging and selected paragraph/run XML atomic glue. This is not full public-feature acceptance.

**Verdict**: One defect, zero smells, zero nitpicks. The namespace corrections and weighted control/text-box arithmetic are supported. Native resource completion remains blocked on D1 and the declared remaining acceptance.

## Defects

### D1, authored nested tables bypass the pre-recursion model guard

`crates/rdocx-oxml/src/document.rs:210`

The Table, Row and Cell cases propagate unchanged path cost. A cell containing another typed table at `crates/rdocx-oxml/src/document.rs:228` therefore permits an arbitrarily deep authored table chain without consuming a control or text-box charge. The iterative guard returns success for that chain. The parser's existing recognized-table limit32 at `crates/rdocx-oxml/src/table.rs:2587` does not apply to authored models. This contradicts the approved requirement to preserve existing table limits across mixed owner paths and refuse unsupported model recursion before serialization or staging clones.

This is an actual recursive output route: `crates/rdocx-oxml/src/table.rs:2149` calls the nested table serializer, which descends through its rows and cells and repeats. On facade save, `crates/rdocx/src/document.rs:16439` runs this accepting guard and then reaches recursive staging clone at `crates/rdocx/src/document.rs:16444`. Sufficiently deep authored input can consequently exhaust the ordinary stack instead of returning a resource error before output. This is a source-derived unbounded-route finding, not a claimed runtime crash from this audit.

Apply the already-established recognized-table policy to borrowed models before any writer event or facade clone. Qualify authored over-limit tables and accepted boundaries, preserving the writer buffer, existing output file, original package and owner state on refusal. Independently assess source and model coupling through controls or boxes before selecting any additional mixed-path policy. The bounded16-table/control and12-table/control/box observations do not establish a deeper allowance or a new quota. This finding does not select one.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: D1 identifies the remaining actual unbounded table route. Zero additional findings in inclusive weighted256 accounting, charge1 for typed controls, charge16 for actual typed text-box bodies, checked overflow and owner-close sibling restoration. Source admission follows the current typed anchor, VML, ruby, field and revision routes rather than qualified names alone.
- Contract: D1 violates borrowed pre-recursion resource refusal. Zero additional findings in the completed bounded control/text-box work. The direct legacy anchor correction preserves actual local-name native ownership. Word URI entity decoding is recognized where native parsing decodes it, while declarations originating inside captured raw anchor/MC sources retain that resolver's raw URI semantics. Opaque inline and rebound descendants remain uncharged and preserved.
- Panics: D1 is the unbounded recursive table output/staging route. Zero further findings in the new admission arithmetic or stack bookkeeping. The borrowed guard inspects stored field cache runs, nested fields, instruction originals, spans, current and original control runs, revisions, ruby runs and both authoritative and legacy anchor text without invoking recursive display selection or cloning those owners.
- OOXML: zero additional findings. Namespace undo restores scope for closed and empty owners. Synthetic wrapper bindings do not override local declarations. Actual anchor construction under an unknown graphicData URI and direct Word drawing anchor ownership remain unchanged. No parser namespace or schema ownership expansion is used to make admission match. Opaque XML remains source-owned.
- Tests: D1 lacks an authored nested-table refusal control. Zero additional findings in authenticated ordinary-stack weighted corners, source and authored control/text-box refusals, direct owner cursor atomicity, loaded main/notes/comments/glossary package refusal, selected XML no-op/atomic controls or namespace distinguishing negatives. Source25 contains two genuine runtime assertion failures and source26 qualifies their corrections. Current27 terminal evidence subsequently supplied by the writer authenticates the current-source ordinary-stack successor.
- Structure: zero findings. Concrete iterative owner and admission frame types remain in existing carriers. No new module, dependency, trait, generic, ambient admission authority, larger public-consumer stack or duplicated parser/model is introduced. The selected XML helpers remain explicitly crate-private and do not claim the full public write surface.

## Authenticated evidence

Checkpoint26 SHA-256 is `049cd41b1ca8f04537a92164c349f09ce9274e9a527f29a95f70952ecd7ca4bb`. Complete source26 map SHA-256 is `cda656bd72c33a0984ec97acd7bd3152d7284f0c26c98a108160828d7e86c068`. Tracked binary diff SHA-256 is `eac4daac26a3d89206eb1d7b5e5ac5d76255384c8e57efe348e48403969f7f2d`.

Authenticated checkpoint-listed receipts, their complete source-map hashes and every actual log hash:

- Ordinary20: `fa278f2336f2be3c58c10ba97902c583779f04bf697fffb970b5704f411a4e43`, eleven terminal zero rows including five accepted weighted lifecycle corners on ordinary2MiB release test stacks.
- Owner21: `41a007afded96daa32a3c642c56a3d56bd7f1d7fdbdd2cd75bc35bb74be117bd`, six terminal zero rows for owner, VML/ruby, opaque edge, anchor and cached-field controls.
- Atomic23: `bba676181f8779a72873a24386e8e4d369e12d36d30bb25039512149ccebaf36`, four terminal zero rows for all-target check, public reader refusal, loaded package save refusal and two selected XML controls.
- Mixed-table24: `afee61e99ce644e32024f60cd873121012c5857eb70828b637044f8fe12f3336`, nine terminal zero rows, including bounded table lifecycle observations. These remain observations rather than a complete mixed-recursion policy.
- Namespace-gap25: `a51751b7e664ad86691c2560825f93090ffef6a6fc81cb8331782b521fa0f903`, two actual exit101 runtime assertions after proving decoded Word controls and the direct legacy foreign anchor's actual typed body. No unsafe deep native parser run is substituted for these distinguishing failures.
- Namespace26: `e990551e0123c86690a73922e0cbfe375c1b0a0b7c7021a211268885967e6318`, eight terminal zero rows, all-target check and nine actual controls covering the namespace successors, captured URI origins and sibling scope, opaque inline route, standard owners, cursor refusal and field/drawing caches.

Every inspected row has authenticated output, no timeout, a gone owned group and a reaped leader. Prior failed compilation, stack-overflow measurements and setup mistakes remain recorded without being relabelled as passing acceptance. Historical source20 through source25 results are evidence at those exact inventories, not current-source26 reruns. Current27 was initially running and received no credit at that point. Before this report ended, the writer supplied terminal receipt `7f003eaeb4a57547801225bca3aed5d5916ae3ce9b35953b44fdba61ba48f0db`. It authenticates fourteen terminal zero rows with safe cleanup and thirteen exact controls on ordinary2MiB release stacks, including all five weighted corners, current cursor/save refusal, mixed-table12 observation and namespace/model successors. The metadata-only checkpoint capsule now hashes to `0372b7506cb02c6035c2a133467117e58bc9182e12628e5786b6a063d4907721`. Source26 remains exact. This completes that native successor, not the unimplemented D1 repair or broader feature. Whitespace diff check is clean. This review executed no Cargo, product or oracle.

## Remaining feature boundary

Remediation and independent re-review of D1, measured mixed-table recursion coupling, known property invariants and relationship kind/cardinality, additional Word XML owners, public Word/PPT bindings and stable handles, native PowerPoint XML ownership, final frozen100 installed consumer acceptance, HLD updates and ordinary feature verification remain due. The report neither grants a feature handoff nor treats those expressly unfinished later phases as completed.

## Review integrity

Read repository CLAUDE.md and canonical workflow/microscope instructions, the approved plan and resource amendments, current in-progress sprint/backlog status, progress and prior boundary passes. All3,203 tracked source hashes, sizes and permission modes match source26 before and after review. HEAD, branch and tracked binary diff remain exact. Physical index remains `5a949eb5421f440d730a533607236de39b84e42a012d523c6e0412701ed84532`. Logical index remains `d1ac0aa6c587289eb6fc90884f73d985c585b9a037387546047402f222247b59`.

Only this review file was written. No source fix, runtime, Cargo, Git mutation, CP, Office, plan, intake or shared ledger edit occurred. Review stops here and returns control and the writer lease to the orchestrator for separate remediation.
