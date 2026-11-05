# Conversation-aware integration

Targeted audits retain the committed inventory for repositories absent from the
local raw cache. This matters in fresh worktrees: querying one repository must
not delete the other donors' records or their hash-bound review evidence.
Refreshed records replace matching repository/number entries; unrefreshed records
remain historical evidence, without being promoted to freshly reviewed status.

## Current source handoff (2026-11-05)

PR43–49 are normally merged. PR49 main merge083de838 has exactly candidate89a63b1's
tree; its complete Build41945829235 and CodeQL41945828068 passed, including515
Rust tests, Windows/Android packaging and primary S3/AVR profiles. PR45 and47
are GitHub-confirmed indirect merges of their complete heads, not merely closed
dependency proposals. No check was waived. The incompatible partial egui0.37
upgrade is preserved but explicitly deferred to issue50.

At the code checkpoint there are zero open PRs, unmerged inventoried local/fork
tips, stashes or dirty worktrees. All complete diffs and paginated conversations
were reviewed; the post-merge inventory refresh does not promote other unread
records. The receipt snapshot precedes its own publication. Any receipt-only
checks and later main runs are separate from the accepted source CI identity.

Pealayer's owner-approved immutable9dfc54d3 supplies the committed donor snapshot;
active newer project/typography drafts and the other registered Pakar worktree
remain untouched. Live donor drift is still reported by local consultation, not
hidden by a weakened guard. See HANDOFF.md for exact hashes and remaining product
gaps. Neither boards, installed clients, private settings nor deployment policy
were changed by this source synchronization.

## Historical handoff (2026-10-29)

PRs #27, #34, #35 and #36 are normally merged. Main code checkpoint b752b4c has
exactly the tree of 224dd2a, which passed Build40765000068, CodeQL40765000034 and the
standalone finding check, including Windows/Android packages and all S3/AVR
profiles. Complete diffs and all paginated conversations were reviewed. The
final refreshed inventory retains all 814 records; fetching never promotes unread
records. Copilot requests are not claimed as completed reviews.

At this checkpoint there are zero open PRs, no unmerged local/remote branch tips,
no stash or dirty source. The other owner's merged worktree and private state
are preserved. Final receipt-only metadata has separate post-merge CI. Issue2,
the remaining product/donor/hardware gates and Linux glib issue17 stay open.
See HANDOFF.md and the current GitHub main identity before continuing elsewhere.

## Main consolidation checkpoint (2026-10-28)

PR27 and PR34 are merged normally as e47b230 and da12fde. Final main has the
exact code tree of ccc061d, which passed Build40753870815, CodeQL40753922292
and the standalone finding check. Parent8102f04 separately passed all checks.
All original checkpoints are preserved as main ancestors. Every PR diff and
paginated body/comment/review/thread was read, including outdated findings;
all four current CodeQL threads are resolved. Copilot requests are not claimed
as completed reviews. No open PR or unmerged local/remote branch remained at
that checkpoint. Dependabot subsequently opened PR35 for rustls0.23.46;
it merged normally as335755a after finalbabf858 passed Build40760281674,
CodeQL40760281681 and the standalone finding check. The concurrent canonical
shell PR36 was consolidated with that merged main and subsequently merged as
recorded above. See the current GitHub list rather
than interpreting a historical zero-PR snapshot as a perpetual status.

Root main was fast-forwarded and the other clean, fully merged PortableShell
worktree is preserved. Private backups/settings/caches were neither deleted nor
published. The final handoff/inventory update is metadata-only with separate
post-merge CI. Issue2 and all hardware, donor and installed-runtime gates remain
open; no production device/client was contacted or changed for this handoff.
See docs/HANDOFF.md for exact hashes, run links and remaining issues.

## Earlier handoff consolidation (historical)

PRs #21–#24 are merged. Main merge f4259db preserves the exact code tree of
c1d2dd17, which passed Build 40508742014 and CodeQL 40508742015, including the
standalone finding check. All workflow, tooling, quality, S3/AVR, Windows and
Android jobs passed; draft publishing was skipped. Original source/dependency
heads and the temporary AVR repair are verified main ancestors. The local
checkout was fast-forwarded and only verified-merged branches removed.
Workflow updates, mdns-sd 0.21.6 and archive 4/MQTT/XML 7 are retained;
eframe/egui 0.37 is deferred until the dock/icon/theme family supports its types.

