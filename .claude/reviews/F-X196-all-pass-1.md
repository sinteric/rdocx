# F-X196, all, pass 1

**Reviewed**: Frozen working implementation against Base `1eafee76`, branch
`work/f-x196-codex`. Five tracked files, 410 inserted and 13 deleted lines,
including the approved revised design, CLI scanner, existing integration
entrypoint and HLD10/14. Archive measurement carriers are still pending and
require a supplemental review when added. No Cargo command ran in this pass.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, the scanner admits control content the facade preserves as opaque

`crates/rdocx-cli/src/commands.rs:2441`

The scanner admits every qualified `sdtContent` whose admitted parent is
`sdt`. The native parser admits only the first nonempty content container and
preserves subsequent containers as raw XML at
`crates/rdocx-oxml/src/content_control.rs:1053` and
`crates/rdocx-oxml/src/content_control.rs:1070`. Its paragraph collector at
`crates/rdocx-oxml/src/content_control.rs:1197` excludes that raw subtree.

For example, a body control containing
`<w:sdtContent><w:p/></w:sdtContent>` twice opens with one typed paragraph and
one preserved content subtree. The scanner emits two paragraph flags.
`crates/rdocx-cli/src/commands.rs:2329` then returns an inventory mismatch
instead of the existing validation report and metadata advisories. A control
whose properties fail typed parsing also remains entirely raw through
`crates/rdocx-oxml/src/content_control.rs:1011` and
`crates/rdocx-oxml/src/document.rs:2760`, but the scanner still enters its
paragraphs. The plan requires alignment with the existing facade inventory
and exclusion of opaque content, rather than merely matching qualified names.

The same unconditional control admission occurs inside a paragraph at
`crates/rdocx-cli/src/commands.rs:2423`, so visible drawings in a preserved
duplicate inline content container can also suppress a legitimate empty
warning without an inventory mismatch. Remediation should distinguish typed
control content from preserved root slots and fully opaque controls, with
source-built public CLI controls for both block and inline cases. Preserve
the source bytes and original metadata diagnostics.

## Smells

None. Zero smells.

## Nitpicks

None. Zero nitpicks.

## Not found

- Correctness outside D1: direct body tables, control-owned tables, nested
  controls, namespace aliases, foreign wrappers, properties, deleted content
  and producer hints follow the intended admitted-path rules.
- Contract outside D1: the source-built reported picture case, metadata
  retention, undefined-style discriminator and read-only byte checks cover
  the approved behavior. No new public library API or dependency was added.
- Panics: no new unchecked slice, index, unwrap or untrusted arithmetic path
  was found in production changes. The scanner rejects unfinished XML.
- OOXML outside D1: expanded namespace names and opaque ancestor exclusion
  avoid foreign-name false positives. The scanner does not serialize XML.
- Tests outside D1: existing entrypoints exercise 29 visible and empty
  discriminators, actual authored pictures and recursive inventory scope.
  The recorded Base failure is independent of the implementation.
- Structure: the private scanner is local to the CLI behavior. No new trait,
  generic parameter, forwarding wrapper, module or test binary was added.

This review ends here. Scoped verification, archive measurement maintenance
and remediation remain implementing-phase work.
