# Hanji issue 69, table alignment hash review

**Reviewed**: Original and patched 49-entry hash manifests, their harness
logs, the sample generator, the unchanged checked-in baseline, and the
before/after invoice page-one PNGs. The restored production diff is still
limited to `crates/rdocx-layout/src/table.rs`, with 120 insertions and 10
deletions including tests.

**Verdict**: 5 explained intentional output deltas, 0 unexplained deltas,
0 review defects, 0 smells, 0 nitpicks. The observed changes match the reviewed
table alignment correction.

## Harness result and acceptance boundary

The original implementation's run reports `49 entries match`. Its captured
manifest exactly equals the entries in `scripts/hash_baseline.json`.

The patched implementation's run reports `output delta detected` for exactly
the five entries below. Both manifests contain the same 49 keys. There are no
added or removed entries, and the remaining 44 entries match.

This is a reviewed expected behavior delta. It is **not** a green patched
`--check` result. The harness returns status 1 for any changed digest at
`scripts/hash_harness.py:393`. The checked-in baseline was not refreshed or
edited. The intentional alignment change must remain identified in the
contribution's commit and review record, as required by the repository
workflow.

## Exact changed digests

| Entry | Original SHA-256 | Patched SHA-256 |
| --- | --- | --- |
| `invoice:page1.png` | `d78bed48649fed8147249da820dde7c5f65499ba85e1493ebd04bd1de5265c4c` | `45df7ea3fcf0309a535385bc271754009907bbf96ee5ae10768cb5fb688c20c9` |
| `invoice:pdf/bytes` | `873181a6bab44d258c70be8c3d5cab9bc67dc5fbf79ed0e899ea9ca2a07062a8` | `3e638ea3f8cb1a5c7f023da094216e5661d46af50ad365532a87a39d3a8b7323` |
| `invoice:pdf/pages` | `8ffd0b50e30b98c135668de72ec961b6d3feecf4239da189aa89792afe2a4e2d` | `3e57259dc267a5cc775774be412f9a6a3815488f4d962fa0996b762167cb64a3` |
| `quote:pdf/bytes` | `c1713e6692c8ca715e57cae133aae834050b9568238183dde3302dcae5476788` | `ea720b5ab0b0156b0cdba8cad68c74a39e3f004137b6f25b6856f8635aea0d83` |
| `quote:pdf/pages` | `315d493b00681f0b60446a78fb24023823162c1d15ce58c46ec1d591d3112716` | `296d748be2d89d5c55df1a0ae0bfb0421b1059ae78d5ce2ab5b698fe26a5f822` |

`pdf/pages` is a fingerprint of page geometry and inflated page content
streams, not a page-count value. The definition is at
`scripts/hash_harness.py:265`. A horizontal table translation is expected to
change it and the complete PDF-byte digest while leaving resource-stream
digests unchanged.

## Why these deltas are expected

The quote totals table explicitly authors right alignment at
`crates/rdocx/examples/generate_all_samples.rs:1166`. The invoice totals table
does the same at `crates/rdocx/examples/generate_all_samples.rs:1385`.
Both generated XML documents have 504 points between their text margins and
468 points in these tables' declared grids. Preserving the direct alignment
therefore moves each totals table 36 points to the right using the existing
grid-width calculation. The authored preferred widths are left to the
unchanged sizing behavior.

The restored fix makes this calculation at
`crates/rdocx-layout/src/table.rs:532`. It changes placement after resolving
column widths. It does not change the sample generator, XML serialization,
font resources, or the hash scanner.

The original quote PDF has two pages, with the `Subtotal` text on page 2.
This agrees with the observed unchanged quote page-one PNG and changed PDF
page-content fingerprint. The PNG harness entry only covers page 1.

The contract's centered table is authored at
`crates/rdocx/examples/generate_all_samples.rs:1980`. Its 468 point declared
grid fills the 468 point text area, so its expected center offset remains 0.
All contract entries match. The source generator uses deterministic PNG and
PDF rendering at `crates/rdocx/examples/generate_all_samples.rs:67` and
`crates/rdocx/examples/generate_all_samples.rs:74`.

## Invoice visual and pixel inspection

The reviewed before and after images both measure 1275 by 1651 pixels. Their
SHA-256 digests match the corresponding manifest entries above.

Visual inspection shows the totals block moving toward the right text margin.
Its text, width, row heights, vertical position, and neighboring content are
preserved. All changed pixels are confined to the bounding box from
`(111, 1430)` inclusive to `(1163, 1526)` exclusive. Every pixel outside that
box is identical.

At the generator's 150 DPI, the expected 36 point translation is 75 pixels.
Comparing the totals crops after that translation leaves only 190 pixels
along the left border, with a maximum difference of 2 in any color channel.
That small border antialiasing difference is consistent with translated
floating-point coordinates. The text and table body match after translation.
No additional visual delta was found.

## Unchanged coverage

- All 21 tracked XML manifest entries match, including the intentionally
  absent invoice numbering part. These cover document, styles, and numbering
  for each sample. This statement is limited to the XML parts the harness
  actually records.
- All 7 PDF resource fingerprints match.
- The other 6 page-one PNG fingerprints match.
- All complete PDF and page-content fingerprints outside invoice and quote
  match.
- The checked-in baseline, hash scanner, and sample generator have no working
  tree changes.

## Evidence and verification scope

Captured evidence was read from these workspace files:

- `rdocx-alignment-base-hash.log`
- `rdocx-alignment-hash.log`
- `rdocx-alignment-base-hashes.json`
- `rdocx-alignment-patched-hashes.json`
- `rdocx-invoice-alignment-before.png`
- `rdocx-invoice-alignment-after.png`

The supplied `rdocx-alignment-tests.log` independently records 309 unit tests
and 1 doc test passing. This review did not execute a build, test, or harness,
did not modify source or images, and did not update a baseline. It only read
the supplied evidence, compared data in memory, and wrote this review record.
The zero-finding source review remains recorded separately in
`hanji-issue69-table-alignment-pass-2.md`.
