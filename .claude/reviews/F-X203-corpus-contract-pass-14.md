# F-X203, corpus contract proposal, pass 14

**Reviewer**: fonts_resume, independent of the proposal.
**Reviewed**: the sealed test-only corpus contract clarification, one design-plan row and four supporting external artifacts. No product implementation change.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Admissible for bounded execution under authenticated current-source, native-artifact and installed-wheel guards.

## Input authority

Receipt `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/corpus-contract-proposal-14/receipt.json` authenticates as `3a72b316c14f937e83e6bdc3512d652b409e6b9b1af814e1eefa39f5f05427ab`. All five declared artifact hashes match. The recorded frozen source is `4de2159fdf9c4e5ea31eb6d5bae48498ccdd5cbd037e9ff466b09371685e7d90`, HEAD `dd7c627b534301c253615d30e35e8748b455a4d0`.

The retained original corpus proof authenticates as `8c980391cf7c5c197e86d8a3ae39784f076d998b6f171d2efcc76c583a42f07a`. Its fourteen failures remain failures. This review does not replace them with success credit.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspects checked

- **Correctness, zero findings.** The consumer calls the actual Rust `Document::to_html` and `to_markdown` directly, without CLI appendix production. The verifier compares complete Python UTF-8 bytes with complete native bare bytes. It separately compares complete CLI bytes against the bare export plus source-authored appendix. The body-only equation control retains direct full CLI equality. See `native-consumer.rs:5` and `verify-corpus.py:33` in the sealed proposal directory.
- **Contract, zero findings.** The existing approved export contract explicitly retains the native bare subset at `plan-before.md:66`. Only the contradictory corpus test row changes at `plan-after.md:240`. The Python bindings still forward to that native provider at `crates/rdocx-py/src/document.rs:4330`. Existing CLI conversion adds readable related stories at `crates/rdocx-cli/src/commands.rs:739` and `commands.rs:746`. The clarification preserves both established surfaces.
- **Panics and error handling, zero findings for the finite proposal.** Expected fixture hashes and exact output assertions fail closed. Native and CLI subprocess failures abort rather than being converted into passing rows. The small consumer's unwraps are explicit bounded gate failures, not new product behavior.
- **OOXML and source preservation, zero findings.** Independently read all seven original ZIP archives and their qualified WordprocessingML section references and package relationships. Their source hashes, every declared appendix member hash and header/footer first-reference order match the sealed matrix. Each literal nonempty paragraph text matches original qualified `w:t` content. Empty graphic headers contain no real paragraph text. These appendix parts contain no direct tables or hidden extra text owners, and this seven-input corpus has no footnote/endnote parts. The verifier checks original input bytes and repeated saved document bytes after read-only exports. It does not rewrite source.
- **Tests and oracle independence, zero findings.** Expected appendix content comes from independently authenticated source literals, not exported CLI/API text. The finite wrapper spelling follows the public CLI output contract, while all paragraph values and part labels remain source-pinned. The verifier neither slices off an observed suffix nor normalizes actual exported bytes. Both HTML and Markdown retain full equality. Main-content semantics continue to rely on the existing HTML14 and Issue332/334/339 deciding controls, not corpus parity alone. See `source-authored-appendices.json:2`, `verify-corpus.py:7`, `verify-corpus.py:34` and `verify-corpus.py:37`.
- **Structure, zero findings.** The proposal adds no product formatter, model, module, dependency or API. Its temporary direct consumer checks the one existing native provider.

## Execution limits

The native consumer and revised corpus verifier are UNRUN. The receipt's native HTML14 reference is null and is not credited by this pass. Binary hashes written into a prospective result alone are not pre-execution artifact authentication. Admission requires the existing owned runtime to bind the linked native consumer, current CLI graph and installed Python wheel before execution and to retain source, input and provenance guards afterward. The proposed assertions themselves are admissible without awarding successor runtime, native application rendering, or whole-feature acceptance.
