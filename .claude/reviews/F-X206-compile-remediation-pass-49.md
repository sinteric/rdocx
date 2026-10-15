# F-X206, additional raw IO compiler remediation, pass49

**Reviewed**: External R14, four additional raw write error conversions in the prospective R12 paragraph union. Actual worker remains on adopted R13, HEAD `7b1e5ef29464e93ae30e68df1523ce09143578ff`. This is an incremental review of the proposed conversions and original-five Base input renewal, not a whole union compiler or runtime verdict.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt SHA256 `d0e3b5d856617f559123b42b1f9c7df2e45fb86e4908b313878cbbf074a459ed`. R13-to-R14 diff SHA256 `6b245b2e0b25db2437d8253640cacba71027f97f42ecc7fad0175854e7531dab`. The complete paragraph candidate SHA256 is `063e7cd68d2c0b16ec2129f9f580eb1911b5ac584e6b0be65686239da9000573`. Independently deriving its diff against the sealed R13 union produces the supplied diff exactly. Only four map_err additions distinguish the candidates. All other paragraph bytes, including earlier R13 fixes and accepted namespace ownership logic, remain exact.

Authenticated all2714 actual worker records by SHA256, size and mode against current R13 map `f1fe07fc52e484c163b1d1a5b9b2d8f37dbe983056b9a1a18b4eaa26d7457895`. R14 remains external and unadopted.

Original-five input manifest SHA256 `3538960e56c3ea4d0fea551e9476e555bfe7e67c15af5222d3d5f540a48c378e` binds that actual R13 map and prospective temporary map `a6771daccb8d82555a8a116ebd0f603cf7680a89a8e0d1d1644438b180a14f2a`. All three candidate bytes/sizes/modes match their map entries, and only those three entries differ prospectively. Both non-paragraph candidates match the original immutable references. Independently compared the paragraph appended helper and both Base bodies byte for byte against original R1. All five command objects equal the previously reviewed R3 queue exactly. latestProducer and admission remain null.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings. Candidate `crates/oxml-drawing/src/text/paragraph.rs:53`, :184, :312 and :909 add only explicit OxmlError conversion to existing raw write_all calls. Receivers, slices, ordering, namespace closure invocation and successful early returns are unchanged. std::io::Error becomes existing OxmlError::Io at `crates/oxml-core/src/error.rs:11`, then existing TextError::Xml at `crates/oxml-drawing/src/text/body.rs:78`. Failure remains propagated, never suppressed.
- Contract: zero findings. No API, owner selection, accepted R12 namespace policy, raw payload or mutation publication changes. The prospective union is distinct from the actual R13 baseline used by the original Base tests.
- Panics: zero findings. The four mappings introduce no indexing, unwrap, arithmetic or new allocation policy. Existing source span validation is untouched.
- OOXML: zero findings. Successful output is byte-identical to R13 union behavior. Source tag spelling, closure bindings, raw text contents and hyperlink opaque particles remain on the same operations.
- Tests: zero findings. All deciding helper/body and five command identities remain exact. Producer/admission null prevents claiming a new execution seal. Original compiler failures are not semantic RED evidence. Corrected Base execution must still qualify exact one-test assertions and restore the adopted R13 source, not an older source map.
- Structure: zero findings. Existing carrier and existing error conversions only. No new type, dependency, file, module or forwarding abstraction.

## Limits and handback

Retained rustfmt check exit1 is not compilation credit. R14 does not establish complete union compilation or semantic acceptance, and this review ran no compiler or runtime. Original five Base controls, actual producer admission and all later approved issue309/323 and frozen PR acceptance remain required. Prior independent union and R13 reviews retain their scopes. Only this review report was written. No source, Cargo, environment, target, cache or UI action occurred. Review ends and returns the worker review lease to ROOT.
