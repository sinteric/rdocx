# F-X210, installed fixture handle lifetime, pass 17

**Reviewed**: Actual installed PPT companion control under current source0520ef, following the actual passing Word companion.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, fixture reuses structurally invalidated handles

`crates/rpptx-py/tests/test_documented_examples.py:6857`

The test creates a shape then replaces shape.text and immediately reads the same shape.text_frame. The current source at crates/rpptx-py/src/shape.rs:671 bumps the document revision on this replacement. The actual installed test raises StaleElementError before any diagnostic assertion. The loop also retains its original slide handle for the next structural insertion. Re-fetch the slide from the presentation before each insertion, and re-fetch the inserted shape after text replacement. This uses the current checked API without depending on F-X206's separate live-handle repair.

The actual installed Word companion passes. PPT pytest exits1 with one failed fixture at the cited line. Raw log SHA0c315d257ca0e9cda4c1a6b7efab2e53caf79c362ada10ded73c9cadc6a50684 is retained in actual-root-installed-acceptance-3. Four typing gates remain unrun, and neither companion failure is relabelled a pass.

## Not found

No new F-X210 product defect is established. Handle lifetime improvement remains owned by F-X206. No additional correctness, OOXML, panic or structural finding, no smells or nitpicks.

## Handback

Only this review was written during review. Return to distinct fixture setup correction, retaining every original diagnostic, payload, repeat and source-preservation assertion. Then independently review the setup and continue installed checks with unchanged production wheels.
