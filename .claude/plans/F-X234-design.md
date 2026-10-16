# F-X234, Preserve supported LaTeX conversion structure

**Status**: completed
**Sprint**: S93
**Size**: M
**Depends on**: none

## Problem

The exact [Issue365](https://github.com/tensorbee/rdocx/issues/365) formula loses
commands in `crates/rdocx/src/math.rs:1963`,2044. Explicit spacing is silently
discarded at2023. Existing MathML import drops unsupported mspace and token
mathvariant semantics (`crates/rdocx/src/math.rs:489`,527).

```latex
\int_{a}^{b} e^{i \pi K} \, dx = \lim_{x \to \infty} \frac{S}{\sqrt{T^2 + C^2}}
```

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X234, Preserve supported LaTeX conversion structure".
- `docs/hld/03-architecture.md`, MathML and LaTeX conversion ownership in the Word facade.
- `docs/hld/04-opc-and-packaging.md`, Presentation MathML boundary.
- `docs/hld/12-testing-strategy.md`, "rdocx MathML and LaTeX conversion" and OfficeMath round-trip coverage in "Test taxonomy".
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering", Pandoc producer identity.
- `docs/hld/06-presentationml-model.md`, "Parts", qualified XML preservation.

## Approach

Extend existing math.rs conversion code without a public type, new product file,
dependency or unbounded TeX vocabulary. Map pi, to and infty to their Unicode
symbols. Preserve upright lim using existing MathRunProperties.normal. Preserve
the explicit thin space as a supported meaningful U+2009 run, with normalized
LaTeX export and supported MathML mspace import/export. Bound accepted mspace
widths and mathvariant values, diagnosing unsupported representations explicitly.

Normalize the exact current Pandoc output deliberately: inline lim with a
subscript uses the existing Subscript variant. Its invisible U+2061 function
application marker is semantic glue with no visible glyph. Normalize that marker
explicitly rather than silently deleting arbitrary operators. Keep the current
integral, fraction, radical and script model and supported n-ary scope rules.
Do not extend display-limit policy unless an independently demonstrated mismatch
requires an approved revision. Meaningful run properties must survive both export
and import. Keep every input, node, text, diagnostic and parser-depth cap. Ordinary TeX
characters are single atoms before run normalization, so adjacent unscripted
characters cannot absorb the base of a following script.

Preserve the historical Pandoc3.10 oracle and results. Add a separately identified
exact-formula live case pinned to actual Pandoc3.11. Record binary identity,
commands and complete normalized trees. No old oracle evidence is relabelled.
The related `4\pi^{2} a^{3}` fraction numerator is a script-attachment control,
not admission of every command in the later contributor's extended examples.

## Rejected alternatives

- Dropping unsupported symbols and calling the rendered result equivalent.
- General TeX support or a new parser dependency for this bounded repair.
- Erasing upright properties or spacing from normalized comparisons to pass.

## Test plan

The **test gate** is **regression**. All new tests, including public facade
save/reopen controls, live in the existing math.rs test module. This story
leases only crates/rdocx/src/math.rs among source/test files.

| Category | Test | Asserts |
|---|---|---|
| regression | `fx234_issue365_preserves_symbols_limits_and_thin_space` | Exact formula has ordered symbols, integral bounds, upright lim subscript, nested fraction/radical and thin space with no supported-loss diagnostic |
| regression | `fx234_pi_scripts_attach_to_their_original_base` | The supplied related mass formula retains pi squared and a cubed independently |
| round-trip | `fx234_supported_formula_normalizes_across_latex_mathml_and_docx` | Normalized typed structure and supported properties survive both conversions and save/reopen, with opaque XML siblings unchanged |
| unit | bounded unsupported and malformed controls | Stable ordered loss diagnostics, meaningful spacing and existing resource caps are preserved |
| differential | exact Pandoc3.11 formula case | Actual pinned independent output matches the declared normalized tree, with explicit invisible-marker and display-wrapper normalization |

Run cargo check -p rdocx --all-targets, focused tests, the exact applicable live
oracle case, scoped verify and independent microscope. F-X235 remains blocked
until this prerequisite is completed through the scoped prefix checkpoint.

## HLD impact

- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/15-build-and-toolchain.md`

## Risk routing

- Parser/serializer: read OPC and model contracts, retain prefix tolerance,
  schema order and byte-identical unmodelled subtree round trips.
- External oracle: exact Pandoc3.11 identity and separately retained3.10 history.
- Layout-facing output: bundled deterministic fonts for any rendered comparison.
  Conversion changes do not authorize equation-layout tuning here.
- Existing package-footprint policy requires the measured `rdocx` row in
  `README.md` and numeric `ARCHIVE_MEASUREMENTS` in
  `scripts/readme_doctests.py` to match this source inventory. Update only the
  affected measured row, preserving the existing 64-byte compression tolerance
  and 10 MiB ceiling. This is acceptance evidence in existing carriers, not a
  changed archive policy.

## Hash harness

Existing49 entries are expected unchanged unless an existing formula actually
contains these newly supported commands. Any affected entry requires exact
formula attribution and an independently reviewed expected delta before recording.
New source-built regression fixtures are separate from the historical baseline.

## Implementation checklist

- [x] Approve sprint designs, claim and establish actual Base failure.
- [x] Implement bounded symbols, upright function and meaningful spacing.
- [x] Qualify normalized round trips, losses, caps and live pinned oracle.
- [x] Pass scoped verification and independent zero-finding microscope.
- [x] Prepare the validated feature-local handoff for the integrator dependency
  checkpoint before F-X235. Integration remains the orchestrator operation.

## Open questions

No user product decision is needed. Inline lim follows pinned Pandoc Subscript
normalization. Actual new oracle identities are distinct from historical pins.
