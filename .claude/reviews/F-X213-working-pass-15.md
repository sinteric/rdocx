# F-X213, installed reopen API, pass 15

**Reviewed**: One call in the existing installed single-entry TOC control, against independently reviewed source6211.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Authenticated all2,852 byte, size and mode records against .claude/scratch/F-X213-native21/source-installed-reopen-qualification.json, SHA256 37cdd32fd33d8ceb3d433c15301ee6532dc75a7d3cad68681cac1e67732545a1. Exactly test_core.py differs from prior6211. Its current SHA256 is 28c3ddf51e24797bac9bbb90f1e8dd70459e0e9d57ea452b8723f38a2e1d89c3. Replacing this one call with its preceding form recovers prior carrier SHA256 889d387b7eede6e68a258c5c209e2e22fa9b399b3f1cdab69304a85370c2949e. All2,851 other records, including production, stubs and wheel inputs, remain exact. HEAD and index retain the identities authenticated in pass14.

Read approved plan, prior review, progress, complete affected control, actual failure and current public method signatures.

## Six aspects

- Correctness: 0 findings. Document.open accepts a path at crates/rdocx-py/src/document.rs:2076. Document.from_bytes accepts an in-memory byte slice at :2083. The corresponding public stubs agree at crates/rdocx-py/python/rdocx/_rdocx.pyi:597 and :599. The corrected call at crates/rdocx-py/tests/test_core.py:4236 uses the existing appropriate API on the actual serialized bytes.
- Contract: 0 findings. This remains an actual package reopen. Both repeat iterations, one-entry and two-paragraph counts, generated text, exact opaque payload, begin/end counts and post-reopen main-source equality remain unchanged at crates/rdocx-py/tests/test_core.py:4227.
- Panics and errors: 0 findings. No exception is caught or waived. The correction removes the observed test-only argument-type failure without changing product behavior.
- OOXML: 0 findings. Original XML and all exact saved-byte assertions remain unchanged. Reopen still consumes Document.to_bytes output and must reproduce the saved main member at crates/rdocx-py/tests/test_core.py:4237.
- Tests: 0 findings. The retained actual installed log reached and passed the opaque and marker assertions before rejecting BytesIO as the path argument. The correction permits the original reopen assertion and second iteration to execute. No current execution PASS is inferred.
- Structure: 0 findings. One existing method invocation changes. No wrapper, dependency, module, file or public API is introduced.

## Limits and handback

The original failure remains in .claude/scratch/F-X213-native21/installed-typing-current-2/execute-two-Word-controls.log. Current installed controls and typing require actual execution. No F213 runtime, source, compiler, environment, cache or UI action ran. Only this review was written. Existing F210 session50877 was polled without restart. Review ends and returns the canonical lease.
