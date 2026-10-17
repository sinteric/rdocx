---
description: The gate. Runs formatting, lints, tests, the hash harness and the prose rules.
---

# /verify [--fast | --scoped F-XXX | --full]

There are three verification scopes:

- `--fast` runs formatting, changed-crate linting and the unique affected local
  regression tests. It does not run whole-crate or complete installed suites.
- `--scoped F-XXX` is the feature-completion gate for local evidence. It binds
  the fast checks and the approved plan's named local RED/GREEN regressions to
  the claimed Base and actual source. Independent `/microscope` remains required.
- `--full` is the final integrated sprint gate. It runs the workspace suites
  and the union of every approved plan's final oracle and risk checks once.

Whole-crate suites, ordinary complete installed suites, external oracle
comparisons, native Office controls, hash/pixel corpora, whole-package typing
and stub checks, WASM, documentation, publication dry runs and supply-chain
checks belong to the final union. A small affected typing or compile check can
be a local regression when the change needs it. Do not promote it to another
whole-suite run. Keep distinct configurations and artifact identities separate.

The caller consumes an applicable successful observation instead of running
it again. Record exact source, command/configuration, tools, inputs and results.
A later failed attempt invalidates an older green result for the same check.
After a source fix rerun affected checks, preserving earlier failures. Never
call an unperformed final oracle a passed local gate.
Record in-flight evidence and pending checks in the feature's existing
`.claude/scratch/<F-ID>-progress.md` before handing over.

## Steps

1. **Format.** `cargo fmt --all --check`.

2. **Lint.** For `--fast` and `--scoped`, run `cargo clippy -p <crate>
   --all-targets --all-features -- -D warnings` for each changed crate. For
   `--full`, run `cargo clippy --workspace --all-targets --all-features
   -- -D warnings`. Include both PyO3 binding crates in this single workspace
   lint. Only cargo test excludes their un-linkable extension test binaries.

3. **Run the affected local tests.** Under `--fast` and `--scoped`, use the
   approved plan's unique regression cases against the F-ID Base, not the sprint
   base. Retain actual old-source RED and fixed-source GREEN evidence. Reuse
   applicable observations already produced during implementation. Bindings
   use focused fresh installed tests when needed, never a PyO3 Rust test binary.
   Under `--full`, skip this category when the workspace or complete installed
   suite already covers the same tests and configuration. Retain a distinct
   focused configuration only when the broader suite does not cover it.

4. **Test the workspace.**
   `cargo test --workspace --all-features --exclude rdocx-py --exclude rpptx-py`.
   `--full` only.

5. **The hash harness.** `python3 scripts/hash_harness.py --check`. Run once under
   `--full`, not under `--fast` or `--scoped`.

   **Mandatory for every story in M1 through M6.** An unexplained delta fails
   the gate. An expected delta must be declared in the design plan's
   `## Hash harness` section, and the reported delta must match what was
   declared. A delta that is real but undeclared is a failure, not a prompt to
   update the baseline.

6. **The repository-policy checks.** Run once under `--full`. A workflow-only
   change runs its unique affected policy tests locally, without product gates.
   Newly written completion/review Markdown and commit messages receive their
   cheap prose check after the full gate. Metadata adoption enforces this check
   without repeating product verification.
   Do not run the entire policy suite after each product story.

   Run `python3 scripts/prose_check.py` over tracked Markdown and the commit
   message. No em-dash, no en-dash, no prose semicolon.

   Then `python3 scripts/sync_agent_skills.py --check`. The `.agents/skills/`
   adapters are generated from `.claude/commands/` and `.claude/skills/`, and
   drift means Codex and Claude are following different rules while believing
   they agree. Regenerate with the same script and commit the result.

   Then `python3 -m unittest scripts.test_sprint_workflow`. This holds the
   release family preflights that `.github/workflows/wheels.yml` invokes by
   name as the publication gate, and the assertions over the pinned CI
   toolchains. A failure means a version carrier moved without its assertion
   moving with it. The fix is the carrier or the assertion, never deleting the
   test.

   Without this step the preflights run for the first time on a tag, after the
   sprint is closed. S42 is the demonstration: F-X022 moved every version
   carrier under `crates/`, passed the entire local gate, and left the
   incubating preflight and the `ci.yml` WASM literal asserting the old
   version. The standalone README gate in step9 owns actual README compilation
   and archive inventory checks. Policy tests retain their unique mutation and
   contract assertions without replaying that full integration gate.

