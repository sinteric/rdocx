# F-X202, installed contract correction, pass 10

**Reviewed**: Immutable r10 capsule77080ce5527f110a4b2f32505bf735f4fb539f9872b5c6a1539a8353e6db630e, three stub/test carriers. Bounded correction of the actual installed43-case failures and stub diagnostics.

**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Authentication

All24 manifest records authenticate. The three complete preimages match the admitted installed source map dacde5ad7eb8aa419652aee8d024e34fd503a7b45bb65874d9a96563feaada85. Both copied native binding/support carriers match that map and the authenticated build map byte for byte. The wheel proof and original actual failed-run logs are retained. No native product source changes are part of this correction.

## Not found

Bindings and contract: `crates/rdocx-py/python/rdocx/_rdocx.pyi:58` declares the actual immutable PyO3 constructor in __new__, preserving all14 keyword operands and defaults and returning PictureOptions. Frozen runtime `crates/rdocx-py/src/document.rs:29` defines that constructor. All28 PictureOptions stubtest errors identify precisely the removed __init__ parameters and missing __new__ parameters. `crates/rdocx-py/python/rdocx/_rdocx.pyi:97` restores the existing RunPosition final marker, addressing its two non-subclassable/disjoint runtime diagnostics. No constructor operand, public property or native branch is weakened.

Tests and source authority: `crates/rdocx-py/tests/test_core.py:5327` now expects the actual unchanged owner-check StaleElementError, retaining the different-document message, package atomicity, subsequent successful resize, stale old snapshot and reopened assertions. The existing ContentPath owner check at `crates/oxml-py-support/src/lib.rs:529` explains this exact class/message. The XML ElementTree import at `crates/rdocx-py/tests/test_core.py:5377` and local rdocx import at `crates/rdocx-py/tests/test_formatting_tables.py:2075` repair NameError setup without changing any deciding assertions or product logic.

Correctness, panics, OOXML and structure: there are no new native operations, parser rules, serialization changes, module files, traits, generics or dependencies. The amended fixture imports use standard-library/existing binding modules. The constructor stub agrees with the approved explicit PictureOptions contract and HLD10 Python API shape.

## Actual evidence and limits

The original installed run remains failed: exact43 collected unique cases from16 functions,34PASS/9FAIL, strict mypy0 and30 stubtest errors. Complete before/current/after PPT distribution equality, final Word RECORD equality, exact source-after guards and safe natural process cleanup authenticate. Four foreign-owner class expectations, four missing ElementTree imports and one missing rdocx import explain the nine failures. The correction does not reinterpret that run as passing.

The candidate has no executed successor. A packaging-only fresh wheel with the same native SO, exact nine failed selectors, strict typing/stubtest and complete paired provenance remain due. No feature completion or sprint closure credit is granted. The review ends here.