Every paginated conversation/review/thread on all four PRs was read. The seven
new CodeQL threads are resolved after source fixes or narrowly documented
test/intentional-transfer dispositions. Later artifact-read races and the
late backup mutation were fixed without suppressing checks. Current CodeQL passes.
No completed Copilot review is claimed: GitHub rejected both reviewer requests.
The master production issue remains OPEN. Its source-sync checklist and the
post-merge conversation inventory are refreshed; final receipt changes are
documentation/audit only, with a separate post-merge CI run. See docs/HANDOFF.md.
Private outputs, secrets, rollback images and installed settings remain local
and untouched; source synchronization is not a board/client deployment.

## Historical pre-merge checkpoint

The fresh Pakar enumeration contains 23 complete records. Dependency PRs 21,
22 and 23 are open; required CI is not green, so they are not merged or certified
deployable. Their fetched conversations contain no comments, reviews or review
threads; fetching release notes is not a completed dependency/source review.
The refreshed conversation projection contains no stale-review decisions.
The file comparison check also passes after rereviewing the 18 changed target
files and PCController's current devices_page.dart. All 375 existing comparisons
are hash-current; 4,322 pending comparisons and 275 known parity gaps remain
explicitly unfinished. Neither zero stale records nor deployed packages closes
the production acceptance issue.

Adapted from the fully read Pealayer `docs/REVIEW-COMMENT-AUDIT.md`, original
SHA-256 `7a91d9fa8778383ee37e33d861ceebc5ab210e46bedab6af8be82d56c27523cc`.
The procedure is now executable in `tools/audit-github-conversations.mjs`.

Before merging, read the entire PR diff, issue comments, review summaries and
paginated review threads, including outdated and unresolved threads. A merged or
closed PR does not imply that its findings were resolved. Quota notices are not
findings, and requesting Copilot is not proof that Copilot performed a review.
Reply with specific implementation/test evidence; resolve only findings actually
fixed. Do not bypass required protections or weaken checks to obtain a merge.

Run the read-only auditor against all relevant repositories, or only Pakar for
routine PR work. It follows every issue/PR, comment, review and nested thread page,
rejects incomplete responses, and keeps one stable ignored snapshot under
`.build/github-audit/`. Identical data is not rewritten. The committed
`github-conversation-inventory.json` records content hashes and pending review,
not a claim that collected bodies were read or imported. `--cached` reprojects
the local snapshot; it is not a live refresh.

The existing source-file ledger remains authoritative for actual code imports.
Conversation decisions are a separate, hash-bound requirements audit, not another
code-completion ledger. Never promote all pending source rows after fetching a
repository, or reinterpret useful generic behavior as excluded business logic.

## Historical integration and discovered risks

- Pakar PR #14 merged as `e9125c766d105b784b1d4a44adcaa5cdde5a44e7`.
  Complete diff/conversations read; zero review threads and prior exact-head CI
  passed. The new icon font dependency does not replace the shared launcher icon.
- Pakar PR #15 merged as `78b6128`. Current main was merged normally into its
  branch; original donor hashes were retained, and exactly two changed workflow
  target hashes rebound after rereading Pealayer CI/CD and CodeQL plus both Pakar
  workflows. Focused governance/ledger acceptance passed. Current-head CI remained
  asynchronous: this is not an all-green claim. No admin override was used.
- Copilot review requested on both PRs; no completed Copilot review was observed.
  All remote Pakar branches except main are gone; the merged local branch and
  obsolete remote-tracking names were removed with normal verified Git operations.
- Pealayer PR #55 retains two unresolved findings: authentication/origin safety,
  and correlated command errors/persistence completion. Pakar already checks
  origin on authenticated HTTP/WS paths and preserves settings drafts through
  accepted-write/readback mismatch. These source checks are not a full DNS-rebinding
  audit or physical durable-write proof. Do not import permissive donor CORS.
- PCController PR #212 retains two CodeQL origin findings for service workers.
  That older checkpoint had no worker. The current canonical web/sw.js only
  handles same-origin shell GETs and has no message-control handler. Future
  message controls must validate origin and sending WindowClient. Do not mark
  those donor threads resolved from this repository.
- PCController #552 distinguishes ACK from authoritative output state. Preserve
  this distinction in every Pakar actuator surface; live output/readback parity
  remains pending, not inferred from mock HTTP acceptance.
