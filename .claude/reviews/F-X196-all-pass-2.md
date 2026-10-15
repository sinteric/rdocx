# F-X196, all, pass 2

**Reviewed**: Independent frozen working diff against Base1eafee76, branch work/f-x196-codex. Seven tracked files, 589 additions and 17 deletions. Approved plan including opaque-control remediation, prior pass1, source, tests, HLD10/14 and actual archive carriers inspected. Source and record hashes remained unchanged during review. PR317 is outside this frozen contract and was not adopted or reviewed here.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None. Prior pass1 D1 is resolved.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: `crates/rdocx-cli/src/commands.rs:2365` maintains the existing recursive body paragraph inventory, excluding direct body tables and direct nested cell tables while following control-owned tables and their physical row/cell paths. `commands.rs:2330` rejects an inventory mismatch rather than pairing unrelated paragraphs. Expanded Word and OfficeMath names distinguish visible nontext content from foreign lookalikes, properties, deleted content and producer hints.

D1 remediation: `crates/rdocx-cli/src/commands.rs:2392` obtains the active Word namespace prefixes and the physical control owner. `commands.rs:2423` uses the existing complete typed-control admission helper, excluding wholly opaque controls. `commands.rs:2427` admits only the first expanded content container, and `commands.rs:2509` records that slot on its control frame. Self-closing raw containers do not consume it. These rules agree with the native parser at `crates/rdocx-oxml/src/content_control.rs:1057` and its raw empty-element handling at `content_control.rs:1071`. They also retain the native paragraph collector's ownership filter at `content_control.rs:1197`.

Contract: the scanner changes only the CLI empty-paragraph advisory. Text remains governed by the existing facade projection, title and author diagnostics remain intact, and independent structural errors do not suppress the visible-content scan. HLD10 and HLD14 describe the admitted counting scope and typed control behavior. No public library predicate, rendering behavior, version, dependency or new file is introduced.

Panics: the production diff introduces no unwrap, expect or unchecked source slice. Reader positions use checked conversions and the control probe bounds its source with get. The scanner continues through the document and rejects unfinished XML, covered by `crates/rdocx-cli/src/commands.rs:2670`.

OOXML: validation reads the original package and emits no XML. The namespace-aware scanner excludes opaque ancestors and preserved duplicate control content instead of normalizing or rewriting it. `crates/rdocx-cli/tests/integration.rs:2057` checks nine block/inline admission discriminators, expanded-empty versus self-closing first slots, aliases and nested opaque controls. Every case verifies original file bytes and metadata advisories.

Tests: the actual picture regression at `crates/rdocx-cli/tests/integration.rs:1852` covers three public authored images and an independent undefined-style error. The 29 visible/empty controls at `integration.rs:1898` distinguish the requested content classes from properties, markers, deleted content and foreign wrappers. The recursive scope control at `integration.rs:2021` distinguishes control-owned tables from excluded direct tables. The recorded D1 red log shows the duplicate-block inventory failure, followed by a nine-case green regression. Final supplied logs record all 4 unit and 67 integration tests passing, with no failures or ignored tests, and renewed all-target/all-feature CLI clippy passing. This review did not rerun those commands.

Archive records: `crates/rdocx-cli/README.md:28` and `scripts/readme_doctests.py:406` agree on the actual worker observation, 76,692 compressed bytes, 347,960 normalized member bytes and 8 members. Independent read-only inspection of the existing rebuilt archive through the existing archive_measurement function observed 76,691 compressed bytes, the same normalized member total and 8 members. The one-byte compressed difference is within the unchanged 64-byte tolerance at `scripts/readme_doctests.py:394`, and the archive is below 10 MiB. Packaged commands.rs, integration.rs and README.md exactly match the reviewed worker source. The two generated carriers change only the affected CLI record, with provenance, inventory and validator rules preserved. Final integrated remeasurement remains required after combining worker sources.

Structure: the private scanner lives beside its CLI consumer, uses concrete existing parser and frame state, and adds no trait, generic parameter, forwarding public wrapper, feature flag, module or test binary.

This review changed only this review file and ran no Cargo or test suite. Hash-harness, package dry-run and other remaining scoped riders are not claimed complete by this review. The clean microscope result is independent source and archive-diff evidence, not a full scoped verification result.
