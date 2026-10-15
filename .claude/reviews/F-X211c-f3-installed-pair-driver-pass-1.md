# F-X211c, f3 installed pair driver, pass 1

**Reviewed**: Frozen external driver0b8a8465, prepared configd94b8841 and exact node inventorye1c83d8f, against fresh terminal producer evidence and the approved collection/runtime custody contracts. No execution.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, observed inventory hash replaces its configured expected pin

`/private/tmp/S91-F-X211c-f3-paired-producer-1/paired-installed.py:12`

The driver loads installed-node-inventory.json without comparing its hash to
the expected inventory hash in Q.toolPins. At line15, PINS.update then replaces
that configured expected hash with sha(N), the bytes observed during launch.
The exact reviewed configuration can therefore remain unchanged while a
pre-launch modified inventory of any20 distinct PPT nodes and34 distinct Word
nodes is accepted and subsequently guarded. Collection/runtime count checks
would validate those substituted nodes rather than the reviewed selectors.

The current physical inventory independently matches e1c83d8f. This finding
concerns the executable pin contract, not an observed mutation or fabricated
runtime failure. Normal external-driver remediation should verify the inventory
against its configured expected SHA before parsing it or replacing the pin.
The current20 node set and meaningful defining controls must remain unchanged.

## Smells

None.

## Nitpicks

None.

## Authentication and checked boundaries

Driver SHA0b8a846522319af9adcca47cc17204e427ae6bcca171c7788d4a2cf5855a6db9,
prepared configd94b88419a7963ddc27e6f2a7914ff23822639c332624a80270d777be7be0490
and inventorye1c83d8fb11a6a24dfc0eb5f8f6f3f7a373d446ab65a8d146891f6e408c6f412
independently authenticate. The exact original16 PPT nodes remain, plus two
axis parameters and two corrected legacy tests. The unchanged34 Word nodes
match the prior actual50 inventory exactly.

Both actual fresh producer receipts e5292f1a and d235df25 authenticate with
three actual qualified exit0 rows each, raw log hashes and safe cleanup. This
review did not access targets or installed CP files. Root's physical fresh
wheel/native/component admissions remain separate prerequisites.

No further finding in the following boundaries:

- Authorization at external `paired-installed.py:6`, line8 and line13 requires exact config, root-pinned status, fresh artifact admission and an explicit Word execute/reuse mode. Prepared nulls cannot execute.
- Source/physical artifact guards at line19 retain complete tracked paths, source bytes/sizes/modes, HEAD/index, captured source, tools, fresh wheels and the current complete installed maps. Producer identity and current source checks remain at line69. Wheel ZIP/RECORD, ABI/platform, native/public bytes and metadata checks remain at line37.
- Word reuse at line83 requires an admitted current-source bridge, actual qualified50 row within the explicitly failed old receipt, exact34 identities and fresh native/all public component equality. Fresh SBOM and RECORD retain their own authenticated identities. Reuse is labeled actualExecutedNowFalse rather than a new34 execution. The alternative executes and counts all34 at line109.
- Fresh paired installation and actual20 at line100 retain offline/no-dependency install, post-install source/native/public member proof, exact collection20 and actual20-pass check. The new controls retain table-owned collections, indexed first-cause retirement, foreign/no-op/failure atomicity, source/save-reopen and removed-parent authority, as independently reviewed in the source microscope.
- Both families' strict smoke typing, full native stubtest and root stubtest remain at line113, followed by complete pair comparison and installed proof. No original100 body is executed by this stage and no metric follows from it.

The original failed receipts remain failed. This report claims no installation,
current20 execution, reused34 admission, typing success or feature completion.
Only this canonical review file was written. No source/driver/config fix,
runtime, compile, CP, target, UI, ledger, network or Git mutation occurred.
The review ends and releases the canonical write lease before remediation.