- PCController #595 distinguishes segment authority from complete front-panel
  authority. Existing individual peripheral ACK gates remain; generic event
  reduction, sequence/gap recovery and multi-client physical acceptance are still
  useful imports, not excluded because full snapshots currently work.
- PCController #291 requires scroll/cancel-safe touch activation; retain this
  open acceptance for future instant/hold controls, including mobile. Current
  click controls are not proof of the complete shared gesture primitive.
- PCController #357 requires reachable/current/rollback-aware storage retention.
  Pakar's protected single release cache and verified debug-only pruning do not
  constitute the full artifact-store CLI/TUI/Web/automatic housekeeping contract.
- Pealayer #27 requires contextual identity and OS-discovered ports, never runtime
  examples. Pealayer #28's shaping discussion separates font, bidi, translation
  and pixels. Rayan BidiLtr and Persian-calendar date functions now serve actual
  Pakar Web protocol/NTP consumers; VM acceptance is not browser/Android/native
  screenshot acceptance.

## Snapshot coverage and unfinished review

The first complete enumeration fetched 792 records: Pakar 11 issues/5 PRs,
Pealayer 31 issues/111 PRs, PCController 226 issues/408 PRs. Standalone comments:
120/623/2534; reviews: 7/40/242; threads: 6/36/106. Unresolved thread counts:
0/2/2. Full bodies are in the stable local snapshot; fetching is not semantic
review of all 792 records. The decisions file identifies the exact reviewed
records/comment scope. Remaining records stay pending.

RayanLamp's verified source is a local filesystem donor without Git metadata.
The referenced Build RayanLamp Go app chat was read. Its last white-border and
settings-geometry work was interrupted, not accepted. No GitHub repository has
been verified; the user was asked for its URL. Never substitute rayanpbx or invent
a repository. Local Rayan source importing continues independently.

Source/CI/package/live proof stay separate. No installed desktop/Android
replacement, board flash, donor-repository edits or physical acceptance occurred.

## Current-source continuation

The newer complete enumeration contains 794 records: Pakar17, Pealayer143,
PCController634. Every comment/review/thread page was collected, but most full
records remain pending semantic review. The explicit decisions now include
Pealayer56/142/143 and PCController53, in addition to the earlier reviewed scopes.
Full conversation-record review is not a PR source-diff or merge-readiness review.

Learned constraints applied to pending Pakar acceptance:

- Pealayer56: one dialog state/renderer across hosts, shared actions and keyboard
  semantics, fallback without losing drafts/preferences. Complete generic native
  host migration remains pending, not another host-specific copy of controls.
- PCController53: authoritative controller enumeration and HELLO, not an incomplete
  secondary WMI inventory. Current OS-discovered port controls are retained;
  SetupAPI identity enrichment and provider-disagreement tests remain pending.
- Pealayer142: no implicit recorder all-selection, no optimistic publish/run,
  cancel-safe previews and retention of the last valid catalog. Preserve complete
  operation timing in saved effects before changing repeat semantics. Timeline
  and media-seat cue business stays excluded; generic effect/recording contracts
  are useful unfinished work, not excluded wholesale with the timeline.
- Pealayer143: prior ACK/build/screenshot is not current physical readiness.
  Actual Pakar a6f13a0a desktop production was replaced and captured, preserving
  settings/rollback; observed UI defects stay open and no board was online.

Fresh Pakar PR listing is empty and its sole local/remote branch is main. No
completed Copilot review was observed in the refreshed Pakar conversations.
Requested review is not acceptance, and unrelated donor threads remain unresolved.
The glib security update is tracked separately on issue17; it is not fixed by
merging the previous governance/import PRs. CI is checked asynchronously, never
used as a blocking wait, and failed mapping evidence is repaired without bypass.

The latest manual conversation projection now has21 full records and2 explicit
comment scopes reviewed;771 of794 records remain pending. Pealayer70/78/87/102
were read completely, with concrete branding/lifecycle/button constraints in
github-conversation-decisions.json. Reread cached records is not a live donor
refresh. Pakar was refreshed live and still has zero open PRs; no completed
Copilot response was observed. Both0c3eca9 CI workflows completed successfully.
That release was deployed and a real framebuffer inspected, but full UI parity
remains unproven. Its minimized queue defect now has an actual source adaptation
and129 passing native tests; source app.rs stays pending complete-file review.
