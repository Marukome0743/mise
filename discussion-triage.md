# Discussion triage

This file is the working queue for GitHub Discussion research. It lives only on
the `triage` branch and is not intended to be merged into `main`.

## Workflow

- Keep only discussions that still need a reply, verification, design decision,
  or implementation.
- Record evidence from the current `upstream/main`, related pull requests, and
  practical reproduction where applicable.
- Remove an entry after its pending action is complete: a reply is posted, the
  corresponding fix is merged and reported, or a deliberate no-action decision
  is made.
- Recheck the discussion and open pull requests before starting implementation
  or posting a reply.
- Skip Windows-only implementation decisions until they can be verified on a
  Windows machine.
- Treat vfox plugins as out of scope. They are vendored from upstream repos,
  `.github/workflows/vendored-file-warning.yml` blocks non-maintainer PRs
  against `crates/vfox/embedded-plugins/**`, and this effort does not work
  on that surface. Record such reports as closed out rather than queuing
  them.
- Keep completed range audits below so a finished number band is not scanned
  again without a new report, regression, or maintainer request.
- Two e2e failures on a local macOS run are environmental, not regressions.
  `e2e/cli/test_prune_tool_stub` uses GNU `touch -d '2001-01-01'`, which BSD
  `touch` rejects. `e2e/cli/test_upgrade` failed because `e2e/run_test`
  forwarded the token variables unconditionally, turning "unset" into "set but
  empty". mise's own resolution is correct — `get_token` (`src/env.rs:1105`)
  and `github::resolve_token` (`src/github.rs:681`) both reject empty values —
  but the `ubi` crate reads `GITHUB_TOKEN` itself with no such check
  (`ubi-0.10.0/src/forge.rs:244`), sends an empty bearer, and GitHub answers
  401. An empty `MISE_GITHUB_TOKEN` also hides a real `GITHUB_TOKEN`, because
  `get_token` takes the first variable that *exists* and stops there, which is
  the behavior `test_token_overwrite` pins down. Fixed on branch
  `e2e-token-forwarding` (`a8eecda5772d`): forward each variable only when it
  holds something. CI was never affected — `test.yml` and `registry.yml` always
  supply non-empty values. Correction: an earlier note here blamed mise's token
  handling; that was wrong.

## Completed range audits

### #3000–#5000 — non-Windows audit complete

- Status: complete; do not routinely rescan this range.
- Audited: 2026-08-14.
- Scope: all 654 Discussions whose actual Discussion number is between #3000
  and #5000. At audit time, 138 had an accepted answer, 160 were closed, and
  408 were open without an accepted answer. Windows-only reports were left for
  verification on a Windows machine.
- Method: reviewed the full title/category inventory, prior triage history and
  posted replies, then inspected the body and reply history of remaining
  non-Windows bug/feature candidates. Plausible reports were compared with
  current `upstream/main`, merged and open pull requests, current docs, and
  local reproduction where the result was not already established.
- Result: no additional high-confidence implementation or reply candidate was
  found. Previously actionable reports from this range have already been
  fixed, replied to, deliberately declined, or are represented by an open PR.
  Remaining open threads are support questions, historical tool/backend
  failures, external-project issues, or product ideas without enough demand or
  a settled design to justify implementation.
- Reopen rule: revisit an individual Discussion only if it receives a new
  reproducible report on current mise, a maintainer asks for follow-up, or a
  related regression is discovered. Do not restart a broad #3000–#5000 scan.

### #5000–#6000 — non-Windows audit complete

- Status: complete; do not routinely rescan this range.
- Audited: 2026-08-14.
- Scope: all 424 Discussions whose actual Discussion number is between #5000
  and #6000. At audit time, 107 had an accepted answer, 96 were closed, and
  249 were open without an accepted answer. Windows-only reports were left for
  verification on a Windows machine.
- Method: reviewed the full title/category inventory and the extensive prior
  investigation, implementation, and reply history for this range. Remaining
  non-Windows bug and feature candidates were checked against their full
  threads, current `upstream/main`, merged and open pull requests, current
  documentation, and focused local verification where needed.
- Result: no additional high-confidence implementation candidate was found.
  The two resolved but unanswered Discussions found by the audit (#5206 and
  #5777) were verified and replied to on 2026-08-14. Other plausible reports
  were already fixed or answered, intentionally declined, backend or
  external-project limitations, support questions, or product ideas without
  enough demand or a settled design.
- Reopen rule: revisit an individual Discussion only for a new reproducible
  report on current mise, a maintainer request, or a related regression. Do
  not restart a broad #5000–#6000 scan.

### #6000–#7000 — non-Windows audit complete

- Status: complete; do not routinely rescan this range.
- Audited: 2026-08-14.
- Scope: all 399 Discussions whose actual Discussion number is between #6000
  and #7000. At audit time, 101 had an accepted answer, 89 were closed, and
  251 were open without an accepted answer. Windows-only reports were left for
  verification on a Windows machine.
- Method: reviewed the complete title/category inventory and the extensive
  prior implementation and reply history for this range. Remaining plausible
  non-Windows reports were checked against their full threads, current
  `upstream/main`, merged pull requests, documentation, and focused offline
  reproduction or regression tests where the result was not already clear.
- Result: no pending non-Windows item remains. Other open threads are already
  answered, external backend/tool issues, intentional behavior, Windows-only
  reports, deliberately declined changes, or product ideas without enough
  demand or a settled design.
- Reopen rule: revisit an individual Discussion only for a new current-version
  reproduction, a maintainer request, or a related regression. Do not restart a
  broad #6000–#7000 scan.

### #7000–#8000 — non-Windows audit complete

- Status: complete; do not routinely rescan this range.
- Audited: 2026-08-15.
- Scope: all 222 Discussions whose actual Discussion number is between #7000
  and #8000. Windows-only reports were left for verification on a Windows
  machine.
- Method: reviewed the complete title/category inventory, accepted-answer and
  reply state (including inline reply threads), and prior implementation
  history. Plausible non-Windows reports were checked against current
  `upstream/main`, merged and open pull requests, current documentation, and
  focused source or local verification where the result was not already clear.
- Result: macOS fish verification is complete. #7217 was implemented in
  PR #12025, but the maintainer declined the dynamic-command design because of
  its task/completion complexity, repeated resolution cost, and lack of a
  persistent-cache model. #7168 is reproducible but is a PATH-precedence design
  question rather than a clear implementation bug; the verified ordering and
  tradeoff were reported to the Discussion on 2026-08-22. The previously
  queued resolved Discussions have also been replied to. Other open threads
  are already answered, external service/tool/backend failures, expected CLI
  semantics, Windows-only reports, or product ideas without enough demand or
  a settled design to justify implementation.
- Reopen rule: revisit an individual Discussion only for a new current-version
  reproduction, a maintainer request, or a related regression. Do not restart
  a broad #7000–#8000 scan.

### #8000–#9000 — non-Windows audit complete

- Status: second pass complete; do not routinely rescan this range. The
  verified follow-up items are listed under Pending below.
- Audited: 2026-08-15; second pass 2026-08-20.
- Scope: all 395 Discussions whose actual Discussion number is between #8000
  and #8999. At second-pass time 77 had an accepted answer, 47 were closed
  without one, and 271 were open without an accepted answer; 46 of those
  already carried a reply from this triage effort. The remaining 110
  open-unanswered bug reports split into 80 non-Windows and 30 Windows-only
  reports. Windows-only reports were left for verification on a Windows
  machine.
- Method: refetched the complete inventory (title, state, category, answered
  flag, and every top-level and inline comment) through the GraphQL API, then
  read each open-unanswered non-Windows bug report that did not already carry a
  reply from this effort. Every candidate was checked against current
  `upstream/main` (619854b468dd, 2026-08-19), its referenced pull requests, and
  current documentation; each item recorded below was additionally reproduced
  or refuted locally with a debug binary built from that revision in an
  isolated `HOME` / `MISE_*_DIR` sandbox.
- Result: the range is largely resolved. Verified fixed on current `main` and
  needing no further work: #8092, #8272, #8280, #8411, #8418, #8478, #8546,
  #8609, #8638, #8726, #8739, #8751, #8834, #8883, #8940 (original activation
  scenario), #8987, and #8090. Closed by merged pull requests: #8093 (#8094),
  #8271 (#9609), #8296 (#8342), #8389 (#8397), #8444 (#8823), #8465 (#8468),
  #8530 (#8532), #8548 (#8589), #8597 (#8686), #8603 (#8716), #8613 (#8616),
  #8615 (#8618), #8632 (#11231), #8677 (#9739, #9741), #8783 (#8798),
  #8804 (#10468), and #8951 (#8952). Deliberately declined or out of scope:
  #8224 and #8235 (jdx: querying `rustup show` on every resolve is too
  expensive), #8312 (auto-install during shim execution is an intentional
  design decision), #8600 (jdx: npm packages with native modules are not a mise
  bug), #8070 (implemented in PR #12038 but backend-wide Go compiler tracking
  was declined), and #8528, #8573, #8643, #8806, #8870 (external project,
  expected behavior, or already answered). #8652 and #8298 were investigated
  in full and then deliberately dropped — see the notes below. The verified
  reply candidates #8255 and #8021/#8381 were answered on 2026-08-22. What
  remains actionable — one implementation candidate, a set of small
  documentation gaps, and a lower-confidence queue — is recorded under Pending
  below.
