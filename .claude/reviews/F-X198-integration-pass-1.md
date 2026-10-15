# F-X198 integration reconciliation, pass1

**Reviewed**: Final staged squash onto sprint/s91 HEAD1c58b093ee489a5abbbb7bbd3d5751189b55aa2a. Eleven staged files, 612 additions and16 deletions. Read the current workflow and microscope command, approved F-X193/F-X198 contracts, production reviews, preserved structured handoff, current progress record and actual saved archive bytes. No Cargo, source modification or gate rerun.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. The production and shared-document reconciliation is consistent. A post-measurement README rebuild remains an explicit archive evidence requirement below. This review does not assert final archive-policy or integrated sprint verification success.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence

Source identity: canonical document.rs, regression_test.rs and Python test_core.py are byte-identical to frozen worker commit7d115db518be063aa1e5d605f411e5f834240ec9, based on1f163e4aff8d9cbdbad4fad53c8d2c67fc4ae811. Both copied production ALL reviews are also exact. The worker checkout has advanced to0b42204b2fd43a6882e44be09f1fc513d98fd3d3 solely to add its handoff. The preserved `/private/tmp/S91-F-X198-integration-handoff.md` is byte-identical to that committed handoff. No canonical ready file remains. Integration bookkeeping and final delivery records remain integrator actions.

Containment: `.claude/plans/F-X198-design.md:39` through42 clarify descendant cell ownership without changing the reviewed source. `crates/rdocx/src/document.rs:8309` detects qualified physical containment, and `document.rs:27106` refuses only after expected-count and zero-match outcomes. `crates/rdocx/tests/regression_test.rs:398040` exercises a publicly inserted descendant cell, mismatch, no-op and complete byte preservation. The installed Python control at `crates/rdocx-py/tests/test_core.py:4132` is unchanged from the verified worker. The additional canonical plan reconciliation at `.claude/plans/F-X198-design.md:131` preserves the atomic safety prefix and leaves successful Issue301 pairing to F-X202.

Independent F-X193 preservation: html.rs, integration_test.rs and HLD03 are byte-identical to preintegration canonical HEAD. Semantic inheritance at `crates/rdocx/src/html.rs:196`, final mark capture at3335, explicit child precedence at3535 and controls at3905,3926 and3956 remain intact. The staged additions/removals in HLD10, HLD12 and HLD14 exactly equal the frozen X198 worker changes against its Base. They preserve the already integrated F-X193 section at `docs/hld/14-development-backlog.md:6685` and describe X198 refusal at6830 without adopting contributor pairing code.

Archive authentication: independently read both `/private/tmp/S91-F-X198-integrated-archive-{1,2}.crate` files. Each has actual gzip SHA23865ce1473c098fc1049bed7401a2187feb371fac3aff2f56c80a42162d2079 and independently recomputed tuple3529449/50785422/37 using the existing VCS-info normalization at `scripts/readme_doctests.py:1291`. Both log hashes match the retained receipt. All ordinary package source, examples, test members and Cargo.toml.orig match current canonical bytes, including the preserved X193 HTML and X198 changes. Generated manifest metadata remains version0.16.0, edition2024 and MSRV1.93. There is exactly one packaged README and the original37-member inventory.

Remaining archive evidence: both saved archives package the earlier README row3525710/50767756/37. Current `README.md:42` contains3529449/50785422/37. These same-length rows are the sole ordinary-member mismatch. Therefore these retained archives prove the combined production measurement before updating its carrier, but do not prove final README byte equality. The approved rebuild requirement at `.claude/plans/F-X198-design.md:54` and unchanged validator at `scripts/readme_doctests.py:1393` still require a newly identified post-row archive and exact packaged README comparison. Preserve the two original archives under their actual hashes. Do not relabel them as final package-policy evidence. This pending gate step is not a production defect or a tolerance relaxation.

Policy strength: `scripts/readme_doctests.py:405` records the observed combined rdocx tuple. The adjacent CLI tuple at406 remains76692/347960/8, with its README unchanged. The staged script changes only the rdocx tuple. Compression tolerance remains64 bytes at394, exact normalized member bytes/count and10MiB ceiling remain enforced at1348, and the22-entry local patch inventory at29 and strict metadata-set check at1239 are unchanged. No manifest, lockfile, package membership, version, baseline, publication action or roadmap-policy code changes occur in this squash.

Gate boundary: worker final policy and patched dry-run log hashes independently match their final receipt, including actual policy exit0,141 tests and2 existing skips. The preserved handoff distinguishes the original300-second consumer timeout failure from its unchanged exact11.62-second passing rerun and retains native, installed Python, WASM and49 unchanged hash outcomes. Those scoped results do not replace the post-row archive requirement, final integrated full verification or sprint review. Only this review file was written. The review pass ends here.
