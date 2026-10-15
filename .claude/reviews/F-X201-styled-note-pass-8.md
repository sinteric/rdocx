# F-X201, styled note references, pass 8

**Reviewed**: single existing engine carrier in immutable styled-note correction r1, receipt08624bdcedf5a574d26dbe7cdfda1d4286dac43cd24805bbc159c6d9af81f392 and diff06d73b12868d3ebf2b32a1a29bdfdd554328f00bfa4863d9c17d125f73fac3bb. Includes the upstream effective formatting calculation and the exact deciding control.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. Explicit superscript or subscript already has a resolved reduced size and baseline shift before note handling. The correction uses those facts once. An ordinary note marker receives its default superscript exactly once, retaining the resolved explicit half-point position. Footnote and endnote stream identity, field/source metadata, glyph shaping, metrics and cache handling are unchanged.

## Smells

None. The fix is a concrete local branch in the existing renderer, without another type, helper, dependency or source file. The deciding control checks actual positioned marker size and baseline against plain body text, both streams, plain and styled input, repeated layout, and positive and negative direct position. The retained original input reaches a genuine styled Footnote failure at6.728pt versus11.6pt after valid plain setup. The corrected control passes once, zero ignores.

## Nitpicks

None.

## Not found

Correctness, approved PR328 contract, panic handling, schema preservation, test authority and structure produced no findings in this correction. Every capsule preimage/candidate and retained RED/GREEN hash was independently authenticated. Actual GREEN has one named command row and one passing control, irrespective of the generic driver's status label. Both RED and GREEN groups terminated naturally, were reaped and are absent, with source guards matching.

This reviews only the intentional note-reference behavior correction. The image ownership defect and other page/story implementation remain separate. The labelled behavior commit, actual deterministic harness measurement and attribution, final installed wheel and full feature/sprint gates remain required. No baseline movement or full-feature completion is claimed.
