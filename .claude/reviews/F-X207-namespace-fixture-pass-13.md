# F-X207, inherited namespace fixture, pass13

**Reviewed**: Test-only correction SHA-256 `49f55ea37c12e3ad4b0c653770e2138aa045fc9ab1f2c2b9a665a4927b9f2e1e` on composed source `6d9ed49a`. No production change.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Findings

None. The prior fixture expected an exact attribute-free text-body opening tag. The completed provider now emits qualified root declarations, so that replacement never installed the fixture namespace before aliasing its children. The failed namespace assertion occurs during initial setup, before import or refresh. It is retained as an invalid fixture setup, without a product Base claim.

The corrected setup preserves existing root attributes and installs the inherited scope at the actual opening token. Moving only fixture child declarations before installing wrapper scope prevents its later removal. All numeric operands, lexical opaque payloads, selected and unrelated owner assertions, no-op checks and namespace refusal assertions remain unchanged. The only other change adds context to the original failing assertion. No assertion predicate is weakened.

## Scope and handback

Actual composed namespace preservation, foreign refusal and nested scaled, rotated and flipped anchor controls each pass exactly once. The affected Clippy, format and both WASM graphs pass in continuation receipt27f5fad7. Earlier passed check and inherited-scope unit retain their exact production-source qualification. Fresh installed and archive gates remain due, with no full paint or native Office replay inferred.

Correctness, contract, preservation, fixture validity and structure produce no further findings. No source, runtime, environment or index was modified during review. Review ends here.
