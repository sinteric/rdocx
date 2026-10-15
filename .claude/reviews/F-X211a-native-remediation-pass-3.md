# F-X211a, native remediation, pass 3

**Reviewed**: Frozen source103 to source106 remediation in `crates/rdocx-oxml/src/text.rs`,221 additions and30 deletions. Capsule SHA-256 `6d9a942c22150d640829844af447ee4dbc97a8c0ab7fe95a096424e86d252cc1`, actual patch `d540ce4c9a174cfdfcf9c2ddd9c61410f709d8d7a19592ed3e478662c64430d7`, source map `78325c7268d47771fef127f735d70b0c5e0537c50582c17d599eed553add5167`. All3228 source entries authenticate. The patch exactly reconstructs current source from the retained actual103 preimage.

**Verdict**: One remaining defect, zero smells, zero nitpicks. D1 generated drawing namespace collisions are repaired, but exact opaque preservation still fails. D2 and D3 remain addressed. Actual source106 full OXML result is714 passed and two failed.

## Defects

### D1, opaque AlternateContent still changes on ordinary save

`crates/rdocx-oxml/src/text.rs:3727`

The current per-slot pipeline discovers used foreign w/wp/r bindings and injects them into the opaque subtree independently of whether its actual enclosing emitted owner already retains that authority. Source namespace closure must preserve the opaque block when no binding would otherwise be lost. The header case at `crates/rdocx-oxml/src/header_footer.rs:1374` has a foreign wp header-root binding and no selected typed alternate Drawing. Its retained q:AlternateContent nevertheless fails the exact source substring assertion after save. This is a genuine mandatory source-preservation failure, not a requirement to normalize the fixture or weaken its assertion.

The separate encoded local MC control at `crates/rdocx-oxml/src/document.rs:4576` also fails its exact opaque substring assertion on current source106. That control deliberately keeps the locally encoded namespace as opaque source. Its precise emitted delta needs a bounded diagnostic, and this review does not invent a shared cause for the two cases. Both actual failed inputs and outputs must remain retained while the owning closure path is corrected. The successful wp/r collision controls must continue to prove that actual generated drawing elements and picture relationships retain their canonical namespaces.

## Smells

Zero additional findings.

## Nitpicks

Zero findings.

## Not found

- Generated drawing and relationship collisions: zero additional findings. `crates/rdocx-oxml/src/text.rs:3692` protects the actual inherited writer prefixes w, wp and r from incompatible raw metadata. Authored inline/anchor and picture relationship emissions consequently retain canonical authority. Compatible/custom/default bindings remain available to opaque content. Other drawing prefixes were audited against their concrete local declarations instead of treating every spelling as reserved.
- Real run-root attributes: zero additional findings. `crates/rdocx-oxml/src/text.rs:577` extends the existing producer alias routine only for an actual generated w:r when wp or r conflicts. Fresh aliases respect both existing source declarations and earlier allocated aliases at line634. Producer attribute expanded names and values survive. Namespace-valued root references to a conflicting prefix refuse before output at line615 instead of silently reinterpreting the value.
- QName discovery: zero additional findings. `crates/rdocx-oxml/src/text.rs:3727` gives discovery the complete actual source facts, including xsi itself, and filters only the declarations injected afterward. This corrects the source105 xsi:type fact loss without changing raw exposed bytes or inventing prefix-to-URI bindings. Internal declarations and shadowing continue to win.
- D2 unrelated opaque-value decoding: zero findings. Selective actual MC and xsi:type decoding remains at `crates/rdocx-oxml/src/text.rs:13711`. Ordinary opaque attribute values are not decoded by namespace discovery. The original malformed legacy-form control now passes in the current full OXML execution.
- D3 repeated metadata extraction: zero findings. `crates/rdocx-oxml/src/text.rs:3402` prepares once for run emission and line2060 once for cached-field admission, then reuses concrete bindings for each child. The causal512/1024 slot-visit control executes successfully. No per-child full-slot metadata scan returns.
- Tests: zero additional findings. `crates/rdocx-oxml/src/text.rs:14340` checks actual opaque expanded names, generated inline/extent/docPr namespaces, picture r:embed namespace and relationship identity, empty/nonempty producer root attributes, occupied aliases, save/reopen and atomic unsupported namespace-value refusal. These controls pass on actual source106. Default/custom/QName fixed-point, foreign-a, foreign-w/cache and original table authority controls also pass. The two full-suite failures remain D1, not credited positives.
- Contract, panics and structure: zero additional findings. No public API, resource bound, consumed-reader cursor proof, fallback classification or drawing marker eligibility changes in this remediation. Existing carriers and the producer attribute helper are reused. No new product type, file, module, dependency, generic parameter, trait, global cache or quota is introduced. No new production unwrap or expect appears.

## Actual evidence and qualification boundary

Read the prior numbered reviews and approved contract. Authenticated current source and every capsule-listed patch/preimage/driver hash, including the exact source105 to106 QName correction. Authenticated terminal receipt `fdff434edcee511d579cd1cae9cf68781a2088f8343114ef40164f254583cfd0` and all nine raw log hashes. All owned processes are naturally terminal, gone and reaped.

Current cheap affected all-target check and release compilation succeed. All six selected authority, collision, fixed-point/visit, foreign-a, cache and marker controls pass. Full OXML is714 passed and two failed, actual exit101, log SHA-256 `c99890a368fb826fdc91a86a1fb3232462800ea009ca7e9afb7a544d0e4afe06`. Full Word unit and regression rows remain unexecuted after the OXML failure. Source105 and earlier failures remain separate original evidence. No current wheel/install, affected archive or full integrated sprint green is inferred.

This review performed no source fixes, runtime probes, Cargo, CP, Office or Git operations. Return control to the orchestrator for a separate bounded diagnosis and remediation phase.
