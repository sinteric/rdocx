# F-X221 wheel routing remediation, pass 1

**Verdict: 0 defects, 0 structural smells, 0 nits.** This bounded independent review covers the new release-routing amendment against `01a4c07a`. It preserves the earlier clean sprint review as historical and does not replace the required final integrated successor review. No tests, source, environment, index or ledger changes were performed by the reviewer.

## Routing and operative consumers

`.github/workflows/wheels.yml:28` defines one positive family inventory with37 Word and41 PowerPoint node IDs. Its canonical identity is `28abec50e648f84c3072d224c947d81bc1cb2d9cc1766cccd61022046a3fa2b8`. These are the existing installed priority and compatibility controls, not newly weakened replacements. Missing exact pytest nodes fail collection.

The native program at `.github/workflows/wheels.yml:165` loads that family inventory and passes every node as a separate subprocess argument with failure propagation. The musllinux path explicitly transports the job environment at line209. Its inner shell invokes the same Python consumer at lines217 through225. JSON node IDs containing spaces, brackets or empty parameters remain structured argv values. The quoted inner heredoc does not expand their contents as shell syntax. Its surrounding shell quotation is compatible with the embedded program. The native and inner musl syntax validation is syntax evidence, not an actual Docker run.

`scripts/test_sprint_workflow.py:2981` validates family membership, uniqueness, counts and exact inventory identity. The same helper requires both operative programs, native family routing and Docker transport. The added test at line3012 exercises the consumer and meaningful missing, additional, duplicate, missing/commented transport or consumer, and ignored-failure mutations. Those controls distinguish operative commands from comments. Metadata validation, native imports, fresh wheel installation, strict Python3.12 typing and recursive stubs, matrix routing and push-only publication guards are unchanged.

## Acceptance partition

The bare native and Alpine hosts now run the admitted portable installed-runtime inventory rather than broad test modules whose independent font, native viewer, Poppler or CLI inputs are absent. `docs/hld/15-build-and-toolchain.md:694` describes the positive78-case contract and its separate equipped acceptance environment. The corresponding architecture around line914 retains complete suites in equipped CI. The amendment removes no font, geometry, native-eight, CLI or original compatibility acceptance obligation. The complete778/854 suites,104 font cases and separate ROOT native-eight remain mandatory in their authenticated equipped gates.

The root-owned F-X221 plan amendment and current sprint/roadmap notes consistently identify this as an amendment after the prior local checkpoint, with hosted rehearsal still due. They preserve completed story history and frozen intake. No terminal feature state was manually reopened, and no publication or closure is claimed.

## Actual verification and source boundaries

`/private/tmp/S91-F-X221-wheel-routing-amendment-1/receipt.json`, SHA-256 `b82fbb3ad684cf5bd9868d4ff930a344547bbb641f088f015c09744a398cf185`, records genuine bare Python3.9 collection and runtime:37 Word and41 PowerPoint passes, all78 collected members passing, with source and installed stability. The original bare mandatory F195 missing-input failure remains retained as the defining pre-repair routing failure. This local result is not a claim of actual hosted Windows, Docker or release execution.

`policy-and-source-custody-receipt.json` in that directory, SHA prefix `dc9a68fc`, records the original full policy invocation:161 tests,159 passes and two existing platform skips, log SHA-256 `55d7c0f8346658fc18244c7e2b2156af85aa651738043d45028610d08872040e`. Operative-line assertions and two commented-musl negative controls were strengthened after that invocation. The four affected current controls then passed, log SHA-256 `ed0770e91399d03f49edafc1bc5dba54194da2d7ac1bc29e9794edff19552db7`. Only the policy-test carrier changed in this successor. Workflow and runtime bytes remained exact. The earlier whole-module execution is not relabelled as execution of the later policy body.

No additional integration, dependency, public API, acceptance-waiver or source-custody finding was identified within this amendment. Actual hosted build-only rehearsal remains a future gate. This pass ends on writing the report and releases the exclusive canonical review-file-only lease.
