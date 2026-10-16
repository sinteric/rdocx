# F-X245, all aspects, pass 1

**Reviewed**: working diff againstddffc4bf6593fa9cf23f4a879830d892030b2748, three files,793 inserted and30 removed lines in the reviewed snapshot. Approved plan and package/comparison contracts read first. This pass reviews the BFS-depth snapshot before the worker's subsequently announced DFS correction.
**Verdict**: 2 defects, 1 smell, 0 nitpicks

## Defects

### D1, shortest-path depth does not bound the recursive graph copier

`crates/rdocx/src/comparison.rs:1699`, `crates/rdocx/src/field.rs:5160`

Discovery records a node's first BFS depth and skips the depth check for known
nodes. A graph with root shortcuts to every node and a long sorted chain between
those nodes passes the128depth rule because every node has depth1. The reused
copier follows sorted DFS and can recurse through the complete chain, reaching
up to4096frames and risking stack exhaustion despite the declared depth bound.
Bound the actual traversal/copy path and add a shortcut-chain control. The worker
independently identified this same issue during final validation while this pass
was underway. Its announced correction is not part of this reviewed snapshot.

### D2, shared edited resources are repeatedly allocated and copied

`crates/rdocx/src/comparison.rs:1540`, `crates/rdocx/src/comparison.rs:1036`

Each unmatched chart relationship creates a fresh complete part map, then copies
with a fresh visited set. Two original drawing relationships sharing one edited
chart become separate full chart/workbook/sidecar graphs in the redline. Repeated
relationships amplify one bounded input closure into arbitrarily many copies,
so the per-signature byte/part caps do not bound staged retained output. Reuse
one operation-local mapping and copied set for the same edited physical source
parts, preserving intentional sharing while isolating original resources. Bound
aggregate copied resources across distinct chart roots as well. Test actual
selected target identity and retained part count/bytes for repeated roots.

## Smells

### S1, sidecar/cycle test follows the unused original relationship

`crates/rdocx/src/comparison.rs:10245`, `crates/rdocx/src/comparison.rs:10062`

fx245_chart_name returns the first chart relationship rather than the surviving
drawing's relationship. The accepted result retains that unused original edge,
so the sidecar/cycle assertions can pass against the original graph even if the
copied accepted graph is wrong. Follow every surviving drawing's actual target
for sidecar bytes, back-edge identity, external metadata and sharing assertions.
Keep the original exact-script failure separate from the corrected observer.

## Nitpicks

None found.

## Not found

- Correctness outside the graph bounds/sharing findings: changed chart and workbook signatures participate before no-op alignment and in reconstruction postconditions.
- Contract: no revision metadata, canonical path policy, new public API, dependency or product file was introduced. The exact reporter script's fixed-part-name assumption is preserved as an observed failure, with a separate relationship-correct observer.
- OOXML: existing package copy mechanisms retain payload bytes and remap relationship targets, with namespace-qualified drawing reference patching retained.
- Panics outside D1: source-part map indexing is established from discovered root membership. Checked byte arithmetic and invalid relationship modes refuse before publication.
- Structure: concrete CarriedChart owns necessary graph allocation data, without new traits, generics or generic import infrastructure.

The review pass ends here. Return control to implementation remediation, then
run a fresh independent numbered review over the corrected frozen diff.
