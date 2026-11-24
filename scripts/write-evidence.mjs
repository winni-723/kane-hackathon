// Writes what a verification found, as JSON, next to the code it ran on.
// See evidence/README.md for the fields and how to check them.
//
//   node scripts/write-evidence.mjs cargo <out.json> --env <local|github-actions>
//        --steps "<name>=<pass|fail|skipped>,..." [--test-log <cargo test output>]
//   node scripts/write-evidence.mjs kane <out.json> --exit <kane exit code>
//        [--suite <testrun ndjson>] [--note "<why it did not run>"]
//
// The files hold names of steps and tests, counts and pass/fail: no record
// text, no Kane account or project ids.
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname } from "node:path";
import { treeHash } from "./tree-hash.mjs";

const [kind, out, ...rest] = process.argv.slice(2);
const opt = {};
for (let i = 0; i < rest.length; i += 2) opt[rest[i].replace(/^--/, "")] = rest[i + 1];

const run = (cmd, args) => {
  try {
    // kane-cli is a .cmd on Windows, which only a shell can start
    const shell = process.platform === "win32" && cmd === "kane-cli";
    // (the arguments here are fixed strings, never input)
    return execFileSync(shell ? [cmd, ...args].join(" ") : cmd, shell ? [] : args, {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
      shell,
    }).trim();
  } catch {
    return null;
  }
};

const tree = treeHash();
const source = {
  commit_at_run: run("git", ["rev-parse", "HEAD"]),
  uncommitted_changes: (run("git", ["status", "--porcelain", "--untracked-files=no", "--", ".", ":!evidence"]) ?? "") !== "",
  tracked_files: tree.files,
  tracked_files_sha256: tree.sha256,
  check_with: "node scripts/tree-hash.mjs",
};
const env = process.env;
const ciRun =
  env.GITHUB_RUN_ID && `${env.GITHUB_SERVER_URL}/${env.GITHUB_REPOSITORY}/actions/runs/${env.GITHUB_RUN_ID}`;

let body;
if (kind === "cargo") {
  const normal = { success: "pass", failure: "fail", cancelled: "fail" };
  const steps = (opt.steps ?? "")
    .split(",")
    .filter(Boolean)
    .map((s) => {
      const [name, status] = s.split("=");
      return { name, status: normal[status] ?? status };
    });
  // "test result: ok. 111 passed; 0 failed; ..." once per test binary
  let rust_tests = null;
  if (opt["test-log"] && existsSync(opt["test-log"])) {
    rust_tests = { passed: 0, failed: 0 };
    for (const m of readFileSync(opt["test-log"], "utf8").matchAll(/test result: \w+\. (\d+) passed; (\d+) failed/g)) {
      rust_tests.passed += Number(m[1]);
      rust_tests.failed += Number(m[2]);
    }
  }
  body = {
    layer: "1: answer keys, build, clippy, cargo test, browser engine equals native, tsc",
    environment: opt.env,
    ci_run: ciRun || null,
    toolchain: run("rustc", ["--version"]),
    status: steps.length > 0 && steps.every((s) => s.status === "pass") ? "passed" : "failed",
    steps,
    rust_tests,
  };
} else if (kind === "kane") {
  const events = opt.suite && existsSync(opt.suite)
    ? readFileSync(opt.suite, "utf8")
        .split(/\r?\n/)
        .filter((l) => l.startsWith("{"))
        .flatMap((l) => {
          try {
            return [JSON.parse(l)];
          } catch {
            return [];
          }
        })
    : [];
  const summary = events.find((e) => e.type === "testrun_summary");
  const exit = opt.exit ? Number(opt.exit) : NaN; // empty when Kane never ran
  body = {
    layer: "2: Kane CLI replays every saved test against the page in a real browser",
    environment: "local",
    kane_cli: run("kane-cli", ["--version"]),
    options: ["--agent", "--headless", "--no-adaptive-heal", "--parallel 3"],
    status: opt.note ? "not run" : exit === 0 ? "passed" : exit === 2 ? "not started" : "failed",
    note: opt.note ?? null,
    kane_exit: Number.isNaN(exit) ? null : exit,
    totals: summary?.totals ?? null,
    // "authored" counts tests Kane had to record afresh (which costs credits);
    // a verification only replays, so it should be 0.
    tests: events
      .filter((e) => e.type === "testrun_member_end" || e.type === "testrun_authored_member_end")
      .map((e) => ({ test: basename(e.path.replaceAll("\\", "/")), status: e.status, duration_s: e.duration_s }))
      .sort((a, b) => a.test.localeCompare(b.test)),
  };
} else {
  console.error("usage: write-evidence.mjs <cargo|kane> <out.json> [options]");
  process.exit(64);
}

mkdirSync(dirname(out), { recursive: true });
writeFileSync(
  out,
  `${JSON.stringify({ format: "deid-test-evidence/1", generated_at: new Date().toISOString(), source, ...body }, null, 2)}\n`,
);
