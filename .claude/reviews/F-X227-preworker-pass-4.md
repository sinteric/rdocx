# F-X227 pre-Worker admission review, pass 4

**Verdict: 0 blocking findings, 0 structural smells, 0 nits in the bounded V3b diff.** Reviewed frozen external wrapper `6616b6dbbfd23950672b603b47fe91e225b678d3c78e2f01972ae99f581e3db3` against reviewed V2 `7b599419247a88a10b8d8e5c5eddce3d9a36f58c264725c6c4d7a8afdf3bb835`. No source implementation, runtime, registration, runner or canary was changed or executed. Only this review file was written.

## Enum representation

`/private/tmp/S91.1-F-X227-preworker-v3-remediation-1/admit_worker-v3b.py:129` through141 admits only exact integer4 or an ASCII case-insensitive Action name, paired with exact integer3 or an ASCII case-insensitive Script name. The reference must still have exactly one type property. Booleans, floats, numeric strings, Unicode lookalikes, repository/container references and background steps remain refused.

The official pinned converters at `StepConverter.cs:35` through45 and `ActionStepDefinitionReferenceConverter.cs:35` through45 support integer discriminators and case-insensitive enum strings. Their enum definitions at `Step.cs:71` and `ActionStepDefinitionReference.cs:19` bind Action4 and Script3. These files are retained in the same remediation directory with the official197ca027 source pins in `source-authority-and-retention.json`. The wrapper accepts a deliberately narrow subset of the official forms. This does not introduce another executable step kind.

## Exact public configuration expression

At `/private/tmp/S91.1-F-X227-preworker-v3-remediation-1/admit_worker-v3b.py:147` through166 the expected configuration remains a validated lowercase64hex owner pin. The added path recognizes only one environment mapping value with exact integer BasicExpressionToken type3, the fixed expression `inputs.config_sha`, and the permitted source metadata keys. It requires the same request's decoded inputs context to contain a string config_sha equal to that pin. The existing literal mapping check then requires the sole F_X227_CONFIG_SHA key and rejects other environment names or entries.

The transformation at164 constructs a new local environment dictionary and a new mapping row for comparison. It does not mutate request, step, raw bytes or the frame. The ordinary projection still represents the original request. The original header and raw bytes remain the forwarded payload, and mandatory same-frame ROOT approval is unchanged. No general expression evaluator, string interpolation, secret lookup or script-body expression was added.

The retained official `StepsRunner.cs:124` passes the step environment and actual expression context to `PipelineTemplateEvaluator.cs:110` through125. `TemplateReader.cs:656` constructs a BasicExpressionToken, and the retained BasicExpressionToken sources declare the expr wire member. These support why a fixed unevaluated expression can occur before official Worker environment evaluation. They do not establish what value the preceding refused assignment contained.

## Authenticated controls and unchanged boundaries

The terminal40-control receipt `/private/tmp/S91.1-F-X227-preworker-v3-remediation-1/actual-owned-v3b-schema-controls-1/receipt.json` physically hashes to `80ae728831cea5e1bdf7791cb0fa2da5e9be17e3bdd828bf791b90e1d303d8e9`. The controller hashes to `d3dc4c3d46a5ee75a7db72e75558b8f9888fc2361eaf481e58ba0d36f901de93`. All40 referenced admission receipts and all40 copied wrapper files were authenticated against their recorded pins.

`owned-v3b-schema-controls.py:14` through16 defines numeric/named combinations, malformed or executable alternatives, twelve expression cases and the prior V2 wire-schema controls. Lines39 through54 cover exact expression binding, optional source metadata, wrong/missing/nonstring inputs, other/secret/composite expressions, extra token fields, wrong environment key and a float token type. Lines55 through68 check eight accepted cases, byte-exact original frame delivery to the owned fake Worker, mandatory prior proposal approval, finite cleanup and credential-marker absence. Lines69 through75 check rejection before Worker launch and static diagnostic shapes for the other32 cases.

The actual diff contains only enum acceptance and the local configuration-expression comparison. Setup denials, identity requirements, script hash/shell/directory pins, tool guards, owner approval, one-job custody, diagnostics and lifecycle cleanup are unchanged. Settled protocol and cleanup reviews were not repeated.

The reviewed V2 source and preceding V3 candidate/28-control evidence remain retained with their original hashes. The second refusal receipt records STEP_KIND, while the actual composite discriminator values were not retained. Neither refusal attempted a corpus case or earns a score. The named-enum repair is supported by official source, without claiming the exact historical discriminator is known.

## Handoff

No remaining finding was established in this bounded diff. These owned controls are not an actual server-frame capture, official serializer execution, registration or actual100 CI acceptance. A successor deployment must pin the new wrapper and retain mandatory same-frame ROOT admission. The review ends on writing this file and returns the canonical writer lease to ROOT.
