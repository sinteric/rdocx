# F-X206, namespace provenance, pass 69

**Reviewed**: Complete50487-byte incremental R20 diff in three existing DrawingML text carriers against reviewed source81c458. Root authenticated all2714 current records and all retained evidence hashes. An independent parallel read-only auditor reconstructed each changed carrier from the complete patch.

**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt147195966800b8b60d5030e86e907bfdb195575ac90198ab7cdf19112e7a50fc, incremental189278410625322d78b7bde37608abbf29e3e0b94d7f66e884dd199a71589f4b and source-afterd3c85493d10a0822e2ad4be552e2ba96d15eb49b976236175eebae7c62398c4c authenticate. Exactly paragraph.rs, list_style.rs and text/mod.rs differ. Other2711 records and modes match the reviewed source. Actual173 inventory has171 passes and two existing ignores. Drawing checks and fifteen-crate Clippy passed. Passing existing tests does not refute the defects below.

## Defects

### D1, regular run namespace closure scans after the owned run

`crates/oxml-drawing/src/text/paragraph.rs:3223`

The parsing loop has already consumed the run end before cloning the reader for capture_element. That helper at crates/oxml-core/src/raw_xml.rs:11 requires the reader immediately after the opening tag. Closure consequently scans following siblings or EOF rather than the owned run. An inherited foreign prefix used only by a retained opaque run child is absent from the computed closure. Relocating the public runs into another paragraph can therefore leave its preserved opaque payload unbound. Compute closure before consuming the owned run and add a real inherited-prefix relocation and repeat-save control.

### D2, DrawingML-only identity changes PresentationML wrapper ownership

`crates/oxml-drawing/src/text/paragraph.rs:54`

text_has_physical_owner recognizes only A_NS, but text_source_tag is also used by CT_TextListStyle::write_xml_as at crates/oxml-drawing/src/text/list_style.rs:220 for PresentationML wrappers. A genuine q:defaultTextStyle bound to the PresentationML namespace, with canonical p bound to a foreign URI, is emitted using p:defaultTextStyle. The owned wrapper acquires foreign semantics and its valid original q identity is lost. Actual callers include crates/rpptx-oxml/src/presentation.rs:507. Preserve the wrapper's expected namespace and genuine alias/default owner without admitting foreign roots. Add aliased and conflicting canonical wrapper round-trip controls.

### D3, descendant namespace refusal tests stop at an invalid root

`crates/oxml-drawing/src/text/mod.rs:653`

The nested fixed-prefix control and the nine descendant cases at crates/oxml-drawing/src/text/mod.rs:672 retain urn:presentation as the wrapper namespace. Newly enforced root rejection fails before reaching the intended descendant rebinding operand. Their green assertions no longer prove their named behavior. Use the official PresentationML root URI, prove an otherwise valid matching control parses, and retain separate foreign-root rejection. Do not weaken or blanket-admit foreign roots to restore the tests.

## Not found

Correctness, contract, OOXML and tests have the defects above. Panics and errors: zero additional findings. Structure: zero findings. Smells: none. Nitpicks: none. The five previously authorized fixture reconciliations preserve their declared operands, but do not resolve D3.

## Handback

Only review records were written during this pass. No source, runtime, environment, baseline or ledger changed. Return for distinct remediation in existing carriers, with focused controls and an independent pass70 before remaining broad acceptance. Whole-feature, installed, native geometry, package and integration gates remain due.
