# Current Sprint, S95

**Milestone**: M24, X, validation, typed form and rendering repairs.

**Goal**: Prioritize the new Issue384 and Issue385 validation, typed form, refusal and PowerPoint rendering repairs after S94 closes. Complete the five bounded contracts below, preserving source ownership and existing valid operations. The user explicitly expands S95 to include Issue386's three new repair stories. Seven medium repairs, one large repair and one large release preparation story total 22 to 31 developer-days of estimated effort. Prepare Word0.16.2 and shared/PowerPoint0.14.2 Rust and Python packages after all eight repairs complete. S95 does not close the whole Issue385 union or M24's final stability boundary.

## Spec references

- `docs/hld/14-development-backlog.md`, F-X248, F-X249, F-X251, F-X252, F-X253, F-X255, F-X256, F-X257 and F-X258 define the eight repair contracts and release preparation, existing provider dependencies, issue ownership and scope limits.
- `docs/hld/03-architecture.md`, Owned publication and causal paths requires candidate staging, allocation before commit, physical owner proof and exact no-op or refusal preservation.
- `docs/hld/04-opc-and-packaging.md`, Checked Word operation source ownership governs qualified SDT values, opaque XML, source namespace scope, physical replacement owners, relationship checks, schema order and parser limits.
- `docs/hld/06-presentationml-model.md`, The shape tree distinguishes slide, layout and master content, retained source, compatibility choices and the physical graph used for visibility.
- `docs/hld/07-inheritance-and-resolution.md`, Draw order supplies the current master/layout passes that the independent visibility truth table must qualify before changing routing.
- `docs/hld/09-charts-spec.md`, The ChartML model and Python authoring and owned replacement define plot direction, grouping, marker and series metadata, unknown imported families, chart source preservation and public projections.
- `docs/hld/08-rendering-spec.md`, The seam that makes this cheap defines the layout-to-paint boundary and the output paths that must consistently omit hidden master shapes while preserving layout and slide content.
- `docs/hld/10-bindings-spec.md`, Python Word document operations, Licensed embedded fonts in Python and CLI, and Live PowerPoint masters and layouts constrain the callable surfaces, typing, form contracts, binary streams, licensed font inputs, safe CLI output and current owners.
- `docs/hld/12-testing-strategy.md`, Binding tests and Integrated Python compatibility qualification require actual installed artifacts, independent saved-file and native observations, atomicity, deterministic rendering, complete source identities and honest observer limitations. The existing F-X211 metric is not the new reporter-test denominator.

- `docs/hld/15-build-and-toolchain.md`, deterministic fonts, pinned native producers and verified package inventories govern output evidence and archive measurements.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X248 | Validate identified paragraphs inside block content controls | M | done | |
| F-X249 | Set typed content control values as Word does | M | done | |
| F-X251 | Refuse invalid Word values before publication | M | done | |
| F-X252 | Refuse newly invalid Word raw XML atomically | M | done | |
| F-X253 | Honor imported chart types and hidden master shapes | M | done | |
| F-X255 | Prepare Word 0.16.2 and PowerPoint 0.14.2 patch releases | L | done | |
| F-X256 | Refuse ambiguous Word numeric units and preserve line spacing | M | done | |
| F-X257 | Complete remaining Word names and legacy dropdown diagnostics | L | done | |
| F-X258 | Complete remaining PowerPoint names color and language contracts | M | done | |

## Sequencing note

F-X255 follows completed F-X221 and all eight S95 repairs. All nine S95 contracts are complete through their declared feature gates, including actual hosted build-only and downloaded-artifact acceptance for the release preparation. Original claim, preserved WIP and intermediate evidence retain their producer attribution. Final installed, public, PowerPoint, Issue386 and pinned reporter observations bind6b7c97987f6edcec5d147116c2778865e289c5fa. The integrated native floor passes all18commands,49unchanged hashes, seven unchanged deterministic pixels and genuine verification of22crates. Formal sprint review remains due. Publication follows close under `/release`, with separate approval at the exact reviewed main SHA.

Rows follow the adopted plan and existing provider dependencies. F-X248 depends on F-X196, F-X211a and F-X222. F-X249 consumes F-X241. F-X251 consumes F-X201, F-X220 and F-X238. F-X252 consumes F-X211a. F-X253 consumes F-X209 and F-X215. Verify those completed providers at opening. Scheduling does not add dependency edges between these five stories.

Use F-X248, F-X249, F-X251, then F-X252 as the initial Word reconciliation order. They overlap native document/source namespace handling, Word binding entrypoints, existing regression carriers and HLD04/10/12. Serialize all same-file writes and integrate each qualified prefix before the next overlapping implementation. F-X248's namespace and owner repair must remain intact through later SDT normalization and edited-XML admission. F-X249's individual selector errors must not alter existing bulk bind contracts. F-X251's valid-value and stream behavior must survive F-X252's bounded raw XML admission.

F-X253 may run beside a Word story only after the actual approved plans establish disjoint source, test, HLD section, oracle, build-cache and output leases. Different document families alone do not prove independence. The archive catalog and affected README observations require a single writer. Intentional master-visibility render changes require one exclusive deterministic baseline lease and a separately labelled behavior commit. Native Office capture and final integrated verification remain coordinated resources.

