# F-X232, integration reconciliation, pass 1

**Reviewed**: frozen staged squash against canonical HEAD on sprint/s93, 37 files, 1452 inserted and 127 removed lines. Read the approved plan, source pass 2, metadata pass 1, prepared handoff, staged HLD and baseline changes, and combined archive receipt. This audit addresses integration reconciliation and preservation, without repeating the original source microscope.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, CLI archive catalog retains the obsolete observation

`scripts/readme_doctests.py:406`

The catalog still records rdocx-cli as `(98_001, 452_334, 8)`. The actual combined archive independently measures `(98_924, 457_581, 8)`, agreeing with `crates/rdocx-cli/README.md:28` and the integration receipt. Calling the existing `validate_archive_measurement` returns false for this package. The member-byte mismatch is exact and the 923 byte compression difference exceeds the unchanged 64 byte allowance. Update this existing catalog tuple to the actual observation and rerun the four affected archive validators before the integration commit.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Source preservation: every staged changed crate source and test file matches reviewed worker source Head e0caa432e6b2659fa207486728b7d57acfd6ca08 byte for byte. The prior F-X234 facade math, F-X236 Run annotations, and F-X245 comparison, field and Python parity test files remain byte-identical to their respective reviewed source Heads. Actual packaged source and test members in all four affected archives match current integrated files.
- Other archive reconciliation: independently reopened the actual archives and invoked the existing measurement and validation functions. rdocx is 3,741,949 compressed bytes, 52,002,082 normalized member bytes and 37 members. rdocx-layout is 326,222, 1,796,099 and 15. rdocx-oxml is 530,470, 3,268,459 and 32. All three validate against current catalog observations. The rdocx compressed observation is exact, and the other differences are two and one bytes. Neither tolerance nor 10 MiB ceiling changed.
- Baseline attribution: the staged baseline retains 49 entries and changes exactly feature_showcase:pdf/bytes and feature_showcase:pdf/pages to the two hashes stated in the approved plan and retained baseline-delta receipt. The reason identifies page 3 double underline geometry. XML, resources, page 1 PNG and other samples remain unchanged. Worker history contains the separately labelled behavioral source commit, and the retained final hash check reports 49 matches.
- Independent geometry authority: the actual double-scale native source and PDF hashes match the plan and export receipt, respectively 52c35e24f7670ed6b9ae70ccb663216f89aa66ba8d5dc8d128ff82b65dd4669f and 8af352f574fdc9c0d0c647de376af2f05812356c8248cd09d6013ebb2177c2f9. Recorded 12, 24 and 36 point measurements remain within the declared 0.36 point bounds, with maximum width and centre errors of 0.30 and 0.12 points. The retained static paint evidence does not claim native tracked-bar selection or full-page font parity.
- Shared contracts: staged HLD08/10/12/15 changes retain the prior math, selective inline and chart comparison sections while adding the legal paint, complete cache identity, checked namespace closure and qualified native paint contracts. The explicit public compatibility inventory includes RenderOptions, LayoutInput, ParagraphBlock and NoteLayout range/color tuples. No additional integrated API or behavior change appears outside the reviewed worker source.

## Verification boundary

The equipped worker handoff records passing scoped gates and preserves the earlier missing-LibreOffice failure honestly. This integration audit did not run or claim a full integrated gate. The CLI catalog defect blocks the reconciliation verdict despite the other three archive validators passing. No source, metadata, baseline, HLD or progress file was changed during this review.

The review pass ends here. Return control and the metadata lease to sprint orchestration for remediation.
