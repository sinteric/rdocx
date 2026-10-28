# F-X268, opened staging identifiers, pass 1

**Reviewed**: Root compared the unchanged-main shortcut with the existing whole canonicalization and inspected the new focused native probe. Mechanism 4's reviewed Python diff remains separate.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D2, unchanged-main reuse skips pending relationship cleanup

`crates/rdocx/src/document.rs:18431`

The shortcut does not perform the pending relationship cleanup at `crates/rdocx/src/document.rs:18549`, and its admission at `crates/rdocx/src/document.rs:18316` does not exclude nonempty pending holds. An opened, warmed one-paragraph document with an absent main-owner `rId999` registered through the existing internal helper still admits reuse. Both algorithms return the same result, model and package, but the shortcut retains that hold while the whole algorithm clears it. `/tmp/fx268-m5-pending-probe.log` records one actual failing test, exit 101, 0.09 seconds. The initial zero-test invocation was not evidence. This is an internal staging-state counterexample, not a claimed public API reproducer.

The shared shortcut must preserve the whole algorithm's identifier state. Conservatively use the old path for a nonempty main-owner pending set, or prove identical cleanup before reuse. Cover expired, still-live unreferenced, and now-referenced holds without discarding a valid hold.

## Smells

None.

## Nitpicks

None.

## Remaining scope

The prepared-publication source, focused table tests and HLD changes are independently ready for their bounded local checkpoint. The checkpoint commit was blocked by the Grok session and has not occurred. This finding concerns the native cache and must remain separate from that checkpoint. Full feature review, mechanisms 5 through 8, release fallback proof, memory and final integrated gates remain pending.
