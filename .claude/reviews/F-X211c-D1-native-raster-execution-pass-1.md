# F-X211c D1 native raster executable, pass 1

**Reviewed**: Proposed finite execute-nine.py SHA256 `e69b47342bc490e9007005121623ecfb000868e13940ae10da2470b3019e5dae`, updated proposal001c7d and actual external compile432a83/current artifact graph6c1858. No driver, renderer, CP or Office execution.
**Verdict**: One lifecycle defect, zero demonstrated product defects, zero smells and zero nitpicks.

## Defect

### D1, a child surviving the termination grace period can be reported safe to release

`/private/tmp/S91-F-X211c-current-output-qualification-preparation-1/D1-native-raster-concrete-preparation-1/execute-nine.py:45`

On timeout, the driver sends SIGTERM then waits10 seconds. If the child does not exit, that wait raises TimeoutExpired before the current process row is appended at line48. There is no SIGKILL fallback or exception-safe record of that active process. Finally at line77 computes safeToRelease using only already appended rows. If those earlier rows are safely terminal, it can report true while the current child/group remains live. An empty earlier row list also yields true.

Record custody of the current process before waiting, use bounded termination escalation, and always retain its real exit/cleanup state on failure. Release must remain false until the current leader is reaped and group absence is confirmed, including any failure during process launch/wait/cleanup. Do not infer safe release from an earlier successful prefix. This is required before execution and does not justify rerunning any glyph comparison.

## Authenticated executable and reference scope

The proposed root admission pins driver, inputs, graph and compile receipt at lines27 to29. Its output directory must be fresh. The nine exact input rows preserve physical font paths/hashes, scalar, regular18/11/15.3/13.95-point sizes, black style, fixed baseline48,128 and256-point canvas. Their actual f6 selected physical face/scalar/style coupling remains the separately reviewed PDF/source authority, not family-label inference from this new provider fixture.

Independently rehashed all208 artifact graph nodes and751 named physical files, current root PPT native0f21, compiled helper binary05df and its retained compile log. The compile receipt is actual exit0 with source and graph unchanged. Its thinLTO successor retains the failed original linker attempt separately. The graph chooses exact Cargo fingerprint edges and explicit oxml_pdf/oxml_layout rlibs, not newest artifact lookup. No raster or glyph comparison follows from successful compilation.

At execute-nine.py lines6 to24, the oracle requires isolated, no-bytecode execution in the exact oracle venv, pinned package lock and original archive/unpacked/installed member bytes. The independent resvg qualification10a8 and renderer e285 are pinned at line30. Guards rehash configured inputs, compile/binary, current root native, graph artifacts and each original font around every subprocess. Exactly five reference renders and one native paint are requested per row, yielding54 subprocesses only if all nine controls proceed. Each successful process must have exit0 and a gone/reaped group. The defective timeout path above is the sole lifecycle exception found.

## Deciding paint method

At lines60 to66, FontTools obtains the original cmap/glyph/hmtx and exact quadratic contours from the pinned full source font. The expected SVG transforms use declared source size/origin and invert the font y axis. The ROI at line62 derives solely from original glyph bounds with fixed padding, asserts canvas containment and minimum SSIM window. No observed bitmap registration, crop-to-ink or fitted displacement occurs.

At lines63 to73, the positive retains exact original contours and the four negatives alter only wrong character, empty paint, six-point offset or size. Actual native ink must be nonempty. Both reference and native image sizes are checked. The original0.99 deciding threshold and each negative below0.99 remain mandatory, with no threshold relaxation. Actual comparison/output facts are saved before asserting the decision, preserving a failed result. Repeated native PNG equality and separately retained actual PDF outline/metric attribution supply distinct facts, without treating a generated product PDF as expected contour geometry.

The updated proposal makes the original Office two-row morphology gate explicitly mandatory. Its source-family circle/small-square observations remain separate from the nine physical-font contour controls. No cross-family0.99 or metric equality is introduced. The previous proposal's optional-native routing finding is resolved by that explicit requirement, while actual Office completion remains root-owned and uncredited here.

## Limits and handback

The current-source public APIs were independently inspected in the earlier proposal review and are now backed by genuine external compilation. All raster comparisons, original Office morphology facts and final typed D1 admission remain unexecuted or separately owned. Unknown/unrelated PUA and caller/Aptos authorities remain independent required controls. No full font acceptance, score or output mutation is assigned here. This review ends with its single concrete lifecycle finding and releases the canonical review-write lease for a separate normal remedy.
