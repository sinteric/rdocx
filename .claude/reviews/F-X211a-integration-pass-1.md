# F-X211a, canonical integration, pass 1

**Reviewed**: Frozen canonical integration checkpoint `8b05293f3852bcc2c1b9e764652a0032bb72a67c34360018fc2caa010639e56b`, HEAD `7fe87def172113437c852ff5fbd9b5848b243987`, source map `79e854f574d621ca759a06c4e274d09cc19dfe63b93ac31a3d9e137f509bfbd0` and complete working patch `f9c177ca624842a2e2c1ffe662c12445a84b28edce06fbcefc52b444be3dc51f`. Valid prepared donor implementation is `cde32530aad7bd62bd5bbe19f355a32c1653d5be`, with handoff SHA `533cd66079bbe4588b9f2feef1133824ecd126c434b61f91437e636ab50b7e28`. This is static semantic integration review, not compiled or artifact union qualification.

**Verdict**: Zero defects, zero smells, zero nitpicks in the bounded canonical composition.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Publisher reconciliation

`crates/rdocx-py/src/document.rs:7590` replaces the legacy chart-only revision counter publication with the approved current-owner provider. The actual candidate is inventoried before publication. AppendBody is selected only for an actual append with no after operand, while explicit after insertion uses Conservative. The returned path is captured through prepared.revision at line7603. The returned paragraph binds its actual prepared physical owner at line7612 before Py::new can allocate. The owner revision and inventory commit at line7614 precede assigning the candidate at line7615. No second independent epoch bump or unbound returned owner remains. Fallible parsing, inventory, binding and wrapper allocation precede live publication.

The existing provider at `crates/rdocx-py/src/document.rs:4742` preserves only proved current routes and retains conservative retirement for unsupported structural changes. `commit_word_revision` at line4900 updates both owner inventory and snapshot authority. The inserted chart cannot leave the old shifted owner inventory installed. Byte-equivalent and refused operations remain distinct no-publication routes. Existing imported paragraph/header/cell preparation and F217 diagnostics-before-publication remain intact. Font metadata publishers use the migrated byte-comparison publication helper, preserving true no-ops and retaining the approved conservative policy for actual changes rather than inventing new surviving-owner guarantees.

## Meaningful source controls

`crates/rdocx-py/tests/test_core.py:5926` retains the two51/65 chart creation parameters and original chart/workbook/ordering checks. It additionally requires the returned Element to be the returned Paragraph, namespace-complete XML, identical XML replacement as a byte no-op, the held shifted tail to retire with Document.add_chart first cause, and the current tail to retain its actual text. Save/reopen checks the chart paragraph XML and exact body text order. Invalid foreign after and invalid data preserve package bytes and the returned owner. The append branch at line5968 preserves the held paragraph's XML/text, checks the new bound paragraph and no-op, and requires both held/current owners and package bytes to survive refusal before save/reopen. These are source assertions only in this pass. No unexecuted parameter receives runtime credit.

Parsed current test, typing, stub and measurement catalog Python ASTs without executing them. Independently checked that both canonical and donor top-level test/type names survive the resolutions. The accumulated public test source retains current font/F217 controls and donor a controls alongside strengthened charts. No namespace or rollback assertion was removed to reconcile the conflict. Current measured README/catalog tuple changes remain attributed worker measurements until actual union archive refresh.

## Native and prerequisite composition

Authenticated all3,292 mapped source bodies by exact hash and byte size, the complete retained patch and its live HEAD-relative bytes. Index digest remains `271bdca070ea4d7cf4cd6d295d798984e867168e00a6bbf3aa4ce5799486c0eb`. Five index paths intentionally remain unmerged although working conflict bodies are resolved. Review does not stage or bless those unresolved index entries as a completed integration.

Independently compared all11 changed OXML native carriers, four Word Python subordinate carriers and native comments with the prepared donor implementation. They are byte-exact. Current native Word document, regression and Word stub are byte-exact retained three-way tentative results. The only production difference between the earlier tentative Python document and current composition is the chart publisher migration described above. Current PowerPoint native facade and shape provider, SML table-header repair, ChartML and both semantic HTML importer/export bodies remain byte-exact HEAD. Shared prepared revision prerequisites retain the approved concrete owner scope mechanism.

## HLD current intent

Six exact child HLD carriers apply the approved root intent once:02,04,06,10,12 and14. Checked owner/resource/source behavior and parent/c acceptance ownership remain distinct. The03 architecture file is byte-exact HEAD because this child introduces no new graph edge, crate or abstraction. Existing B, font, chart, semantic export and F223 contracts remain in their current source. The HLD describes current intent rather than asserting a new compatibility score or replacing retained failures.

## Evidence and acceptance boundary

Standalone source123/129 reviews, wheel124, installed127, verified archives129, exact native/oracle successors and scoped WASM/hash/policy records retain their original source identities. They are not combined-prefix compiled, installed or archive evidence. Original full regression2392 passed, four failed, seven ignored remains failed. Historical native suites, original maximal85 and other genuine failed predecessors retain their explicit dispositions. No current full-suite, frozen100 measurement or whole-child score is inferred.

Current combined compilation, exact public85 controls, fresh paired wheel/install typing/stubs, native reconciliation deltas, verified affected archives and final prefix/sprint acceptance remain pending. Root may run source-guarded target-only work separately, but this audit executed no runtime, compiler, interpreter provider, target, CP or UI step. Only this canonical review file was written under the granted lease. No source repair, staging, Git reference, ledger or baseline mutation occurred. Review ends and returns the writer lease.
