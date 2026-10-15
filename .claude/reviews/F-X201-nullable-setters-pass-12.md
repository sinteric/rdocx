# F-X201, nullable setter correction, pass 12

**Reviewed**: Exactly ten setter annotation lines in the immutable nullable-setter-correction capsule. No production change.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority and scope

Receipt `/private/tmp/S91-F-X201-focused-review/nullable-setter-correction/receipt.json` SHA256 `f0d51eca0ad9539aa2166707b57db8aee615f1bec869a361429637b3cffb6daf` and all three references authenticated. Exact correction SHA256 `44c1ef57e4628326b95569b5bdbb8c19cfba183180a6c465f451c52bad26fdfd`. Before stub `87a98d82ccf9c6170c18448ad103da0253aff834ab0c2d517b58b26a11d7d731`, after `79b4297ac3a3f97e3b847a78d7ac004e8084979394d0ecccd33fcd55c41c322d`.

## Resolution

Pass 11 D1 is resolved. Nine numeric setter parameters at `after.pyi:407` through `after.pyi:439` now accept `int`. Orientation at `after.pyi:443` accepts `str`. The optional property getters and every other stub byte remain unchanged. This matches immutable production `crates/rdocx-py/src/document.rs:840`, checked integer helper `:3487`, and orientation setter `:998`. No absent-direct-source getter contract or clear operation is changed.

The existing current `crates/rdocx-py/tests/test_core.py:4339` control adds None rejection and unchanged document bytes for numeric setters and orientation at `:4360` and `:4363`. Its current source was read only as supporting test intent. This review does not claim an installed test, strict typing, or stubtest pass.

## Not found

Correctness, contract, panics, OOXML, tests, and structure: zero additional findings in this ten-line annotation amendment. No runtime, source edits, or broad confirmation review performed.
