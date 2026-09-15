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
- **A flag or setting whose name promises coverage has to have it.** PR #12501
  was closed for covering most caches but not all: jdx's objection was that users
  would believe an invocation fetched everything fresh while an inner cache was
  still in play, and that every future cache would then have to remember to
  honor the policy. Documenting the exclusions is not a substitute — the user
  reads the name, not the docs. Before proposing anything global, ask whether
  every implementation of the thing can be made to honor it, and whether anything
  keeps future ones honest. If not, prefer a narrower name or the existing
  per-area controls.
- **Read the hidden flags on existing commands before proposing a new one.**
  `mise cache clear --outdate` already did most of what #9601 asked and is marked
  `hide = true`, so it is absent from `--help`. Neither the reporter nor I found
  it before an implementation was written and closed.
- Skip Windows-only implementation decisions until they can be verified on a
  Windows machine.
- Treat vfox plugins as out of scope. They are vendored from upstream repos,
  `.github/workflows/vendored-file-warning.yml` blocks non-maintainer PRs
  against `crates/vfox/embedded-plugins/**`, and this effort does not work
  on that surface. Record such reports as closed out rather than queuing
  them.
- Keep completed range audits below so a finished number band is not scanned
  again without a new report, regression, or maintainer request.
- `e2e/cli/test_prune_tool_stub` fails on macOS for an unrelated reason: it
  uses GNU `touch -d '2001-01-01'`, which BSD `touch` rejects. Not a regression.
- A CI failure that looks like a broken tool may be `minimum_release_age`
  (default 24h) doing its job. watchexec published v2.6.0 with **zero** assets
  and fixed it in v2.6.1 two and a half hours later; for the ~2.5h window when
  2.6.0 had aged past 24h and 2.6.1 had not, `watchexec@latest` resolved to the
  only release that could not install, and every `mise watch` e2e test failed.
  Measured: the versions-host file holds 59 entries including 2.6.1, `ls-remote`
  returns 58, and disabling the host changes nothing — the filter runs after
  listing. It resolves itself; do not pin the tool in tests over it.
- e2e cannot assert that a Landlock sandbox denied something. `e2e/run_test`
  builds every isolated directory with `mktemp --tmpdir`, so the workdir is
  under `/tmp`, and a sandbox restricting both reads and writes grants `/tmp`
  full access (`src/sandbox/landlock.rs`). Writes there succeed through that
  rule whatever the tested rule does. Assert on the message instead.

## Shipped

Entries are removed from Pending once they reach this list; it is here so a
later scan can tell what was already handled.

Merged on 2026-08-24, shipped in v2026.8.12, and the resolution posted to each
thread on the same day. Replies went into the thread where the report was last
moved forward rather than to the top, except where a discussion had no comments.
Note that #8877 and #9826 are duplicates of #8261 and #9324, and were answered
separately.

- #8261 (and #8877, the same failure) — `go install` no longer inherits a GOROOT
  belonging to a different Go. PR #12342. Isolated under docker: GOROOT was the
  only differentiator between the reported wall of `compile: version ... does
  not match go tool version ...` and a clean install.
- #8269 — `mise watch --clear=reset` no longer leaves the terminal without echo.
  PR #12328. The 2026.2.21 fix had been ineffective for five months and only
  started working as a side effect of #11440; this restores from a `Drop` guard
  and captures `/dev/tty` rather than stdin.
- #8232 — the unexpanded-`$VAR` warning now names the key and the file behind
  it. PR #12316.
- #9324 / #9826 — an install that provides no executables is reported instead of
  passing silently. PR #12321.
- #9482 — a task killed by SIGINT reads as an interruption, not a failure.
  PR #12323.
- #9642 — `oc = "stable"` installs; the entry now points at the channel
  directory's unversioned filenames. PR #12326.
- #10650 — `mise edit` keeps comments when saving. PR #12319.
- #10829 — age decryption says why an SSH identity could not be used. PR #12339.

Merged on 2026-08-23, and the resolution posted to each thread.

- #8940 — trust prompt wedged the terminal when stdin was not a tty. PR #12268,
  plus the tri-state confirmation refactor jdx asked for in PR #12273.
- #9045 — `mise prune --dry-run` now says why each version is prunable. PR
  #12304. Reported in jdx's own sub-thread, where he had asked for it.
- #8586 — strict lockfile mode documents which backends it skips. PR #12306.
- #8797 — the `raw` setting documents that it serializes execution, and the
  task-level page no longer claims the opposite. PR #12307.
- #8735 — the `_.source` cacheable example used `file`, which never parsed;
  the field is `path`. PR #12278.
- No discussion: PR #12305 stopped `e2e/run_test` forwarding empty token
  variables, which broke local e2e for anyone without a token.

Merged on 2026-08-24 but not yet in a release — v2026.8.12 predates the merge,
so the reply says "merged, ships in the next release" rather than naming a
version. Posted 2026-08-24.

- #12238 — `--output keep-order` no longer drops the output of tasks started
  through a task reference. PR #12370. Found by the audit of this band, not by a
  reporter follow-up: the mechanism predicted that a single-line task loses
  everything, and that reproduced 4 runs out of 4. Block ordering for those tasks
  was deliberately left alone and took two further PRs to close: #12397, then
  #12450.
  - Closed out on 2026-08-25 after #12397 merged. The reply answered the
    reporter's "so trivial resulted in such huge changes" with the split — 275
    of 528 added lines are tests — and why the rest could not be smaller: block
    order comes from a registration taken before the run starts, and an injected
    task does not exist then.
  - **Corrected a claim I had made to the reporter**: I told them the array form
    `run = [{task="a"},{task="b"}]` "registers the children". It does not —
    `exec_task_run_entries` awaits each entry, so the array form is sequential
    and `test_task_sequence_keep_order` passes for that reason. The defect was
    never mapping-form-specific.
  - Also closed the loop on the `slow`/`fast` example I had given while stating
    it was reasoned from code and not run: it is now an e2e test and reproduced
    as described. It needs `MISE_JOBS=4` and a sleep skew — under `--jobs 1` the
    semaphore serializes the tasks and the defect cannot appear, which is why the
    existing cases never caught it.
  - **Reopened by the reporter, correctly.** On 2026-08-25 they read the
    "not fixed" table in that reply, matched it against the `depends` /
    `depends_post` / `wait_for` docs and asked whether keep-order was still
    imperfect. It was: **#12450** (another session on this account, merged
    2026-08-26) gave the whole scheduled sub-graph slots rather than only the
    names written in the run entry. So the row I had written as "an injected
    task's own `depends` — not fixed" is now false and was retracted in the
    reply.
  - Worth keeping from #12450's measurements: three roots with one `depends`
    each at `MISE_JOBS=4` produced **four different orders across twenty runs**
    of the injected form, while the top-level form gave one order twenty times.
    The framing that makes this a bug rather than a design argument: *which*
    order is right is a design question, but varying between runs is not — and
    `keep-order` exists precisely to take the scheduler out of the answer.
  - `wait_for` was never affected, structurally: it adds an edge to something
    already scheduled rather than a task occurrence, so there is no block to
    misplace. Said so in the reply, since it is not something the docs reveal.
  - Two leftovers were named to the reporter rather than left to be found. Only
    one survives: where a `depends_post` block belongs relative to its trigger.
    #12450 deliberately leaves that unanswered and asserts only that the injected
    form equals the top-level form, which holds whichever way it is later
    decided. That one really is a maintainer decision; the other was not.
  - **Measured 2026-08-26: the mixing-parent case is not only a design question.**
    I had filed both leftovers as maintainer decisions. One is: where a
    `depends_post` block belongs has no answer derivable from the code. The other
    is not, and the same test #12450 used settles it — *which* order is right is
    a design question, but **varying between runs** is not.

    `insert_tasks_before` takes the no-anchor path when `task_needs_permit` is
    true, i.e. exactly when the parent has scripts of its own, and appends the
    injected tasks with `init_task` instead. Two such parents injecting
    concurrently append in whatever order they got there. Measured on released
    2026.8.14, `MISE_JOBS=4`, `--output keep-order`, twenty runs each:

    ```
    two parents, each `run = ["echo …", { tasks = [c] }, "echo …"]`
      sleep on c1:   15  p1 p2 c1 c2      5  p1 p2 c2 c1
      sleep on c2:   10  p1 p2 c2 c1     10  p1 p2 c1 c2

    the same children named directly (`mise run c1 ::: c2`)
                     20  c1 c2
    ```

    The control is stable 20/20 and the split moves with the sleep, so this
    follows scheduling rather than any rule. This is the same defect class as
    #12450, in the branch #12450 left alone.

    Safe to measure on a release even though #12450 is not in one: #12450 does not
    touch `insert_tasks_before`'s anchor fallback — that came from #12397, which
    *is* in 2026.8.14 — so the behaviour measured is the behaviour on `main`.
  - **Fixed by PR #12466** (opened 2026-08-26; taken out of draft and put back
    on 2026-08-28, not by me; **merged 2026-08-29**).
    The move is to insert
    *after* the parent's slot rather than before it. #12397 gave a printing
    parent no anchor for two stated reasons; the second — moving it behind its
    children reorders lines it had already buffered — is **specific to inserting
    before**. Inserting after leaves the parent's slot alone and takes the
    position from that slot instead of from the scheduler. The first reason
    stands and the PR says so: a printing parent's later lines still land in its
    own block, ahead of the children. No stable layout is being given up, because
    today's is not deterministic.
  - The unit test reproduces this **without concurrency at all** — injecting the
    two parents in reverse order is enough, and returns `[p1, p2, c2, c1]` before
    the fix. Worth remembering as a technique: a scheduling-order defect often
    has a deterministic shadow at the API the scheduler drives.
  - The e2e case asserts **two runs agree** rather than a literal order, with the
    sleep inverted between them, and was checked against released 2026.8.14
    first: the two disagreed (`p1 p2 c2 c1` vs `p1 p2 c1 c2`), so it does catch
    the defect rather than passing on unfixed code.
  - The reply as first posted closed with "Neither is the class of bug you
    reported — output going missing, and output order depending on timing. Both
    of those are closed." The measurement above made that false: order depending
    on timing survives for a parent that both prints and injects. The comment was
    edited the same hour to separate the two leftovers by kind — the mixing
    parent gets fixed, the `depends_post` position is a design question — and to
    end with "the first is closed everywhere, and the second is closed everywhere
    except a parent that both prints and injects".
  - Edited in place rather than appended as a correction, at the user's
    instruction. Worth knowing for next time: GitHub stamps an edited marker and
    keeps the edit history public either way, so an in-place fix hides nothing
    from a reader who looks — it only changes what the comment reads as at a
    glance.
  - Replied 2026-08-26: https://github.com/jdx/mise/discussions/12238#discussioncomment-18161897
- #10556 — PR #12309. Careful with the framing here: the reporter's actual ask,
  naming a not-yet-created path in an allow-list, is **not** implemented and
  cannot be — `PathBeneath` binds a rule to an open descriptor. What changed is
  that the failure explains itself: the warning says the rule was *dropped*
  rather than "may not apply", stops claiming "does not exist" for any
  `PathFd::new` error, names the parent directory to allow instead, and fires
  once per path rather than once per allow-list. Auto-widening to the nearest
  existing ancestor was left to the maintainer, since it grants more than the
  config asked for. The reply says all of that rather than "fixed".
- No discussion: PR #12357 replaced GNU-only `env -C` in the e2e harness so fork
  PRs can run e2e on GitHub's macOS runners, and PR #12358 stopped a `pipefail`
  race failing `test_trust_safe_config`.
- No discussion: PR #12380 makes `mise prune` remove tracked-config entries that
  cannot be a config file. **Merged 2026-08-24, not yet in a release** —
  v2026.8.12 predates it. Found on the user's own machine rather than in a
  thread: `mise prune` warned about a `/dev/null` entry on every run and could
  not clear it. Cause: `clean_in` asked whether the *entry* existed while
  `list_all_in` resolved it first, so a symlink to `/dev/null` survived forever,
  and on Windows, where an entry is a plain file holding the path, stale entries
  were never removed at all. Both now share one resolver and one rule — an entry
  counts only when it resolves to a regular file. A malformed `mise.toml` is a
  regular file and still warns, which is the point. Related to #12246 only as
  history: nothing creates these entries any more, measured, so this was purely
  leftover state. The Windows half is reasoned, not measured, and says so.
  - **It merged with a correct review comment unaddressed, and the miss was
    mine.** CodeRabbit pointed out that `removes_an_entry_whose_target_is_gone`
    asserts nothing: `Path::exists` follows symlinks, so once the target is
    deleted the entry is dangling and `!entry.exists()` is true whether or not
    `clean_in` removed it. I opened the PR and moved on without checking it for
    review comments. PR #12396 fixes the test with `fs::symlink_metadata`.
  - This is the second tautological test in two days — the same shape as the
    `finishing_leaves_no_buffer_behind` one I caught and dropped in #12370.
    **Before writing a test, state what makes it fail on unpatched code**; and
    check a PR for review comments before treating it as finished.

From this session:

