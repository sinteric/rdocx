# F-X211c, frozen render assumptions, pass 1

**Scope**: Three failures from the unchanged first 100-script attempt. Read-only provider and approved-contract diagnosis, with no remediation, specimen changes or new successful-file credit.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Two additional failures have incompatible specimen assumptions and should remain named deliberate non-successes.

## Defect

### D1. Refresh omits owned Aptos approximation diagnostics and request-level deduplication

`crates/rpptx/src/lib.rs:24990` constructs per-owner requested/measured messages at line24995 instead of the current approximation facts. `crates/rpptx/src/lib.rs:2994` concatenates owner diagnostics without request-level deduplication. The approved contract requires Aptos, the selected actual face and an explicit approximation at `.claude/plans/F-X210-design.md:31`, stable first-use deduplication at line33, and measurement/render diagnostic agreement at `.claude/plans/F-X207-design.md:104`. The existing provider already owns this precise approximation and deduplication at `crates/oxml-layout/src/font.rs:1682` through line1712.

The original specimen creates two Aptos frames at `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-11.py:20`, filters approximation messages at line27 and fails at line28. A separately constructed, in-memory read-only diagnostic on the exact installed PowerPoint native SHA `34b052b7480a7a8b38df61bae18f4ea718c2ca58ce189956f036f32130b534c0` produced these two messages:

```text
/ppt/slides/slide1.xml [2]: requested font "Aptos" measured with "Carlito"
/ppt/slides/slide1.xml [3]: requested font "Aptos" measured with "Carlito"
```

Neither message says approximate. This is a real provider reporting defect. It is not evidence that caller Calibri was selected. The observed selected face is Carlito. Repair belongs to the provider, without changing the frozen expectation or recording the failed case as success. Later PDF assertions were not reached and receive no qualification from this diagnosis.

## Specimen dispositions

### W-AREA-19. Normal Word host discovery conflicts with mandatory Gelasio

The specimen calls normal Word PDF rendering with additional Arial and Calibri bytes at `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/W-AREA-19.py:13`, then insists on Gelasio at line18. The approved normal Word contract retains host discovery at `.claude/plans/F-X195-design.md:35` and explicitly uses the normal-base additional-font operation at line68. Caller inputs override their selected families, not every otherwise available host family. F210 preserves authentic selected faces at `.claude/plans/F-X210-design.md:31`.

The original saved PDF has SHA `4df10bfb22f0d00b5ec7c1b85e6b0c530fded6d7d0800a7e927105131411d548`. Its font table records Georgia at `/private/tmp/S91-F-X211c-final-frozen100-attempt-1/cases/W-AREA-19/word-selected-fonts-pdffonts.txt:4`. Independent extraction of its Flate-decoded FontFile2 object15 found a12476-byte sfnt program with nameID1 and nameID6 both Georgia. This is actual Georgia font-program evidence, not only a PDF resource label. The original traceback points to the Gelasio assertion at `/private/tmp/S91-F-X211c-final-frozen100-attempt-1/cases/W-AREA-19/stderr.log:2`.

Disposition: named deliberate non-success caused by the deterministic-bundled expectation applied to normal Word rendering. Do not force Gelasio to make this body pass. Subsequent PNG, immutability and saved-document assertions were not reached and remain unqualified. No provider defect was found for this bounded selection question.

### P-AREA-02. Relative ZIP member spelling conflicts with absolute owner part

The failing filter uses `ppt/slides/slide1.xml` at `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-02.py:39`. Native authoring locations retain the package part name at `crates/rpptx/src/lib.rs:25544`, and existing F207 assertions use `/ppt/slides/slide1.xml` at `crates/rpptx/src/lib.rs:31927` and line31939. ZIP member spelling and the returned owner identity are distinct representations.

A separate in-memory H457 diagnostic using the exact unchanged native above and the original qualified H457 XML returned owner `/ppt/slides/slide1.xml`, shape2, fontScale92500, lineSpacingReduction0 and status updated. This confirms the expected computation while explaining why the specimen filter returns no match. It does not execute or qualify the remainder of the frozen body.

Disposition: named deliberate non-success caused by the relative owner-path assumption. Do not change the provider owner identity or rewrite the frozen filter. Later save/reopen and idempotence assertions remain unqualified. No autofit algorithm defect was found for this bounded control.

## Smells and nitpicks

None within the three bounded questions.

## Evidence and acceptance boundary

The first attempt remains exactly100 scripts, with72 exit-zero attempts and28 failures. This report does not change that result or the full correct-file metric. Its attempts digest is `d35816d6d5df58b190e9984f2d15fd821df84b6f2ea770fa33097fa6c35cdefd`. The unchanged specimen SHA-256 values are W-AREA-19 `5330041cc1dd2e6a21f7994c6674d60490aeb53fb136c1de52584c295b838d8d`, P-AREA-02 `0b634948a09e7002762432f39ea604c5a7cc9d9f3687d8481e04c28f91c827fd`, and P-AREA-11 `3f6a15a635a6bcdb4c2ebe2b849d3be79db255897a4ce4e294ebb63f05b268d7`.

`.claude/plans/F-X211-design.md:104` requires deliberate divergences to remain in the denominator without success credit. These two dispositions do not waive the complete XML, lifetime, native, rendered or correct-file gates. The PAREA11 defect remains unexpected failure until remediation and independent qualification. No frozen script body was changed or rescored. Diagnostic objects existed only in memory under the coordinated read-only CP194 lease, using `python -I -B`. No native UI, source, installed runtime, output artifact, dependency or Git state was modified. Only this canonical review report was written after the exclusive write lease was released.
