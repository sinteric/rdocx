# F-X239, all, pass 1

**Reviewed**: Frozen working diff in `/private/tmp/rdocx-s94-x239` against Base `56685341fc62273b16c88c3be6c88d6dfbf91583`, 17 files, 757 insertions and 19 deletions. Read the approved conditional-wave plan, progress record, workflow, microscope command and cited HLD contracts. F-X238 implementation is absent from this base. No source edits, builds, cache mutations or GUI actions were performed by this review.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Not ready for completion. Return to implementation for D1, then review the successor.

## Defects

### D1, new regression test fails the required warnings-as-errors gate

`crates/rpptx/tests/integration.rs:36121`

The new ambiguous-notes-body regression declares `presentation` mutable, but all subsequent operations borrow it immutably. This produces `unused_mut` and prevents the test target from compiling under the required `-D warnings` clippy profile. The actual scoped clippy command exited 101 at this exact line. Remove the unnecessary mutability and obtain a passing scoped check without suppressing the warning or weakening the regression.

Evidence: `/private/tmp/F-X239-scoped-1/clippy.json` records the actual command and exit 101. Its log `/private/tmp/F-X239-scoped-1/clippy.log` contains this diagnostic and has independently checked SHA-256 `e37bcb23c22aa3984cb415452c16a33573b8cbfdc1f4621c97ac13158c4b06b5`.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: 0 additional findings. The five selected setters test the actual storage boundary, reject booleans before conversion, retain explicit zero and None where valid, and preserve meaningful signed character spacing. Native unit constructors remain unchanged. Reviewed Word size at `crates/rdocx-py/src/formatting.rs:531` and PPT spacing helpers and setters at `crates/rpptx-py/src/text.rs:89`, `:793`, `:1065` and `:2357`.
- **Contract**: 0 findings. Native notes projection distinguishes the single effective direct body from absent, grouped, multiple and AlternateContent bodies at `crates/rpptx/src/lib.rs:21226`. Existing malformed-part and relationship validation remains authoritative. The Python layer preserves missing versus empty text bodies, uses scalar text replacement, and documents the existing string-valued ColorFormat.type rather than claiming an enum migration. The theme alias preserves identity and MSO_COLOR_TYPE has the approved import-only scope at `crates/rpptx-py/python/rpptx/enum/dml.py:179`.
- **Panics**: 0 findings. No new production unwrap or unchecked untrusted indexing was found. Notes access and mutation revalidate physical slide identity. Lazy creation allocates its result before publishing the staged owner update at `crates/rpptx-py/src/slide.rs:671`. Notes frame access and source-equality no-op handling use the existing checked owner machinery at `crates/rpptx-py/src/slide.rs:2603`.
- **OOXML**: 0 findings. The bounded native projection selects the same direct body the existing scalar setter can write. Opaque notes content outside the selected body is retained. Missing txBody differs from missing body, and grouped or ambiguous effective bodies refuse the frame projection without aggregation. Tests cover namespace aliases, default namespaces, grouped and AlternateContent bodies, duplicate bodies, save/reopen and source equality.
- **Tests**: 0 additional findings beyond D1. Reviewed fresh and warm numeric refusals, exact EMU boundary controls, signed spacing, tiny positive line multiples, held owners, move/remove behavior and pinned structural differentials. New native coverage stays in the existing integration entrypoint at `crates/rpptx/tests/integration.rs:36089`. Installed exports, stubs and strict typing cover the actual notes and enum surface.
- **Structure**: 0 findings. No new product file, module, trait, generic parameter or dependency is introduced. Notes wrappers hold the existing owner and physical path and perform validation and staged publication rather than duplicating the model or parser. HLD updates describe the bounded scalar behavior and conditional evidence obligations.

## Evidence and remaining gates

Independently checked the original five-assignment script SHA-256, paired wheel bytes and recorded members, installed package/native/stub identities, and the actual command logs in `/private/tmp/F-X239-frozen-five-final/receipt.json`. All five unchanged original assignments produce the specified ValueError diagnostics with unit guidance. The original script catches its exceptions, so its zero exit alone is not treated as acceptance evidence.

Authenticated all six log hashes and successful exits recorded by `/private/tmp/F-X239-installed-2/focused-receipt-2.json`: Word 14 focused tests, PPT 41 focused tests, both strict mypy checks and both stubtest checks. These focused results do not replace the complete suites or scoped gates.

The complete 901-test PPT run, complete changed native suite, remaining scoped checks, raw corpus classification and genuine affected-archive/catalog rider were still pending during this pass. An initial native run had corpus prerequisite failures, which are not counted as passing runtime evidence. No final package or integrated gate is claimed. Any later tracked archive or catalog changes require review of that successor before preparation. The shared catalog lease remains outside this frozen source review.
