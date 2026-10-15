# F-X213, proposal, pass 1

**Reviewed**: External complete proposal against claim Base `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, three existing carriers, 1460 patch lines. Complete diff SHA256 `4f1a784f4345aa5edfd360c866805f3397d14e209998617f2b9395934263fd54`. Receipt SHA256 `3d4e73451fd10b6ca64297dd73745ba2f5d0bb6f807f1f1d205e9192ea5d35e3`. All three candidate and preimage hashes match the receipt, and all preimages match actual Base Git blobs.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

Source citations below refer to complete candidates in `/private/tmp/S91-F-X213-implementation-proposal/candidates`, retaining repository paths and candidate line numbers. This is independent proposal review against the approved plan and authenticated PR341 assessment. It is not a final working microscope or runtime acceptance. Genuine Base, compiler, focused native, installed, deterministic and risk gates remain pending.

## Defects

### D1, single-owner expansion loses the retained end run namespace scope

`crates/rdocx/src/field.rs:9283`, `crates/rdocx/src/field.rs:9309`, `crates/rdocx/src/field.rs:9316`

When one entry-owned paragraph expands to multiple entries, the first original paragraph closes and the final entry uses a new canonical `w:p` root. The original end-marker run and suffix remain outside the result replacement, beneath that new root. The single-owner branch changes only the final paragraph closing tag. Namespace closure for retained end content runs only in the distinct two-owner-to-one-entry branch.

A valid single paragraph with a locally declared Word alias or default Word namespace, or a locally declared foreign prefix used inside the end run, therefore loses that original scope on expansion. The moved run can become unbound, foreign or rebound. This causes a refused rebuild or changes opaque ownership instead of preserving the supported entry-boundary case. Close the retained end interval using the original paragraph namespace map whenever expansion relocates it into a generated root. Add one-to-many local-alias, default-binding and foreign-shadow controls with strict repeat/save/reopen URI and opaque-byte assertions. Existing alias controls declare their bindings at document scope and do not expose this loss.

### D2, multi-paragraph zero-entry rebuild retains obsolete entry styles

`crates/rdocx/src/field.rs:9221`, `crates/rdocx/src/field.rs:9159`, `crates/rdocx/tests/regression_test.rs:12643`

An empty generated cache with distinct begin/end paragraphs immediately returns to the generic splice. That splice retains the begin properties outside the replacement and copies the original end opening/properties back into the replacement. Thus a plain three-entry TOC rebuilt with no headings leaves two empty boundary paragraphs still styled `TOC1`.

The approved approach explicitly requires a balanced zero-entry owner without an obsolete TOC entry style. The specialized zero-entry style removal is reachable only for a single paragraph. The contributor zero-entry test instead collects and indexes both obsolete styled paragraphs, so it preserves the defect rather than proving the selected contract. Handle eligible zero-entry entry-boundary roots with source-aware complete style removal while retaining opaque properties, instructions and balanced markers. Test both single and multiple old entries, then a later positive rebuild and save/reopen. Distinct original marker-only paragraphs must retain their existing policy.

## Smells

0.

## Nitpicks

0.

## Aspect results and limits

- Correctness: D1 and D2. No additional findings in the complete proposal.
- Contract: D1 violates namespace closure and ordinary supported topology. D2 violates the selected zero-entry policy. No additional findings.
- Panics: 0 findings. Generated slices and the new scanner were inspected in their actual internal call context. No runtime panic claim is made.
- OOXML: D1. No additional child-order or qualified ownership findings. The source-aware style and tab overlays, complete started-style ranges, protected end roots, duplicate-property refusal and staged validation address the cited PR341 preservation gaps within the reviewed scope.
- Tests: D1 lacks the local-root one-to-many discriminator. D2 is asserted with the obsolete contributor policy. Other new source-built controls remain meaningful but unexecuted. No collection or PASS count is inferred from definitions.
- Structure: 0 findings. No new public API, dependency, product file, module, trait or generic is introduced.

No source, test, proposal, HLD, environment or cache was changed during this pass. Parked bibliography and full navigation remain outside this repair. Later composition must preserve the independently owned F-X200/F-X206 publication regions.