- **PR #12415 — merged 2026-08-25** as `3fee4da7ddbf`. `#MISE` header keys that
  need quoting were dropped silently. Found while verifying #11195. Two
  independent silent-drop paths: the key-path pattern rejected quoted and
  uppercase segments, and the header fold replaced a table instead of merging it,
  so splitting one tool across two lines lost the earlier field.
  - Review round: greptile P2 (escaped quotes) and two CodeRabbit findings
    (whitespace around the dots; whitespace after the marker) fixed. greptile P1
    (recursive merge surfaces a `version` + `path` conflict that used to be
    silently resolved) answered and **accepted** by the reviewer.
  - **`unit-macos` failed on the first push, and the cause was mine.** I deleted
    the key-path check in `extract_usage_from_comments` as "redundant with the
    entry ranges". It is not: `scan_mise_header_entries` matched `#MISE` while
    the extractor also matched `# MISE`, so a spaced marker never produced an
    entry range. `test_parse_task_script_usage_hoists_mise_root_mount` caught it.
    Both markers now allow `\s*` after the comment character.
  - **A test I added asserted the wrong thing**, and CodeRabbit caught it:
    `a_spaced_marker_with_a_quoted_key_is_still_config` checked only that the
    line stayed out of the usage text, which passed while the setting was still
    being thrown away. It now asserts the key lands in the header table. That is
    the third test of mine in this effort to assert something other than the
    defect (after #12328 and #12380) — the rule stands: **state what makes the
    test fail on unpatched code, and check that the assertion covers the loss,
    not just the symptom.**

Open follow-ups from other sessions on this account, recorded so a later scan
does not re-derive them:

- #12396 — fixes the tautological test #12380 shipped (see above).
- #12397 — implements the block-ordering half of #12238 that #12370 deliberately
  left alone: anchoring tasks started from a task reference at their parent's
  keep-order slot, so they appear in declaration order rather than
  first-output order. **Merged.**
- #12450 — the last ordering hole in #12238. #12397 handed slots only to the
  tasks *named* in the run entry; the `depends`/`depends_post` they drag in were
  scheduled in the same sub-graph with no slot, so their block landed wherever
  their first line did. Now the whole sub-graph takes slots, ordered by the same
  `Deps::new` that `mise run a ::: b ::: c` already uses — so the two forms agree
  by construction rather than by a hard-coded order that would rot if `Deps`
  changed. **Merged 2026-08-26.**
- #12399 — drops the stale linux/macos restriction on the aqua backend for
  claude-code so Windows stops falling through to `http:`.

Rejected by the maintainer, kept in Pending with the reasoning: #9926 (SOPS
dotenv, PR #12331) and #8070 (compiler-keyed `go:` installs, PR #12038).

Answered by reply on 2026-08-22, no code change. Kept here rather than in
Pending because nothing is owed unless someone responds.

- #9263 — bun's musl build needs `libstdc++`; mise was fetching the correct
  musl artifact all along. Posted the `-vv` output roele had asked for in April,
  plus the one-line `apk add libstdc++` fix.
- #8336 — conda packages installing as broken symlinks. Fixed by #8325, the
  rattler rewrite, which landed the day the discussion was opened and shipped in
  2026.2.20; the reporter was still on 2026.2.19 when they followed up. Pinning
  their `conda:gfortran@15.2.0` shows it both ways — 2026.2.19 drops
  `gcc_impl_osx-arm64` as "not available for platform osx-arm64" and installs 1
  package where 40 were needed, 2026.2.20 installs all 40 and `gfortran
  --version` runs. Their second point is covered too: the skip path is gone from
  the source, and an unsatisfiable request now fails with `conda solve failed`
  instead of succeeding quietly.
- #8650 — stale python-build-standalone `_sysconfigdata` paths. **My own note
  was wrong**: I had this filed as needing an environment to reproduce, when the
  fix has shipped since #3706, 2024-12-19. `src/sysconfig/` patches
  `_sysconfigdata` after every precompiled install, on by default
  (`patch_sysconfig = true`), derived from bluss's sysconfigpatcher with the MIT
  notice kept in the module — it rewrites every `/install/...` prefix to the real
  install root, drops `-isysroot`, maps `clang`/`clang++` to `cc`/`c++` across
  `CC`/`CXX`/`BLDSHARED`/`LDSHARED`/`LDCXXSHARED`/`LINKCC`, sets `AR`, and adds a
  `PYTHON_BUILD_STANDALONE` marker. Verified `INCLUDEPY` correct and zero
  remaining `/install/` on macOS arm64 with mise 2026.3.9 + py3.10 (a release
  from the week the report was filed), macOS arm64 with 2026.8.10 + py3.12, and
  Linux arm64 with 2026.8.10 + py3.10. Replied with that evidence and asked what
  differed, rather than closing it: the report gives no mise version or OS, so
  why the reporter hit it is still unknown. The patch only runs at install time,
  so a Python unpacked before #3706 keeps its stale data until reinstalled —
  that is the most likely explanation, but it is a guess.
- #9360 — llama.cpp's failure is real but the reported `LD_LIBRARY_PATH` cause
  is not: the binary carries `RUNPATH: [$ORIGIN]`, so the bundled libraries
  resolve on their own and the only gap is the system `libgomp.so.1`. The reply
  also put the shared question to the maintainer — both threads are an
  undeclared system dependency of an upstream build, which `SystemDepsMode`
  could cover from the registry.

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

### #12370–#12414 — audited 2026-08-25, jdx is covering this band himself

Twelve discussions opened 2026-08-24/25, none previously recorded here. The
useful finding is that **there is almost nothing left to pick up**: jdx merged
five fixes within roughly a day of the reports.

- Fixed by jdx, merged: #12374 (PR #12376), #12389 (PR #12390), #12393
  (PR #12395), #12398 (PR #12406), #12404 (PR #12407).
- In flight by jdx: #12414 (PR #12416, open).
- Windows-only, out of scope per the Workflow rule: #12383, #12391, #12400.
- Unattended and non-Windows: **#12401 and #12402** only.

Method note: searching PRs for the string `discussions/<n>` produced a **false
negative** on #12398 — PR #12406 fixes it without linking that URL. Searching the
bare number found it. Use both, or the band looks emptier than it is.

The one implementation candidate here is #12401 / #12402, adding `--global` to
`mise tool` and `mise upgrade`. jdx answered the umbrella request #12403 with
"it should not be global, but we can add it to more things" — so the global-flag
framing is declined and the per-command additions are explicitly welcome. The
current workaround is `mise --cd "$HOME" tool …`.

Not carried further: the user asked on 2026-08-25 to stop tracking newly opened
discussions and to finish closing out what had already been investigated. This
band is recorded for completeness, not as a queue.

### #11000–#12369 General / Q&A / Show and tell — audited, band now complete

- Status: complete. With this, every category in the band has been through a
  pass; do not rescan without a new report or a maintainer request.
- Audited: 2026-08-24.
- General 5 open and unanswered, of which 3 already have a maintainer in the
  thread. Q&A 4 open and unanswered, **all four** maintainer-engaged. Show and
  tell 1.
- Untouched, and both are questions rather than defects:
  - **#11880 — answered by reply on 2026-08-24, no code needed.** It looked like
    the cheapest docs gap in the band and turned out to be already closed, the
    same shape as #8650: `docs/configuration.md` gained a *Which fields mise
    reads* section in PR #12043 (iloveitaly), merged **four days after the
    report** and live since v2026.8.7. Found by bisecting the file's history for
    the text rather than assuming a July report implied a current gap.
    - It answers the first two questions exactly — `node` and `deno` read
      `devEngines.runtime` only, `npm`/`yarn`/`pnpm` read
      `devEngines.packageManager` then the top-level `packageManager`, and `bun`
      tries both. Confirmed against `parse_with_options` in
      `src/config/config_file/idiomatic_version/package_json.rs`.
    - The third question stands: **`aube` is not supported**, and two things are
      missing rather than one — `registry/aube.toml` declares no
      `idiomatic_files` *and* `parse_with_options` has no arm for it, so a
      registry entry alone would do nothing. Left as a maintainer call.
    - Deliberately no docs change. The reporter asked for it on the Node page,
      but #12043 centralised the text where it also covers deno and bun; a second
      copy would drift.
  - **#11195 — verified 2026-08-25: the premise is wrong, it already works.**
    The report says HTTP-backend tools need structured `url`/`platforms`/
    `checksum` config that "doesn't fit in the inline `#MISE tools={}` syntax".
    Measured on 2026.8.12 against a local HTTP server serving a fake tool, an
    inline table carries all of it and the tool installs and lands on the task's
    PATH:

    ```
    #MISE tools={ "http:ruff" = { version = "0.11.0", platforms = {
    "macos-arm64" = { url = "…", checksum = "sha256:…" } } } }
    ```

    `url` + `checksum` without a platforms map works too, as does an unquoted
    dotted key for a simple tool (`#MISE tools.jq="1.8.1"`).
    - **The trap that probably produced the report:** header values are rendered
      by the config template engine before the backend sees them, so a `url`
      containing the http backend's own `{{version}}` placeholder fails with
      `error: Variable 'version' is not defined`. It has to be escaped —
      `{% raw %}{{version}}{% endraw %}`. Anyone hitting that would reasonably
      conclude the syntax cannot express it.
    - **A separate defect found while checking:** dotted keys with a *quoted*
      segment across multiple `#MISE` lines — `tools."http:ruff".version = …`
      plus `tools."http:ruff".platforms.… = …` — parse without a hard error and
      produce **no tool at all** (`ruff: command not found`), alongside a noisy
      `failed to parse task file … with usage: KdlError` warning. A quoted key
      inside the *inline* table is fine, so it is the dotted path specifically.
      Silent no-op on a config that looks right; worth its own report.
    - Their other two questions — whether tool stubs are the recommended pattern,
      and how to make stubs discoverable through a git include — remain
      maintainer territory, and #9477 is a reminder that jdx has firm views on
      tool stubs.
    - Replied 2026-08-25 with the working syntax and both traps, and pointed
      questions 2 and 3 at jdx while flagging #9477 as prior art on stubs.
    - Next action: the silent dotted-key failure still needs its own report.
- **#11805 needs nothing.** A positive write-up of `mise bootstrap plan`; the one
  surprise it raises (`[tools]` absent from `plan` but present in `status`) the
  author looked up and confirmed as documented and intended.

### #11000–#12369 Ideas — audited, same shape as the bug reports

- Status: inventory and triage complete; nothing started.
- Audited: 2026-08-24, after the bug-report pass. Inventory refreshed at the same
  time — the band had grown to 231 discussions, newest #12369.
- 58 Ideas in the band, 48 open and unanswered. **37 of those already have jdx,
  roele or risu729 in the thread.** One more has another community member
  replying. That leaves 11.
- Applying the linked-PR check from the start this time, 4 of the 11 already have
  a merged PR against them — but only one was worth a reply, and checking which
  mattered more than the count:
  - #11289 — replied 2026-08-24, and **not** as "fixed". PR #11300 ships an
    `llms.txt` docs index (v2026.7.14, live and returning 200); the report asked
    for a curated agent-skills library, which does not exist in mise and has no
    maintainer commitment. The reply says the adjacent thing landed and the idea
    itself is still open.
  - #11431 — skipped. This effort's other account already answered it in full on
    2026-08-15, including "merged, ships in the next release"; only the version
    number (v2026.8.7) was missing.
  - #11994 and #11996 — skipped. Reporter and implementer are the same person
    (stevenpollack), who had already posted the PR links, exactly like #11998.

The 7 genuinely untouched:

- **Answered by verification.** #12281 asks "is this already possible or could
  it be added?" about attaching `depends` to monorepo inferred tasks. Measured on
  2026.8.12 with a two-package npm workspace, `monorepo_root = true`,
  `experimental = true` and `task.auto_infer = ["node"]`:
  - **Monorepo-wide: yes, today.** `[monorepo.task_defaults.build] depends =
    ["//:prepare"]` reaches the inferred `node:@acme/web#build` — the dependency
    runs first and `mise task info` reports `Depends on: //:prepare` with both
    the `package.json` and the root `mise.toml` as sources.
  - **Per project: no.** The root default hits *every* project's `build`
    (confirmed with a second package). Narrowing it means an explicit task at
    the project path, and that **replaces** the inferred task rather than
    extending it: a `[tasks.build]` carrying only `depends` ran the dependency
    and nothing else — the `package.json` script was gone. There is no
    add-deps-only form.
  - **Cross-project ordering does work per project**, through a different knob:
    `[monorepo.projects.<id>] depends_add` adds a graph edge that the inferred
    tasks follow — `api#build` ran before `web#build`, and `tasks graph
    --explain` attributes the edge to `configuration`. That is the answer if the
    reporter meant "build the other package first".
  - So the honest answer has three parts: cross-project ordering works per
    project, arbitrary task deps work monorepo-wide, and arbitrary task deps for
    a single project would be a new feature. Posted 2026-08-24 with the measured
    output for each.
  - Entry can be dropped once nobody follows up.
- **Small, concrete, no maintainer signal.** #11636 (`mise config ls` showing
  what each config layer requested, so an override is visible without opening
  every file), #11708 (`mise tasks deps --local` / `--global`), #11573 (a list
  form for pipx subpackages instead of hand-building one long `uvx_args`
  string), #11530 (an `--appdir` equivalent for brew casks, for managed machines
  where `/Applications` is not writable).
- **Large, and one of them is jdx's own.** #11667 is jdx proposing registry-wide
  executable discovery in isolated sandboxes — real maintainer signal, but it
  means enumerating every registry shorthand and installing each under isolation,
  which is a project rather than a patch. #11709 (locking mise itself through
  `mise.lock`) is a bootstrapping design question.
- Standing caution for this category: an unsolicited feature carries the same
  risk as an unsolicited registry addition. Four of the seven have no maintainer
  reply at all, so implementing one is speculative unless jdx says he wants it.
- Next action: #12281 is the only one that can be closed by verification.
  Everything else in this list needs a maintainer decision before it is worth
  writing.

### #11000–#12360 — first pass done, surface is far smaller than it looks

- Status: inventory and triage complete; one candidate reproduced. The shortlist
  below is what a second pass should work through.
- Audited: 2026-08-24.
- Scope correction worth keeping: Discussion numbers share the numbering space
  with issues and pull requests, so "#11000 onwards" is **228 discussions**, not
  the ~1300 the numbering suggests. 40 closed, 72 answered, 135 open and
  unanswered (Ideas 48, bug reports 77, General 5, Q&A 4, Show and tell 1).
- **The decisive finding: 52 of the 77 open unanswered bug reports already have
  jdx, roele or risu729 in the thread**, and 8 more have other community members
  replying. This band is being covered by the maintainers in a way #8000–#11000
  was not.
- **Method correction, learned the hard way.** My first pass classified by
  comment participants alone and called 17 reports untouched. Checking for linked
  pull requests cut that to 13: #11156 (PR #11164, merged 2026-07-21), #11672
  (#11673, 2026-08-04), #11723 (#11727, 2026-08-05) and #11998 (#12017,
  2026-08-15) were all already fixed, some by the reporter themselves. A thread
  with no replies is not an unworked thread. Search PRs by discussion number and
  confirm the PR body actually references it — number search alone matches any PR
  that happens to mention the digits.
- Those were fixed but nobody told the reporters. Notified on 2026-08-24:
  #11156 and #11157 (both PR #11164, v2026.7.12 — the same PR fixed two of
  donbeave's brew-cask reports from different causes), #11672 (#11673,
  v2026.8.2) and #11723 (#11727, v2026.8.3, confirming the reporter's own
  guess). #11998 was left alone deliberately: its reporter wrote the fix, so a
  "shipped in v2026.8.7" note would tell the author what they already know.
- None of the 77 were filed by this effort's accounts.

The 13 genuinely untouched, grouped by what they would need:

