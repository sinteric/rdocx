# F-X200, archive metadata, pass 67

**Reviewed**: Four metadata carriers, 8 insertions and 8 deletions. Packet `/private/tmp/S91-F-X200-owned-history-adoption-prepared/measured-metadata-proposal`. Patch SHA-256 `86154998ab0bb2fc994cbf2448cf4b17a9e6f18f441dd06ceca8aa32fa23be74`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authenticated scope

Proposal JSON SHA-256 is `862c4facedae7efc8a9aebe39473b2d93b878e7649747f73d6fdd6a7588794cb`. All 3,069 current source records match map `49b804d863f48a1d4171970d2cdfaf5e59071e2e2e51382ca775b1f795a4c9c9`, including sizes and modes. Actual HEAD is `0854cf1c42a36c39ec47a13fe639889c686dccac`. Staged entries match the adoption receipt. All four preimages and candidates match declared hashes and retain equal byte lengths.

Actual verified publication receipt SHA-256 `a4b7b4ba6b7546a47e8df41ac2d21adace2db556cc7bd96c53551c3a428ea855` records exit zero and natural process-group disappearance without signals. Raw log SHA-256 `78b019610519aa14a19c96feb0bf910b53aacf29cd4e0c5f630467a98be2aea9` contains all three Packaging and Verifying results and three dry-run upload-abort markers. The argv retains all 22 local crate patches and `--locked`. No publication occurred.

| Package | Compressed bytes | Normalized member bytes | Members |
|---|---:|---:|---:|
| rdocx-oxml | 474777 | 2947600 | 32 |
| rdocx-layout | 320435 | 1768091 | 15 |
| rdocx | 3590097 | 51125574 | 37 |

Independent reads reproduce all three tuples. Archive evidence SHA-256 is `5881dac0fed3ef3f6428e9966478784b29eb7ba3e23ff56fe1865a727d5246b2`. Every ordered member payload hash, size and mode matches its inventory. Source members match current owned bytes and modes. Actual VCS records retain the correct HEAD and dirty state. Independent normalization removes only dirty and replaces the commit SHA with 40 zeroes, matching the retained helper at `/private/tmp/S91-F-X195-integration-continuation-r62-prepared/run.py:435`. Generated manifests and lockfiles remain separately identified. Both actual temporary archive copies and registry checksums match retained archives. Their mtimes fall within the verified command window.

## Not found

- **Correctness**: Zero findings. Candidate rows at `README.md:42`, `crates/rdocx-layout/README.md:21` and `crates/rdocx-oxml/README.md:20` agree with actual measurements. The rdocx README uses the declared root README delegate.
- **Contract**: Zero findings. Candidate `scripts/readme_doctests.py:405` changes only these three tuples. Other 19 tuples remain exact. Its date map at `:369` changes only layout and OXML to 2026-10-12. The rdocx date was already correct. Other dates remain exact.
- **Panics**: Zero findings. Literal metadata changes introduce no execution path or input handling.
- **OOXML**: Zero findings. No XML, serializer or package ownership logic changes.
- **Tests**: Zero findings. No assertions or predicates are removed. Proposal `proposal.json:35` explicitly requires a current metadata measurement and declines new verified publication credit from that successor.
- **Structure**: Zero findings. No new product files, modules, traits, dependencies or wrappers. Only three measurement rows, three tuple constants and two date constants change.

## Evidence limits

This is an independent incremental metadata review, not a whole-feature review. Verified original archives remain authoritative for their recorded source. Changed README bytes, although equal in length, require subsequent measurement-only packaging to establish current compressed bytes within the existing 64-byte tolerance. This review claims no such result, feature completion or fresh native, installed or publication pass. No source, runtime, Cargo or CP changes occurred. The designated old worker review directory was absent after cleanup and was recreated solely for this authorized report.
