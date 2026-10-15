# F-X214, bounded final documentation and archive metadata, pass 4

**Reviewed**: source14 to16, seven existing carriers. The bounded scope is two semantic prose corrections in HLD02 and HLD06 plus four README measurement rows and the matching archive table in readme_doctests.py. The five metadata carriers contain8 added and8 removed lines. Earlier HLD additions and the source13 product implementation are outside this successor pass.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound source and evidence

Worker branch `work/f-x214-codex`, HEAD `639d354eeee67592bb32da6fd62cb88b7d900530`. Both sprint status rows identify F-X214 as in-progress. The canonical workflow, microscope command, approved design plan and preceding clean pass3 were read. This is a documentation and measurement successor, not another whole working review.

Source14 map SHA256 `d2a8ef62780a4b53fd8e94114c7995e79680bd161815abb88c1a7d483b2e78f8`. Source15 map SHA256 `ef7e9edc7d6eae29e82ba4aea74876af655309ff40b6d84a887239406c81f089`. Source16 map SHA256 `6b0d5e1cd8cd84d89747f866ce2bf975ba974906ab18537a6d4272c0c1e326dc`. Independently compared complete maps:14 to15 changes only HLD02 and06,15 to16 changes only the five admitted measurement carriers. Every source16 recorded file independently matches current hash and size. No product carrier changes in these successors.

Archive proposal receipt SHA256 `975c59a53983287e3810f45cfbb6a4138899dbc906af83dea0e231ad4b65fc4d`, complete diff `f9b894a36a224e1349799ce55e87bfc624fe3874da5219f5687db434095462b0`. All five before and after files match their recorded hashes and sizes, and current files exactly match the admitted after copies. Parsing both Python versions without executing them proves the AST outside ARCHIVE_MEASUREMENTS is identical. Only the four affected tuples change, leaving18 other tuples, date labels and the64-byte compression tolerance untouched.

Actual verified archive receipt `/private/tmp/rdocx-f-x214-codex/.claude/scratch/F-X214-start/actual-affected-verified-publish-2/receipt.json`, SHA256 `ec7250fdca424257c3fcb11054bc07930720b1cfbfa82fd69d15fe32636a1c04`. All seven command logs independently match their receipt hashes. Four actual publish dry runs show packaging, verification and aborting upload due to dry run. All seven rows record terminal0 and safe natural cleanup. The original carrier mismatch remains explicit in the receipt rather than being relabelled as a matching old measurement.

All55 retained archive members independently match their recorded path, byte size and hash. Recomputing member totals with the existing scanner's VCS normalization yields the following compressed/member-byte/member-count tuples:

- rpptx-oxml:191298/1231922/20, archive SHA256 `12eec7e454447d8fe630eb8b832b9c966bdaa11f67ac7a08bdf766c71261b6e7`.
- rpptx-layout:97450/565621/11, archive SHA256 `9d81e3f5a0fb42b486f0e29c195d10b1c4c1152a2a5ecbd4cbd7049c2b237af0`.
- rpptx-render:83186/454262/8, archive SHA256 `ccd8e07ec8681cb146c5f5a419f8d8ee0aad1d7847ab7dcbdf01299d8607e890`.
- rpptx:597130/3150452/16, archive SHA256 `4ff067d7b7febfe2b09343d711c0c4798156bc5c382414ccd4d054f5c23a8de2`.

These are the preserved pre-metadata archive observations, not a claim that the later README bytes were repackaged by this reviewer. README byte lengths remain unchanged. The documented measurement mechanism and compression tolerance remain unchanged.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspects without findings

- Correctness and contract: `docs/hld/02-scope-and-non-goals.md:109` confines the authoring guarantee to supported routes, consistent with the approved design's explicit refusal to promise safety for arbitrary unchecked Rust constructors. It retains the depth32 success and depth33 checked refusal contract.
- Correctness and contract: `docs/hld/06-presentationml-model.md:1010` states that boxing reduces individual projection size and explicit bounded parser frames handle descent. It removes the outdated explanation that the allocation itself makes recursive group parsing safe. No authoring or parser acceptance is relaxed.
- Measurement correctness: `crates/rpptx-oxml/README.md:20`, `crates/rpptx-layout/README.md:20`, `crates/rpptx-render/README.md:21` and `crates/rpptx/README.md:31` match independently authenticated actual archive tuples. `scripts/readme_doctests.py:412` and its three subsequent affected entries match the same values.
- Tests and evidence: actual logs and retained tar members support the admitted updates. The review does not count an archive mismatch as a successful old baseline check, nor claim execution of a new package or test command.
- Panics, OOXML and structure: the bounded successor edits only prose and four numeric measurement tuples. No product logic, parser ownership, schema sequence, raw preservation, dependency, public signature, module or file changes are introduced.

## Limits

The canonical scheduling amendment847010cb is outside this worker review. The existing three reviews remain untouched. No Cargo, CP, Office, source, scratch, Git or remediation operation was performed. This clean bounded verdict does not replace final policy closure, handoff validation, integration verification or the final sprint gate. The pass ends with this report.