- **Reproducible without special hardware.** #12238 (`mise run --output
  keep-order` through a task reference), #11602 (`y`/`on`/`n`/`off` only
  partially accepted as boolean literals where `yes`/`true`/`1` are), #11647
  (`mise lock -g` writes platforms from the project config into the global
  lockfile), #11485 (relative symlinks for `.mise-bins`, a feature request aimed
  at bind-mounted containers).
- **Reproduced already.** #12238 is **not Windows-only** despite being reported
  from pwsh. On macOS with 2026.8.12, a task whose `run` is a task reference
  (`run = { task = "test-*" }`) under `--output keep-order` loses output lines
  non-deterministically — five runs of a six-line expectation produced 5, 5, 5,
  4, 5 lines with the two task blocks in varying order. Naming the same tasks
  directly instead of through the reference gives all six lines every time. The
  variability points at a race rather than a formatting bug.
- **Needs a pty, which this effort has.** #11970 (interactive filter pickers
  intercept `j`/`k` as vim movement, so a query containing those letters cannot
  be typed).
- **Deliberately not to be patched.** #12019 (whether a global `redactions`
  pattern reaches a caller-supplied variable depends on whether the task's config
  file happens to have an `[env]` table). The reporter implemented the adjacent
  #11998 themselves and then wrote this one up *as a report on purpose*: the two
  candidate fixes move in opposite directions, one of them removes masking users
  may rely on, and they escalated it as a product decision with three options and
  two of them already rejected. Taking a patch to it would be stepping over a
  judgement its reporter deliberately handed to the maintainer.
- **macOS `brew-cask` cluster.** #11879 (percent-encoded font filenames), #12293
  (gcloud-cli fails at binary linking after preflights). Reproducible on macOS in
  principle, but each installs a real application through Homebrew, so they are
  slow and invasive to verify.
- **Needs an environment this effort lacks.** #11878 (remote task cache rejecting
  a directory entry because file nodes carry the regular-file type bit in the
  mode — needs a mise-cache server), #11510 (macOS tests not hermetic under a Nix
  build sandbox).
- **Not actionable here.** #11537 and #11728 are the same thing twice: the Fedora
  COPR publishing credentials expired and only the maintainer can renew them.
  #11256 is about `mise-versions` assigning ingestion time as a release date,
  which lives in a separate repository — the same data-quality surface as the
  versions-host findings under #10017.
- #12238 is done — PR #12370, merged and answered. Next action: pick from the
  remaining list. Recheck each thread before starting; this band moves quickly.

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

## Owed once a release ships

Nothing has been released since v2026.8.14 (2026-08-26). The Shipped convention
here pairs **merge + release + reply** — a merge on its own is not the trigger,
because the reporter cannot yet run the fix.

Re-verified 2026-08-30: v2026.8.14 is still the newest release, and the backlog
ahead of it has grown to **ten merged PRs** — #12451, #12470, #12454, #12465,
#12494, #12466, #12506, #12553, #12567, #12583. Only the two below owe a reply;
the rest answer no discussion.

- **#9100** — `config_source` template variable, PR #12454, merged 2026-08-28.
  **Not replied to.** Waiting for the release.
- **#9708** — `release_url` in `mise outdated --json`, PR #12494, merged
  2026-08-28. **Not replied to.** Waiting for the release.
- **#9715** — `task_source_files(only_changed=true)`, PR #12470, merged
  2026-08-27. **Already replied, 2026-08-28**, ahead of the release rather than
  after it. The reply states plainly that it is not in a release yet and names
  v2026.8.14 as the newest, so nobody is sent to install something that is not
  there. Deliberately left in place: the thread had sat with zero comments for
  four months, and telling the reporter the outcome is worth more than holding to
  the convention exactly. **Do not post a second reply when the release lands** —
  the first one already covers it.
- **#8269** — separate case, and not waiting on a mise release at all: the
  reporter is waiting for clearscreen#50 to reach the `watchexec` version mise
  installs. Checked 2026-08-27, clearscreen's newest release is still v4.0.2
  from 2025. Do not reply before that moves.

PR #12465 also merged 2026-08-28 but answers no discussion — it was found while
fixing a Windows unit-test failure on #12419. **PR #12583 (2026-08-30) is the
same shape**: found by reading code during this triage, with no discussion behind
it, so nothing is owed when the release lands.

**Rule this clarified:** the trigger is the release, not the merge, and the reason
is the reporter's ability to run the fix — not ceremony. Replying early is
acceptable when the comment says so explicitly; claiming a version that does not
exist is not.

## Registry modernisation — the next line of work

Discussion triage is paused, not finished: enough of the backlog has been
answered, and newer threads now get picked up quickly by others. Resume later
rather than chasing the front of the queue.

The work that replaces it is **updating existing registry entries**, not adding
new ones. Many were written when asdf and vfox were the normal way to install a
tool; `AGENTS.md` now places `aqua:` / `github:` / `gitlab:` as tier 1 and says
new `asdf:` and `vfox:` plugins are not accepted at all. The registry has not
caught up with its own policy in places.

**This is explicitly not new-tool submission.** The popularity bar and the
"@jdx won't explain why a given tool wasn't accepted" warning apply to
additions; editing an existing entry is maintenance and does not need the
popularity check.

### Measured 2026-08-28, across all 976 entries

First-listed backend — the one mise actually uses, since resolution takes the
first *supported* entry:

| backend | entries | tier |
| --- | ---: | --- |
| aqua | 663 | 1 |
| github | 171 | 1 |
| vfox | 41 | **not accepted for new entries** |
| npm | 32 | 3 |
| pipx | 20 | 3 |
| http | 13 | — |
| core | 13 | — |
| conda | 12 | 2 |
| go / gem / cargo / spm / gitlab | 11 | 3, except gitlab |

**No entry lists `asdf:` first.** That part of the cleanup already happened.

### Where the gap actually is

Two different problems, and they need different work:

1. **Ordering — 3 entries, small and checkable.** A not-accepted backend sits
   *ahead* of a tier-1 one that is already in the same entry, with no recorded
   reason:

   - `1password`: `vfox:mise-plugins/vfox-1password` before `aqua:1password/cli`
   - `neovim`: `vfox:mise-plugins/vfox-neovim` before `aqua:neovim/neovim`
   - `yarn`: `vfox:…/vfox-yarn`, then `asdf:…/mise-yarn`, then `aqua:yarnpkg/berry`

   Nothing new has to be validated for these — the aqua backend is already
   declared. What has to be established is whether the ordering is deliberate,
   because **absence of a comment is not absence of a reason**.

2. **Missing tier-1 alternative — 116 entries** (an earlier note said ~70; the
   actual count of non-tier-1-first entries is 41 vfox + 32 npm + 20 pipx +
   13 conda + 10 go/gem/cargo/spm). The other vfox-first and
   tier-3-first entries have no aqua/github backend recorded at all, so each
   needs checking against the aqua registry or the project's releases before
   anything can move. That is per-tool research, not a sweep.

### The four ordering candidates — all checked, all left alone (2026-08-28)

Measured by installing and running each, in an isolated `MISE_DATA_DIR`, on
released 2026.8.14. **Every one of them turned out to be correctly ordered, and
not one of the reasons was visible from the backend list.** This is the record
so nobody repeats the work.

| entry | why the current order is right |
| --- | --- |
| `tokei` | aqua declares the package as **`package type: cargo`** — for `14.0.0` as well as `13.0.0` — so `mise install aqua:XAMPPRocky/tokei@…` fails with *"package type `cargo` is not supported in the aqua backend"*. aqua never ships a tokei binary; it delegates to cargo. `cargo:tokei` first is correct. |
| `1password` | `mise ls-remote aqua:1password/cli` returns **zero versions** — *"aqua package 1password/cli does not have repo_owner and/or repo_name"*. `AGENTS.md` makes version listing mandatory, so aqua cannot be promoted. vfox lists 120. |
| `neovim` | aqua lists 49 versions but **no `stable` or `nightly`**; vfox has both. `mise install aqua:neovim/neovim@nightly` fails with *"no asset released"*. Promoting aqua would delete both rolling channels — the very thing #10017 is about. aqua does install concrete versions fine (`NVIM v0.12.5` ran). |
| `yarn` | `aqua:yarnpkg/berry` starts at **3.0.0** (67 versions); vfox starts at **1.0.0** (150). Promoting aqua would drop Yarn Classic and 2.x outright — `aqua:yarnpkg/berry@1.22.22` 404s, while vfox installs it and `yarn --version` prints `1.22.22`. |

**The `tokei` comment was worth re-testing and turned out to still hold**, though
its wording is narrower than the truth: it is not that *v13.0.0* lacks binaries,
it is that aqua has no binary package for tokei at any version. Not worth a PR on
its own.

The lesson generalises: a backend being *listed* proves nothing. What decides it
is whether the backend lists versions, covers the same version range, carries the
same channels, and actually installs. Three of these four fail on a dimension the
`backends = [...]` line does not show.

### Entries checked from the ~70 (2026-08-28)

Same discipline: listing, version range, binaries, and a real install. **All
five checked so far keep their current backend**, and the reason differs every
time.

| entry | finding |
| --- | --- |
| `bfs` | Upstream ships **source only** — `bfs-4.1.4.tar.gz` and a signature, no binaries — and aqua has no package. Nothing to promote to. |
| `ag` | `ggreer/the_silver_searcher` publishes **no releases at all**. |
| `clickhouse` | 50 release assets, but the entry declares **8 binaries spanning several tarballs** (`clickhouse-client`, `clickhouse-common-static`, `clickhouse-server`, …). `github:` installs from one asset, so it cannot assemble that set. |
| `teleport-community` / `teleport-ent` | The closest call so far. `aqua:gravitational/teleport` exists and lists the **same 94 versions with the same latest (18.10.0)** as vfox — identical on every check that can be done from a listing. But installed, **aqua provides only `tctl` and `tsh`; `tbot` and `teleport` itself are missing**, while vfox provides all four and `teleport version` runs. The entry declares all four. |

`teleport` is the case that justifies the "actually install it" rule: version
count, latest version and the aqua package's existence all matched, and the
promotion would still have silently dropped the server binary. **A listing
comparison would have passed it.**

### The fact that governs all of this: there is no per-version fallback

**`backends = [...]` is a static preference filter, not a runtime fallback
chain.** `RegistryTool::ba()` in `src/registry.rs` takes `.first()` of the list
after filtering for platform, enabled backend types and experimental status, and
every resolution path — `backend_arg.rs:474`, `backend_arg.rs:523`,
`cli/use.rs:379` — does the same. Nothing merges the later entries in.

Measured on `yarn`, whose entry lists four backends:

```
registry listing        148 versions
vfox (first entry)      148   <- identical, 0 differences either way
aqua berry + aqua yarn   67 + 81 = 148 unique, but 31 differ each way
```

The registry listing is **exactly** the first backend, not a union — the totals
coinciding at 148 is a coincidence that would have been easy to misread.

The consequence is the single most important constraint on this work:
**promoting a backend swaps the version universe wholesale.** Any version the new
first backend does not carry becomes uninstallable through the shorthand, and the
entries behind it do not fill the gap. Most of the "keep it as it is" verdicts
below follow directly from this.

### Narrowing the ~70 mechanically (2026-08-28)

Rather than researching 70 tools one at a time, cross-reference them against the
**full aqua package list** — shallow-clone `aquaproj/aqua-registry` and match
`pkgs/**/registry.yaml` basenames against mise's short names:

```
2288 aqua packages
  x 70 non-tier-1-first mise entries
  = 7 with a same-named aqua package
```

Three were already settled (`neovim`, `tokei`, `yarn`). That left **four new
candidates**, and the whole remaining ~63 have **no same-named aqua package at
all** — consistent with `bfs` / `ag` / `clickhouse`, where nothing existed to
promote to. The cheap mechanical filter should always come before per-tool work.

### The four new candidates — all checked, all left alone (2026-08-28)

| entry | current | finding |
| --- | --- | --- |
| `oapi-codegen` | `go:` | aqua's package is **`type: go_install`**, which the mise aqua backend rejects outright — and its error message *recommends the current entry verbatim*: "package type `go_install` is not supported in the aqua backend. Use the go backend instead: `go:github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen`." aqua lists 87 versions to go's 13, but not one of them installs. |
| `protoc-gen-connect-go` | `go:` | Identical `go_install` failure, with the same self-recommending error. |
| `oxfmt` | `npm:` | aqua tracks the **oxc monorepo `apps_v*` tags**, a different version namespace from the tool's own. `mise install aqua:oxc-project/oxc/oxfmt@1.80.0` succeeds — and the installed binary prints **`Version: 0.65.0`**, matching `npm:oxfmt@0.65.0` exactly. Promoting it would make the entry's own `test = { expected = "Version: {{version}}" }` fail and would hide `0.65.0`, the version every other ecosystem uses. |
| `ant` | `vfox:` | The only one where aqua is genuinely deficient rather than inapplicable — see below. |

**`go_install` and `cargo` aqua packages are source-build shims, not binaries.**
Together with `tokei`, that is three entries where the aqua package exists, lists
plenty of versions, and cannot install anything. *An aqua package existing does
not mean aqua ships a binary* — and mise's own error names the correct backend,
so a failed install is a cheap and authoritative check.

### `ant` — the first aqua-side defect worth a PR

aqua covers **1.10.1 through 1.10.17 (17 versions)**; vfox covers **47**, back to
1.6.0. Both install and run: `Apache Ant(TM) version 1.10.17` from aqua, and
vfox genuinely delivers the old ones — `1.9.16` installs and reports
`Apache Ant(TM) version 1.9.16 compiled on July 10 2021`. With no fallback,
promoting aqua would remove 30 working versions.

The cutoff is **format-driven, not a support decision**. Measured against
`archive.apache.org`:

- `.tar.xz` exists only from **1.10.1** onward — exactly where aqua's
  `semver("<= 1.10.0") -> error_message` sits
- `.tar.gz` exists for **every** version
- `.tar.gz.sha512` exists from **1.8.0** onward; 1.7.1 and older have sha1/md5 only
- all **23 versions in 1.8.0–1.10.0** have `.tar.gz` + a bare-128-hex
  `.tar.gz.sha512`, the same shape the existing checksum regexp already matches
- `rel/` tags exist for all of them, so `version_source: github_tag` needs no change

And the package author invited exactly this follow-up in
[aquaproj/aqua-registry#49947](https://github.com/aquaproj/aqua-registry/pull/49947),
the only commit ever to touch the file:

> For simplicity, ignores versions `1.10.0` and earlier (before smaller `.xz`
> archives were being published). ... but someone can add support later if they like.

**Handed off as a prompt for the aqua repository** (second one, after
`teleport`). Recovers 23 of the 30 missing versions; Ant 1.6.0–1.7.1 stay out
because they publish no sha512. Until it lands, `ant` keeps `vfox:` first.

**Submitted as [aquaproj/aqua-registry#59559](https://github.com/aquaproj/aqua-registry/pull/59559)
and closed unmerged on 2026-08-30** on maintenance cost, with a reopen condition
stated. `ant` keeps `vfox:` first indefinitely — see
"The two aqua-side items — both resolved" below.

### Round two: match on repo and aliases, not just the name (2026-08-28)

The first cross-reference matched aqua package **basenames** only. Redo it over
every token aqua exposes — package `name`, `repo_owner`/`repo_name`, `aliases`
and `search_words` (3431 tokens across 2288 packages). That found **7 more**
matches out of the 103 still open.

**Four of the seven are different programs that happen to share a name**, and
promoting any of them would silently swap the implementation the user gets:

| entry | current | what aqua actually has |
| --- | --- | --- |
| `cowsay` | `npm:cowsay` | `Code-Hex/Neo-cowsay` — a Go rewrite |
| `yamllint` | `pipx:yamllint` | `wasilibs/go-yamllint` — a Go reimplementation |
| `playwright` | `npm:playwright` | `mxschmitt/playwright-go` — Go bindings, not the CLI |
| `bundler` | `gem:bundler` | truffleruby builds — a search-word collision |

**A name match is not a tool match.** Check what the aqua package actually builds
before treating it as a candidate at all.

### The three real ones — all checked, all left alone (2026-08-28)

| entry | current | finding |
| --- | --- | --- |
| `trunk` | `npm:@trunkio/launcher` | Same launcher, but the aqua package carries **no `repo_owner`/`repo_name`** and `github.com/trunk-io/launcher` **does not exist** (404). `mise ls-remote aqua:trunk-io/launcher` returns **0 versions** — *"does not have repo_owner and/or repo_name"*. Second instance of this shape after `1password`: a `type: http` aqua package that expects the user to pin a version. |
| `ffmpeg` | `conda:ffmpeg` | aqua's source is **`Tyrrrz/FFmpegBin`, a 50-star third-party rebuild**, latest **8.1.2** (2026-07-01) against conda's **9.0.1** — a whole major version behind, from an unofficial mirror. |
| `gcloud` | `vfox:` | See below — the one with a genuine aqua-side defect. |

### `gcloud` — aqua's version source has been dead for five months

aqua downloads from the **official** `dl.google.com` URL, but takes its version
list from `twistedpair/google-cloud-sdk`, a 96-star mirror repo. That mirror's
newest tag is **562.0.0, committed 2026-03-24**, and the repo has not been
pushed since. vfox lists **541 versions, latest 582.0.0**; aqua lists **100,
latest 562.0.0**, a strict subset — 20 major versions behind.

The defect is isolated to version listing, and the evidence is clean:

```
aqua:twistedpair/google-cloud-sdk@582.0.0  ->  installed, `gcloud version` prints 582.0.0
mise latest aqua:twistedpair/google-cloud-sdk  ->  562.0.0
dl.google.com .../google-cloud-cli-582.0.0-darwin-arm.tar.gz  ->  200
```

So the download side is fine and **pinning works** — which is exactly the case
`AGENTS.md` rules out: *"A backend that can install only an explicitly pinned
version is not sufficient."* Keep `vfox:`.

No aqua issue exists for it (searched `google-cloud-sdk` and `twistedpair`). It
is reportable, but the fix is not obvious — aqua would need a version source
other than the mirror, and their `AGENTS.md` says the maintainer will not
repoint a package at a different repo. **An issue, not a PR**, and aqua's call.

### Two things I got wrong, both caught by measuring

- **"aqua declares only 4 `files`, so 4 of the 8 bins would be missing."** Wrong.
  All eight of `gcloud`'s declared bins resolved under the aqua install, because
  the extracted tree's own `bin/` lands on `PATH` rather than only aqua's
  declared files. *Reading the aqua YAML is not a substitute for installing it* —
  the same discipline that caught `teleport` also refutes predictions made from
  the YAML alone.
- **"`github:vlang/v` lists 100 versions against vfox's 311, so it loses 212."**
  Wrong. The 100 is a **deliberate default** in
  [`list_releases_`](src/github.rs:192) — one page, with more fetched only under
  `MISE_LIST_ALL_VERSIONS` or while everything seen so far is a prerelease
  (bounded, ref #10343). With the flag it returns **303**, and versions outside
  the default page **still install** (`github:vlang/v@0.2.4` installed fine).
  It is a display default, not a coverage limit. Two related traps here: an early
  `403` on that install was **rate limiting, not a missing version** — set
  `GITHUB_TOKEN` before concluding anything — and the first flag test returned
  100 because the **listing was cached**; use a fresh `MISE_CACHE_DIR`.

### `magika` — the first entry that should actually change

`registry/magika.toml` is `cargo:magika-cli`. Google publishes prebuilt CLI
binaries under `cli/v*` tags, and `version_prefix` makes them listable:

```
mise ls-remote 'github:google/magika[version_prefix=cli/v]'
  -> 0.1.3 0.1.4 1.0.0 1.0.1 1.0.2 1.1.0      (latest 1.1.0)
