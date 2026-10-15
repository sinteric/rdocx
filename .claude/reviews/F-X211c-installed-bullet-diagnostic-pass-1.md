# F-X211c installed bullet diagnostic, pass 1

## Verdict and actual evidence

One defect, zero smells and zero nitpicks. Read-only diagnosis, no source edits, test changes, installation, runtime probes or corpus retries.

The original installed receipt at `/private/tmp/S91-F-X211c-PPT-final-residual-producer-2/actual-installed-1/receipt.json` has SHA256 `d0e25ddb6a2d4fcdfd8d5cf639d1c6f83953cae4e772ac2cdeee14535606aa2f`, failed status and safe terminal cleanup. Its deciding raw log has SHA256 `082f0538bf69bc48584e1ff9b399bcf078191cfc62063afe92b0dee21dee455c`, independently rehashed. Actual result is15 passed and1 failed, no timeout. The failed control stops at `crates/rpptx-py/tests/test_documented_examples.py:10307`, requiring an actionable missing or unknown Wingdings diagnostic. Source is admitted a726, installed native is `d00ab97bc9326a93bf8a2c2189e5423fcc429ed58d3388e4992fc82bf819da25`. The fresh physical artifact admission remains f43a. Neither typing nor later assertions in the failed control were reached.

## Defect D1, unknown explicit symbol markers have no honest diagnostic

`/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1753` leaves an unknown Wingdings code unmapped. At line1762 it correctly retains the original character, but lines1765 to1766 return the selected fallback without recording any unknown-symbol fact. `resolve_font_for_text` at line1647 likewise returns the primary face when coverage cannot be found without recording missing coverage. These silent routes cannot satisfy the approved source-preserved unknown-code diagnostic contract in `/private/tmp/rdocx-f-x211c-codex/.claude/plans/F-X210-design.md:316` to320.

This is an actual provider defect, not an arbitrary new test assumption. The unchanged original `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-03.py:18` includes Wingdings U+F0FF, and its line27 requires the same diagnostic as the failed owned control. Its earlier bullet_char facade failure originally prevented reaching this condition. That earlier failure and candidate remain retained.

The marker path at `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-render/src/text.rs:515` calls this resolver and then shapes the returned text. The accepted visible-marker contribution at line885 records its requested family, but `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1686` restricts approximation facts to Aptos and Aptos Display. That path supplies no Wingdings unknown-code fact. The final renderer already appends current font diagnostics at `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-render/src/lib.rs:367`. Therefore the observed failure is not simply an approximation message using different wording, or absence of a public diagnostic tuple. The explicit unknown-code event is never emitted along this marker path.

A fallback face having some glyph at a private-use code does not establish the requested Wingdings symbol meaning. Keep U+F0FF unchanged, never invent a semantic Unicode substitute. Conversely, the genuine physical requested-family coverage fast path at `/private/tmp/rdocx-f-x211c-codex/crates/oxml-layout/src/font.rs:1726` must continue winning without a false unknown or missing claim.

## Bounded remediation qualification

Keep the failed installed run and unchanged corpus bodies. Add deciding controls in existing entrypoints for an unknown explicit Wingdings U+F0FF marker with the actual family and original code in the same-pass diagnostic, original source bytes unchanged, and saved/reopened original marker retained. Qualify actual missing glyph coverage separately from unknown semantic mapping, so wording does not falsely claim that a glyph is absent when only the symbol mapping is unknown.

Inverse controls must retain the approved known Wingdings and Symbol mappings, genuine requested-face original-code coverage, and unrelated-family PUA preservation without a false Wingdings warning. Repeated cold/warm actual marker layouts should retain the same bounded ordered facts without probe-only or historical leakage. Empty or noncontributing markers must not acquire drawn-text facts. Preserve the current successful fifteen controls and the nine-control intent rather than weakening the assertion. If diagnostic storage or trace replay changes, its existing bounds and rollback controls remain required. No new font asset, dependency or semantic mapping is justified by this failure.

No raw diagnostic tuple was retained in the pytest assertion log, so this review does not invent its complete contents. Later package/source/PDF assertions remain unqualified. A distinct remediation, independent review and fresh installed qualification are required. This pass ends with this report and releases the canonical review-file lease.
