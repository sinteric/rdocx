# F-X212, tests, pass 9

**Reviewed**: Supplemental publication carrier update in four existing files: root README.md, rdocx-oxml README.md, rdocx-cli README.md and scripts/readme_doctests.py. Independent archive measurement, member/source/mode authentication and whole tracked-input guard review. No production or baseline remediation is included in this pass.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Authenticated carrier-update-r7.json SHA-256 `fc96544d28cd91868f06515dcd595221274cf32ade856c350001e139a153fa53`, prior snapshot `3c3dd43a875928764585f7ceaffaca8fd4fd138f913779ff5bd800525db09f4a` and fresh source-snapshot-r7-carriers.json SHA-256 `657b32066e846aa6c3cc18b56b8a17b74de471bad2478be358a157af820fe089`, all under /private/tmp/S91-F-X212-publication-r7.

The branch remains work/f-x212-codex with HEAD `ea803be1e59ba588d4c8da3f1231d84314cd847a` and claim Base `66d06bf3e3f4287afb35dea8217e07f98efa7bfd`. All 2,679 tracked paths match the fresh snapshot's file sizes, SHA-256, filesystem modes, Git index modes and blob OIDs. The two snapshots have identical path inventories and differ in exactly the four declared carriers. Rechecked all current input hashes, sizes, modes and HEAD immediately before writing.

The measurement receipt SHA-256 is `73dc3ff548e868cfdedfbd1eb918af310821b877afecdba4456451d2373f24b0`. All three retained command logs match their hashes and record true exit0, without timeout, for package measurement using locked, allow-dirty, no-verify and the complete 22 local patches. This is measurement evidence, not package verification.

## Actual archive measurements

Independently read each retained compressed archive. Recomputed its whole-file SHA-256, compressed size, unique regular-member inventory, every member size/hash and tar mode, and normalized member-byte total. Normalization matches scripts/readme_doctests.py:1291: remove dirty, substitute forty zeroes for git.sha1, and count json.dumps(indent=2) bytes without a trailing newline. Actual VCS HEAD and package path agree with the source snapshot.

| Package | Compressed bytes | Normalized member bytes | Members | SHA-256 |
|---|---:|---:|---:|---|
| rdocx | 3,536,946 | 50,836,206 | 37 | `3b7f336b5eae8abd221544115338219b119143cac4989ff4cc361096eb9c1256` |
| rdocx-oxml | 457,659 | 2,842,528 | 32 | `c8905e2a2e1e00021a6b18293807ae8590a30cf2fde57400e9e60e2aaa592cd6` |
| rdocx-cli | 76,293 | 350,712 | 8 | `f82742dc7fcc507807163718101e012c82aba767c7424869b94d18b756dd9312` |

The archives contain respectively 34, 29 and 5 exact tracked-source members plus three explicitly generated Cargo members. Exact members match the pre-carrier source snapshot's bytes and modes. Cargo.toml.orig matches the owned source manifest. Normalized Cargo.toml, package Cargo.lock and VCS metadata remain correctly classified generated members, rather than claimed exact source copies. All unchanged packaged source also matches current worker bytes.

The three packaged README files precede the carrier-row edits, as expected for the measurements that supply those rows. Those edits preserve their byte lengths. These retained archives do not prove that final changed README bytes are already packaged. Fresh verified packaging and actual final archive equality within the existing tolerance remain necessary. The record preserves that distinction.

## Carrier assessment

README.md:42, crates/rdocx-oxml/README.md:20 and crates/rdocx-cli/README.md:28 record the exact measured tuples above with date2026-10-10. scripts/readme_doctests.py:378 and :404 carry their corresponding dates and measurements. Independent AST comparison with the authenticated prior script proves exactly three tuple and three date changes, with the other 19 package records unchanged.

The rdocx-oxml README already states date2026-10-10. The script now states that same date, preserving the intended date reconciliation rather than restoring the old2026-10-09 mismatch. No unrelated accepted source prefix row is overwritten. The newer canonical F-X194 union still requires separate integrator measurements and carrier reconciliation.

ARCHIVE_COMPRESSION_TOLERANCE_BYTES remains64 at scripts/readme_doctests.py:394 in both old and new source. Exact normalized bytes and member-count comparisons, the10MiB ceiling and pinned/nonpinned compiler branches at :1348 remain byte-identical. No new waiver, archive member relaxation, threshold change or date fallback is introduced. Manifest versions, patch membership, full inventory and file modes are unchanged between the two snapshots.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. All three tuples, dates, archive hashes and normalized totals agree with independently recomputed retained evidence. |
| contract | 0 findings. Only the four existing carriers change, preserving source ownership, strict package membership and the measured-only qualification. |
| panics | 0 findings. No executable control-flow or input-handling change exists in this carrier delta. |
| ooxml | 0 findings. No source, model, XML serialization, namespace or output baseline change exists in this delta. |
| tests | 0 findings. Other19 records, unchanged64-byte tolerance, exact normalized members/counts and date reconciliation are preserved. Final verified archives remain pending rather than inferred. |
| structure | 0 findings. No new file, module, dependency, public API or alternate carrier is introduced. |

Root's verified workspace package run45991 was active when this review was requested. This review grants no terminal verification, policy, archive-validation, feature completion or integrated sprint credit. It ran no Cargo, imports, formatting or source/progress edits. Only this review record was written. The supplemental pass ends here and returns control to the source owner.