mise ls-remote magika            (cargo)
  -> 0.0.0 ... 1.1.0                          (latest 1.1.0, 10 versions)
```

All four spot-checked versions install and run through `github:`, across the
asset rename at 1.1.0 (`magika-<triple>` -> `magika-cli-<triple>`) — mise's
autodetection handles both.

The decisive measurement, in one `env -i` shell with no toolchain on `PATH`:

```
cargo:magika-cli@1.1.0                       -> ERROR "you need to install Rust first"
github:google/magika[version_prefix=cli/v]   -> installed; magika 1.1.0 standard_v3_3
```

That is `AGENTS.md`'s tier-3 objection made concrete: today `mise use magika`
**fails outright on any machine without Rust**. Build time is not the argument —
the cargo build took only 27s here (100+ crates including `ort-sys`), so do not
claim it is slow.

Cost of switching: `github:` covers **macos-arm64, linux-x64, linux-arm64,
windows-x64** only. Google dropped `x86_64-apple-darwin` at 1.0.2, and there is
no windows-arm64 build. So scope the backend by platform and leave `cargo:` for
the rest — `azure.toml` is the precedent (a `github:` backend restricted to
`platforms = ["windows"]` with a `version_prefix` option, falling back to
`pipx:`). Because `backends()` filters by platform *before* `.first()`,
**platform fallback is real even though version fallback is not.**

Versions 0.0.0-0.1.2 would become unavailable on the four covered platforms.
They predate the CLI's first GitHub release; state it in the PR rather than
hiding it.

**Submitted as [#12553](https://github.com/jdx/mise/pull/12553) (draft) on
2026-08-28** — the first registry change of this whole effort, after 16 entries
that all stayed as they were.

#### musl, settled in Docker

`platforms` cannot express libc (the schema pattern is
`^(linux|macos|windows)-(x64|arm64|...)$`), and a libc mismatch is a **-10 score
rather than an exclusion** ([asset_matcher.rs:553](src/backend/asset_matcher.rs:553)),
so a musl host still selects the gnu asset. On Alpine (musl, aarch64) **neither
backend works**:

```
cargo:   ort-sys@2.0.0-rc.12: ort does not provide prebuilt binaries for the
         target `aarch64-alpine-linux-musl`
github:  installs, then "…/magika" couldn't exec process: No such file or directory
```

Not a regression, then — but the failure moves from a clear build error to an
install that looks fine and cannot exec, which is worse to diagnose. Said so in
the PR and offered to drop `linux` if jdx prefers the cleaner failure.

**Three of the four Alpine runs failed for reasons that had nothing to do with
musl** — Alpine's Rust below the crate's MSRV, cargo invisible to a nested
`mise install`, and a missing `openssl-dev`. Each one looked like "musl is
broken". Only the last, where `ort-sys` names the target itself, is evidence.
**When a container test fails, find the sentence where the failing component
names the cause; anything short of that is the environment talking.**

### `v` — works both ways, not worth a PR

`github:vlang/v` installs and **compiles and runs a V program**; so does
`vfox:jdx/vfox-v`, in 6s. Full asset coverage on both. The only gains are
dropping a vfox dependency and its supply-chain surface; the cost is the default
listing falling from 311 to 100. Note also that the plugin is **jdx's own**
(`jdx/vfox-v`, not `mise-plugins/`). Marginal — leave it.

### Resolving upstream repos mechanically (2026-08-28)

The 96 with no aqua match need a `github:` candidate found by hand — except that
the package registries already record one. npm's `repository`, PyPI's
`project_urls`, rubygems' `source_code_uri` and crates.io's `repository` all
point at the upstream repo, so **34 of 44 npm/pipx/gem entries resolved without
any manual lookup**. Do this before researching anything by hand.

Of those 34, checking `releases/latest` for platform-shaped asset names left
**six** with real binaries: `hatch`, `conan`, `viteplus`, `nub`, `ghui`, `qwen`.
`gemini-cli` ships darwin-only (and unsigned), `danger-js` macOS-only. Everything
else — `prettier`, `cspell`, `svgo`, `vercel`, `wrangler`, `serverless`,
`quicktype`, `heroku`, `ansible-core`, `cookiecutter`, `dvc`, … — publishes **no
release assets at all**.

### `npm:` is fragile too, but at run time (2026-08-28)

I expected mise's embedded aube to make `npm:` self-contained, and the install
step does succeed with no node and no npm anywhere on `PATH`. **The installed
tool then cannot run.** Measured through the ordinary flow, not a raw backend
invocation:

```
$ mise use prettier          # on a machine with no node
  use: ok
$ prettier --version
  .../installs/prettier/latest/node_modules/.bin/prettier: 18: exec: node: not found
```

mise does not pull node in as a dependency. So `npm:` carries the same tier-3
fragility as `cargo:` and `pipx:`, in a **worse** form: `cargo:` and `pipx:` fail
at install with mise's own message naming what to install, while `npm:` succeeds
and then dies at run time with a raw shell error.

*(That is arguably a mise UX defect independent of this registry work — worth a
discussion if it keeps coming up.)*

**My first measurement here was wrong and nearly stood.** `mise install npm:…`
in a container returned "ok", and I recorded that npm was therefore exempt. It
was only running the binary — and then repeating it through `mise use` rather
than `mise x <backend>` — that showed otherwise. **Install success is not tool
success.**

### The six candidates, checked one at a time (2026-08-28)

| entry | verdict | why |
| --- | --- | --- |
| **`hatch`** | **promote** | `pipx:hatch` fails on a machine with neither pipx nor uv. `github:pypa/hatch` with `version_prefix = "hatch-v"` (the repo interleaves `hatchling-v*` tags) lists **43 versions against pipx's 45** — only `0.23.1` and `1.0.0` are lost — and covers every mise platform but windows-arm64, **including `x86_64-unknown-linux-musl`**. 1.18.0 / 1.14.0 / 1.9.0 all install and run; verified on macos-arm64 and, in a container, linux-arm64. |
| **`qwen`** | **promote** | `npm:@qwen-code/qwen-code` installs and then `exec: node: not found`. The GitHub tarball **bundles its own node runtime** (`qwen-code/node/bin/…`), so it runs on a bare host — verified, prints `0.22.2`. |
| **`ghui`** | promote, marginal | Same run-time failure via npm; `github:kitlangton/ghui` is a single static binary and printed `0.9.0` on a bare host. But darwin and linux only — Windows would need the npm entry behind a platform scope — and the entry exists with `allow_low_downloads = true`, i.e. deliberately curated below the usual bar. |
| `conan` | keep | **36 of the newest 100 releases publish no binary at all** (every 1.x and every 2.0.x), and `linux-aarch64` only appears around 2.20. `github:` would list versions that cannot install — worse than not listing them. pipx has 312. |
| `viteplus` | keep | The entry declares four bins (`oxfmt`, `oxlint`, `vp`, `vpr`); the release tarball contains **only `vp`**. The `teleport` failure mode exactly. |
| `nub` | keep | Declares `nub` and `nubx`; the tarball has **only `bin/nub`**. |

### The vfox 28, swept by reading the plugins (2026-08-28)

vfox plugins have no package-registry metadata, but **the plugin source names the
upstream host**. Fetch every `.lua` in the plugin repo and grep the URLs.

- **17 download from somewhere other than GitHub releases**, so there is nothing
  to promote to — `dl.google.com` (aapt2, android-sdk), `repo1.maven.org`
  (asciidoctorj), `archive.apache.org` (groovy), `download.clojure.org`,
  `downloads.mongodb.org`, `cdn.mysql.com`, `ftp.postgresql.org` and
  `download.redis.io` and `www.lua.org` (source), `call-cc.org`,
  `scala-lang.org`, `getcomposer.org`, `pypi.org`,
  `install.python-poetry.org`, `repo.maven.apache.org`, and
  `googlechromelabs.github.io` (JSON endpoints, not releases).
- **11 use GitHub**, and seven of those publish nothing usable: `bpkg` and `jib`
  have **no release assets at all**; `Carthage` ships a lone `.pkg`; `ChezScheme`
  an `.exe` plus a source tarball; `leiningen` a `.jar`;
  `ciscoski/gcc-arm-none-eabi-dist` a single `.json` the plugin reads for URLs;
  `oracle/oci-cli` only OS-specific *offline installers*.

That left three worth installing. **All three failed, each for a different
reason, and none of the reasons was visible before the install.**

| entry | why it stays on vfox |
| --- | --- |
| `azure-functions-core-tools` | Coverage looked perfect — linux/osx/win x arm64/x64 with `.sha2` files. `github:` installs, and then `func` **cannot be executed**: the upstream `.zip` does not carry the executable bit, so the binary lands `-rw-r--r--`. The vfox plugin chmods it in `post_install`. vfox prints `4.14.0`; `github:` gives *"couldn't exec process: Permission denied"*. |
| `tinytex` | **81 versions and the same latest (2026.08) on both** — the closest match seen so far. But TeX Live puts its binaries in a platform-specific `bin/<arch>-<os>/` (here `bin/universal-darwin/`), which the plugin computes at run time via `util.bin_path()` and adds to `PATH` in `EnvKeys`. `github:` installs 538 MB and `pdflatex` still will not run. The entry declares **84 bins**. |
| `graalvm` | Different version namespaces: vfox lists 97 (`25.0.2` latest), `github:` with `version_prefix = "graal-"` lists **6** (`25.3.4.1` latest), overlapping in 3. There is also **no `macos-x64` asset**, and the entry declares 32 JDK bins in a nested layout. |

**vfox: 28 checked, 0 changes.** `azure-functions-core-tools` is the canonical
example of why installing is not enough either — it installed cleanly and only
*running* it exposed the missing exec bit.

*(Possible mise improvement, noted not pursued: the `github:` backend could set
the executable bit on files named in `bins` after extracting a `.zip`. That is
what the vfox plugin does by hand.)*

### The conda 10 (2026-08-28)

**conda is not fragile the way the tier-3 backends are, and that decides most of
this.** `AGENTS.md` says the conda backend needs no separately-installed package
manager, and it holds: in a Debian container with no conda and no micromamba,
`mise use make` installed and `make --version` printed `GNU Make 4.4.1`. So the
"fails outright without a toolchain" argument that justified `magika` and `hatch`
has no equivalent here — there is no user-visible breakage to fix.

**aqua has none of the ten**, under any name (`clang`, `llvm`, `clang-format`,
`ghc`, `luajit`, `make`, `mosh`, `mysql`, `mysql-client`, `sbt`, `sqlite`,
`sqlite3`, `vim` all miss). Upstream GitHub releases exist for only two:

| entry | finding |
| --- | --- |
| `luajit`, `vim`, `sqlite`, `ghc` | **No GitHub releases at all.** Nothing to promote to. |
| `make`, `mysql-client` | GNU and Oracle distribution; no GitHub release binaries. |
| `mosh` | `mobile-shell/mosh` publishes a `.pkg`, a source tarball and a build report — the same `.pkg` dead end as `Carthage` and `teleport`. |
| `clang`, `clang-format` | `llvm/llvm-project` does ship binaries, but the per-platform `clang+llvm-*` archives are **400-860 MB**. `clang-format` is one ~5 MB binary; conda has a small dedicated package for it. Downloading half a gigabyte to get one formatter is worse for users, so both stay. |
| **`sbt`** | The one real candidate — see below. |

#### `sbt` is promotable, and it does not depend on the #12553 pattern

`github:sbt/sbt` ships a universal `sbt-<version>.tgz`, installs, and runs
(`sbt --version` prints the runner banner, which the entry's loose
`expected = "sbt"` matches). It lists **63 versions against conda's 51**. The six
that conda has and the default listing does not — 1.5.2 through 1.6.1 — **exist
upstream with `.tgz` assets** and install when pinned, so nothing is actually
lost.

It needs **no `platforms` scoping and no `version_prefix`** — a plain `github:`
first with `conda:` behind it. That makes it independent of whatever jdx decides
about #12553, so it can go on its own. It is tidiness rather than a fix, though:
conda works today.

#### A fourth false alarm, and the rule that catches them

`mise latest github:sbt/sbt` returned **2.0.7 while conda returned 2.0.8**, and
2.0.8 was missing from the listing even with a fresh cache, with the versions
host bypassed (`MISE_USE_VERSIONS_HOST=0`), and with `MISE_LIST_ALL_VERSIONS=1`.
It looked like a real backend defect.

It was **`minimum_release_age`, which defaults to 24h**. v2.0.8 was published
that morning. With `MISE_MINIMUM_RELEASE_AGE=0s` the listing contains 2.0.8 and
`latest` is 2.0.8. Nothing is wrong.

That makes four measurements this session that looked like tool defects and were
configuration or environment: the Alpine MSRV/openssl/nesting failures, the
cached listing, the `403` rate limit, and now the release-age filter.

### The last 11 — sweep complete, 116 of 116 (2026-08-29)

`pipx` 6, `npm` 2, `gem` 2, `spm` 1. **All keep their current backend.**

**Two were false negatives from assuming upstream is on GitHub**, which is worth
fixing in the method rather than in these two entries:

- **`gallery-dl` moved to Codeberg** (`codeberg.org/mikf/gallery-dl`). It does
  publish binaries — `gallery-dl.bin` and two Windows `.exe`s — but **no macOS
  build**, and `forgejo:` has **zero precedent in the registry** (0 entries use
  it). Keep `pipx:`.
- **`purty` lives on GitLab** (`gitlab.com/joneshf/purty`), where `gitlab:` *is*
  tier 1. Its 20 releases carry **source zip/tar.gz only**. Keep `npm:`.

`bashly` also moved (`DannyBen/bashly` -> `bashly-framework/bashly`); the correct
repo still publishes no release assets.

The rest have nothing to promote to: `ansible`, `awscli-local`, `awsebcli`,
`sshuttle`, `cocoapods` and `bashly` publish **no release assets**; `amp` has no
public repository at all (npm metadata gives only `ampcode.com`); `danger-swift`
is an SPM package.

#### `pdm` — the conan failure again

`github:pdm-project/pdm` looked ideal: full coverage (macos and linux x
arm64/x64, windows-x64) with `.sha256` for each, same latest as pipx. But
**68 of the newest 95 releases publish no tarball at all** — binaries begin
around 2.23.1 and are still patchy after it (2.26.9 has none). Measured:

```
mise ls-remote github:pdm-project/pdm | grep -x 2.22.4   -> present
mise install github:pdm-project/pdm@2.22.4
  -> ERROR No matching asset found for platform macos-arm64
