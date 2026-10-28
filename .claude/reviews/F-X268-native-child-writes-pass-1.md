# F-X268, native child writes, pass 1

**Reviewed**: Codex root reviewed the mechanism 7A native write cache and holds_field binding integration on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The two-file native and binding diff has 479 added and 13 removed lines. The OXML serializer prerequisite has its separate clean pass 2.
**Verdict**: 2 defects, 1 smell, 0 nitpicks

## Defects

### D1, the serialized-child count drops retry work

`crates/rdocx/src/document.rs:7584`
`crates/rdocx/src/document.rs:7596`
`crates/rdocx/src/document.rs:21171`

Child writes accumulate in a local fresh count. A root mismatch recursively retries without carrying the first count, and main_part_xml records only the returned count after successful serialization. The counter therefore measures the final successful output rather than all attempted child writes.

The actual discriminator in `/tmp/fx268-m7-write-probes.log` warms five children including one raw field, then changes the root attribute. The first attempt writes the field once and the retry writes all five children. Output bytes match the whole writer, but slot 4 reports 5 instead of 6. The probe failed in 0.02s. Record work at actual write attempts, including retries, fallback writes and work before an error. Do not fix the metric by lowering the expected work.

### D2, the new write-cache test does not compile in release

`crates/rdocx/src/document.rs:43876`
`crates/rdocx/src/document.rs:43890`

The write-cache regression calls take_staging_work without configuration guards. That helper is debug-only. `/tmp/fx268-m7-write-release-red.log` records exit 101 and 12 E0425 errors in this test. The library target itself is not reported as failing.

Gate the counter calls and their dependent assertions while retaining functional checks in release. The earlier addressing D2 repair demonstrated this same conditional-test requirement, but the new case does not follow it.

## Smells

### S1, warmup and mixed-block checks do not enforce the named workload

`crates/rdocx/src/document.rs:43882`
`crates/rdocx/src/document.rs:43941`

The test makes two edits before one write and then performs an unchanged write. This is not two distinct changed warmup calls. Write after each changed warmup.

The mixed assertion accepts any count at least four. It therefore accepts rewriting every child at both sizes. Require the exact four writes for the changed neighbor, field, tab and bookmark, or an equally discriminating paired bound that proves other neighbors are reused. Keep the actual extra work of invalidated-root retries visible when correcting D1.

## Nitpicks

None.

## Not found

The quoted-angle root-range concern was refuted. The actual writer emits producer as a&gt;b, and both the current range and the checked XML event report 55..387. Changing the suffix changes the retained root bytes. That probe passed and is not a finding.

No additional output-byte, resource-admission, raw-preservation or history-chain defect was established in the inspected write-only path. main_part_xml retains whole-model resource admission before the block writer. Binding alignment is separated from reuse and retains the existing private-fact model guards.

The previous write group and seven address regressions passed, 8 cases in 20.28s. That debug result does not refute D1 or D2. Child-read staging, first-four-slot bounds, full A10, Python rebuild, HLD, release behavior, composition, memory and final gates remain pending. Temporary reader fields and their dead-code allowances are not final mechanism 7 acceptance.
