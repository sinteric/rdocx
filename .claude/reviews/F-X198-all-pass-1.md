# F-X198, all, pass1

**Reviewed**: Independent root review of the frozen working diff against Base1f163e4aff8d9cbdbad4fad53c8d2c67fc4ae811,7 files,502 additions and10 deletions. Approved contract, retained source capture and existing scoped block matcher inspected. Archive maintenance and remaining scoped gates are pending.
**Verdict**:0 defects,1 smell,0 nitpicks.

## Defects

None.

## Smells

### S1, foreign MC control loses its VML namespace scope

`crates/rdocx/tests/regression_test.rs:397853`

The foreign_alternate fixture moves a VML Fallback body outside the original
MC wrapper, which holds xmlns:v and xmlns:o. Its new shadow wrapper declares
only shadow. The retained authored source confirms v:shape, v:shapetype and
o:spt depend on that original wrapper. The copied inner VML is therefore
unbound, so this negative control does not isolate the foreign MC namespace
from invalid VML namespace context. Retain v and o declarations on the foreign
wrapper and assert the source has no unbound element or attribute prefixes.
Keep the intended otherwise qualified text-box representation inside the
foreign MC wrapper. Rerun the single-representation control after remediation.

## Nitpicks

None.

## Not found

Correctness: qualified structural routes at `crates/rdocx/src/document.rs:8332`
recognize both MC branches without treating opaque ancestors as owners. Exact
containment and distinct branch identity at `document.rs:8392` prevent a later
unrelated paired box from redirecting the selected owner. The detector does
not compare text. It includes descendant table-cell scopes by containment.

Contract: `document.rs:27104` applies refusal after expected-count mismatch and
zero-match handling, and before splicing or publication. Existing full-block
source and namespace validation precedes this point. Document-wide replacement
is unchanged. HLD10,12 and14 describe the approved atomic refusal.

Panics: the production diff adds no unwrap, expect, unchecked slice, index or
arithmetic. Stack and copied spans use existing checked reader conventions.

OOXML: the guard reads actual retained source and emits no XML. Qualified Word,
DrawingML, VML and MC ownership rejects foreign and opaque ancestors. The
positive refusal test at `regression_test.rs:397787` compares complete bytes
with retained comment threads. Schema ordering and source namespaces are not
rewritten by refusal.

Tests: the Base reproducer actually failed with Choice-only Ok1. Clean native
execution passed39 of40 text-box controls, including paired, namespace,
physical-owner, descendant-cell, no-op, mismatch, stale and global-replacement
cases. The remaining standalone fixture has been revised but not rerun yet.
The new installed Python control at `crates/rdocx-py/tests/test_core.py:4132`
checks error mapping, complete bytes and reusable held handles, but its fresh
extension execution remains pending. No broader gate pass is claimed.

Structure: one private detector reuses existing XmlElementFrame and routing
functions. No new production file, module, trait, generic, dependency or API.