mise install github:pdm-project/pdm@2.28.2               -> ok, PDM, version 2.28.2
```

pipx lists 230 versions and installs all of them. **A listing that advertises
versions which cannot install is worse than a narrower one**, which is exactly
why `conan` was kept too.

### Sweep complete: 116 entries, 4 changes

| outcome | count | entries |
| --- | ---: | --- |
| promote, **merged** | 1 | `magika` ([#12553](https://github.com/jdx/mise/pull/12553), merged 2026-08-29) |
| promote, **unblocked** | 3 | `hatch`, `qwen`, `ghui` — #12553 merged, so the pattern is accepted |
| promote, **merged** | 1 | `sbt` ([#12567](https://github.com/jdx/mise/pull/12567), merged 2026-08-29) |
| keep | 111 | |
| aqua-side prompts handed off | 2 | `gravitational/teleport` (**merged**, not yet vendored into mise), `apache/ant` (**closed**) |

**The hit rate is about 4%, and that is the finding.** Most entries are on a
tier-3 or tier-2 backend because the tool genuinely has no standalone binary
release — not because the registry is stale. `AGENTS.md`'s tier ordering governs
**new** additions where an alternative exists; it does not imply the existing
non-tier-1 entries are wrong. Anyone resuming this should expect to confirm the
status quo far more often than to change it.

### `sbt` submitted as [#12567](https://github.com/jdx/mise/pull/12567) (2026-08-29)

`backends = ["github:sbt/sbt", "conda:sbt", "asdf:mise-plugins/mise-sbt"]`, plus
`tools = ["java"]` on the test. No `platforms`, no `version_prefix`, so it does
not ride on whatever jdx decides about #12553.

Two things the plan flagged and the implementation settled:

- **The "six versions lost" worry was wrong.** 1.5.2, 1.5.5 and 1.6.1 all
  install when pinned and print their versions — they are merely past the
  100-release page boundary. Verified before submitting rather than asserted.
- **`--version` behaves differently between the two backends, and it decides
  CI.** `conda:sbt` answers `sbt runner version: 2.0.7` with no JDK present;
  the upstream script returns *"Unable to locate a Java Runtime"*. `test-tool`
  runs `sbt --version` inside `ghcr.io/jdx/mise:e2e`, so the entry needs
  `tools = ["java"]` — the same thing `gradle` already declares. `test.tools`
  is a schema field, used by 15 entries.

  That is also a **small real regression**: a user with no JDK loses
  `sbt --version`. sbt does nothing useful without a JVM either way, so
  `--version` is the whole of it, but the PR says so plainly rather than
  burying it.

The PR is framed as **alignment, not a fix** — `conda:sbt` works, and the PR
says closing it is a reasonable answer if the churn is not wanted.

### The two "mise defects" from the registry sweep were both already known (2026-08-29)

Checked before filing anything. **Neither is a new finding, and one is not a
defect at all.** Recording so nobody re-files them.

**`npm:` tools failing at run time without node — documented behaviour.**
`docs/dev-tools/backends/npm.md` states it outright: *"An installed package may
still require `node` at runtime... the npm backend does not add or install
`node` automatically."* That wording landed in **PR #11637, merged 2026-08-02**,
the same day as [#11634](https://github.com/jdx/mise/discussions/11634), which
raised exactly this. So it is intended, and the measurement in the registry
sweep only rediscovered it.

[#11634](https://github.com/jdx/mise/discussions/11634) had been **open and
unanswered since 2026-08-02**: jdx replied *"where does it say that?"* and the
reporter never came back. **Replied 2026-08-29.**

The answer to jdx's question was in the pre-#11637 text, which did say it —
just badly placed:

> By default mise handles `npm:` tools without needing node or a package manager
> CLI installed. [...] `node` is only needed to _run_ the installed tools (and any
> package lifecycle scripts), not to install them.

Accurate, but sitting one sentence after "without needing node", which is how
the thread happened. #11637 replaced it with the explicit statement quoted
above. The reply gives both texts, the container measurement, and the config
that fixes it, and says the thread looks closeable.

**The reporter's link was `mise.en.dev`, not `mise.jdx.dev`.** I suspected a
stale mirror was the whole explanation and checked: it is a live alternate
domain for the same docs (`mise-en-place` -> `mise.en.dev`), serving current
content. Not the answer — but worth having ruled out rather than assumed.

**The `github:` backend not setting the executable bit after extraction —
[#5552](https://github.com/jdx/mise/discussions/5552), filed 2025-07-09 and
marked answered.** The answer names the cause: the upstream archive ships the
binary without the bit, and *"Aqua (not backend, Aqua itself) fixes its
executable flag after installation"* while mise's backends do not. The
`azure-functions-core-tools` case from the vfox sweep is a second instance of
the same gap — there the vfox plugin chmods in `post_install`. The thread is
answered and 13 months old, so **adding a duplicate would be poor form**; if it
is ever worth reviving, do it with the second instance as evidence in that
thread.

#### What made this check work, after it first failed

`gh search issues` **silently returned `[]` while rate-limited**, and the first
pass recorded "no existing reports" on that basis. Two things fixed it:

- **Run a positive control.** Searching a term that must have hits (`shim`,
  `npm`) exposed that the API was refusing everything, not that the corpus was
  empty.
- **Use GraphQL search instead.** It draws on the 5000/hr GraphQL budget rather
  than the 30/min search budget, and it returns `discussionCount` so an empty
  result is distinguishable from a failed one.

### `gcloud`: aqua cannot really fix this, and the root cause is upstream (2026-08-29)

Asked whether aqua-registry could fix the stale version source. Investigated;
the answer is essentially no, for a structural reason plus a policy one.

**aqua has exactly one version source.** Its JSON schema is
`"version_source": {"type": "string", "enum": ["github_tag"]}`, and `github_tag`
is the only value used anywhere in aqua-registry (30 occurrences). There is no
equivalent of mise's `version_list_url`. So a `type: http` package whose real
upstream is `dl.google.com` — which publishes no GitHub tags — has to borrow
tags from *some* GitHub repo. That is the whole reason a mirror is in the
picture at all.

**The mirror is broken for a reason nobody downstream can fix.**
[twistedpair/google-cloud-sdk#14](https://github.com/twistedpair/google-cloud-sdk/issues/14)
(opened 2026-07-07, still open): *"The polling action has been failing for some
months - looks like due to an auth error with GitHub (I guess either the token
has expired/been revoked...)"*. That matches the last tag, 562.0.0 on
2026-03-24. Only the repo owner can refresh that token.

**A live alternative exists, but aqua's guide pre-refuses the obvious move.**
`google-cloud-sdk-unofficial/google-cloud-sdk` — 534 tags, newest `v580.0.0`,
pushed 2026-08-12, unarchived, 20 stars — would work with `version_prefix: v`.
But aqua's `AGENTS.md` says plainly: *"The maintainer will not repoint an
existing package to a fork — submit the fork as a new package instead."* So the
one registry-level fix that would work is exactly the form they decline; it
would have to be a second gcloud package, leaving the stale one in place.

**Conclusion: the highest-value action is not an aqua PR.** One token refresh on
twistedpair#14 restores every downstream consumer at once. An aqua issue would
be informative but leaves them nothing they can act on. mise needs no change
either way — `gcloud` correctly stays on `vfox:`.

**Generalises:** before writing up a defect in a downstream registry, check
whether that registry *can even express* the fix. aqua's single-valued
`version_source` made this unfixable there regardless of goodwill.

### Verdict on #12553: merged, and so was #12567 (2026-08-29)

**Both registry PRs were merged by jdx, unchanged, with no review and no
comment** — `magika` at 00:08 and `sbt` at 00:31. The merged files on `main`
are byte-for-byte what was submitted, comments included.

That settles the question the queue was waiting on. The pattern #12553
introduced is accepted:

- a `github:` backend scoped with `platforms` to only what upstream builds for
- `version_prefix` to pull one train out of a monorepo's tags
- the tier-3 backend left behind it to catch the platforms that are excluded
- some old versions knowingly given up, stated plainly in the PR body

`sbt` also establishes that the plain form — `github:` first with no scoping,
plus `tools = ["java"]` on the test — is fine.

**`hatch`, `qwen` and `ghui` are unblocked.** Submit them in that order;
`hatch` has the strongest evidence of the three (pipx fails outright without
pipx or uv, only two versions are lost, and coverage includes an x86_64 musl
build).

Worth noting for calibration: **no reviewer asked for anything on either PR.**
The AI reviewers found two real issues before submission — CodeRabbit caught the
bare `linux` selector matching riscv64 and loongarch64 on `magika` — and jdx
added nothing. Front-loading the measurement and stating the costs in the PR
body appears to have been the right shape.

### The two aqua-side items — both resolved 2026-08-30

| package | defect | outcome |
| --- | --- | --- |
| `gravitational/teleport` | darwin override installs `teleport-tools-{version}.pkg` (2 binaries) when `teleport-v{version}-darwin-{arch}-bin.tar.gz` carries all four. Open question: whether `.pkg` was chosen for notarization. | **[aquaproj/aqua-registry#59555](https://github.com/aquaproj/aqua-registry/pull/59555) merged 2026-08-28.** Not yet reachable from mise — see below |
| `apache/ant` | refuses <= 1.10.0 although `.tar.gz` + sha512 exist back to 1.8.0 | **[aquaproj/aqua-registry#59559](https://github.com/aquaproj/aqua-registry/pull/59559) closed 2026-08-30** — see below |
| `twistedpair/google-cloud-sdk` | version source is a mirror whose newest tag is 562.0.0 (2026-03-24); pinning 582.0.0 installs fine | **root cause found upstream — not an aqua PR.** See below |

**Neither mise-side reorder is available yet**, for two different reasons.

#### `teleport` — merged upstream, but mise ships a pinned snapshot

The darwin override now uses `teleport-{{.Version}}-darwin-{{.Arch}}-bin.tar.gz`
and declares all four binaries. Verified live on `aqua-registry` `main`.

**It does not reach mise yet, and the reason is a mise fact rather than an aqua
one.** `aqua.baked_registry` defaults to **`true`** (`settings.toml`), so mise
reads the copy vendored at `vendor/aqua-registry/`, pinned by `metadata.json` to
`6d546dfab62d3ff4ee47312222c9559b82f6b3f4` — a commit from **2026-08-25T21:45**,
three days before the fix. That copy still carries the `.pkg` URL, and
`.github/workflows/vendored-file-warning.yml` forbids editing it in this repo:
upstream changes are "picked up here when we next update the vendored copy".

Measured on macOS arm64 with released 2026.8.14, both directions:

```console
$ mise install                      # aqua:gravitational/teleport@18.10.0
  download teleport-tools-18.10.0.pkg          <- baked copy, old definition
$ ls installs/aqua-gravitational-teleport/18.10.0
  Distribution  tctl-18.10.0.pkg  tsh-18.10.0.pkg   <- tbot and teleport absent

$ MISE_AQUA_BAKED_REGISTRY=0 mise install
  download teleport-v18.10.0-darwin-arm64-bin.tar.gz
$ for b in tbot tctl teleport tsh; do mise x -- which $b; done
  .../teleport/tbot
  .../teleport/tctl.app/Contents/MacOS/tctl
  .../teleport/teleport
  .../teleport/tsh.app/Contents/MacOS/tsh
$ mise x -- teleport version
  Teleport v18.10.0 git:v18.10.0-0-gddaa46b8 go1.25.11
```

So the fix is real and complete — all four binaries, and `teleport version`
runs — but only with the baked registry disabled. Listings already matched
before the fix and still do: `aqua:gravitational/teleport` and
`teleport-community` both list **94 versions, latest 18.10.0**.

**Next action:** watch `vendor/aqua-registry/metadata.json` for a bump past
`997069de` (the teleport commit). Only then is reordering
`registry/teleport-community.toml` onto `aqua:gravitational/teleport` safe.
`teleport-ent` is **not** a candidate at all — aqua's package is the OSS build;
Enterprise ships different artifacts.

This is a general lesson, not a teleport one: **an upstream aqua merge is not
"shipped to mise".** The baked registry is the default, so every aqua-side fix
has a second gate.

#### `ant` — closed, with the reopen condition stated

suzuki-shunsuke's first response was a maintenance-cost question — *"Do you
really need these very old versions? ... We simply recommend to use newer
versions."* The reply did not argue demand it could not evidence. It gave the
counts (aqua 17, this PR 40, mise's vfox plugin 47), said outright that
availability is not demand, and **quantified the cost this PR would add**: two
extra pinned test versions means two more `archive.apache.org` downloads per CI
run, on a host that had already timed out once on `ubuntu-24.04-arm` in this
PR's own first run. Three options were offered, one of which was closing it.

Closed 2026-08-30 with the third:

> For now, I'd like to close this pr. When some people will request the support
> of old versions, or the support will be required for mise, it may be worth to
> reconsider.

**Not a flat rejection — a stated reopen condition**, and one of the options
that had been offered, so no surprise. `ant` keeps `vfox:` first, exactly as
this file already assumed.

The second reopen condition is circular and should not be chased: mise cannot
move `ant` to aqua *because* aqua lacks the versions, and a mise PR proposing
`aqua:apache/ant` would have to declare the loss of 30 versions (47 -> 17) under
the rules below. Wait for condition one.

**Volunteering the PR's own cost is what made this a clean close rather than an
argument.** The CI timeout was evidence against the PR, and surfacing it first
is why the exchange ended in two comments.

### Rules for this work

- **Check for a recorded reason before reordering.** `tokei` lists `cargo:tokei`
  ahead of `aqua:XAMPPRocky/tokei` and says why in a comment —
  `# v13.0.0 doesn't have binaries`. Reordering it would regress. That comment
  was found by reading the file, not by the backend-tier scan, so read every
  file being changed.
- **`ls-remote` must work.** `AGENTS.md`: a backend that can only install a
  pinned version is not sufficient. Run `mise ls-remote <backend>` for any
  backend being promoted and confirm it lists versions.