Design the finite compatibility decisions before implementation. Acquire the pinned reporter fixtures, independent Word-produced typed controls, authoritative paragraph grammar and inherited MC scope evidence, and actual PowerPoint/LibreOffice logo controls. Resolve date formats, dropdown ambiguity, language and path admission, font-name validity and edited-XML depth as bounded evidence-backed design choices. This does not imply a full locale, IANA, schema, forms or generic XML framework, or new user approvals for ordinary evidence acquisition.

Keep worker artifacts and installed environments isolated. A blocked worker must checkpoint and release overlapping resources before independent work uses them. The integrator owns delivery totals, consolidated archive measurements and final gates. Estimates describe developer effort, not promised elapsed time. Open S95.11 and later boundaries sequentially through their own workflow.

## Definition of done for this sprint

- Complete all eight repair contracts and F-X255 release preparation through implementation, independent zero-defect and zero-smell microscope, scoped riders, one integrated full verification and clean sprint review. Preserve parser limits, namespace-qualified ownership, schema child order and unmodelled XML. Explain and review every intentional deterministic output change.
- F-X248 resolves Issue384's unchanged reproduction through `rdocx validate`, `Document.validate()` and `rdocx.validate_file`. Valid block-control paragraphs keep paraId, textId, namespace bindings and opaque payload through open, save and reopen. Word TOC, nested-control and ordinary-paragraph companions pass. Genuine owner mismatch and malformed-source controls still refuse without stripping identifiers or bypassing validation.
- F-X249 removes the qualified placeholder flag on successful selected fills, writes the corresponding fullDate under supported producer format metadata, maps declared dropdown values or display text to the declared display, and refuses invalid choices before publication. Individual tag and alias selectors raise KeyError when missing. Preserve native all-match counts, unsupported-kind refusal, distinct bulk bind rules, bound XML consistency, late-failure atomicity and exact no-op authority. Python and CLI results agree after save/reopen, with independent Word-produced controls and explicit finite date/calendar admission.
- F-X251 refuses the five reported Word cases before mutation or output: a DOCX saved to .pdf, duplicate stored-position tab stop, header page break, malformed language tag and authored font lacking its name table. Valid document variants, binary streams, licensed fonts, legitimate language and body-break controls remain supported. Word font CLI errors stop recommending unavailable --force while retaining collision and no-overwrite safeguards.
- F-X252 refuses the exact newly introduced invalid Word child and non-Ignorable foreign child atomically, preserving bytes, held authority and unrelated parts. Independent schema or Word authority qualifies refusal. Valid extensions, inherited namespace and MC scope, unchanged imported opaque islands and exact no-ops survive. The design names the finite edited-fragment grammar and provenance rule without substituting a renderer-modeled whitelist for schema authority.
- F-X253 independently proves the four requested imported chart subtypes rather than relying on the reporter's permissive None branch. Genuinely unknown types return None. Preserve data, plot order and opaque XML through reopen. PDF, PNG, convert and render all omit master-owned shapes when show_master_shapes is false, with actual native PowerPoint and LibreOffice evidence and sensitive master-logo controls. True and absent flag controls, layout and slide shapes, placeholders, inherited styling and backgrounds retain their distinct behavior.
- Preserve the original Issue384/385 bodies and immutable skills commit `91a57108f13877cfb6179a70077d425d6ebf098b`. Establish actual RED against final integrated S94. Retain unchanged strict XFAIL and XPASS(strict) outcomes. Separately execute unchanged bodies/checks, such as with --runxfail, and bind actual outcomes to current installed artifacts and independent semantic or atomicity observers. Never label a strict expected-failure observer contradiction as raw green.
- Pass the applicable installed Python, typing, stub, native, WASM, documentation, package, supply-chain, deterministic hash/pixel and differential riders. Preserve prior evidence and explicit historical/oracle limits. Release preparation is part of this sprint. Publication follows sprint close and separate final approval under `/release`.
- Record Issue385 as partially addressed until S95.11's F-X250 and F-X254 groups, including the remaining CLI point, also pass. Do not duplicate S94 names, signing or five-kind inventory work, claim full Issue385 closure, consume the later form-authoring programme or claim M24's end gate. M24's final stability boundary remains S99.

## New repair intake, Issue386

The user explicitly brings F-X256, F-X257 and F-X258 into active S95 before
release preparation completes. Their eight to eleven developer-days expand
this sprint to nine stories, overriding the usual capacity split. Design the
numeric and enum/string compatibility contracts before implementation. Run
Word changes in sequence where source, tests or HLD sections overlap. Run the
PowerPoint story alongside only with disjoint leases. Preserve the exact
Issue386 reproduction and independently qualify each row. Existing list-style
and picture refusals receive no duplicate completion credit. Python3.14 is
not a new qualification target. Issue386 closure requires the complete accepted
union, rather than the self-catching script's exit status.

The former four-story group remains in S95.11. The now-empty S95.1 intake
boundary is absorbed into S95. F-X255's preserved local artifacts qualify only
their intermediate source. Fresh final artifacts and release acceptance must
include the new repairs. No publication has started.