- Closed out — #8298 (embedded vfox-poetry calls `mise` from its own EnvKeys
  hook): out of scope by decision — vfox plugins are not a surface this effort
  works on. Do not reopen. The defect is real and unfixed:
  `hooks/env_keys.lua:8` runs `io.popen("mise which python3")` inside
  `PLUGIN:EnvKeys`, which is the recursion jdx identified in the thread. It
  cannot be fixed in this repo in any case:
  `.github/workflows/vendored-file-warning.yml` fails any PR from a
  non-maintainer that touches `crates/vfox/embedded-plugins/**`, the vendored
  copy is byte-identical to `mise-plugins/vfox-poetry` and is only ever
  refreshed by release commits, so the change would have to go upstream and
  then wait to be re-vendored. A fix there would also have to preserve what
  upstream PR #1 ("fix: use mise Python when creating virtualenvs", merged
  2025-12-19) was for — that PR is what introduced this `mise` call, so simply
  deleting it regresses poetry back to picking up the system Python.
- Closed out — #8652 (`prefer_offline` never expires the remote-version cache):
  reproducible but deliberately not pursued. Do not reopen without a new
  maintainer request. The behavior is real: with `prefer_offline = true`,
  `Settings::fetch_remote_versions_cache()` returns `None` and
  `CacheManager::is_fresh()` treats a `None` window as always fresh, so a
  version list backdated to 2020 is still served by `ls-remote`, `outdated`,
  and `upgrade --dry-run`; `MISE_PREFER_OFFLINE=0` refreshes it immediately and
  is the workaround jdx himself gave in #11185. It was dropped because the cost
  of a correct fix outweighs the benefit here: jdx ruled "working as expected"
  in the thread, the `prefer_offline` setting docs describe exactly this
  behavior, and he considers the setting a bad idea for most users. A safe fix
  is also not the one-line change it first appears to be — `CacheManager` has
  no fallback to a stale entry when a refetch fails, and an empty listing
  clears the cache file, so simply honoring the freshness window for
  remote-fetch commands would break `upgrade` for the offline users the setting
  exists to serve. Arguments that would justify revisiting, if a maintainer ever
  asks: the `fetch_remote_versions_cache` docs in `settings.toml` name
  `mise ls-remote` as a "slow" command that should use the hourly window, which
  contradicts the observed behavior, and PR #11190 already exempted `lock`,
  `ls-remote`, `outdated`, and `upgrade` from `prefer_offline` on the timeout
  axis.
- Reopen rule: revisit an individual Discussion only for a new current-version
  reproduction, a maintainer request, or a related regression. Do not restart a
  broad #8000–#9000 scan.

### #9000–#10000 — non-Windows audit complete

- Status: second pass complete; do not routinely rescan this range. The
  verified follow-up items are listed under Pending below.
- Audited: 2026-08-16; second pass 2026-08-22.
- Scope: all 231 Discussions whose actual Discussion number is between #9000
  and #9999. At second-pass time 50 had an accepted answer, 34 were closed
  without one, and 147 were open without an accepted answer; 40 of those
  already carried a reply from this effort. The remaining 46 unreplied bug
  reports split into 32 non-Windows and 14 Windows-only. Windows-only reports
  were left for verification on a Windows machine.
- Method: refetched the complete inventory (title, state, category, answered
  flag, and every top-level and inline comment) through the GraphQL API, then
  read all 32 open-unanswered non-Windows bug reports that did not already
  carry a reply from this effort. Referenced pull requests were checked for
  merge state, and candidates were compared against current `upstream/main`.
- Result: most of the range closed itself while the earlier passes were in
  flight. Answered by maintainers and needing nothing further: #9085, #9130,
  #9317, #9336, #9359, #9602, #9778, #9821, #9857, #9952. Closed by merged pull
  requests: #9730 (#9737), #9754 (#9765), #9797 (#9816 — note #9980, which
  covered the remaining `latest`-during-activation case, was closed), #9869
  (#9893), #9934 (#9977), #9978 (#10114). Already fixed on `main` without a
  linked PR: #9693 (jdx confirmed the reproducer passes), #9324 (the reporter
  traced it to a regression fixed in v2026.4.19), and #9692 — the `pi` registry
  entry now reads `aqua:earendil-works/pi` / `github:earendil-works/pi` /
  `npm:@earendil-works/pi-coding-agent`, which is exactly the requested change;
  verified against npm, where the old `@mariozechner/pi-coding-agent` carries a
  deprecation notice and the new package is current.
