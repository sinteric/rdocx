# F-X210, export HLD reconciliation, pass 26

**Reviewed**: current source206f09 native and installed export companions, existing HLD10 coverage paragraph and the approved remaining export route contract.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, remaining export companions are absent from the binding spec

`docs/hld/10-bindings-spec.md:2449`

The current paragraph describes Word PDF and raster companions and PPT main-slide PDF/PNG companions, but omits the implemented native archival PDF, PPT native notes and handout PDF/PNG, and installed Python notes companions. Update the existing paragraph with those actual surfaces and their native versus Python availability. Preserve the stated absence of Python archival or handout companions. Existing implementation and stubs already expose the notes methods, so this is current spec reconciliation, not new API scope.

## Smells

None.

## Nitpicks

None.

## Not found

Font mapping, bounded metric limitations, source-code preservation, same-layout ownership, cache rollback and ordinary-warning retention produced no findings in the inspected composition. Final whole-feature review and package/policy completion remain separate. Review ends before documentation remediation.
