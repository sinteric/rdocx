# F-X211a, diagnostic isolation, pass 1

**Reviewed**: Proposal90 only, four added ignore attributes in one existing native test carrier. No production or test body changes are proposed.
**Verdict**: 0 defects, 1 smell, 0 nitpicks. The three runner-dependent diagnostics can be isolated. The fourth invalid-input test should remain historical evidence rather than an ignored success test in the product suite.

## Authentication

Proposal SHA is `7117d36890bcc65d87a2d0f9320a776ca0c92bb39e2dc664c9319a4b266da488`. Proposed patch SHA is `cd9e238d9b5808170bc98e592d529e03744c0ef4e7cbdc2c7df59bb462ff77a2`. Candidate SHA is `e112ed7aae2f43a038e2f598d750e3971ab2708402fcc1acce3ef440dac787a6`. Current native document source remains `884989ece529c92da43ece6270971dd41797b209ca6139fb5077a77771d56838`. Removing exactly the four proposed attributes reconstructs current source byte for byte, including existing ignore attributes. No candidate was installed into product source.

Worker HEAD is `7f93a1e22b42cf93646953c5f1d3d34ddd6a9f5c`, with logical index `7daa6ae31de3979a75bf6339ec855ff8c6f7da9360956fdef2dc8a66513e869b`. Read the child Test plan and original85 boundaries, parent reviewed native recursion resource boundary, and independent clone19 and retained-construction28 reviews.

## Defects

None.

## Smells

### S1, an ignored success test retains an input that cannot satisfy its own contract

`crates/rdocx/src/document.rs:37775`

The fourth proposed attribute retains `fx211_checked_table_transaction_accepts_maximal_table_control_path` as a manual-only success test. Its input comes from raw StoryItem snapshot XML at37776, then calls checked standalone replacement at37782. The actual retained failure proves that this input has an unbound w prefix. The test has no environment-selected runner input or namespace-closing branch. Running it manually cannot reach the asserted success while the checked replacement contract remains correct.

The proposal therefore preserves a permanently misleading ignored test, rather than an executable diagnostic entry point. Its name promises maximal transaction acceptance, although it never supplies valid standalone XML. The original body and failed result are already preserved as historical evidence, and the controlled valid-root successor separately proves the required transaction. Remove this invalid diagnostic from the product test suite while retaining its original body, source, failure receipt and logs in the authenticated evidence. Alternatively, an explicitly approved new meaningful control can assert the namespace refusal, without relabeling original85 as acceptance. The other three diagnostic ignore attributes do not have this problem.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings in production behavior. The three parameterized diagnostic entry points require FX211_TABLES, FX211_CONTROLS or FX211_CLONE_MEMBER before executing their actual finite cases. Default unfiltered execution currently panics at the absent environment lookups, not at the accepted document lifecycle.
- Contract: zero additional findings. `.claude/plans/F-X211a-design.md:43` requires issue331 native resource lifecycle and installed public acceptance, while its Test plan at51 explicitly retains original85 as unqualified. `.claude/plans/F-X211-design.md:1097` requires normal 2 MiB lifecycle, actual weighted budget corners, related-story/VML/mixed topology and installed acceptance. Ignoring diagnostic entry points cannot waive those obligations or count their cases as passed.
- Panics: zero additional findings. The proposed attributes change test selection only. The three runner-dependent unwraps remain confined to explicitly selected diagnostics with required inputs. S1 identifies the fourth input's unconditional failure separately.
- OOXML: zero additional findings. No root, namespace, parser, resource limit, source closure, relationship or serialization behavior changes. Refusing unbound standalone XML is consistent with the checked owner contract.
- Tests: one smell, S1. The three useful parameterized entry points can remain ignored in the default suite because their finite normal-stack runner executions have independently qualified evidence. Their runner settings, actual cases, cleanup and acceptance credit remain explicit. The invalid fourth success test provides no such runnable input variation.
- Structure: zero additional findings. No product file, module, crate, generic, trait, dependency or feature flag is added. Historical evidence belongs in the retained diagnostic records, rather than a nonfunctional success test in the permanent suite.

## Actual failure and qualification boundaries

Retained unfiltered receipt `8ab06541a3bcef52bf32417282fb8280543bf24abcbb0cb9cadea330a58b470d` and every recorded raw log hash authenticate. The actual Word result remains 615 passed, four failed, ten ignored, zero filtered, exit101. Three failures report missing runner environment. The fourth reports InvalidNamespace for unbound prefix w. The earlier shared-support row passes all19 controls. This review does not rewrite the failed Word result as a green suite.

Clone19 and resource28 retain their independently reviewed controlled evidence. Resource28 qualifies actual ordinary 2 MiB, 30-second SAVE and complete valid-root 32-table/256-control transactions plus nineteen affected controls. Its receipt `b9127526522ea0604af474ed7882f6bd681e5f8ead0f68f58dac825059fa9e30` remains separate from original85. Installed public12, strict scoped gates and remaining declared native acceptance remain mandatory at their actual current source boundary. Ignored tests, compilation, early refusal and invalid fixtures receive no passing acceptance credit.

No tests, builds, CP, Git writes, UI, network, production or candidate edits occurred. Only this canonical review file was written. Review ends and returns the source freeze and canonical review writer lease to ROOT for a distinct remediation decision.