7. **The no-default-features path.**
   `cargo test -p oxml-layout --no-default-features`. This is the only thing
   that exercises system font discovery being off while bundled fonts remain
   available. Run once under `--full`. The plan records its
   applicability there. Skip it under `--fast` and `--scoped`.

8. **The wasm targets.**
   `cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm`.
   Run once under `--full`, not under `--fast` or `--scoped`.

9. **Docs.** `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS=-D warnings`,
   then `python3 scripts/readme_doctests.py`. Run once under `--full`, not under `--fast` or `--scoped`.

10. **Packaging.** Run the workspace dry run with every publishable internal
    crate patched to its reviewed local source:

    ```bash
    cargo publish --workspace --dry-run \
      --config 'patch.crates-io.oxml-core.path="crates/oxml-core"' \
      --config 'patch.crates-io.oxml-drawing.path="crates/oxml-drawing"' \
      --config 'patch.crates-io.oxml-layout.path="crates/oxml-layout"' \
      --config 'patch.crates-io.oxml-media.path="crates/oxml-media"' \
      --config 'patch.crates-io.oxml-opc.path="crates/oxml-opc"' \
      --config 'patch.crates-io.oxml-pdf.path="crates/oxml-pdf"' \
      --config 'patch.crates-io.oxml-sml.path="crates/oxml-sml"' \
      --config 'patch.crates-io.oxml-cli-support.path="crates/oxml-cli-support"' \
      --config 'patch.crates-io.oxml-chart.path="crates/oxml-chart"' \
      --config 'patch.crates-io.rdocx.path="crates/rdocx"' \
      --config 'patch.crates-io.rdocx-cli.path="crates/rdocx-cli"' \
      --config 'patch.crates-io.rdocx-html.path="crates/rdocx-html"' \
      --config 'patch.crates-io.rdocx-layout.path="crates/rdocx-layout"' \
      --config 'patch.crates-io.rdocx-opc.path="crates/rdocx-opc"' \
      --config 'patch.crates-io.rdocx-oxml.path="crates/rdocx-oxml"' \
      --config 'patch.crates-io.rdocx-pdf.path="crates/rdocx-pdf"' \
      --config 'patch.crates-io.rpptx.path="crates/rpptx"' \
      --config 'patch.crates-io.rpptx-cli.path="crates/rpptx-cli"' \
      --config 'patch.crates-io.rpptx-chart.path="crates/rpptx-chart"' \
      --config 'patch.crates-io.rpptx-layout.path="crates/rpptx-layout"' \
      --config 'patch.crates-io.rpptx-oxml.path="crates/rpptx-oxml"' \
      --config 'patch.crates-io.rpptx-render.path="crates/rpptx-render"'
    ```

    Cargo rewrites packaged path dependencies to the registry. The local
    patches keep archive verification on this exact reviewed source graph,
    including versions that have not yet been published. They do not enter
    any generated archive. Then assert every generated archive is below the
    crates.io 10 MiB limit:

    ```bash
    oversized=$(find target/package -name '*.crate' -size +10485760c -print)
    if [ -n "$oversized" ]; then
      echo "package exceeds 10 MiB: $oversized" >&2
      exit 1
    fi
    ```

    Run once under `--full` on the clean integrated tree without
    `--allow-dirty`. Do not run publication dry runs under `--fast` or `--scoped`.
    Its retained verified archives also supply the release preflight packaging
    evidence at the same source. No second publication dry run in that preflight.

11. **Supply chain.** `cargo deny check`. Run once under `--full`, not under
    `--fast` or `--scoped`.

## Reporting

Report each step as pass or fail with its command. On failure, show the actual
output rather than summarising it, and stop at the first failing step unless
the failures are independent.

**Never report a pass you did not observe.** If a step was skipped, say it was
skipped and why.

## Refused situations

- **`--fast` as the gate for `/complete-feature`.** Run `/verify --scoped
  F-XXX` instead.
- **Dropping a declared final rider or oracle.** Local scoped readiness does
  not waive final acceptance. Preserve named pending checks in the handoff.
- **Updating the hash baseline to make step 5 pass.** The baseline changes only
  through a declared, reviewed delta.
