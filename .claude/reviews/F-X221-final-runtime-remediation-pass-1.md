# F-X221 final runtime remediation, pass 1

**Reviewed**: Five working carriers on canonical HEAD `4024a82b`,89 insertions and24 deletions. Scope is the distinct current-runtime repairs and their test-oracle corrections, not previously reviewed clean source. Source was read-only throughout this pass. Only this report was written by the reviewer.

**Verdict**: Zero defects, zero smells, zero nitpicks.

## Checked product changes

`crates/rdocx-py/src/table.rs:1287` uses the existing TableGrid publication with the actual current native table owner after a successful staged row removal. The source owner is resolved from the unchanged document, while candidate inventory and revision successors are prepared before publication. The existing selected-table route at `crates/rdocx-py/src/document.rs:4815` preserves the table and unrelated physical owners while retiring affected grid descendants. Error and invalid-index paths still publish nothing. This implements the approved table-grid ancestor contract rather than broadening unrelated ownership.

`crates/rdocx-py/src/run.rs:99` supplies complete body and nested-cell Run recovery expressions to the existing first-cause stale validation. Related rich-story runs retain their separate recovery route. Scope checks still occur before reading or editing content, and no revision, physical owner, error class or returned Run behavior changes.

## Test-oracle corrections

The Word picture repair calls the existing relationship-wide sizing API with its actual relationship ID. The preserved-control test now qualifies its real checked paragraph route, namespace-complete XML, no-op byte equality, wrong-namespace and multiple-root atomic refusals, wrapper preservation, alias update and save/reopen. It retains actual regular-Run retirement after Document.add_paragraph, consistent with the physical-owner and conservative-Run distinctions in `docs/hld/10-bindings-spec.md:3832`.

The independent python-pptx notes helper at `crates/rpptx-py/tests/test_documented_examples.py:734` registers the actual notes-master relationship only when its XML registration is absent. It inserts the notes list immediately after the slide-master list, before later presentation children. Existing exact local/global relationship identity, absence of external targets and final XML registration assertions remain. The correction completes the independent fixture rather than loosening consumer validation.

The cell append oracle now requires the actual conservative Paragraph.add_run retirement and fresh paragraph recovery, while retaining the affected RunCollection check. The ungroup oracle asserts explicit namespace closure, original whitespace payload and exact remaining sibling order rather than requiring an unclosed raw prefix. These changes preserve the approved lifecycle and unmodelled XML contracts.

The current generated SmartArt pin changes only `M21_CURRENT_MINIMAL_SOURCE_SHA256` at `crates/rpptx/tests/integration.rs:12207`. The external proof `88268e1255cbe2dae87699af30e1b3ba98d3fdfbe30e5e3e3363e5a8124a370a` authenticates the old and current whole archives, identical60-member order,58 unchanged members and exactly two2735-byte notes-placeholder insertions with unchanged old prefixes and suffixes. Historical source/native pins remain untouched. Rendering assertions after the former failed pin remain mandatory.

## Evidence boundary

The preceding actual runtime receipt retains5 Word and8 PowerPoint failures, including separate LibreOffice process failures outside these source repairs. This static review does not turn that receipt into a pass. Root's current focused checks, fresh successor artifacts and final required gates retain their own actual outcomes. No test, package build, installed environment, target or native application was executed or modified by this reviewer.

Zero additional correctness, contract, panic, OOXML ordering/preservation, lifecycle, atomicity, test-strength or structure findings. This review ends here and releases the review-file write lease.