- **`tiny` is a CI fixture** (`vfox:jdx/vfox-tiny`, "mostly a fake plugin to
  check mise in CI"). Never touch it.
- Prefer one tool per PR, or a small group with the same evidence, so a single
  bad promotion does not sink the rest.
- **Cross-reference against the whole aqua package list first.** Shallow-clone
  `aquaproj/aqua-registry` and match `pkgs/**/registry.yaml` basenames against
  mise short names. It cut ~70 entries to 7 in one pass. Per-tool research is for
  what survives the filter, not for the whole list.
- **Try the install before reading anything else.** `go_install` and `cargo` aqua
  packages list versions and install nothing, and mise's error names the backend
  that is already configured. That is one command, and it settled three entries.
- **A version count is not a version range, and a version number is not a
  version.** `teleport` matched on count and latest and still dropped two
  binaries; `oxfmt`'s aqua "1.80.0" installs a binary that calls itself `0.65.0`.
  Diff the actual lists, and run the binary's own version command.
- **Check whether an upstream cutoff is policy or convenience before proposing to
  lift it.** For `ant` the answer was in the PR that added the package — the
  author said outright that someone could add the old versions later. One
  `gh pr view` replaced a guess with an authorisation.
- **A name match is not a tool match.** Four of seven round-two candidates were
  different programs sharing a name — Go rewrites of `cowsay` and `yamllint`, Go
  bindings for `playwright`. Read what the package builds before checking
  anything else.
- **Verify a listing gap before calling it a loss.** `github:` lists 100 releases
  by default on purpose; the versions past it still install, and
  `MISE_LIST_ALL_VERSIONS=1` shows them all. Check `GITHUB_TOKEN` (a `403` reads
  like a missing version) and use a fresh `MISE_CACHE_DIR` (listings are cached)
  before drawing a conclusion from a count.
- **Platform fallback works even though version fallback does not.** `backends()`
  filters on `platforms` before taking `.first()`, so a tier-1 backend can be
  scoped to the platforms it actually covers with the old backend left behind it.
  `azure.toml` and `magika` both rely on this.
- **Verify in a container that matches the platform being claimed.** The `magika`
  case turned on Alpine behaviour that no amount of reading could settle, and the
  first three attempts produced convincing wrong answers. Budget for that: a
  container failure is the environment until the failing component names the
  cause itself.
- **Ask the package registry where upstream lives.** npm `repository`, PyPI
  `project_urls`, rubygems `source_code_uri`, crates.io `repository` resolved 34
  of 44 entries with no manual lookup. It is the same cheap-filter-first move as
  the aqua cross-reference.
- **Install success is not tool success.** `mise install npm:prettier` returns ok
  on a machine with no node; the binary then fails with `exec: node: not found`.
  I recorded the wrong conclusion from the install alone. Run the tool, and run it
  through `mise use` rather than `mise x <backend>` so dependency resolution is
  not bypassed.
- **Read the plugin source to find upstream.** vfox plugins have no registry
  metadata, but every `.lua` in the plugin repo names the download host. That
  resolved all 28 in one pass and settled 17 of them immediately.
- **Run the binary, not just the installer.** `azure-functions-core-tools`
  installed cleanly from `github:` and then refused to exec, because a `.zip`
  does not carry the executable bit. Listing, install and file presence all
  looked right.
- **Suspect the configuration before the tool.** Four "defects" this session were
  the environment: an Alpine Rust below MSRV, a missing `openssl-dev`, a cached
  listing, a `403` rate limit, and `minimum_release_age`'s 24h default hiding a
  release published that morning. Find the sentence where the failing component
  names the cause before believing it.
- **Never write a file with `open(p, "w")` in the same expression that reads it.**
  `io.open(p,"w").write(io.open(p).read().replace(...))` truncates on the open,
  so the read returns nothing and the file is emptied. That destroyed this file
  once, and the empty version reached a commit and the remote because I checked
  the pushed SHA and not the pushed content. **Verify content after pushing, not
  just the hash.**
- **Upstream is not always on GitHub.** `gallery-dl` had moved to Codeberg and
  `purty` lives on GitLab; a GitHub-only sweep reports both as "no releases".
  `gitlab:` is tier 1 and `forgejo:` exists, so check where the package registry
  actually points before concluding there is nothing to promote to. Repos also
  move (`DannyBen/bashly` -> `bashly-framework/bashly`).
- **Search for the duplicate before believing you found something, and control
  the search.** Both "defects" from the registry sweep were already known — one
  documented, one answered 13 months ago. `gh search issues` returns `[]` when
  rate-limited, so a negative is worthless without a positive control; prefer
  GraphQL search, which has a far larger budget and reports a total count.
- **Do not queue PRs behind an unjudged one** — but let the verdict release the
  queue promptly. `hatch`, `qwen` and `ghui` were held because they repeat the
  pattern #12553 introduced; #12553 merged unchanged on 2026-08-29, so the hold
  is over. The caution was still right: one objection would have cost all four
  at once, and `AGENTS.md` says no reason is given.

## Pending

### #8000–#8999 — unresolved, lower-confidence follow-up queue

These were read and left unresolved: each needs an environment this audit could
not reproduce, or a design decision, so none is implementation-ready.

- #8629 (monorepo task cannot find `pnpm` installed through an idiomatic
  version file) and #8953 (intermittent CI freeze after a task completes) need a
  specific toolchain or CI environment to reproduce.
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

### #9926 — SOPS dotenv: implemented, rejected by jdx on a premise that does not hold

- URL: https://github.com/jdx/mise/discussions/9926
- Status: **PR #12331 closed unmerged by jdx on 2026-08-24**, with the comment
  "I don't think we should do this, they should just turn sops on".
- Confidence: high — the gap and the rejection premise are both measured.
- Last checked: 2026-08-24.
- **The stated alternative does not work.** Measured on released 2026.8.11 with
  a real age-encrypted dotenv file and the `sops` CLI on PATH:
  `MISE_SOPS_ROPS=0 mise env` fails with exactly the reported error —
  `failed to parse dotenv file ... Error parsing line: '-----BEGIN AGE
  ENCRYPTED FILE-----'`. Turning the CLI on changes nothing because the dotenv
  path never reaches `sops::decrypt` at all: `EnvResults::file` routes every
  unrecognised extension straight to `dotenvy` with the raw bytes, so no SOPS
  detection happens in either mode. `sops.rops` only chooses between rops and
  the CLI *once a file has been detected as SOPS*.
- So the reporter's problem stands, and the workaround jdx named is not one. The
  implementation was ~40 lines plus tests and rode the existing `_.file`
  machinery for watching and redaction.
