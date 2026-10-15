# F-X216, public and CLI import surfaces, pass 3

**Reviewed**: Five frozen production carriers against claim Base5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572,389 additions and152 deletions in the637-line patch. Manifestb5f09173c23a30fa4c8890cc4281e25ee9a8bcf853b15b9def62416c28b58bbb and patch185615e18ae34198b3c96595b2ded45e8a6529e014d7b48cdf6c9a19bcb8946d were authenticated. Prior public import passes1 and2 and the approved F216 plan were read. This is a bounded static public/CLI review, not whole native import or installed acceptance.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, CLI import bypasses the output package class and macro-free refusal

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-cli/src/commands.rs:1563`

slide_import calls presentation.to_bytes before publishing the requested output. That method retains the opened package class. The established publish_presentation helper at commands.rs:1141 instead calls to_bytes_for_path at :1145. The native path-aware provider at crates/rpptx/src/lib.rs:2400 maps the output suffix to its package class and at :2414 refuses a macro-free target when the main part carries a VBA project. The existing public contract at docs/hld/10-bindings-spec.md:1163 requires package outputs of both CLIs to follow this extension rule.

Importing into a destination template and requesting merged.pptx therefore publishes bytes that still declare the template class. A macro-enabled destination carrying a VBA project can likewise be published to merged.pptx through this command instead of receiving the existing pre-write refusal. The new file and inputs are otherwise protected, but atomic publication does not correct the wrong package class.

Use the existing path-aware serializer or publication helper before staging the output. Add deciding controls in the existing CLI entrypoint for template-to-presentation conversion and VBA-bearing destination refusal, with both inputs unchanged and no output on refusal. Preserve the source graph and opaque package bytes according to the existing provider contract. No runtime outcome is claimed by this static finding.

## Smells

None found in the frozen five-carrier diff. The concrete request carries actual native options and source snapshot. The result record retains the separately needed allocated Slide inventory and tuple output. No new trait, generic, dependency, product file or forwarding-only API was introduced.

## Nitpicks

None recorded.

## Aspects checked

**Correctness**: No other defect found. Single import retains the old Slide return. The diagnostic companion returns a Slide and immutable diagnostic tuple. Batch returns immutable ordered Slide and diagnostic tuples. Source snapshots precede destination mutation, including same-presentation use. Selection and explicit maps normalize indexes before native staging. Empty successful selection returns without owner publication.

**Contract**: Prior pass2 D1 is corrected in the frozen production source. import_options checks actual layout presentation identity at slide.rs:1145 before self.len at :1153. add_slide and add_slide_at do the same at :1295 and :1335 before stale collection validation. Legitimate source Slides from another presentation remain accepted. Old positional single-import calls and return types remain intact, while source formatting is keyword-only. Source sections and jump maps remain independent batch options. The identified CLI package-class defect remains open.

**Panics and allocation**: No additional defect found. Diagnostics, prepared returned owner paths, each Slide object, batch tuple and final result tuple are allocated before publish_owners at slide.rs:1264. The single-result indexing and final pop expectation depend on the successful checked nonempty single native import invariant. InsertMany checked addition at presentation.rs:3624 uses the successful bounded candidate count and proven existing owner positions. These observations do not replace actual allocator, overflow or current installed qualification.

**OOXML and protected output**: The public layer delegates native graph mutation and serialization. Source graph transfer, inheritance, notes suppression, comments, media and opaque native consumers are evolving and outside this scope. CLI protects both source and destination input identities at commands.rs:1536 and :1537, refuses existing outputs before import, and uses staged atomic publication at :1565. Range components preserve caller component order and explicitly reject duplicates. The shared range parser at crates/oxml-cli-support/src/lib.rs:106 rejects zero, reversed and oversized components. Package-class output semantics fail as D1 describes. No complete native preservation or repair-free Office result is inferred.

**Tests and evidence**: Fresh installed nodes, strict typing, stubtest, returned-object allocation refusal, CLI package-class controls and final native acceptance remain pending. The five-carrier snapshot contains production only. Evolving test and native bytes are not certified here. No source presence or earlier pass1 zero verdict substitutes for current runtime qualification.

**Structure**: No smell or nitpick found. The new methods reuse one actual native batch operation and the existing transactional owner publisher. Stubs name both return shapes and every keyword option. CLI main.rs exposes the approved slide import spelling, one-based selection and insertion options, structured output and mandatory new output path.

## Authentication and boundary

All five live production files, sealed copies and retained Base copies matched manifest SHA, size and mode before and after the audit. The five carrier hashes remain:

- slide.rs69aa4abe61ec1c826524201ea612d6ceb265c1d9190eb910e66098367382c590
- presentation.rscb9d189d9e9a22c837210be92b624b6cf619fac793e7b05abcb297f225795beb
- _rpptx.pyi2e6586cf2751b665589679fb484e6c02f077eb5299a639e3411f07c52feddd7d
- commands.rsffae18c2601edeac23a9bced168883357e4ed897161433db85b9449424c2ab93
- main.rs883b90dbdbbfaf128377774e3367d14e2e2619e99079dc16561f9de2e2fee9a8

Only this review file was written. No source, test, Cargo, interpreter runtime, CP, target, Git, network or Office operation was performed. Native consumer evolution is deliberately excluded from the frozen public scope. This pass ends here and returns the lease for distinct remediation.
