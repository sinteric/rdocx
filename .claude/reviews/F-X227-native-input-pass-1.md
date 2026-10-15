# F-X227, native input revision, pass 1

**Reviewed**: external immutable proposal SHA89128119ce153b54b14264540f97a3c0eedc06c70c97c62d4cf2b07582ca7d60, four PPTX files, five logs, seed custody and52 prior approved-folder file hashes. No product or input was changed.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Package and derivation checks are clean. Native witness custody must be completed before input admission.

## Defects

### D1, native positive observations have no authenticated execution record

`/private/tmp/S91.1-F-X227-native-fixture-revision-1/native-revision-proposal.json:6` (`/ordinaryFirstReopenExit`)

`/private/tmp/S91.1-F-X227-native-fixture-revision-1/native-revision-proposal.json:7` (`/reopenedSaveExit`)

`/private/tmp/S91.1-F-X227-native-fixture-revision-1/native-revision-proposal.json:8` (`/ordinaryFinalReopenExit`)

`/private/tmp/S91.1-F-X227-native-fixture-revision-1/native-revision-proposal.json:10` (`/CUANativeRepairDialog`)

The proposal reports three exit0 observations, the exact final native window path and no repair dialog. The supplied reopen logs are empty and the save log says only Task SaveAs returned. They do not retain process exit status, exact invocation, process cleanup or the actual CUA state that establishes the file and absence of repair. ROOT confirmed it does not hold those earlier positive tool responses. These assertions therefore cannot yet be independently authenticated from the sealed evidence.

Retain the real earlier failures separately: native-first-save.log records AppleEvent timeout1712, while approved-folder-first-save.log records stale object1728. Both remain failed. Neither an empty log nor the eventual existence of a valid PPTX proves an ordinary repair-free reopen.

Supply the original actual process/tool receipts and CUA state if retained. Otherwise use a distinct bounded native witness capture after this review ends, retaining the original observations and failures. Do not overwrite the input files or relabel an earlier failed wrapper as success. This finding concerns evidence custody, not a demonstrated native repair or malformed package.

## Clean package and source findings

The seed exactly matches historical P12 output SHA8b83c694a9a90a4c9408bb65e6d537bf3887b7330a5d29f8ad914e48f3f14ff4. It is a historical binding output used as a new native seed. It is not either missing native original. The proposal correctly keeps nativeOriginalRecovered false and status proposed, with independent review and three fresh specimens pending.

The revised notes file is43475 bytes, SHAc74dc1a4a0abbdd3736ddcfddfdb6df4fc19f823f36cfcc445c886543121aa4f. The transition file is40496 bytes, SHAf18114cde8876d7ad0e2cb9646400fa0702a7a472507dc6eaf79ec0c7a17390f. The final approved-folder copy matches the revised notes SHA. Every one of52 old approved-folder files still matches its pre-action hash.

Both packages have unique ZIP member names, parseable XML, unique relationship IDs and no missing internal relationship target. They contain two slides, only notesSlide1 and no handoutMaster. Presentation root order and notes slide/master child order are valid for the inspected modeled children. Notes size6858000 by9144000 and the slide-image/body master offsets and extents match the retained model. The real note body is intact. The trailing0 in the proposal's joined firstNote comes from a separate slide-number placeholder, not appended speaker-note text.

A read-only reconstruction of build_manifest.py:713-722 reproduces EVERY transition package member byte for byte from the revised notes package. Exactly three existing members change: Content_Types, slide1 and its relationships. Only corpus-tone.wav is added. The1644-byte generated PCM WAV has SHA20702f3b44ece6beba2239c39e23aaad68455abfcc1ebc3c656de60eb36a73c8. Transition ordering, fade/timing attributes, the audio relationship and the foreign trace are retained exactly as the original recipe specifies. No original frozen script or support was executed or edited for this check.

## Qualification boundary

This pass does not admit the input revision. P12 still requires actual unchanged body/save/reopen plus Notes2pages, Handouts1page and raster2/1 with the new source. P25 requires actual theme application and retained direct-content/layout facts. P26 requires actual unchanged body/save/reopen, fresh revised audio/opaque relationship graph and the literal native timeline obligations. Existing historical native proof is reusable only through the applicable exact input/output and concrete current provider bridge. No95 score or CI success follows from these package checks.

## Not found

Correctness of the retained package model and recipe: zero additional defects. OOXML child order, internal relationships and opaque payload preservation: zero defects. Source identity and old-folder immutability: zero defects. Panics and structure: no new product code in this review. Smells: zero. Nitpicks: zero.

Review ended. Return the canonical review-file write lease to ROOT. No remediation was performed.
