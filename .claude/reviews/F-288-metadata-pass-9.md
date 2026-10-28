# F-288, metadata, pass 9

**Reviewed**: Codex independent metadata-only reconciliation review. Three files, 33 insertions and 28 deletions relative to the previous metadata content. Metadata diff against claim Base SHA-256 87010f1253b6f999c0e3a1d3b10aca960a415ed1715262c04af29a5ee00fb7ab. Product crates diff remains b12b38bcd396d4f540ff787b16004370ab79c139560f42a9ec19b81b9391acfe.
**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Defects

### M1, dependency readiness contradicts the shared status

`.claude/plans/F-288-design.md:239`
`docs/sprints/BACKLOG.md:521`
`docs/sprints/CURRENT_SPRINT.md:49`

The newly edited paragraph says the shared ledger still records F-288 pending and unclaimed. Both actual shared records say in-progress, and CURRENT_SPRINT names grok. Correct the factual readiness statement while retaining that integration and formal Q4 completion are pending.

### M2, A7 is credited with a Python fill observation it does not execute

`.claude/plans/F-288-design.md:349`
`crates/rdocx/tests/integration_test.rs:15626`
`crates/rdocx-py/tests/test_core.py:9333`
`crates/rdocx-cli/tests/integration.rs:6836`

The checked item attributes Python fill, CLI text and PDF observations to the Rust A7 case. A7 covers native text, layout, PDF extraction and CLI text. Python fill is a separate actual focused pytest case. CLI fill is also separate from CLI text. Name and consume the corresponding observations separately. Run only a genuinely missing or inapplicable focused CLI fill check on current built source if required, rather than crediting it to A7 or rerunning unrelated suites.

### M3, later AcroForm work is placed into the current feature acceptance checklist

`.claude/plans/F-288-design.md:14`
`.claude/plans/F-288-design.md:222`
`.claude/plans/F-288-design.md:350`

The plan expressly keeps Issue #381 separate and later, and the metadata audit found no named F-288 AcroForm case. The new unchecked item nevertheless describes its requalification as a pending F-288 implementation obligation. Clarify the consumer boundary and applicability: no existing DOCX legacy-default to AcroForm observation was identified in the source audit, no AcroForm runtime pass is claimed, and the future exporter remains outside F-288 acceptance. Keep actual Word A6 and final union debt pending. Do not invent a converter or count unrelated low-level PDF tests as legacy qualification.

## Smells

None.

## Nitpicks

None.

## Accepted reconciliation

DOCX-055 truthfully describes the bounded additive creation/current/default surface. Remove NA follows the existing DOCX-054 additive-surface convention and explicitly does not claim removal. Native Y describes the implemented Rust facade with authentic Word qualification still pending. Python Y and narrower WASM/CLI boundaries remain supported. Clearing the resolved row's owner and excluding 288 only from the expected incomplete-owner inventory do not weaken policy assertions.

Both selected classification and live-owner tests actually passed, and prose passed. Product source did not change, so clean product microscope pass 8 and its applicable local evidence remain reusable. This review requests metadata correction only.

## Review boundary

Codex changed no file. Grok may transcribe this exact report only. End the review pass when written, then return to a separate metadata remediation phase. No source repair, full product retest, commit, integration or provider completion is authorized by this report alone.
