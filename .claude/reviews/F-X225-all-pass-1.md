# F-X225, all aspects, pass 1

**Reviewed**: Working diff against claimdc43151b8046e3fa678bde75959b6e671dcfee11, eight files,184 insertions and46 deletions. The approved design includes its measured private master-caller and master-style extensions.

**Verdict**: 0 defects,0 smells,0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: optional body list defaults are privately boxed and remain absent without allocation. The borrowed getter uses the same reference type. Private boxed parse results preserve existing error propagation and parser logic. Master-style staging moves the same three native values into unchanged public fields.
- Contract: the master resolver's sequential loop retains original lookup, type, external-target and missing-part checks, source order, retained parse errors and record fields. Public APIs, model fields, input bounds, feature defaults and versions remain unchanged.
- Panics: production changes add no unchecked indexing or unwrap. Existing missing/duplicate style errors and namespace validation remain in the same parse path.
- OOXML: parsing still captures the same source roots, namespace scopes, attributes and ordered opaque children. Serialization and model mutation logic are unchanged. The native source/public-caller control covers borrowed access, deep clone, mutation, all levels, ordered raw content and save/reopen.
- Tests: the original complete facade Node case is unchanged. The added case parses a real authored nontrivial package with populated defaults and retained opaque source, consumes the real author afterward and checks the complete package round-trip. Both tests actually pass under the original default stack, without a linker or harness allowance.
- Structure: concrete private heap ownership reduces measured live storage. The new private parsing entry performs the original parse work. The resolver loop removes adapter result slots without introducing a public helper or abstraction. Changes remain in approved existing files, with no new dependency, module, trait or generic.
- Documentation: HLD06 describes private list ownership and source preservation. HLD10 requires the real default-stack facade controls. Neither claims a new public model or a larger runtime budget.

## Evidence and completion boundary

`.claude/scratch/F-X225-checks/node-private-master-style-11.log` records the actual original and populated Node controls, two passed, zero failed and zero ignored. Earlier default-stack failures remain retained. The successful-module static frame record is `/private/tmp/F-X225-green-frame-proof/frame-facts.json`. It records the body frame at29696 bytes, root capture5952 and private master-style parsing91824, with initial stack1048576. The measurement is diagnostic and does not replace executed tests or establish a portable ABI guarantee.

Remaining affected scoped checks, real archive observations and policy readiness must pass before worker completion. Missing ignored corpus and offline registry setup failures receive no pass credit. Any later numeric README/catalog changes require their own bounded review. Combined F-X226 integration, full verification, hosted CI and release gates remain separate obligations.
