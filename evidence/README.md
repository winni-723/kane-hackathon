# Test evidence

What the last full verification found, written by `bash scripts/verify.sh`
and committed with the code it ran on.

| File | Layer | Where it runs |
|---|---|---|
| `local-cargo.json` | 1: answer keys, build, clippy (native and browser builds), `cargo test`, browser engine equals native, `tsc` | this machine |
| `local-kane.json` | 2: every saved Kane CLI test replayed against the page in a real browser, with `--no-adaptive-heal` | this machine |
| `cloud-cargo.json` | 1, the same steps | GitHub Actions, on every push; attached to the run as the `test-evidence` artifact, not committed |

Kane does not run in the cloud: it needs a signed-in Kane CLI account.

## Fields

- `source.tracked_files_sha256`: a fingerprint of every file git tracks,
  except this folder, as they were when the tests ran. It is built from the ids
  git gives each file's contents, so line endings do not change it.
- `source.commit_at_run`: the commit checked out when the tests ran. The
  evidence is committed together with the change it tested, so this is the
  commit *before* the one that carries the file, and
  `source.uncommitted_changes` is `true`.
- `status`: `passed` only when every step passed. A failed run is written too,
  with the step that failed.
- `local-kane.json` `totals.authored`: tests Kane had to record afresh. A
  verification only replays saved recordings, so it is 0 (and no credits are
  used).
- `local-cargo.json` `rust_tests`: Rust tests passed and failed, summed over
  every test binary.

The files hold step and test names, counts and results: no record text, and
no Kane account or project ids.

## Checking that the evidence belongs to a commit

```bash
git checkout <commit that changed evidence/>
node scripts/tree-hash.mjs
```

The printed fingerprint equals `source.tracked_files_sha256` in both files
when the committed code is exactly the code that was tested.
