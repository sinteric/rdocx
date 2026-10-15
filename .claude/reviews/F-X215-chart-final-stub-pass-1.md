# F-X215, Chart final stub, pass 1

**Reviewed**: One annotation in the existing PowerPoint stub, preserved preimage SHA `cf0eaee682f9b7f648d6e8881fc0ac40da212225fddf0d7c6ae8b886c6bd4fd4` and corrected SHA `96a6b5943d58c48780ff715dec712664c377f433d90dec017920760cc56fc152`.

**Verdict**: Zero defects, zero smells, zero nitpicks in the bounded correction.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Runtime and stub agreement

`crates/rpptx-py/src/shape.rs:2455` declares PyChart as a PyO3 class without subclass support. Its actual native class is therefore not subclassable. `crates/rpptx-py/python/rpptx/_rpptx.pyi:1737` now marks Chart with the already imported typing.final alias. This accurately describes the existing runtime restriction without changing constructors, properties, methods or the native implementation. Neighboring concrete native wrappers already use the same convention.

Independently authenticated the preserved and current stub and reconstructed the entire byte difference: one @_final line immediately before class Chart. Current stub AST parses. No allowlist, error suppression, new dependency, runtime mutation or unrelated stub change is present.

The installed stubtest implementation at `/private/tmp/S91-F-X194-evidence/python/lib/python3.12/site-packages/mypy/stubtest.py:483` requires finality when actual subclass construction raises TypeError. Its disjoint-base check at line556 explicitly excludes final classes. The single declaration therefore addresses both reported consequences by describing actual runtime authority, rather than suppressing the failure.

## Evidence and boundary

Authenticated original full PowerPoint native stubtest failed log SHA `419292307c548055444f995719a444630d2a7b7f93195b1d01768458a3c9e3e1`. It remains failed. Root's actual85 public passes and other current terminal scoped gates retain their own evidence and are not rerun or requalified by this review. Word native stubtest concise zero-exit output remains its separate actual result.

The corrected stub requires a fresh PowerPoint wheel and reviewed installed continuation with provenance and remaining typing/stub controls. Current Word artifact and all native Rust bodies remain outside this one-line correction. No final corrected-wheel, installed-stub or whole-union green is inferred before those actual results. This audit ran no provider, test, compiler, CP, target, UI or Git operation and changed no source, index or ledger. Only this canonical review file was written under the granted lease. Review ends and returns that lease.
