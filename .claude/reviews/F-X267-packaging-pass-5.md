# F-X267, packaging, pass 5

**Reviewed**: Working correction to prepared tip 58c3e0cf252ad31ffc5c77723b21ca70b72a669e. Two tracked files, 66 insertions and 7 deletions. Incremental diff SHA-256 58279f008c837291004a8fb83ed47490703d4766e36032c74315eabc6a930007. Targeted review of the new shared-writer interaction from packaging pass 4.
**Reviewer**: Codex root independently of Grok implementation. Grok mechanically transcribes this report.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D3 is closed.

written_as_is keeps the existing canonical-name checks and calls the shared reject_zip_local_entry_name on the ordinary output name after stripping one leading slash and on the actual relationship entry mapping. A failed eligibility check returns false and reaches the existing ZIP fallback. It does not propagate a new early error. Therefore the complete writer remains authoritative for error order.

The new regression uses an immutable read baseline and edited candidates. Ordinary and relationship names at entry lengths 65504 and 65505 agree with write/read. The invalid XML control on a refused long name requires U+0001 and rejects the later ZipLocalBlock error. Long otherwise valid names may conservatively fall back, as the approved contract permits. Short canonical names retain the same fast route.

## Smells

None. The implementation reuses the existing validator and mapping in the existing source file. No independent replacement constant, module, trait, generic, feature or public API.

## Nitpicks

None.

## Not found

No new correctness, contract, panic, OOXML, opaque-byte or ownership issue established in this bounded correction. The three labelled Q13 commits are unchanged. Prior all-pass 3 and packaging evidence retain their limited applicability. No confirmation suite was run.

## Evidence consumed

/private/tmp/s95.4-fx267-d3-remediation-ready.txt records an actual current-source semantic RED: one test selected, status 101, where reread returned Ok for an ordinary entry name of 65505 bytes while write/read returned the shared local-name error. Compilation succeeded. Later cases were not reached in that RED and are not claimed as separate failures.

The fixed-source binary oxml_opc-7aaac1354a37b776 executes reread_matches_shared_zip_name_limits, reread_matches_write_and_read, reread_raises_the_error_a_write_raises and zip_entries_and_write_to_share_pinned_zip_name_limit. Four pass, 68 filtered, child 0, 0.03 seconds. Scoped check, all-target/all-feature clippy, formatting and prose checks pass. package.rs SHA-256 is f9379b42810e314580b49c58b7f29205ec5b7058ace3858da5cdf6c7f27f9a6e.

The hash harness has not run for this sprint. The receipt's reuse-list mention does not supply hash evidence and is explicitly excluded. Final hash remains pending, as its command record states. Preparation must keep that debt explicit.

Root changed no source during review.

## Acceptance boundary

Local implementation, affected scoped evidence and independent review permit renewed sprint-final-acceptance preparation. Refresh SourceHead and the stale handoff before integration. Keep the plan approved, delivery in-progress, M21 original and unmeasured, and all final catalogue, release and fallback timing, hash, Word 49, installed typing, native oracle, WASM and archive debt pending. This pass ends and returns to orchestration.