- What was in the PR, for whoever picks this up: detection by content
  (`sops_version=` plus `sops_mac=ENC[`, both at the start of a line, measured
  as the only markers SOPS writes in every encryption mode), dotenv routed to
  the CLI regardless of `sops.rops` because rops has no dotenv `FileFormat`
  (gibbz00/rops#99), two e2e tests and four unit tests.
- Re-measured on 2026.8.12 (the release current when this was written) with the
  `sops` CLI genuinely on PATH: `MISE_SOPS_ROPS=0` and `=1` fail identically.
- Posted a short factual note on the closed PR on 2026-08-24 saying so, without
  arguing the design call, and offering the smaller alternative: if dotenv is out
  of scope, the docs page that says `env._.file` handles SOPS should name the
  formats it actually covers.
- **Replied on the discussion 2026-08-25** (it had zero comments until then, so
  the reporter had heard nothing): the PR's fate, the measured fact that
  `MISE_SOPS_ROPS=0` and `=1` fail identically because the setting is never
  reached, that no in-mise workaround exists for the Kustomize case, and an
  offer to send the docs line.
- **The "docs residue" recorded here earlier was wrong, and the reply repeated
  it before being corrected.** `docs/environments/secrets/sops.md` already lists
  `.env.json`, `.env.yaml`, `.env.toml` under **Formats** on line 5, above the
  example. I had read line 22 ("mise will automatically decrypt the file if it is
  sops-encrypted") in isolation and concluded the formats were unnamed. The docs
  are accurate; there is nothing to send. The comment was edited on 2026-08-25 to
  retract the claim and the offer.
  - Lesson, same shape as the test failures in this effort: **read the whole
    surface before asserting an absence.** "The docs do not say X" needs the
    whole page, exactly as "this test fails on unpatched code" needs the
    unpatched run.
- What actually cost the reporter the trip is the error text — `failed to parse
  dotenv file` plus the AGE header reads as a corrupt file rather than "SOPS
  detection does not run for this extension". A hint near `errfn`
  (`src/config/env_directive/file.rs:220`) when the content starts with a SOPS or
  AGE marker would be a few lines and is a diagnostic change, not the declined
  feature. **Not sent**: it sits close enough to the rejection to read as
  re-litigating it, and that is a judgement call rather than a clear win.
- Next action: none.

### `file::remove_all` and symlinks — investigated 2026-08-25, no change made

Recorded so nobody re-derives it. While reading the deletion paths for #9477 I
noticed `remove_all` (`src/file.rs:144`) decides what to delete from
`path.metadata()`, which **follows symlinks** — so the returned `FileType` is
never a symlink and the `x.is_symlink()` half of its guard is unreachable.

It looked like a bug. It is not, and each reason I had for thinking so failed:

- **Not data loss.** Measured with a standalone `rustc` program on 1.81 and
  1.97: `fs::remove_dir_all` given a symlink to a directory returns `Ok`,
  removes **the link**, and leaves the target and its contents intact. So the
  symlink-to-directory case reaches the right outcome by the wrong route.
- **dotnet is fine.** `src/plugins/core/dotnet.rs:214` calls `remove_all` on an
  install path that is a symlink to `DOTNET_ROOT`; per the above the link goes
  and `DOTNET_ROOT` is untouched.
- **The shim case never reaches it.** A dangling shim would indeed be a silent
  no-op (`metadata()` fails, the `_` arm does nothing), but `get_actual_shims`
  (`src/shims.rs:621`) keeps a symlink only when `read_link() == mise_bin`, so a
  shim pointing at a moved binary is filtered out before `extra` is computed and
  is never passed to `remove_all` at all.

What is left is dead code plus a silent no-op on dangling paths that no caller
appears to hit. Changing a function called from ~150 places for no observable
behaviour difference is not worth the regression risk, so nothing was submitted.
`remove_file_or_dir` (`src/file.rs:198`) has the same shape and the same
non-consequence.

Useful by-product: this confirms the assumption PR #12419 rests on — uninstalling
an http tool removes the install symlink and leaves the `http-tarballs` entry
behind, which is exactly the state the sweep is designed to find.

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
- Re-verified 2026-08-25 on current `main`: `HTTP_TARBALLS_DIR` is still
  `dirs::DATA.join("http-tarballs")` and nothing under `src/cli/` references it,
  so no command reaches these entries.
- **The held reply was discarded on 2026-08-25 — two of its claims were wrong.**
  It said `mise cache clear` leaving these behind was a surprise: it is not, the
  entries live in DATA precisely so `cache clear` cannot break live installs, and
  `e2e/backend/test_http_caching:8` says so in a comment. It also "corrected" the
  reporter's premise that installs are symlinks into these folders: that premise
  is current and `test_http_caching:96` asserts it. `install_path_is_explicit` is
  only set by `--system` / `--shared` / `--install-dir`.
- **PR #12419 opened instead** (2026-08-25), which is a better answer than a
  reply. Reachability is measured from the symlinks on disk, so the sweep never
  asks jdx's question about tool stubs, and it runs after `delete()` so it
  inherits the stub protections `e2e/cli/test_prune_tool_stub` already pins.
  Four failure modes each have a test: a link into the entry rather than at its
  root, the two-hop `latest` alias, Windows junctions (which is why the walk
  judges by resolved destination rather than link type), and an install
  finishing between the scan and the delete (locks plus a second scan).
- **Review rounds changed the implementation materially** (greptile 6, CodeRabbit
  4 including nitpicks). Recorded because several were defects, not polish:
  - **CodeRabbit found a real bug I had shipped.** `extract_to_cache` builds each
    entry in a `<key>.tmp-<pid>-<ms>` directory *beside* it inside
    `http-tarballs`, and the installer's lock is on the key rather than the temp
    name — so the sweep could delete an extraction in flight and break the
    install. `.tmp-` names are now skipped outright.
  - **Three separate places broke the same rule**: "the only thing that may be
    silently skipped is something proven absent". A root whose `canonicalize`
    failed, then a root checked with `Path::exists` (which reports false for a
    permission error too), then `entry.file_type()` and `canonicalize` on
    children. Each was a silent skip that would have reported a live entry as
    unreferenced. All now distinguish `NotFound` from every other error.
  - **The forced-install race was real** and my dismissal of it in the PR body
    was wrong — I had reasoned about the sweep deleting *before* extraction, not
    about the entry being renamed into place with the symlink landing after. Now
    narrowed by a timestamp guard, and closed by passing `false` instead of
    `ctx.force` to `lock_file::get` for the cache entry. That is the one
    behaviour change outside prune and is flagged for jdx to veto.
  - **One suggestion was refused and withdrawn.** greptile asked to delete the
    e2e `trap` that kills the background HTTP server, citing the AGENTS.md rule
    about cleanup. That rule is about `rm` of test files; the trap is process
    lifecycle, and four existing e2e tests use the identical line. greptile
    accepted the correction.
- **A second review round found the same mistake in three places**, and the
  pattern is the durable lesson: `Path::is_dir` and `Path::exists` fold a
  metadata error into `false`, which turns "cannot tell" into "holds nothing" —
  the one conclusion a sweep that deletes things must never reach by default.
  The three were child entries (`entry.file_type()` now propagates), the system
  installs root (`shared_install_dirs` filtered it out with `is_dir()` *before*
  the walk could apply its own rule, so the root is now handed over
  unconditionally), and the root check after canonicalisation. `NotFound` skips;
  anything else fails the sweep.
- **And then I made the same mistake a fourth time, in the fix for jdx's
  review.** The bounded walk he asked for skipped a tool's payload unless
  `install_state` named it as an http backend — so a tool whose install state
  could not be read had its version directories left closed, and a link living
  below the version level was never seen. That is "cannot tell" meaning "holds
  nothing" again, arriving through an identity lookup instead of a `is_dir()`
  call, which is why the earlier rule did not catch it. CI did:
  `cli/test_prune_http_tarballs` lost a raw-file install's tarball, and two unit
  tests reported a freed entry. Now the payload is skipped only on a *positive*
  identification as another backend — install state for backend installs,
  `CORE_PLUGINS` for the rest — and anything unplaceable is walked.
  - The test I had written to assert the bound,
    `a_payload_tool_has_its_versions_left_unopened`, used a tool name the bound
    no longer applies to, so it had been encoding the bug. **A test that passes
    because it asserts the defect is worse than no test**: it was the only thing
    standing between the regression and review. Rewritten against a core tool's
    name, with a second test for the unidentifiable case it had stood in for.
- Out of draft as of 2026-08-28, still `CHANGES_REQUESTED`; the three findings
  from jdx's review are answered and pushed.
- **Back in draft as of 2026-08-30** — not by me. Check who moved it and why
  before pushing anything else to that branch.
- Next action: watch #12419 through review; jdx still owes a call on the
  reference-marker redesign versus the residual mixed-version window.

### PR #12583 — a legacy lockfile answering `latest` always picked the older entry

- URL: https://github.com/jdx/mise/pull/12583
- Status: **merged by jdx 2026-08-30**, `12a9dcc2e4dd`. Unreleased.
- **Answers no discussion.** Found by reading code while looking for unreported
  defects, after the registry sweep produced none of its own (both of its
  suspects turned out to be known — see 2026-08-29 above).
- The defect: `get_locked_version` resolved `latest` against a legacy lockfile
  (`lockfile_version` absent) by sorting entries with `versions::Versioning` and
  taking the head. That comparator reduces an alphanumeric chunk to its leading
  digits and stops, so `3.7b` and `3.7c` both become `7` and tie
  (fosskers/rs-versions#39, open with no comments).
- **The outcome was not arbitrary — it was always the older entry.** Entries are
  written sorted by the version *string* (`merge_tool_entries`) and `sort_by` is
  stable, so a tie leaves that order and `.first()` returns `3.7b`. Stating this
  as "always wrong" rather than "non-deterministic" is what made the report
  land; a coin-flip is easy to deprioritise.
- The fix adds the version string as a secondary key — the tie-break
  `install_state.rs` and `runtime_symlinks.rs` already carry. `lockfile.rs` was
  the only site comparing bare `Versioning` values.
- **Both AI reviewers demanded the same wrong remedy, and pushing back was
  correct.** greptile (P1) and CodeRabbit (Major) both cited `AGENTS.md`'s
  "lockfile versions are opaque, never order them" and asked for delegation to
  `Backend::version_order`. Measured instead of argued: that enum has exactly
  two variants, so delegation can only produce one of two behaviours, and
  neither improves on the fix. Entries in stored (lexicographic) order:

  | entries | `Source` `.last()` | `Semver` `.last()` | the fix |
  | --- | --- | --- | --- |
  | `3.7b`, `3.7c` | `3.7c` | `3.7c` | `3.7c` |
  | `1.10.0`, `1.9.0` | **`1.9.0`** | `1.10.0` | `1.10.0` |
  | `1.10b`, `1.9b` | **`1.9b`** | **`1.9b`** | `1.10b` |
  | `1.0.0`, `1.0.0-rc1` | **`1.0.0-rc1`** | `1.0.0` | `1.0.0` |

  `Source` is the **default** — only aqua, github and http override
  `Backend::version_order` — and `order_by` returns its input unchanged, so
  `.last()` is "the lexicographically largest string": the very thing greptile
  objected to, reached another way. jdx merged without further comment.
- **greptile's stated mechanism was wrong and saying so plainly mattered.** It
  claimed the comparator "treats the lexicographically greatest string as
  newest". It does not — the parsed versions are the primary key and the string
  is consulted only on a tie, which a pre-existing test already pinned.
- Adopted the half of the review that was right: CodeRabbit asked for non-semver
  coverage, so `1.9b` / `1.10b` (the pair that separates this from any
  file-order fallback) and `1.0.0-rc1` / `1.0.0` were added, plus a doc comment
  spelling out why `Backend::version_order` is not used.
- Flagged in the PR body rather than changed: whether the ordering should be
  removed from this path entirely (it would need an answer for what `latest`
  picks among several legacy pins, and `e2e/lockfile/test_lockfile_version` pins
  the current behaviour), and `select_unbound_lockfile_tool`'s asymmetry — no
  backend gives `.first()`, a backend gives `.last()`, and on the default
  `Source` that is the lexicographic maximum.
- **CI lesson worth keeping: run a positive control before blaming your own
  change.** Three consecutive runs died in 3–14s at `Run ./.github/actions/mbx`
  with `could not resolve mbx 0.7.0: GitHub returned 403`. Checking an unrelated
  PR (`fix/elvish-path-separator`) showed the identical error, and `main`'s own
  `docs` job was failing too — a repo-wide outage from ~10:40Z, cleared by
  11:15Z. `gh run rerun` needs admin rights, so the only lever from a fork is a
  force-push; re-stamping the commit date leaves the tree untouched
  (`sl amend --date now`, then confirm `sl diff` against the old node is empty).
  Do not re-push into an outage — wait for another branch's `build-ubuntu` to go
  green first.

### PR #12572 — `github:` listed versions that cannot be installed

- URL: https://github.com/jdx/mise/pull/12572
- Status: **open, out of draft as of 2026-08-30** (not moved by me), CI green.
- The defect: `_list_remote_versions` mapped releases from `tag_name` alone and
  never looked at `assets`, so a release with nothing attached was advertised as
  installable. mise never falls back to GitHub's generated source archive, so
  those versions can only fail. Measured: `hatch` 16 of 43, `ghui` 26 of 40.
- **This is what blocked three registry promotions** (`hatch`, `qwen`, `ghui`) —
  the fix is upstream of that sweep, not a side quest.
- Filters on **emptiness, not platform fit**: a per-platform filter would make
  the version list platform-dependent and break cross-platform lockfiles. GitLab
  keys on `assets.links`, because `assets.sources` is the generated archive and
  is always populated.
- **Review found four real defects, three of them mine**: url-configured tools
  hidden; the `/releases/latest` shortcut bypassing the filter, which would have
  made `latest` and the listing disagree — worse than before the PR; a bare
  top-level `url` treated as an exemption though the install path never reads
  it; and cache-context assertions left contradicting the narrowed exemption.
  One greptile finding was **withdrawn by greptile** after it was shown to be
  pre-existing and that narrowing would cost the cross-platform invariant.
- Next action: watch it through jdx's review.

### PR #12420 — failed `http:` extractions leak a temp dir forever

- URL: https://github.com/jdx/mise/pull/12420 (opened 2026-08-26)
- Status: **merged 2026-08-25**, independent of #12419.
- Found while reading `extract_to_cache` during the #12419 review rounds.
- Two things combine: the temp directory is named `<key>.tmp-<pid>-<ms>`, so the
  "clean up any stale temp directory" check ahead of it can only ever match the
  same process inside the same millisecond and never sees what an earlier run
  left; and none of the three failure paths after it removed the directory.
  Result: every failed extraction leaves a hash-named directory in
  `http-tarballs` permanently. It is part of the disk growth #9477 reports.
- **The argument is symmetry, not design.** `extract_to_install_path` — the
  sibling forty lines up, used for `--system` / `--shared` — already cleans up on
  the same paths. `extract_to_cache` now has the same shape, and the one place
  the sibling still missed (`remove_install_path` failing before the rename) is
  closed too.
- Deliberately out of scope: reclaiming directories already leaked, and
  extractions killed with `SIGKILL`. That needs the invariant #12419 establishes
  (nothing holding the cache lock is mid-extraction), and tying them together
  would couple the two PRs. #12419 does not touch `extract_to_cache`, so they do
  not conflict.
- e2e `test_http_failed_extraction_cleanup` serves a `.tar.gz` whose bytes are
  not gzip — with no checksum configured `verify_artifact` has nothing to reject,
  so the failure lands in extraction, which is the path under test.

### PR #12455 — clippy's update broke `lint` on main, and on every PR

- URL: https://github.com/jdx/mise/pull/12455 — **merged 2026-08-26**.
- A clippy update started firing `useless_format` on a `format!` whose only
  argument is a literal with nothing to interpolate. `lint` runs `-D warnings`,
  so two assertions in the `path_env` tests failed the job — on main as well,
  since nothing had run main's lint against the new clippy since 2026-08-19.
- **Diagnosis worth keeping:** a red `lint` on a PR is not evidence the PR caused
  it. #12451 and #12419 were both red at the same moment for entirely different
  reasons — #12451 for this main-side lint, #12419 for a genuine
  `cloned_ref_to_slice_refs` in its own new code. Checking whether main carries
  the same source is the cheap way to tell them apart, and `test-ci` failing
  alongside is just the aggregate gate reporting `untrusted=failure`.
- Scanned the whole tree for the same pattern before submitting: only those two.
  Clippy stops at the first failing target, so a green lint after this was not
  guaranteed — said so in the PR rather than implying the job would pass.

### PR #12451 — `mise watch` leaves the terminal broken when it is signalled

- URL: https://github.com/jdx/mise/pull/12451 — **merged 2026-08-27**, not yet in
  a release.
- **My own loose end from #12328**, whose doc comment claimed restoring from
  `Drop` "covers every one of those exits". It covers every exit that *unwinds*.
  mise handles only SIGINT (`tokio::signal::ctrl_c`), so SIGTERM killed the
  process where it stood. The path that reaches it is the one the user pointed
  out: a `mise watch` nested under a `mise run` is killed by that run's
  `exit::kill_all()`, which sends SIGTERM.
- Fix arms a `signal_hook` thread for SIGTERM/SIGHUP/SIGQUIT alongside the guard,
  matching `CmdLineRunner`'s existing shape. Registering replaces the default
  action so the process no longer dies immediately; `emulate_default_handler`
  then lets the signal finish the job, so the exit status still reports what
  killed it. SIGINT deliberately excluded — tokio owns it and that path unwinds.
- The e2e test needs a pty: `TerminalState::capture()` prefers `/dev/tty` and
  saves nothing when no descriptor is a terminal, so without one there is no
  defect to observe. A stub watchexec clears ECHO directly, following
  `e2e/cli/test_watch_default_task`.
- **The e2e suite is Linux-only** — no macOS e2e job exists. The `unit` (macOS)
  job runs named e2e tests explicitly, so this one was added there: `TIOCSCTTY`
  and signal-driven termios restore are both places the two platforms have
  differed before.
- Review: both bots converged on the same weakness — cleanup starting after the
  `waitpid`, and an unbounded `waitpid`. The second is sharper than it looks for
  *this* change: replacing SIGTERM's default action means a regression leaves the
  process alive forever, so the one failure the test exists to catch would have
  become a silent hang until the CI timeout.
- **No reply is owed on #8269 for this merge, and posting one would mislead.**
  #12451 covers the SIGTERM exit. The reporter's remaining case is the *second*
  Ctrl+C, which is SIGINT — deliberately excluded, and already said so on the
  thread. Announcing the merge there would read as "your case is fixed" when it
  is not.
- **What #8269 is actually waiting on**, in the reporter's own words: *"I will
  mark the question as answered when the fix makes its way into a mise release."*
  That fix is clearscreen#50, and the chain is entirely outside mise —
  clearscreen release → watchexec release → the `watchexec` version mise
  installs. Checked 2026-08-27: #50 merged 2026-08-26 but clearscreen's newest
  release is still **v4.0.2 (2025)**, so nothing has moved. Reply when it does;
  do not reply before.

### PR #12465 — Windows cannot link a raw binary, and says nothing about it

- URL: https://github.com/jdx/mise/pull/12465 — **merged 2026-08-28**, not yet in
  a release. No discussion to report back to: this one was found while fixing a
  Windows unit-test failure on #12419, not reported by anyone.
- Found while fixing a Windows unit-test failure on #12419, which is the only
  reason it surfaced: my test happened to build the same shape the production
  code builds.
- `create_install_symlink` has one branch that links **file to file** — a raw
  binary declared with `bin_path`. On Windows `file::make_symlink` goes through
  `junction::create`, and a junction is a *directory* reparse point. It cannot
  name a file.
- **It does not fail.** `junction::create` returns `Ok` and leaves a link that
  cannot be resolved, so the install reports success and the binary does not run.
  The evidence is #12419's CI: `make_symlink` returned `Ok` and the failure only
  appeared at the first attempt to resolve the path
  (`The directory name is invalid`).
- Why nobody noticed: the one Windows `http:` test installs a raw `.exe` but sets
  no `bin_path`, so it takes the other branch and links a *directory*. On Linux
  the raw-plus-`bin_path` combination only appears via `--system`, which sets
  `install_path_is_explicit` and never calls `create_install_symlink`. The broken
  combination was the one thing neither side covered.
- Fix is `file::make_symlink_or_copy`, which is what the codebase already uses
  for a file that has to be executable at the link path (swift, github, conda,
  aqua). Windows keeps a copy, so dedup is given up there — a duplicate that runs
  beats a link that does not. A hard link would keep both but introduces a
  mechanism for one call site, so it is noted rather than done.
- **General lesson worth keeping:** `file::make_symlink` is directory-only on
  Windows. Any new call with a file target is broken there, silently. It is also
  why the #12419 test that tripped over this is now split — the portable case
  links a directory, the file case is `#[cfg(unix)]`.

### PR #12506 — a bare `[tool_alias]` value resolved as an asdf plugin name

- URL: https://github.com/jdx/mise/pull/12506 (draft, opened 2026-08-27) — the
  #9316 implementation, see the design queue entry.
- **Two vacuous assertions were caught in one PR**, which is the lesson worth
  keeping. I removed one before opening it (a check that `plugins/` was empty,
  which passed pre-fix too because `mise tool` never reaches the clone), and
  review caught a second (a bare `assert_fail`, which accepts any nonzero exit
  and would have stayed green if the failure moved elsewhere). Running each
  assertion against the released binary *first*, to see it fail for the right
  reason, is what surfaced both.

### PR #12501 — a stale cache could only be fixed by destroying it

- URL: https://github.com/jdx/mise/pull/12501 — **closed unmerged by jdx on
  2026-08-28**, with a reason. The #9601 implementation; see the design queue
  entry for why it was attempted and what the closure settles.
- Two review findings were worth more than the change they asked for.
  - **`global = true` was missing, and I had verified nothing.** Measured against
    released 2026.8.14 using the sibling flag declared the same way:
    `mise version --no-config` gives `error: unexpected argument`. So without it
    `mise install --ignore-cache` — the position most people write first — would
    have been rejected outright. The e2e now pins the post-subcommand position.
  - **The suggested fix for the argv overlap would have made things worse.**
    Review pointed at `first_non_global_arg_idx_cached`, which exists for exactly
    this kind of caller. But it stops at the first argument not starting with
    `-`, so scanning to there would have made `mise install --ignore-cache` stop
    working while `mise --ignore-cache install` kept working. The boundary is
    right for `escape_task_args`, which protects task arguments during parsing;
    it is not right for "was this flag passed at all". Both bots withdrew their
    findings on this after the reasoning.
- **Closed on partial coverage, and the reasoning is worth keeping.** jdx:

  > The cache layers have different owners and semantics, and not all of them
  > pass through `CacheManager` (for example, some backend and upstream-library
  > caches). A partial implementation would make `--ignore-cache` **misleading**:
  > users could reasonably believe an invocation fetched everything fresh while
  > an inner cache was still being used. It would also create an ongoing
  > requirement for every new cache implementation to honor this global policy
  > correctly.

  I had scoped this deliberately and documented the exclusions in the PR — the
  env cache and task artifacts kept their own controls — and treated "covers the
  large majority" as good enough. **It is not, for a flag that makes a promise
  about the whole invocation.** A name like `--ignore-cache` claims completeness,
  and partial completeness is worse than no flag, because the user cannot tell
  which half they got. The second half of his reason is the one I had not
  weighed at all: every future cache would have to remember to honor it, and
  nothing enforces that.

### PR #12497 — auth headers followed a URL replacement to a host that never asked

- URL: https://github.com/jdx/mise/pull/12497 (draft, opened 2026-08-27) — the
  #9781 implementation, see the design queue entry.
- **jdx rejected the global setting and named the shape he wanted** (2026-08-28).
  I had shipped `url_replacements_forward_auth_to_new_host` and flagged the
  per-replacement question as his call; he answered it: *"a configuration may
  contain both proxies that need upstream auth and mirrors that must not receive
  it, and the original request asks for control over specific replacements."*
  Reworked to a per-replacement `forward_auth` on the value, with the plain
  string form kept so existing configs parse unchanged.
- Two further corrections in the same comment, both right:
  - **Origin, not host.** `url.origin()` covers scheme, host and port, matching
    #12167. Host comparison alone would have let a credential follow an
    https-to-http downgrade or a port change.
  - **The wording claimed something the code cannot do.** It said the header was
    "built for the original URL"; the implementation cannot identify provenance
    and removes whatever `Authorization` the caller passed. Docs and doc
    comments now say that.
- Also asked for, and done: the matched rule's policy travels with the rewrite
  (`replace_url` returns it) rather than being looked up from a setting
  afterwards. `apply_url_replacements` stays as a wrapper because
  `vfox_plugin.rs` hands it to `set_url_rewriter` as a function pointer.

### PR #12494 — `mise outdated` knew the release URL and threw it away

- URL: https://github.com/jdx/mise/pull/12494 — **merged 2026-08-28**, not yet in
  a release — the #9708 implementation, see the design queue entry.

### PR #12470 — `task_source_files` could not name only what changed

- URL: https://github.com/jdx/mise/pull/12470 — **merged 2026-08-27**, not yet in
  a release — the #9715 implementation, see the design queue entry for what the
  re-investigation turned up and where my own note was wrong.
- **A reply to #9715 is owed once this ships.** Follow the Shipped convention:
  merge alone is not the trigger, the release is.

### PR #12466 — a printing parent's injected tasks were ordered by the scheduler

- URL: https://github.com/jdx/mise/pull/12466 (opened 2026-08-26, **merged 2026-08-29**) — the fix for
  the #12238 leftover measured above; see that entry for the numbers and the
  reasoning. Draft was lifted and reinstated on 2026-08-28, neither by me.
- **Eight rounds of greptile findings, and the last one was the point.** Each
  earlier round was a real case and each fix was correct, but they were all
  patches on the same wrong shape: `injected_by` was `HashMap<Task, Task>`, one
  parent per task, a *tree*. Adoption does not make a tree — two run entries can
  name the same task, and an existing slot is adopted rather than moved, so it
  genuinely belongs to both groups. The structure is a **DAG**, and
  `or_insert_with` was discarding the second edge. It is
  `HashMap<Task, Vec<Task>>` now with a visited-set graph walk. Worth
  remembering as a shape of mistake: repeated near-miss findings in one area are
  a signal the model is wrong, not that the edges are unusual.

### PR #12454 — templates cannot name the config file they are written in

- URL: https://github.com/jdx/mise/pull/12454 — **merged 2026-08-28**, not yet in
  a release — the #9100 implementation, see the design queue entry for why it
  moved. It was taken out of draft by someone other than me on 2026-08-27,
  presumably jdx, and merged the next day.
- **A reply to #9100 is owed once this ships.**

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

### #9000–#9999 — remaining, needs an environment this audit lacks

- #9495: corporate endpoint-security products making activation slow — a real
  performance thread with no single fix, and the products cannot be reproduced
  here.

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

### #10017 — rolling `neovim@nightly` has no baseline to compare against

- URL: https://github.com/jdx/mise/discussions/10017
- Status: **not fixable in mise, and deliberately not pursued.** Investigated to
  the end on 2026-08-23 and set aside without a PR; a reply is the only
  deliverable left, and it is not owed to anyone waiting.
- Confidence: high — measured throughout, not reasoned.

#### Why the reported case cannot work

- `is_rolling_version_outdated` (`src/backend/mod.rs:3077`) has two silent
  `false` exits and both are blocked for this tool. Callers are
  `src/backend/mod.rs:3290` and `src/toolset/outdated_info.rs:144`, so neither
  exit surfaces anything to the user.
- Gate 1, "not rolling". The versions host answers first and returns straight
  out of `list_remote_versions_with_info_and_options`
  (`src/backend/mod.rs:2443`) without ever calling `_list_remote_versions`. Its
  wire schema, `VersionEntry` (`src/versions_host.rs:58`), carries only
  `created_at`, `release_url` and `prerelease`; the mapping to `VersionInfo`
  fills the rest with `..Default::default()`, so every host entry arrives
  `rolling: false, checksum: None`. Confirmed against the live data —
  `https://mise-versions.jdx.dev/data/neovim.toml` has
  `"nightly" = { created_at = ... }` and nothing else.
- Gate 2, "no checksum". Even through the plugin the entry carries no checksum,
  matching #10528: vfox-neovim used to read `.sha256sum` files the GitHub
  release no longer publishes.
- So fixing the shadowing alone only moves the failure from gate 1 to gate 2.

#### The shadowing is wider than neovim

- Measured, `ls-remote --json`, default vs `MISE_USE_VERSIONS_HOST=0`:
  neovim `nightly`/`stable` and rust `nightly`/`beta`/`stable` all lose
  `rolling: true`. rust is a *core* plugin that sets the flag explicitly
  (`src/plugins/core/rust.rs:388-398`) and `BackendType::Core` is on the
  versions-host allowlist (`src/backend/mod.rs:2347`), so it is shadowed too.
- The whole `VersionInfo.rolling` / `.checksum` pathway has exactly two
  producers — the vfox listing (`src/backend/vfox.rs:238`) and rust's core
  plugin — and one consumer, `is_rolling_version_outdated`. Both producers are
  disabled by default through the host.
- Local stamping after listing has precedent (`mark_prerelease`,
  `src/backend/mod.rs:5171`) but it is applied only on the backend branch, never
  to host results.

#### mise already solved the same user problem another way

- `Backend::is_rolling_channel` (`src/backend/mod.rs:2570`) is a cheap,
  network-free predicate added by #10251, with `resolve_channel_version` and
  `latest_installed_channel_version` beside it. It is wired through
  install_context, tool_version, tool_version_list, link and lock. zig
  (`master`) and rust (`nightly`) implement it — rust implements *both*
  mechanisms, and only this one survives the host.
- Measured contrast: `mise install zig@master` lands in
  `installs/zig/0.17.0-dev.1857+3c46da14d` — re-resolved to a concrete version,
  so upgrade/outdated have something to compare. `mise install neovim@nightly`
  lands in `installs/neovim/nightly` — the channel name is pinned forever.

#### Feasibility, which was the question asked

- Gate 1 can be fixed inside mise alone: have `is_rolling_version_outdated`
  also consult `is_rolling_channel`. No host change, no mise-versions change, no
  extra network call. But it changes nothing observable, because every tool that
  declares `rolling` also lacks a checksum.
- Gate 2 cannot: the only producer of `VersionInfo.checksum` is the vfox
  listing, and the data no longer exists upstream.
- Putting neovim on the #10251 path cannot be done in mise either. Every route
  is blocked: `is_rolling_channel` is documented as network-free so it cannot
  consult the listing; declaring channels in the registry would answer the
  predicate but not supply the concrete version `resolve_channel_version` must
  return; and neovim-specific logic does not belong in mise for a vfox/aqua
  registry tool.
- The missing piece is upstream. vfox-neovim lists `0.11.7 … 0.12.4`, `stable`,
  `nightly` and **zero** concrete dev builds, while the binary it installs
  reports `NVIM v0.13.0-dev-1389+gd3b4f562a6`. If `Available` returned that
  concrete version for the channel, the existing #10251 machinery would work
  unchanged — which is exactly what zig's download index and rust's rustup
  channel provide.
- Decision: no PR. A gate-1-only change is a behavior-neutral fix to a pathway
  that #10251 has largely superseded, and reviewing it would cost more than it
  is worth. Reconsider only if checksums or concrete channel versions ever start
  arriving.
- Correction to an earlier note here: it proposed returning an enum to
  distinguish the two exits. That was written assuming gate 2 fires, and would
  not have fixed the reported case.
- **Replied on the discussion 2026-08-25** (zero comments until then): both
  gates, the wider shadowing including rust, the measured zig/neovim install-path
  contrast, and that the missing piece is upstream vfox-neovim returning a
  concrete dev version. Stated plainly that no PR is coming and why.
- Next action: none. Revisit only if checksums or concrete channel versions start
  arriving.

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
- **Replied on the discussion 2026-08-25** (zero comments until then): the
  mechanism, that the resolve-cost half is the already-declined ask and not worth
  reopening, and that the new half is a cache-key question — mise-action's key
  does not include the tool options that determine what gets installed —
  belonging on mise-action#215.
- Re-verified 2026-08-25 on current `main`: `is_version_installed` still decides
  on path existence, incomplete marker and symlink validity only; rust's
  `install_args()` still reads `components`/`targets`.
- Next action: none from here. It needs a maintainer decision on mise-action.

### #10000–#10999 — scope notes

Every unattended candidate identified in the first pass has now been worked and
has its own entry above. Two were set aside rather than investigated: #10216
(the embedded vfox `htmlparser` Lua module is weaker than upstream vfox's
goquery — vfox plugins are out of scope per the Workflow rule above) and #10682
(msys2 zsh activation, Windows-only — note the Windows filter matched only
bodies and comments, so a title-only mention like this one has to be caught by
eye).

### #9000–#9999 — design queue, **re-investigated 2026-08-26**

The user's observation that prompted this: investigating a discussion deeply
enough often turns "needs a design decision" into ordinary work. That held —
three of these moved, and in two cases my original classification was wrong
rather than merely cautious. What follows replaces the earlier queue.

#### Moved to implementable

- **#9100 — merged 2026-08-28, PR #12454.** "There doesn't seem to be *any* variable
  expansion which points to the checkout" was accurate: `config_root` returning
  `MISE_GLOBAL_CONFIG_ROOT` for a global config is deliberate
  (`config_root.rs`), and it is the only path variable put into a config file's
  template context. The premise also changed under it — `conf.d` only became a
  supported feature in #12395 this month, so symlinking a shared config into it
  went from a trick to a first-class arrangement. Fixed by exposing
  `config_source` (the file itself) and letting the existing `dirname` /
  `canonicalize` filters compose, rather than adding a directory variable that
  would decide the symlink question for the user.
- **#9601 — attempted and rejected. PR #12501 closed unmerged 2026-08-28.**
  jdx had said "I'm open to it", and it was still the right thing to try; the
  closure is about scope rather than about the request. **Do not reattempt this
  as a global flag.** It would need every cache layer to honor the policy,
  including ones outside `CacheManager` and inside upstream libraries, plus a way
  to keep future caches honest. Nothing short of that avoids the objection, which
  is that a flag promising a fresh invocation must not quietly leave an inner
  cache in play.

  **`mise cache clear --outdate` already does most of what was asked, and I did
  not know that when I started.** Measured 2026-08-28 in an isolated cache dir:
  it zeroes the mtimes rather than removing the files, so the entry survives and
  the next run refetches and rewrites it — the non-destructive refresh the report
  asked for. It is two steps rather than one invocation, and it is a **hidden
  flag** that does not appear in `mise cache clear --help`, which is why neither
  the reporter nor I found it. **Read the existing commands' hidden flags before
  proposing a new one.**

  **Replied on the discussion 2026-08-28** (zero comments in four months until
  then): the closure and its reasoning, `--outdate` with the measurement, and
  both of its caveats stated plainly.

  Original note, kept because the measurement in it still holds: Not a rejection; my "needs a precise
  scope" was right but the priority was set far too low. All three `CacheManager`
  read paths funnel through one `is_fresh()` gate (`cache.rs`), so 16 files' worth
  of "read nothing, still write" is a single change. `task.remote_no_cache`
  already exists as a precedent for the name and shape. jdx's "harder than it
  sounds" is the caches that bypass `CacheManager`, and that set is enumerable.
  The reporter offered to implement it and has not, four months on.

  **Correction from implementing it: the bypass set I listed here was wrong.**
  I had named the github/gitlab/forgejo API caches as bypassing `CacheManager`.
  All three use it (`github.rs:113` and neighbours), so the single gate reaches
  further than I had estimated — which is most of why the change came out small.
  The one cache on that path that really does roll its own freshness check is the
  floating registry archive (`registry.rs`), and it took one line. The env cache
  and task artifacts were left alone deliberately: each already has its own
  control (`--fresh-env` / `settings.env_cache`, and `mise run --no-cache` /
  `mise cache clear --task`), and a second entrance is worse than none.

  Also worth keeping: `--no-cache` was unusable as a name. `mise run --no-cache`
  and `mise oci push --no-cache` already exist, and because this family matches
  by scanning argv, a global `--no-cache` would have silently widened what
  `mise run --no-cache` does. The flag is `--ignore-cache`; `MISE_NO_CACHE` was
  free and is the name the report asked for.
- **#9715 — merged 2026-08-27, PR #12470. The definition I said was missing is already in
  the code.** `save_checksum`'s own comment says it persists the source hash
  after success "rather than in `sources_are_fresh`" precisely so "a failed run
  never advances the baseline". That makes `sources_hash_path`'s mtime the
  marker, in both freshness modes, with no new state. Each case the old note
  worried about has an answer: first run — no file, everything changed; failed
  run — marker does not advance, the same files stay changed, which is what a
  linter wants; mtime mode — compare source mtimes to the marker; content mode —
  the same marker works, and the per-file `ContentHashCache` allows a more
  precise answer later; watch — re-evaluated per run.

  Two corrections came out of implementing it. **"No new state" was right;
  "read the marker directly" was wrong.** `task_state_key` hashes the working
  directory, and the template function's root (`self.dir` or `dirs::CWD`) can
  differ from `task_cwd`'s (`config.project_root`), so a call from a
  subdirectory would have looked up a marker that does not exist and reported
  everything as changed. `TaskScriptParser` has no `config`, so the path has to
  be resolved by the caller — that plumbing, not the definition, was the real
  work. And the marker is written in *two* places, not one: `sources_are_fresh`
  also writes it when a freshness check finds nothing to do. That is still
  correct here — fresh means the work is done — but "written only on success"
  was imprecise, and the docs and code comments say so properly now.

#### Corrected, but still not a default change

- **#9781 — done, PR #12497.** Half my note was right and half was wrong. Right: the docs state
  that forwarding auth across a replacement is *by design*, for internal proxies
  that relay to upstream, so the default cannot change. Wrong: the reporter did
  not ask for a default change, they asked whether it can be **disabled**, and
  there is no answer. mise's own code already says the header "is likely wrong
  for the replacement target" (`http.rs`) and acts on that only through netrc —
  so when netrc has nothing for the new host, the header mise has called wrong is
  sent anyway. An opt-in setting needs no policy decision and has that internal
  precedent behind it.

#### Verified as a concrete defect, not a design question

- **#9316 — done, PR #12506.** Measured on released 2026.8.12 in a fully isolated environment: a
  `[tool_alias]` naming a bare registry short name still fails with
  `not found in mise tool registry`. Worth a closer look before implementing:
  during that measurement, `mytool = "fzf"` caused mise to clone the **asdf**
  fzf plugin into `$MISE_DATA_DIR/plugins/mytool`, after which
  `plugin_overrides_registry` (`backend_arg.rs`) made it win over the config —
  so changing the alias target afterwards silently returned a different tool's
  versions. A bare alias value being taken as an asdf plugin name deserves
  scrutiny on its own given the repo's stance on new asdf plugins.

  Implemented after re-measuring on 2026.8.14. `BackendArg::full()` returned the
  alias value verbatim, and a bare name is not a backend, so it fell through to
  the plugin path. Fixed by resolving a bare value through the registry first.
  **Resolving the backend was only half of it**, which review caught: `registry_opts`,
  `registry_version_order` and `is_os_supported` all key on `self.short`, still
  the alias key — so the alias would have installed the right tool and then read
  no registry metadata. `llama.cpp` is in the registry *for* its
  `version_prefix = "b"`, so losing it would misread every version. The resolved
  key is now carried into those lookups too, and legacy spellings are normalised
  (`mytool = "nodejs"` finds `node`).

#### Advanced but not promoted

- **#9708 — merged 2026-08-28, PR #12494.** The reporter's guess was right: `VersionInfo.release_url` is
  already populated by the versions host, aqua, github/gitlab and the rust core
  plugin. It is **discarded at the `OutdatedInfo` boundary** —
  `OutdatedInfo::new` takes `latest` as a `String` — and no command surfaces it.
  So the work is threading an existing field through, not inventing one. The
  smallest useful step is `release_url` in `mise outdated --json`, which is
  enough for the reporter to build their own commit message.

#### Closed out — the implementable pool is empty

Every entry that was a measured bug, or that jdx had said yes to, has a PR.
What is left is not "not investigated"; each was looked at and set down for a
stated reason. **Do not reopen these speculatively** — this repo closes
unsolicited design PRs without explanation.

- **#9864 — answered by the feature that shipped.** The download-and-re-exec
  mechanism now exists: `auto_update` (#12288, in **v2026.8.11**; verified two
  ways, since `gh api --jq .content | base64 -d` silently returns nothing and
  gave a wrong answer first). But it is declared `global_only = true`, and
  `settings.toml` says why: *"This setting is global-only so a project's
  configuration cannot opt users into replacing their mise binary."* That is
  precisely what #9864 asks for, so the request is not a missing piece — it is
  the line drawn when the mechanism landed. No `mise_version` field exists and
  there is no registry entry for mise itself, so there is no other route.
  Replied 2026-08-28 explaining which half shipped and which half was ruled out.
- **#9896 — held, on the user's call.** The request is sound and is *not* asking
  mise to generate completions: with one shared `_foo` / `foo.fish`, a project on
  `foo@1` and one on `foo@2` overwrite each other, and mise is the only thing
  that knows which version is active. The proposal is that mise expose
  per-version completion directories through `mise env` / `hook-env` and leave
  generation outside. Held anyway: jdx may do this himself, and in #3757 he said
  the work belongs in aqua first — *"none of the backends mise relies on
  themselves contain completion information."* Revisit only if that changes
  upstream. Zero comments on the thread.
- **#9075 — dropped.** Backend aliases with default options. The prior attempt,
  PR #8895, ran to 113 files (greptile refused it as over its review limit) and
  was closed by its own author, with no recorded design rejection from jdx. The
  need is not established, so it is off the list rather than pending a smaller
  design.
- **#9480 — closed upstream.** Grouping markers in `mise task ls`. No longer
  open, so no action.
- #9070: not a queue item at all — jdx closed the adjacent PR #11823 with "not
  ready to support this", which is an explicit rejection.
