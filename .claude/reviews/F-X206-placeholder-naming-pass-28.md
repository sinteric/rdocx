# F-X206, placeholder naming, pass 28

**Reviewed**: Independent external naming packet R1, three existing carriers,
191 added lines and8 removed lines. Source and proposals are unapplied by
this review. **Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Identity and evidence

Read CLAUDE, WORKFLOW, microscope, the approved F-X206 plan, its named
capability, packaging, rendering, binding and backlog boundaries, worker
progress and prior model/operation reviews. Both worker ledgers record
in-progress. Branch is work/f-x206-codex, HEAD
7b1e5ef29464e93ae30e68df1523ce09143578ff. Existing implementation changes
are present. This review writes only this artifact in the sprint worktree.

Authenticated receipt
e74d4d2633d0f3b57c6c12974ae1b1d94054cfd5bfe5c7022ff66c4c50eda24f
and complete.diff
9c601c1576c18a428ef158fae3e41d751e32eec5668eef8a3c33bb6c607067cd.
All2714 current worker sizes, modes and hashes match source map53debc8932.
Authenticated all three complete candidates, both phase diffs, retained
contributor patch, reference table and pinned python-pptx source/metadata.
Independently applying every complete patch hunk in memory reproduces the
three declared candidates. Python AST parsing succeeds, which is not product
import or runtime evidence. The actual bundled template hash is
8ecd98d4e52c8ece061cb36c8baa9f0424b362454d820f6df1e425c662b9a057.
All eleven recorded layout XML hashes match actual ZIP members.

## Assessment

Candidate native lib.rs:3422 selects the existing direct modeled layout
placeholder owners in their original order, retains latent exclusion and
clones the complete placeholder record. At lib.rs:3431 new owner IDs come
from the existing fresh-tree allocator. New names are assigned before live
slide publication. Source layout names, IDs, relationships and placeholder
records are not rewritten. Existing imported slide owners are unaffected.
The duplicate layout name/ID control checks source preservation independently
from fresh slide owner IDs.

Candidate placeholder.rs:209 reads only the unqualified orient attribute.
Foreign f:orient does not become an owned orientation. The original parser
and serializer retain opaque attributes and child bytes, while cloning the
record preserves orientation and size as well as type/idx. The new accessor
has a concrete current native naming consumer, without another module,
trait, generic, dependency or forwarding facade.

Candidate lib.rs:6869 maps concrete placeholder types and owned vertical
orientation, uses fresh owner ID minus one for known types, and increments
collision suffixes with checked overflow. The search intentionally covers
the fresh flat tree created by add_slide, rather than renaming imported
recursive trees. The documented unknown-type branch retains generic
ID-based naming. Native vertical aliases are compatibility choices with
explicit limits, not claimed measured PowerPoint type mappings.

The actual pinned python-pptx1.0.2 source at shapetree.py:125 and:182 supplies
the corresponding standard type labels, vertical prefix, ID-minus-one and
collision behavior. Source reference and template agreement are independent
of executing the proposed product helper. They do not prove current native
PowerPoint naming or no-repair behavior.

The two test-only native controls use existing public methods and complete
parsed placeholder records. They do not call the new helper or accessor.
They exercise all eleven bundled layouts, source order and immutability,
orientation/size, foreign attributes, opaque child content and ordinary
library save/reopen. The installed test pins python-pptx1.0.2, loads the same
actual source template in both implementations, retains layout handles,
compares all created names/unique IDs and reopens the saved deck. The added
local pathlib import addresses the retained draft omission.

Post-production-only type/collision and notes controls are separated from
Base evidence. Existing notes-specific naming remains unchanged. No test
uses a contributor-generated output as the native oracle. The new controls
cannot themselves close missing operation-specific native qualification.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Aspects

- Correctness: zero findings. Mapping, order, flat fresh-owner collisions and imported-source preservation are coherent.
- Contract: zero findings. Generated-owner naming and complete placeholder metadata are covered within the declared slice, with full acceptance retained.
- Panics: zero findings. Production suffix overflow is checked. Test indices are limited to the authenticated template and explicit fixture shapes.
- OOXML: zero findings. Owned orientation is unqualified, and the complete existing raw-preserving placeholder record is copied without child reordering.
- Tests: zero findings. Base discriminators use existing APIs and independently bound references. Post-production controls are explicitly separated.
- Structure: zero findings. Three existing carriers, one concrete used accessor and one actual private naming operation, with no new generic architecture.

## Remaining evidence

No Cargo, compiler, product import, installed test, cache mutation, native
application or save operation ran in this review. Native Base discriminators,
fresh-wheel installed coverage, public API/WASM/archive riders and intentional
hash effects remain unexecuted. All supported non-template kinds still need
required native naming qualification and saved-file no-repair evidence.
Reference-derived vertical aliases are not independently measured native
XML kinds. These are explicit remaining acceptance obligations, not a static
claim of full F-X206 completion. Review phase ends with this artifact.
