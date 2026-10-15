# F-X217 whole-feature implementation review pass 1

**Reviewed:** Frozen working implementation on `work/f-x217-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. The tracked diff contains 23 files, 2,598 insertions and 113 deletions. Read the repository workflow, approved plan, referenced HLD, progress and native reviews 1, 2 and 3.

**Verdict:** Zero defects, zero smells, zero nitpicks in the implementation reviewed. Final feature verification and acceptance remain pending.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Implementation audited

The previous native findings remain resolved in the current persisted import/export paths. The approved pinned parser and limited options remain unchanged. Markdown projection uses actual persisted styles and quote owners, with whole-definition collision checks for template styles.

The additional local-image loader at `crates/rdocx/src/html.rs:750` uses the real Markdown projection and HTML importer. It preflights the tree, checks percent decoding, rejects absolute and parent paths, and canonicalizes paths within the input directory. Escaping symlinks are refused. Missing images retain importer diagnostics. Checked metadata and bounded actual reads at `crates/rdocx/src/html.rs:1720` enforce part and shared byte limits. No network fetch route was added.

Python constructors at `crates/rdocx-py/src/document.rs:4728` share native import routes. File input is bounded at `crates/rdocx-py/src/document.rs:3870`. Templates and resource bytes are obtained before detached native work. Diagnostic records at `crates/rdocx-py/src/document.rs:3845` are immutable and returned in a detached tuple. Public exports and stubs agree with the implemented signatures.

Insertion at `crates/rdocx-py/src/document.rs:4769` validates live paragraph ownership, resolves physical story position and mutates a candidate. Body, cell, header and footer locations use actual owners. Diagnostics are allocated at `crates/rdocx-py/src/document.rs:4821` before candidate publication at `crates/rdocx-py/src/document.rs:4822`. Failure therefore leaves document bytes and held handles unchanged. The existing revision and candidate publication path remains authoritative.

CLI import at `crates/rdocx-cli/src/commands.rs:691` enforces DOCX output and template constraints, resolves bounded local images and publishes through the existing staged path. Source and template paths remain protected even with force. Existing DOCX conversion defaults remain represented by option-presence handling. Runtime controls at `crates/rdocx-cli/src/commands.rs:5955` and `crates/rdocx-cli/src/commands.rs:6061` exercise import and local resource refusal.

The seven planned HLD impact files describe current import, public and CLI intent. They do not claim native Word qualification or completed final delivery gates.

## Authenticated actual evidence

Authenticated capsule SHA-256 `113fc84cb11bb2e78bbeed3f5d110ad49ef267583d20b45a3f3c50751012b8d9`, source map, receipts and retained raw log hashes. This reviewer ran no product, Cargo, CP or Office operation.

The original installed public run retains one failure and five passes. Its construction assertion incorrectly expected `w:highlight` where the approved CSS background mapping produces `w:shd`. The exact construction successor passed after that test-only correction. These qualify six exact installed public nodes without relabelling the failed original. The construction successor retains its separate stubtest failure. Final typing successor 3, receipt `376db15fa5a96bd1ec9dd602e4a4afe6f3e0193d03b7b15faedf1d0412482fe9`, records actual strict mypy and complete stubtest success, with paired distribution checks before and after installation.

Independently authenticated final Word wheel SHA-256 `0a2e1c9c4104caf6bc13f331286176fd5e7752dd8ff6097485ccd15a76b3e238`, all 12 member hashes and every RECORD digest and length. Current Python files and stubs match archived bytes. Native member SHA-256 is `8769ec0ef716dc95bc05689a9386fed34da0f19f48d8f846ae4f03846a3936a2`. Retained intake records matching built native source, and actual distribution receipts establish installation provenance.

The allocation control is causal. The inspected C helper intercepts only `HtmlDiagnostic.tp_alloc`, restores the original allocator when fired and returns real `PyErr_NoMemory`. Its finite Python child catches MemoryError, checks exactly one interception, restores state in finally, verifies unchanged document bytes and held paragraph, then performs positive insertion and save/reopen. Receipt `2db83119ee59152dfce8d5371d1a1c2ac209fbac2d39e28117e7cd91f399a1c5` records qualified zero exit and positive diagnostics. This exercises actual allocation before publication.

Actual CLI and local-resource controls passed, receipts `744f6977a7611a1f217bdefeb624dad5e867d089d9f41401401e4f3994f449ff` and `66a66f6c1504a0967036d5ceddc34517b676ba19336221b547bee3fad46f2f23`.

Current formatting and strict changed all-target, all-feature Clippy passed. The broad native original retains 583 passes, two wrong-Poppler-version failures and nine ignores. Both exact pinned-Poppler successors passed. The following integration original retains 371 passes, four missing-soffice setup failures and eight ignores. Its receipt remains failed. Four exact unattended pinned LibreOffice successors subsequently passed, receipt `5df004c28ba1b468b568331d77d2dd2aea173199ba247e18e68fd361446b0dbf`. Authenticated retained tool pins, four input/output conversion artifact pairs and raw successor logs. These are genuine LibreOffice successors, not native Word authority. Failed originals remain preserved.

## Not found

- Correctness: zero findings in import, bounded resource loading, persisted export, ownership or staged output.
- Contract: zero implementation findings against the approved parser, supported subset, template, diagnostics and public surfaces. Final acceptance gates remain pending.
- Panics: zero findings in path decoding, bounded reads, resource accounting or insertion on untrusted input.
- OOXML: zero findings in schema order, namespaces, persisted ownership or preservation of unmodelled template content within this change.
- Tests: zero additional findings. Failed originals and successors remain distinct. The allocation control exercises a real failing allocation.
- Structure: zero findings. Existing concrete owners and publication paths are reused without speculative abstractions or new product files.

## Remaining gates

Native Word open, save and render remains pending activation permission. LibreOffice cannot substitute for that authority. Remaining scoped regression, emitter, CLI and documentation runs are in progress under root ownership. Final current-source MSRV, WASM, verified crate archives, dependency and repository policy checks, hash qualification and completion handoff remain due or uncredited here. Prior bounded native evidence does not qualify these final gates. This report does not authorize feature completion, sprint closure or release publication.

## Review integrity

Compared all 3,205 tracked file hashes, sizes and modes, HEAD, branch, physical index, logical index and complete binary diff before reading, immediately before writing and after writing. Source-map SHA-256 remained `2b86f4898099034e139f11356294cc0a800680ffea4b09073d7355874d370ed6`. Binary diff SHA-256 remained `50dcd8f90b6d24059d071aad57b59ac555d71aeb2dec5cd1781d953a07a4b636`. Logical index SHA-256 remained `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`. Physical index SHA-256 remained `dbf10b6b2ca5439a3cdfff68e9c1d99ff671e2867326afc30233d904fc993871`.

Only this review file was written. No source, plan, progress, HLD, tests, formatting, ledger or runtime changes were made. This pass ends here and returns control to the orchestrator.