- Reopen rule: revisit an individual Discussion only for a new current-version
  reproduction, a maintainer request, or a related regression. Do not restart a
  broad #9000–#10000 scan.

### #10000–#11000 — non-Windows audit complete

- Status: second pass complete; do not routinely rescan this range. The
  verified follow-up items are listed under Pending below.
- Audited: 2026-08-22.
- Scope: all 268 Discussions whose actual Discussion number is between #10000
  and #10999 (created 2026-05-20 to 2026-07-14). 76 had an accepted answer, 38
  were closed without one, and 154 were open without an accepted answer; only 18
  of those already carried a reply from this effort, so this band was far less
  covered than #8000–#10000. The 64 unreplied bug reports split into 52
  non-Windows and 12 Windows-only.
- Method: refetched the inventory and every top-level and inline comment through
  the GraphQL API, read all 52 open-unanswered non-Windows bug reports, checked
  the referenced pull requests for merge state, and verified selected items
  against current `upstream/main` locally.
- Headline finding: **this range is already heavily attended by the
  maintainers.** jdx or risu729 have opened fixes for a large share of it —
  #10003 (#10102), #10016 (#10111), #10033 (#10103), #10080 (#10081), #10126
  (#10706), #10141 (#10142), #10177, #10198 (mise-versions), #10284 (#10341
  plus mise-versions #246/#247), #10303 (#10310, #10344), #10407 (#10410),
  #10487 (#10494), #10508 (#10511), #10625 (#10626), #10664 (#10703), #10668
  (#10697, #10774), #10721 (fixed on main), #10763 (#10799), #10772
  (mise-versions #254), #10809 (#10810) — and jdx is actively working #10507 and
  #10528. Answered without code: #10352, #10486, #10537, #10540, #10643. The
  useful contribution here is therefore narrow, and mostly in the unattended
  corners.
- Verified locally this pass:
  - **#10662 does not reproduce.** The reported double-panic → SIGABRT needs
    `check_working_directory` to `eprintln!` into a broken stderr; it now uses
    `warn!`, which routes through the logger's `safe_eprintln!`
    (`src/logger.rs:97`) and discards write failures. With the cwd deleted and
    stderr an unwritable pipe, mise exits 0. The harness was validated: a Python
    control under the same fds exits 120 on `BrokenPipeError`.
  - **#10437 is fixed.** `routed_tuf_url()` in `src/github/sigstore.rs` applies
    `url_replacements` to the sigstore TUF endpoint, with a unit test
    (`test_routed_tuf_url_applies_url_replacement`) pinning it.
  - **#10762 is blocked upstream.** jdx agreed mise should use full Racket "if
    it is available" — it is not: `aquaproj/aqua-registry` `pkgs/racket/racket/`
    contains only `minimal`. The path forward is adding the package to
    aqua-registry first, or moving the entry to another backend.
  - **#10565 is inconclusive.** `mise run` completion is backed by
    `mise tasks ls --complete`; on a small project that takes ~28 ms against a
    ~20 ms baseline, no worse than `mise ls -i` (~33 ms). The reporter measured
    the fish + `usage` CLI path (`usage complete-word`), which is not installed
    here, so their 1.68 s figure is neither confirmed nor refuted.
- Second pass (2026-08-22): re-fetched the range — nothing had changed state —
  then read the 74 open-unanswered items in the categories the first pass did
  not cover (Ideas 51, Q&A 12, General 8, Show and tell 3; only 2 already
  carried a reply from this effort). As in the earlier bands, these are
  overwhelmingly product ideas without maintainer signal; the top-upvoted are
  #10582 (5), #10782 (4), and #10732 / #10207 / #10065 (3 each), none of them
  implementation-ready. Three were bugs filed outside the bug-report category,
  and two of those are already resolved:
  - **#10957 is fixed.** The reporter saw `ls-remote pnpm` interleave 10.x after
    11.x because the list followed upstream order. `version_order` is now a real
    tool option accepting `"source"` or `"semver"`
    (`src/backend/options.rs:24`), 963 registry entries set it (807 semver, 156
    source), and `registry/pnpm.toml` itself carries `version_order = "semver"`.
    Verified: `mise ls-remote pnpm` now ends in a clean ascending run and
    `mise latest pnpm` returns 11.22.0. Worth noting the option landed even
    though jdx replied "I do not think it could work" when risu729 proposed it.
  - **#10526's requested default is implemented.** jdx proposed showing the
    untrusted-config warning once on entering a directory and staying quiet
    afterwards. Verified: the first `hook-env` in an untrusted directory emits
    exactly one "not trusted" line and exports
    `__MISE_LAST_UNTRUSTED_CONFIG_WARNING_KEY`; a second run carrying that
    marker emits none. The reporter's complaint was four ERROR lines on every
    prompt.
  - #10174 (nushell `activate` setting `PATH` rather than modifying it, so a
    saved autoload file goes stale) is a real design question with no maintainer
    signal yet, and stays in the idea queue.
  Also of note: in #10759 jdx split the stale-action report into #10803 /
  #10804 / #10805 and said of the snapcore actions "we can just ignore that",
  which independently corroborates the #9239 reply.
- Reopen rule: revisit an individual Discussion only for a new current-version
  reproduction, a maintainer request, or a related regression. Do not restart a
  broad #10000–#11000 scan.

## Pending

### #8735 — documented `_.source` cacheable example does not parse

- URL: https://github.com/jdx/mise/discussions/8735
- Status: Draft PR #12278 is open with the one-line documentation correction.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification: `docs/cache-behavior.md:60` documents
  `_.source = { file = "dynamic.sh", cacheable = false }`. Copying that line
  into a `mise.toml` fails with `data did not match any variant of untagged
  enum MiseTomlEnvDirectiveValue`. The directive object in
  `src/config/config_file/mise_toml.rs` accepts `path` (with deprecated aliases
  `value`, `values`, `paths`) and has no `file` key, so
  `_.source = { path = "dynamic.sh", cacheable = false }` works and exports the
  sourced variables. `_.file = { path = ".env", cacheable = false }` works too.
- Implementation: PR #12278 changes only the example in
  `docs/cache-behavior.md` from `file` to the supported `path` field. It does
  not add a new alias to the directive schema.
- Next action: wait for #12278 to merge, then post the resolution to the
  Discussion and remove this entry.

### #8940 — trust prompt blocks forever when stdin reaches EOF

- URL: https://github.com/jdx/mise/discussions/8940
- Status: Ready PR #12268 is open for the confirmed remaining defect. Draft PR
  #12273 is a maintainer-requested confirmation API refactor stacked on top of
  #12268.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification: `mise hook-env -s zsh` in an untrusted directory now prints a
  "not trusted, run mise trust to enable it" warning and exits cleanly, so
  shell activation no longer prompts — the reported unrecoverable
  terminal state does not reproduce. However, running `mise env` in an
  untrusted directory under a pty whose stdin immediately reaches EOF (a
  `pty.fork()` harness that writes `\x04`, equivalent to `docker run -t`
  without `-i`, or a `script`-wrapped CI step) leaves mise blocked on the
  "Trust them?" confirm prompt indefinitely — killed after 12s with no exit and
  no further output. With plain pipes (no tty) mise correctly errors with
  "Config files ... are not trusted."
- Implementation: PR #12268 requires both stdin and stderr to be terminals
  before opening the dialog, handles EOF fail-closed, avoids persisting an
  ignore marker when no answer could be read, restores the cursor on prompt
  errors, and adds PTY regression coverage. PR #12273 introduces the tri-state
  confirmation API requested during review; it is a structural follow-up, not
  an additional Discussion fix.
- Next action: wait for #12268 to merge, then post the resolution to the
  Discussion and remove this entry. Rebase and undraft stacked PR #12273 after
  #12268 lands.

### #8000–#8999 — small documentation and message gaps

- #8586: **written** on branch `docs-lockfile-backends` (`b2907744879a`). The
  `--locked` URL check in `src/backend/mod.rs:3221` is guarded by
  `supports_lockfile_url()`, so backends that cannot record a URL are skipped
  rather than failed, and tool stubs are skipped too. The "Strict Lockfile
  Mode" section said only that install "will fail if a tool doesn't have a
  URL". Corrections to an earlier note here: the backends returning `false` are
  `asdf`, `cargo`, `gem`, `go`, `npm`, `pipx`, `ubi`, `core:dotnet`,
  `core:rust`, and `core:swift` — `pkgx` returns `true`, the `dotnet:` backend
  uses the default `true` (only the core tool opts out), and vfox opts out only
  for backend plugins (`!self.is_backend_plugin()`).
- #8797: **written** on branch `docs-raw-serializes` (`25446036f163`). The
  mechanism is not what an earlier note here claimed:
  `TaskOutputHandler::jobs()` returns `1` only for the `--raw` CLI flag. What
  serializes everything else is `RAW_LOCK` in `src/cmd.rs:992` — a raw command
  takes the write side for its whole duration while non-raw commands take the
  read side, so nothing runs beside a raw command. The setting, `MISE_RAW`, and
  per-task `raw = true` all reach it (`src/task/task_executor.rs:1563`).
  Documented on the `raw` setting as the reporter asked. The same commit
  corrects `docs/tasks/task-configuration.md`, which claimed raw "screws up the
  output whenever mise runs tasks in parallel" and floated a future `single =
  true` property — the overlap it warns about cannot happen, though the lock is
  per command rather than per task, so that ticket invitation was kept.
- #8232: the `$`-expansion warning now names the missing variable
  (`env var 'X' is not defined and will be left unexpanded`), which is already
  better than what the reporter saw, but it still does not say which `[env]`
  key or which config file contains the value. Adding both would close the
  report; low priority.

### #8000–#8999 — unresolved, lower-confidence follow-up queue

These were read and left unresolved: each needs an environment this audit could
not reproduce, or a design decision, so none is implementation-ready.

- #8261 (`go:` installs failing with `compile: version "go1.26.0" does not
  match go tool version`), #8336 (conda packages installing as broken symlinks
  when dependencies are missing), #8629 (monorepo task cannot find `pnpm`
  installed through an idiomatic version file), #8650 (stale
  python-build-standalone `_sysconfigdata` paths, which `uv` patches), and
  #8953 (intermittent CI freeze after a task completes) all need a specific
  toolchain, package, or CI environment to reproduce.
- #8269 (`mise watch --clear=reset --restart` leaves the terminal without echo
  after Ctrl+C) is plausible and was confirmed by a second reporter in March,
  but reproducing it needs a pty harness around `watchexec`.
- #8622 (`url_replacements` breaking aqua checksum resolution) depends on an
  Artifactory-style mirror; the reported failure may be mirror configuration
  rather than a mise defect.
- #8551 (PPA metadata advertised for pre-26.04 Ubuntu with no binaries),
  #8827 (`self-update` failing to update SSH-hosted private plugins), and
  #8924 (`vp` registered as `npm:vite-plus` although upstream ships a split
  npm + native distribution) are packaging or registry decisions rather than
  code fixes.

### #9070 — usage values in task `env._.file` and `env._.source`

- URL: https://github.com/jdx/mise/discussions/9070
- Status: reproducible feature gap, but implementation is deferred pending a
  maintainer-approved argument-dependent rendering model.
- Confidence: high on both the gap and the current design constraint.
- Last checked: 2026-08-16.
- Verification: a task with `_.file = "{{usage.profile}}.env"` fails before
  execution with `Variable usage is not defined`, for both the default and an
  explicit profile. `prepare_task_context()` renders task env before
  `parse_task_usage()` builds the usage context. Discussion #6767 independently
  requests the same capability, so the use case is not isolated.
- Design constraint: PR #11823 implemented the adjacent and smaller feature of
  argument-dependent task sources/outputs, but jdx closed it with “not ready to
  support this”. Dynamic env files/sources would require the same occurrence
  preflight/runtime split and additionally affect environment construction.
- Next action: do not implement this independently. Revisit only if the
  maintainer accepts an argument-dependent task metadata model or asks for a
  narrower design.

### #9926 — SOPS-encrypted dotenv files in `env._.file`

- URL: https://github.com/jdx/mise/discussions/9926
- Status: confirmed implementation gap; external-CLI support is feasible, but
  built-in rops support is blocked on a format/design decision.
- Confidence: high.
- Last checked: 2026-08-16.
- Evidence: JSON/YAML/TOML paths inspect the parsed `sops` metadata and call the
  shared decryptor; the dotenv path passes raw file bytes directly to
  `dotenvy`, so a SOPS dotenv document cannot reach either rops or the external
  CLI. A real age-encrypted dotenv fixture fails in both mise modes before
  decryption, while `sops decrypt --input-type dotenv --output-type dotenv`
  succeeds directly.
- Boundary: external-SOPS support can be added in mise by detecting SOPS dotenv
  metadata and decrypting before `dotenvy`. rops 0.1.7 has no dotenv
  `FileFormat`; upstream issue gibbz00/rops#99 remains open and describes the
  missing flattened-dotenv representation as non-trivial.
- Next action: decide whether an external-CLI-only first step is acceptable. If
  built-in parity is required, implement or wait for dotenv support in rops
  rather than adding a second SOPS document model inside mise.

### #9045 — `mise prune --dry-run` does not say why a version is prunable

- URL: https://github.com/jdx/mise/discussions/9045
- Status: implemented on branch `prune-explain-dry-run` (`a24ea849b035`, off
  `upstream/main` at `6f52dcdf99e2`). No pull request yet — CI on main is red
  for unrelated reasons, so the branch is pushed and held.
- Confidence: high.
- Last checked: 2026-08-22.
- Report: a JDK that is not named by any config is proposed for removal on every
  `mise prune`, and the user has no way to see why.
- jdx in the thread: "do we have a --dry-run for prune that shows config
  locations? if not we should include that".
- Why nothing pointed at a cause: pruning decides by *absence* — a version goes
  because nothing among the tracked configs and stubs resolved to it — so there
  is no file to name as the reason. `delete()` printed only
  `<tool>@<version> [dryrun]`.
- Implementation: report the other side. `--dry-run` now names, per prunable
  version, either the versions of the same tool that were kept and the files
  keeping them, or the fact that nothing tracked mentions the tool at all:
  `java@21.0.1 is prunable: java is required at 17.0.9 by ~/b/mise.toml` and
  `jdk@1.8.0 is prunable: no tracked config or tool stub requires jdk`. To
  carry the paths, `get_versions_needed_by_tracked_configs` and
  `get_versions_needed_by_tracked_stubs` return a map from
  (short name, version) to the files needing it instead of a bare set;
  `upgrade` only asks whether a key is present, so it is unchanged. The
  explanation is confined to the dry-run path — a real prune and `mise unuse`
  print what they did before. `prunable_tools` was kept as a thin wrapper so
  `mise ls --prunable` is untouched, which the existing comment at
  `src/cli/ls.rs:270` explicitly warns about.
- Verified: clippy clean with no exclusions, `cargo fmt --check` clean, the
  toolset (98) and prune/upgrade (39) unit test selections, and four prune e2e
  tests including a new `e2e/cli/test_prune_dry_run_explains` covering both
  branches and confirming a real prune stays silent.
- Next action: open the PR once main's CI is green, then post the resolution to
  the Discussion and remove this entry.

### #9477 — orphaned `http-tarballs` survive every cleanup command

- URL: https://github.com/jdx/mise/discussions/9477
- Status: reproduced on current `main`, and broader than reported.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification (macOS arm64, `main` at 6f52dcdf99e2): a user-defined
  `[tools."http:…"]` with a `url` extracts into
  `$MISE_DATA_DIR/http-tarballs/<hash>` (828K for a single small binary).
  After `mise uninstall`, the install directory is empty but the tarball entry
  remains — and it still remains after `mise prune`, after `mise cache prune`
  ("pruned 0 files, 0 B"), and after `mise cache clear`. Only manual deletion
  removes it.
- Why even `cache clear` misses it: the directory is
  `dirs::DATA.join("http-tarballs")` (`src/backend/http.rs:263`) — it lives in
  the data dir, so the cache commands never look at it. The report frames this
  as prune not covering the entries; in fact no command covers them.
- Scope correction for #9707: the premise that "the installs are just symlinks
  to these folders" no longer holds for new installs. Registry `http:` tools
  with an explicit install path skip the shared cache entirely
  (`tv.install_path_is_explicit`, `src/backend/http.rs:1208`), and
  `remove_install_path` carries migration code for installs created by older
  versions "from a cache symlink to a real directory" (`:271`). Re-check #9707
  against that before treating it as current.
- Design constraint: jdx noted the entries are excluded because tool-stubs make
  it unknowable whether one is still referenced. Any fix has to answer that
  first — but the fact that `cache clear` leaves data behind is worth raising
  regardless.

### #9482 — a task interrupted without mise seeing the signal reads as failure

- URL: https://github.com/jdx/mise/discussions/9482
- Status: reproduced, with a sharper boundary than the report describes.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification (three shapes, same task runner):
  - `SIGINT` to the whole process group, which is what a terminal Ctrl-C does:
    mise exits **130 with no error output**. Correct, and not what the reporter
    hit.
  - A task that exits 130 on its own with no signal anywhere:
    `[task] ERROR task failed`, mise exits 130. So the exit *code* alone is
    treated as failure.
  - `SIGINT` delivered only to the child, so mise never registers an interrupt:
    `ERROR sh exited with non-zero status: no exit status` followed by
    `ERROR task failed`, mise exits 1. This reproduces the reporter's message
    verbatim.
- Reading: mise does correlate its own interrupt (`Error::TaskInterrupted`
  covers "interrupted before process start", and the group case is handled), so
  the gap is a child that dies from a signal mise did not see — a tool that
  re-raises, or one that puts itself in its own process group.
- Next action: decide whether a child killed by `SIGINT` should be reported as
  a failure when mise itself was not interrupted. Treating exit 130 as clean
  unconditionally is the wrong fix — 130 is a legitimate failure code — so this
  needs a maintainer decision rather than a patch.

### #9324 / #9826 — an empty install directory still reports success

- URLs: https://github.com/jdx/mise/discussions/9324,
  https://github.com/jdx/mise/discussions/9826
- Status: reproduced on current `main`.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification: install `jq`, then empty its install directory while leaving the
  directory in place (what an interrupted download leaves behind). `mise
  install` then prints **"all tools are installed"** and exits 0, and `mise ls`
  lists `jq 1.8.2` as installed.
- Partly better than reported: `mise which jq` fails with an actionable message
  — "No executable found for configured tool: jq … Reinstall it with: mise
  install --force jq@1.8.2" — and `mise install --force` does recover it.
- Still missing: `mise doctor` does not identify the broken install. With the
  directory empty it reported only "unused shims are present … Unused shims:
  jq", which is a symptom rather than the cause. #9324's complaint was
  precisely that doctor said the system was clean.
- Next action: have the install path (or `doctor`) notice an install directory
  with no executables instead of trusting its presence.

### #9460 — not reproducible

- URL: https://github.com/jdx/mise/discussions/9460
- Status: does not reproduce on current `main`; no action.
- Last checked: 2026-08-22.
- Verification: ran the reporter's exact `#USAGE` probe task with
  `usage_model=LEAKED` exported in the parent process and the `--model` flag
  omitted. The task saw `usage_model=UNSET`. Passing the flag explicitly still
  works (`usage_model=real`). The stale-inheritance the report describes is
  gone.

### #9462 — not reproducible

- URL: https://github.com/jdx/mise/discussions/9462
- Status: neither the cause nor the crash reproduces on current `main`; no
  action.
- Last checked: 2026-08-22.
- Cause is fixed: the reporter traced their `shims_on_path: no` to
  `/Users/Olfway` versus `/Users/olfway`. `file::paths_eq`
  (`src/file.rs:758`) now lowercases each path component on macOS and Windows
  before comparing, and `shims_on_path()` (`src/cli/doctor/mod.rs:1060`) goes
  through it. Verified on a case-insensitive volume: with the mise env vars
  spelled one way and the `PATH` entry spelled the other — in both directions —
  `mise doctor` reports `shims_on_path: yes`. A control with the shims genuinely
  absent still reports `no`, so the check is not simply always true.
- The crash does not return even when the detection is forced to fail: reaching
  the shims through a symlinked alias path makes `shims_on_path: no` while the
  shims are still executable from `PATH`. Under that state, `mise x -- npm -v`,
  an `npm:` backend install with `npm.package_manager = "npm"` so mise shells
  out to the real `npm`, and the reporter's own `mise up --bump` all completed
  with exit 0 and a peak of at most three processes in the group. A runaway
  guard would have killed anything past 40.
- Limitation: this reproduces the reported *condition*, not the reporter's whole
  environment (their config also drove pipx, github and aube-backed tools).
  Reopen if someone hits it again with a current version.

### #9642 — `oc = "stable"` builds a URL out of the alias and 404s

- URL: https://github.com/jdx/mise/discussions/9642
- Status: reproduces, but the failure has moved since the report.
- Confidence: high.
- Last checked: 2026-08-22.
- What changed: `registry/oc.toml` now lists `http:oc` first, ahead of
  `conda:openshift-cli`, so the conda solve the report shows is no longer
  reached. The reporter's exact config (`helm = "4"`, `oc = "stable"`) still
  fails, with a different error.
- Verification (macOS arm64): `mise install` reports
  `Failed to install http:oc@stable: HTTP status client error (404 Not Found)`
  for `.../ocp/stable/openshift-client-mac-arm64-stable.tar.gz`. The alias is
  being substituted into `{{ version }}` in the filename.
- The mirror layout says the URL is simply wrong for an alias: the `stable/`
  directory exists (HTTP 200) and holds both
  `openshift-client-mac-arm64-4.22.9.tar.gz` and an unversioned
  `openshift-client-mac-arm64.tar.gz`. Nothing named `*-stable.tar.gz` exists.
- Why `stable` reaches the URL at all: `version_regex` in the entry scrapes only
  `\d+\.\d+\.\d+` directories, so `stable` never appears in `mise ls-remote
  oc` and is passed through as a literal version rather than rejected.
- Isolated to the alias: `oc = "4.22.9"` installs cleanly, `oc = "latest"`
  resolves, and `ls-remote` lists 4.22.9 / 4.22.10 / 4.22.11.
- Next action: decide between teaching the entry to map the alias directories to
  their unversioned filenames and rejecting versions absent from the listing
  with a clear message. Either is small; a 404 naming a URL the user never typed
  is the worst of the options.

### #9239 — not actionable on mise's side

- URL: https://github.com/jdx/mise/discussions/9239
- Status: the pin is real, but there is nothing newer to move to; no action.
- Last checked: 2026-08-22.
- Verification: `snapcore/action-publish` is still pinned at
  `214b86e5ca036ead1668c79afb81e550e6c54d40 # v1.2.0`
  (`.github/workflows/snapcraft-publish.yml:96`), exactly the SHA the report
  flags. But `v1.2.0` is the newest tag that repository has published, and
  `snapcore/action-build` is likewise pinned at its newest, `v1.3.0`. Both
  declare `using: 'node20'` in their `action.yml`, so the Node deprecation
  warning originates upstream and cannot be resolved by bumping the pin.
- Also checked the other 11 pinned third-party actions (`actions/checkout`,
  `actions/setup-node`, `actions/upload-artifact`, `actions/download-artifact`,
  `Swatinem/rust-cache`, `taiki-e/install-action`, `nick-fields/retry`,
  `crazy-max/ghaction-import-gpg`, `apple-actions/import-codesign-certs`,
  `zizmorcore/zizmor-action`, `jdx/pr-closer`) against their latest releases:
  all are on their current major.
- Remedy, if one is wanted, is replacing the snapcore actions or waiting for
  upstream to ship a node24 build — not a version bump here.

### #9813 — does not reproduce here

- URL: https://github.com/jdx/mise/discussions/9813
- Status: no repro on an identical curl build; likely network-path dependent.
- Last checked: 2026-08-22.
- Verification: this machine ships the same curl the reporter quoted —
  `curl 8.7.1 (x86_64-apple-darwin25.0) libcurl/8.7.1 (SecureTransport)
  LibreSSL/3.3.6`. With it, `https://mise.run` returns 200, and so do
  `https://mise.en.dev/VERSION` and `https://mise.jdx.dev/VERSION`. The
  installer script now references `mise.jdx.dev` and `github.com`.
- Caveat: `SSL_ERROR_SYSCALL` from LibreSSL is usually a property of the path to
  a particular CDN edge rather than of the client build, so a clean run here
  does not disprove the reporter's. Their `MISE_INSTALL_FROM_GITHUB=1`
  workaround remains the answer if it recurs.

### #9263 — answerable: bun's musl build needs `libstdc++`

- URL: https://github.com/jdx/mise/discussions/9263
- Status: reproduced and diagnosed. The thread stalled in April because roele
  asked for `mise use -vv -g bun` and never got it; this is that output.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification (docker `alpine:latest`, mise 2026.8.10 linux-arm64): a bare
  alpine reproduces the report — `Error relocating .../bin/bun:
  _ZTVN10__cxxabiv117__class_type_infoE: symbol not found`, and the install
  fails with exit code 127.
- mise is not choosing the wrong artifact. The verbose log shows it downloading
  `bun-linux-aarch64-musl.zip`, i.e. the correct musl build — bun does publish
  musl assets for both x64 and aarch64.
- The missing symbol is C++ ABI, from `libstdc++`, which bun's musl build links
  dynamically and a bare alpine image does not ship. After
  `apk add libstdc++`, `mise use -g bun` completes: `bun@1.4.0 ✓ installed`,
  including the post-install `1.4.0` version check.
- Next action: reply with the verbose output and the one-line fix. Optionally
  consider whether this belongs in mise's system-dependency mechanism rather
  than a docs note — see the shared note under #9360.
- Note: reproduced on arm64; the report was x64. The asset naming and the
  libstdc++ dependency are the same on both.

### #9360 — reproduced, but the reported cause is wrong

- URL: https://github.com/jdx/mise/discussions/9360
- Status: the failure is real; the "libraries not on `LD_LIBRARY_PATH`"
  explanation is not.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification (docker `ubuntu:24.04`, mise 2026.8.10 linux-arm64):
  `mise use -g llama.cpp` installs successfully and extracts the `.so` files
  flat beside the binaries, exactly as the report shows. `llama-cli --version`
  then fails — but with `libgomp.so.1: cannot open shared object file`, not the
  bundled `libllama-common.so.0` the report names.
- `ldd` lists exactly one missing library, `libgomp.so.1`, and `readelf -d`
  shows the binary carries `RUNPATH: [$ORIGIN]`. The bundled libraries next to
  the binary therefore resolve without any `LD_LIBRARY_PATH` involvement; the
  premise of the report does not hold on the current build.
- `libgomp.so.1` is GCC's OpenMP runtime — a system library the archive does
  not ship. After `apt-get install libgomp1`, `llama-cli --version` prints
  `version: 0.1.2-dev (build 10549, commit b2e5e9b28)`.
- Shared with #9263: in both cases mise fetches the right artifact and reports
  `✓ installed`, and the tool is still unusable because of an undeclared system
  dependency of the upstream build. mise already has a system-dependency
  mechanism (`SystemDepsMode`, `src/toolset/helpers.rs`), so the durable fix may
  be declaring these in the registry rather than answering each report. Worth
  putting to the maintainer as one question covering both.
- Also note the install reported success for a binary that cannot execute, which
  is the same class as #9324 / #9826 above. `registry/llama.cpp.toml` does carry
  `test = { cmd = "llama-cli --version" }`, but that is registry test tooling,
  not an install-time gate.

### #9000–#9999 — remaining, needs an environment this audit lacks

- #9495: corporate endpoint-security products making activation slow — a real
  performance thread with no single fix, and the products cannot be reproduced
  here.

### #10650 — `mise edit` cannot preserve comments by construction

- URL: https://github.com/jdx/mise/discussions/10650
- Status: confirmed at both the code and behavior level. No maintainer response
  on the thread yet; 3 upvotes.
- Confidence: high.
- Last checked: 2026-08-22.
- Mechanism: `TomlDocument::parse` reads the file with
  `let doc: DocumentMut = content.parse()?`
  (`crates/mise-interactive-config/src/document.rs:97`), lifts the structured
  data into its own `sections` model, and drops the document. `to_toml`
  (`:302`) then starts from `DocumentMut::new()` — an empty document — and
  rebuilds every section from that model. Comments and blank lines are
  therefore unrepresentable at save time, not merely lost by an oversight. The
  reporter's point that reaching for `toml_edit` implies preservation was
  intended looks right.
- End-to-end (pty, save with `s`): a config carrying a two-line banner comment,
  an in-section `# language runtimes` comment, a blank-line group with a
  `# build helpers` header, and a trailing `FOO = "bar"   # trailing comment`
  came back with every one of those removed.
- Also observed, worth separating before reporting: the same save wrote
  `bun = "latest"`, which the file never contained. The editor's "Detecting
  tools" step appears to pre-populate the model with detected tools, so a plain
  save commits them. That may be intended behavior for the editor; it is
  surprising enough to be worth asking about alongside the formatting loss.

### #10829 — age decryption fails for a passphrase-protected SSH identity

- URL: https://github.com/jdx/mise/discussions/10829
- Status: implemented on branch `age-ssh-identity-diagnosis` (`86be05154665`,
  off `upstream/main` at `6f52dcdf99e2`). No pull request yet — CI on main is
  red for unrelated reasons, so the branch is pushed and held. Zero comments on
  the thread.
- Confidence: high.
- Last checked: 2026-08-23.
- Verification: two runs differing only in whether `ssh-keygen` was given a
  passphrase, each with `[settings.age] ssh_identity_files` pointing at the
  generated key and the value encrypted through
  `mise set --age-encrypt --age-ssh-recipient <pub> HOGE=…`:
  - key without a passphrase — `mise env` returns `HOGE=supersecret`.
  - key with a passphrase — `[experimental] Failed to decrypt HOGE` /
    `Failed to decrypt: No matching keys found`.
- Mechanism: `ssh::Identity::from_buffer` *succeeds* for a passphrase-protected
  key and returns `Identity::Encrypted`, which `src/agecrypt.rs` pushed into the
  identity list without inspecting. age then maps `Encrypted` and `Unsupported`
  to `None` while matching stanzas (`age-0.12.1/src/ssh/identity.rs:293`), so
  the key silently contributes nothing and the decryptor reports the one thing
  that is not wrong. `age.strict` defaults to `true`, so this error — not the
  non-strict `debug!` — is what users see.
- Implementation: classify each SSH identity as it is loaded and, when
  decryption fails, name the ones that could never have worked and why.
  `UnsupportedKey` separates encrypted PEM, an unsupported cipher, a
  hardware-backed key, and an unsupported key type, so each gets its own
  wording. Unusable identities stay in the list — dropping the only one would
  report "No age identities found" instead, trading one wrong diagnosis for
  another. Nothing is printed when decryption succeeds.
- Deliberately not prompting for the passphrase, even though age supports it:
  env resolution runs through `hook-env` on every shell prompt, so blocking
  there would be worse than the message. That belongs behind an explicit opt-in
  the maintainer decides on.
- Verified: clippy clean with no exclusions, `cargo fmt --check` clean, three
  new unit tests over the classifier and hint formatting, a new
  `e2e/env/test_env_age_ssh_passphrase` asserting the contrast (guarded by a
  skip when `ssh-keygen` is absent, since no other e2e test needs it), and the
  existing age and SOPS e2e tests re-run.
- Next action: open the PR once main's CI is green, then post the resolution to
  the Discussion and remove this entry.

### #10553 — not reproducible; the external `gpg` import is gone

- URL: https://github.com/jdx/mise/discussions/10553
- Status: does not reproduce on current `main`; the failure mode no longer
  exists. Zero comments on the thread.
- Last checked: 2026-08-22.
- Verification (docker `ubuntu:24.04`, gpg 2.4.4, mise 2026.8.10 linux-arm64):
  with `node.gpg_verify = true`, `mise install` of node@24 completes cleanly,
  including the post-install `node -v` / `npm -v` checks.
- Verification is still happening, and it is not being skipped: `mise settings
  get node.gpg_verify` reports `true`, and the trace shows mise downloading
  both `SHASUMS256.txt` and `SHASUMS256.txt.sig`.
- Why the report cannot recur: no external `gpg` process is spawned at all — a
  full `MISE_TRACE=1` run shows only `$ node -v` and `$ npm -v`. mise verifies
  the detached signature in-process with the `pgp` crate (`pgp = "0.20"`,
  `src/gpg.rs`, whose own comment reads "Verify a detached signature entirely
  in-process (no external `gpg` binary)") against the bundled
  `assets/gpg/node.asc` embedded by `include_str!`. The reporter's failure was
  mise's own `gpg --import` aborting; there is no import step left to abort.
- Note: reproduced on arm64 while the report was x64 CI. The signature and the
  bundled key are architecture-independent, and the removal of the external
  binary applies to both.

### #10556 — Landlock rules cannot name a path that does not exist yet

- URL: https://github.com/jdx/mise/discussions/10556
- Status: implemented on branch `sandbox-missing-path-warning` (`c64a31a09d0f`,
  off `upstream/main` at `6f52dcdf99e2`). No pull request yet — CI on main is
  red for unrelated reasons, so the branch is pushed and held. Zero comments on
  the thread.
- Confidence: high.
- Last checked: 2026-08-23.
- Verification (docker `ubuntu:24.04`, kernel 7.0.12-linuxkit): the reporter's
  task reproduces exactly, including the doubled warning —
  `mise sandbox: path '/work/test.txt' does not exist, sandbox rule may not
  apply as expected` twice, then `touch: cannot touch 'test.txt': Permission
  denied`, and no file is created.
- Isolated with two controls, which show the rule itself is fine and only the
  not-yet-existing case fails:
  - same rule with the file pre-created — succeeds.
  - `allow_read`/`allow_write` on the parent directory instead — succeeds.
- Reading: this is a Landlock property rather than a mise bug — rules bind to
  paths that must exist when the ruleset is built. What is mise's to fix is the
  handling: it already detects the condition and prints "may not apply as
  expected", then proceeds into a sandbox that is guaranteed to deny the task.
- Implementation: report it from the parent, once per path, naming the parent
  directory as the way out. Three things were wrong with the old handling: "may
  not apply as expected" described a rule that was *dropped*, after which the
  task is certain to be denied; "does not exist" was printed for any
  `PathFd::new` failure, naming a cause the branch never checked; and nothing
  pointed at the workaround. The doubling came from the same path appearing in
  both allow-lists. The message lived inside `pre_exec` — post-fork, where the
  logger is unavailable, hence the bare `eprintln!` — so moving it ahead of the
  fork is what allows `warn!`, deduplication, and naming the parent. It runs
  before the sandbox is applied, so it survives Landlock itself failing. macOS
  is untouched: Seatbelt rules are path patterns that bind nothing. The
  constraint and workaround are now in `docs/sandboxing.md`, which is what the
  reporter asked for in the absence of a fix.
- Not done: widening the rule to the nearest existing ancestor.
  `allow_write = ["node_modules"]` can only be honored by granting create rights
  over the whole parent directory, which loosens what the user wrote — a
  maintainer decision, and the thread has no maintainer comment.
- Verified on real Landlock, not just by inspection. The Linux-gated call sites
  do not typecheck on macOS, so mise was built for Linux in docker
  (`rust:bookworm`, arm64) and the reporter's config run against it: the
  warning appears **once** and names `/work`, where it previously appeared
  twice. The advised workaround was checked too — `allow_read`/`allow_write` of
  `"."` creates the file with no warning. The new
  `e2e/tasks/test_task_sandbox_missing_path` passes there via
  `MISE_E2E_DOCKER=1` with an arm64 image (the stock `ghcr.io/jdx/mise:e2e` is
  amd64-only); it skips on non-Linux hosts.
- Next action: open the PR once main's CI is green, then post the resolution to
  the Discussion and remove this entry.

### #10017 — rolling `neovim@nightly` has no baseline to compare against

- URL: https://github.com/jdx/mise/discussions/10017
- Status: reproduces, and the cause sits in the vfox plugin rather than in
  mise's rolling machinery. Zero comments on the thread.
- Confidence: high.
- Last checked: 2026-08-22.
- Verification: installed `neovim = "nightly"` into an isolated sandbox.
  `mise outdated neovim` reports "All tools are up to date", and **no
  `.mise.checksum` file exists** in the install directory.
- Why it can never detect a move: `install_state::read_checksum` reads
  `<install_path>/.mise.checksum` (`src/toolset/install_state.rs:964`), and the
  only writer is `src/backend/vfox.rs:414`, guarded by
  `if let Some(sha256) = result.sha256`. The plugin supplied none, so nothing
  was recorded, and `is_rolling_version_outdated`
  (`src/toolset/outdated_info.rs:143`) has no baseline. `outdated` and
  `upgrade` are therefore permanently "up to date" for this tool.
- This matches the diagnosis in #10528: vfox-neovim used to read checksums from
  `.sha256sum` files that the GitHub release no longer publishes. The machinery
  itself is in place and was recently improved — #10827 ("fix(backend):
  reinstall rolling versions when outdated") merged 2026-07-12 — it is simply
  starved of input.
- Scope: the checksum has to come from `mise-plugins/vfox-neovim`, so the fix is
  out of scope here under the vfox rule above, same as #8298. What *is* mise's:
  a rolling tool whose backend supplies no checksum silently reports
  up-to-date forever, with no diagnostic. A warning when a `rolling` version is
  installed without a checksum would make this self-diagnosing.

### #10568 — fixed upstream in aube

- URL: https://github.com/jdx/mise/discussions/10568
- Status: resolved; verified. No action.
- Last checked: 2026-08-22.
- jdx root-caused this to aube's pre-resolve `add` gate checking OSV by package
  name only, so an exact pin like `nx@23.0.0` was rejected because OSV flags the
  name for MAL-2025-41443. Fixed in jdx/aube#923 ("fix(add): check exact pins
  with versioned OSV gate"), merged 2026-06-22.
- Verification: `mise install npm:nx@23.0.0` now completes — `✓ installed`, no
  `ERR_AUBE_MALICIOUS_PACKAGE`, and without setting `advisoryCheck = off`.

### #10327 — confirmed, but it is the request jdx already declined

- URL: https://github.com/jdx/mise/discussions/10327
- Status: behavior confirmed at the code level; needs a maintainer decision, not
  a patch. Zero comments on the thread.
- Last checked: 2026-08-22.
- Confirmation: `Backend::is_version_installed` (`src/backend/mod.rs:2572`)
  decides purely on path existence, the absence of an incomplete marker, and
  symlink validity. No tool option participates, so changing `components` or
  `targets` leaves the install "already installed" and the change is dropped.
  `install_args()` (`src/plugins/core/rust.rs:106`) does read both and pass them
  to rustup, so they are honored on a *fresh* install only.
- This is the same ask as #8224 and #8235, which jdx declined: querying
  `rustup show` on every resolve is too expensive, and `mise install -f rust` is
  the supported workaround.
- What is new in this report is the consequence, not the mechanism: with
  `jdx/mise-action`'s default caching, the stale toolchain is restored on every
  subsequent CI run, so the workaround has to be applied inside CI rather than
  once locally. That is worth putting to the maintainer as a caching question
  rather than reopening the resolve-cost decision.

### #10000–#10999 — scope notes

Every unattended candidate identified in the first pass has now been worked and
has its own entry above. Two were set aside rather than investigated: #10216
(the embedded vfox `htmlparser` Lua module is weaker than upstream vfox's
goquery — vfox plugins are out of scope per the Workflow rule above) and #10682
(msys2 zsh activation, Windows-only — note the Windows filter matched only
bodies and comments, so a title-only mention like this one has to be caught by
eye).

### #9000–#9999 — design and lower-priority follow-up queue

- #9075 / #9316: backend aliases with default options and registry-aware short
  names. Prior PR #8895 was substantial and closed without a recorded design
  rejection, but this needs a smaller current design and clear layering rules.
- #9601: bypassing cache reads without clearing them is useful for diagnosis,
  but spans several unrelated caches and needs a precise scope before coding.
- #9715: `task_source_files(only_changed=true)` needs a durable definition of
  “changed” across first run, failed run, mtime mode, content mode, and watch.
- #9781: cross-host `url_replacements` intentionally preserves Authorization
  today and the docs warn about it. A safe stripping option/default has security
  value but can break authenticated mirrors; this requires maintainer policy.
- #9896: project/version-aware completion directories are a broad shell
  integration feature and still lack an agreed producer/consumer contract.
- #9100, #9480, #9708, and #9864 remain product/design ideas without enough
  maintainer signal to treat as implementation-ready.
