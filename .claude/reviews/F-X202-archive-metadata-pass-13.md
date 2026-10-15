# F-X202, archive metadata, pass 13

**Reviewed**: sealed unadopted archive-metadata-proposal-r13, three existing carriers, four replaced lines and no product code.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input

- Capsule receipt SHA256 `2aacc3f99a7ed92e68e60c0b7f2898746ca73ce4d6805117b8bc81fb88d79120`.
- Metadata diff SHA256 `834227ee5474bd64b803e917b2baf5b9f227f995c9f5548d3958172950cd0b6c`.
- Full 3148-record source map SHA256 `0df6376ef180fd3e43e0fbb746e018de9be13f68aec5e20634be1a29481c9b5d`, source37.
- Archive evidence SHA256 `db06d85c3afc8e555ff8212f1049f5be0a493fe49603a136f1aa6f7c70ae3836`.
- Verified publish receipt SHA256 `9884c97dbb54f5c5ae97ce4c5d0bd965998e9ee3012301b48e2d7363d204850c` and raw log SHA256 `d7da43f5fb0b61ff7d7fd8f350dc8818db71a17a005a1cdd91c7f3039c9a6152`.

Read canonical microscope rules and approved F-X202 archive risk obligations at `.claude/plans/F-X202-design.md:273-283`, with HLD15 packaging normalization. Authority is the immutable proposal and retained archives, not the moving worker tree.

## Authentication and disposition

All three before/after file hashes, sizes and modes match the receipt. Preimages match the frozen source map. Only root `README.md:42`, `crates/rdocx-oxml/README.md:20` and the two constants at `scripts/readme_doctests.py:405/410` change. Version, platform, dates, command strings and carrier byte lengths are exact. Policy AST equality after restoring those two literals proves all other logic, tolerances, inventories and dates remain unchanged. All other 20 of the 22 tuples are exact. No surplus carrier, source or behavior delta is proposed.

Independently read every regular member in both retained gzip archives. Ordered inventories contain exactly 32 and 37 unique safe paths. All 69 payload hashes, sizes and tar modes match archive evidence. Every non-generated member matches its exact frozen source hash, size and mode. Cargo.toml.orig matches the owned manifest. Normalized manifests and package locks parse and remain explicitly Cargo-generated, with owned manifest/workspace lock attribution rather than a false source-byte claim.

Actual `.cargo_vcs_info.json` records HEAD `30775a483ae73c06e448dd594b002dbb4ea2881d` and dirty=true, with correct package paths. Only the declared footprint normalization removes dirty and substitutes a 40-zero revision. Independent normalized-size recomputation gives the proposed exact tuples:

| Package | Compressed bytes | Normalized member bytes | Members | Archive SHA256 |
|---|---:|---:|---:|---|
| rdocx-oxml | 486894 | 3014162 | 32 | `17034743b12723152433bb8aa4b7af648a4d4bff184b0515f0d85a1337995cf9` |
| rdocx | 3645871 | 51436597 | 37 | `6bae8d83d4d5688dc124a27753e95d6dcb6123223a28570021cd8b8b2f236f6c` |

Both are below 10MiB. Retained actual tmp-registry and tmp-crate copies still match each archive SHA, and registry index records match their evidence hash and checksum. Their generation mtimes fall within the exact command interval. Authenticated admitted archive helper source defines the same normalization and generated/source ownership distinction.

The actual terminal command is patched `cargo +1.97.1 publish --dry-run --allow-dirty --locked -p rdocx-oxml -p rdocx` with all 22 workspace LOCAL_PATCHES, no --no-verify. Its authenticated raw log shows Packaging and Verifying for both packages and two genuine `aborting upload due to dry run` markers. Exit0, no timeout, sourceAfterMatches=true and natural owned-group cleanup are recorded. This is verified dry-run evidence with no upload, not crates.io publication.

## Findings

Defects: none. Smells: none. Nitpicks: none.

Correctness, contract, panics, OOXML preservation, tests/evidence and structure each have zero findings within this metadata-only scope. No production correctness confirmation is implied.

## Limits and admission

The exact tuple-only proposal is admissible against source37. Candidate is unapplied. Equal README lengths preserve member footprint size, but changed README bytes can change compression. Measurement-only successors and the exact metadata/policy consumers remain due as disclosed in the capsule. Existing 64-byte tolerance is unchanged. No successor runtime, complete feature, installed binding, native Office, full sprint or publication credit is awarded. Original failed download/first archive attempt and prior feature failures remain retained. Reviewer performed no runtime or source edits. Pass ends at this report.
