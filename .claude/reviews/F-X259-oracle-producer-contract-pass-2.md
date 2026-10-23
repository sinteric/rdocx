# F-X259, oracle-producer-contract, pass 2

**Reviewed**: independent `/microscope F-X259 --working --aspect oracle-producer-contract`, branch `sprint/s95.3`, HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. Two-file diff envelope: 17 insertions and 1 deletion. Narrow scope: the six added prose lines at `.claude/skills/differential-testing.md:174` through line179 and the generated adapter hash replacement. The separate nine-line cache paragraph is excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Scope and prior conclusions

Read AGENTS.md, CLAUDE.md, canonical WORKFLOW and microscope, F-X259's approved plan and progress. CURRENT_SPRINT and BACKLOG retain in-progress status. Consumed the clean conclusions of `.claude/reviews/F-X259-toc-and-cache-contract-pass-1.md` and `.claude/reviews/F-X260-issue398-pass-5.md` without repeating their product audits. Other working product changes are excluded.

This is a feature microscope. It does not reset the exhausted three sprint-review passes, supply final qualification or close acceptance. No builds, tests, oracle preparation, source edits, adapter regeneration, state, progress or plan changes were performed. Only this report was written.

## Defects

Zero.

## Smells

Zero.

## Nitpicks

Zero.

## Producer and command contract

The producer claim at `.claude/skills/differential-testing.md:174` is supported by HLD15's binding-job description at `docs/hld/15-build-and-toolchain.md:996` and the actual CI invocation at `.github/workflows/ci.yml:350`. HLD15 describes the carrier and independence contract. CI supplies the concrete invocation, isolated interpreter and selected package family.

The real binding-mode command shape is:

```bash
"$binding_python" -I -B crates/rdocx-py/tests/test_rendering_threads.py \
  --prepare-fx195-inputs "$fresh_output_directory" \
  --family rdocx
```

`rpptx` is the other accepted family. The preparation option takes a required directory argument, rather than acting as a Boolean flag, at `crates/rdocx-py/tests/test_rendering_threads.py:2395`. Binding mode requires a family at line2402. The destination must not already exist at line2062. The paragraph identifies the producer and options rather than presenting a complete copyable shell command. Its reference to the documented producer leads to CI's complete command, including the directory operand. No contradictory invocation contract was found.

Native CI uses the same carrier with `--native-tools-only` at `.github/workflows/ci.yml:211` and line767. That mode excludes `--family` at `crates/rdocx-py/tests/test_rendering_threads.py:2400` and returns after preparing HarfBuzz tools at line2137. It does not produce the full binding input bundle. The paragraph's selected-family instruction agrees with binding preparation and does not claim the native-only mode produces that bundle.

## Independence and provenance

The carrier imports pytest but does not import either product to prepare inputs. CI installs the preparation dependencies before invocation at `.github/workflows/ci.yml:328`. HarfBuzz14.4.0 source is authenticated by SHA-256 at `crates/rdocx-py/tests/test_rendering_threads.py:2087`. Tool versions and executable hashes are retained at line2130. Binding preparation checks bundled font and license hashes at line2153, obtains original/static shaping and SVG observations from HarfBuzz at line2273, and writes the variable receipt at line2309.

Independence does not mean preparation avoids Cargo. It builds the selected CLI and obtains actual renderer dependency artifacts at `crates/rdocx-py/tests/test_rendering_threads.py:2311`, then compiles the standalone path reference with guarded inputs at line2346. HLD15 explicitly describes that contract at `docs/hld/15-build-and-toolchain.md:1003`. Product-rendered expected paint is not substituted by this preparation route.

Child failures raise after command receipts are written at `crates/rdocx-py/tests/test_rendering_threads.py:2066`. Successful preparation emits environment paths, qualification hashes and a root receipt at line2375. CI receives the environment through GITHUB_ENV. Local use must consume the emitted environment.json or printed mapping before dependent tests. The new paragraph's requirement to retain failure and provenance agrees with these mechanisms. It provides no Word-produced evidence or waiver for unrelated missing oracle inputs.

## Adapter and explicit zero categories

The adapter points to the canonical source at `.agents/skills/differential-testing/SKILL.md:8`. Its line10 SHA-256, `870eb5eb77436641d54182e029561d67f62bce4691dd219878ecbbeb0923aadb`, matches the current canonical file, independently computed read-only. No adapter drift was found.

Producer existence, command semantics, independent expected-output provenance, failure handling, adapter consistency and preservation of pending qualification each produced zero findings. Runtime availability and successful preparation were not asserted or executed. The review ends here.
