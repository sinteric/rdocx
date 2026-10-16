# F-X233, cached table border continuity, pass 1

**Reviewed**: read-only supplemental audit of integrated source12fa27101f153614ac60bff324b0ef2cdd586e29, focusing on the existing table-cache reconstruction and grouped-border paint boundary. Approved F-X233 contract and source pass2 read first. Canonical source remained clean throughout the external reproduction.
**Verdict**: 1 defect,0 smells,0 nitpicks

## Defects

### D1, A table-cache hit discards grouped border continuity metadata

`crates/rdocx-layout/src/engine.rs:6078`
`crates/rdocx-layout/src/table.rs:1798`
`crates/rdocx-layout/src/paginator.rs:5583`

Cold table layout marks the second same-border cell paragraph with
border_group_before and removes the group's facing edges. Cached table
reconstruction hardcodes that retained semantic flag to false. The paginator
then omits the already-priced interior spacing from the vertical border bridge.
Text and frame height remain stable, but both sides of the frame become
visibly discontinuous after a warm cache hit.

The external public Engine consumer constructs two20point cell paragraphs
with identical3point single borders,8point border space and after8/before12
paragraph spacing. It selects the equivalent typed-authoring PPr model rather
than retaining raw pBdr source siblings, so the table is legitimately cacheable.
The initial pass equals a fresh engine. After changing an unrelated preceding
paragraph, the next two retained-engine passes differ from their fresh-engine
counterparts at exactly two vertical frame starts. At x61and551points, cold
paint starts at y125.49076334635416 while warm paint starts at
y137.49076334635416. Both still end at y168.49076334635416 with3point width.
The12point missing painted bridge is the collapsed paragraph gap. This is an
actual draw-operation difference, not a check of flag spelling.

Preserve retained grouping semantics when rebinding cached table sources.
Add a real warm/cold regression with a cacheable typed-authoring cell containing
multiple joined bordered paragraphs and nonzero spacing. It must establish
actual table-cache reuse and compare painted geometry, including recursive
nested table semantics where applicable.

## Evidence

Reproducer and exact artifacts are retained at
`/private/tmp/S93-table-cache-probe/src/main.rs`, its Cargo.toml/Cargo.lock,
`run.log`, `input.txt`, `cold-1.txt`, `warm-1.txt`, `cold-2.txt` and `warm-2.txt`.
The command used the exclusively released existing234target directory:

```bash
CARGO_TARGET_DIR=/private/tmp/rdocx-s93-fx234-target cargo run \
  --manifest-path /private/tmp/S93-table-cache-probe/Cargo.toml --offline
```

The recorded outcomes are pass0equal true, pass1equal false and pass2equal
false. A preliminary XML-loaded input remained equal because its retained raw
PPr revision_xml makes it non-cacheable under the existing admission rule.
That preliminary result is diagnostic and is not evidence against this defect.
No canonical test, runtime source, baseline or documentation was changed.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Border units and paragraph flow: no additional issue in this bounded pass. The defect changes bridge paint, not sz/8conversion or text positions.
- Stored OOXML and atomicity: no source mutation observed. The independent consumer uses transient public layout inputs.
- Layering, dependency and public surface: no additional issue in this cached metadata boundary.
- Integrated sprint acceptance: not decided. This is a supplemental feature microscope, not the formal sprint review.

This audit ends here. Return to the orchestrator for separate remediation and
an independent successor pass. Do not remediate within this review pass.
