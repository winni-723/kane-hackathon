// Claude Code Stop hook.
//
// Before the agent is allowed to finish a turn, this checks whether the engine,
// the page or the Kane tests changed since the last full verification. If they
// did, it runs both layers (bash scripts/verify.sh), which replays every saved
// Kane test in a real browser.
//
//   both layers pass   -> the agent may stop
//   a test fails       -> exit 2: the failing Kane tests and Kane's own
//                         explanation go back to the agent, which keeps working
//   Kane did not start -> the agent may stop, with a warning (sign-in expired,
//                         browser could not launch): not something code fixes
//
// If the same failing state is seen twice with no change in between, the agent
// is let through with a warning rather than held in a loop.
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(process.env.CLAUDE_PROJECT_DIR ?? process.cwd());
const stateFile = join(root, ".kane-logs", "verified.json");

function files(dir, keep) {
  const abs = join(root, dir);
  if (!existsSync(abs)) return [];
  return readdirSync(abs).flatMap((name) => {
    const rel = `${dir}/${name}`;
    if (statSync(join(root, rel)).isDirectory()) return files(rel, keep);
    return keep(rel) ? [rel] : [];
  });
}

const tracked = [
  ...files("src", () => true),
  ...files("web/src", () => true),
  ...files("tests/kane", (f) => /^tests\/kane\/[^/]+_test\.md$/.test(f)),
  ...["web/index.html", "Cargo.toml", "tests/fixtures_test.rs", "tests/extraction_test.rs", "tests/documents_test.rs"].filter((f) => existsSync(join(root, f))),
].sort();

const hash = createHash("sha256");
for (const f of tracked) hash.update(f).update(readFileSync(join(root, f)));
const digest = hash.digest("hex");

let state = {};
try {
  state = JSON.parse(readFileSync(stateFile, "utf8"));
} catch {
  // first run
}
const say = (systemMessage) => console.log(JSON.stringify({ systemMessage }));

if (state.passed === digest) process.exit(0); // nothing changed since the last pass
if (state.failed === digest) {
  say("Verification is still failing and nothing has changed since it last ran. Run `bash scripts/verify.sh` to see why.");
  process.exit(0);
}

const r = spawnSync("bash", ["scripts/verify.sh"], { cwd: root, encoding: "utf8", timeout: 28 * 60 * 1000 });
const out = `${r.stdout ?? ""}\n${r.stderr ?? ""}`;
const save = (next) => {
  mkdirSync(join(root, ".kane-logs"), { recursive: true });
  writeFileSync(stateFile, JSON.stringify({ ...state, ...next, at: new Date().toISOString() }, null, 2));
};

if (r.status === 0) {
  save({ passed: digest, failed: null });
  say(out.split(/\r?\n/).find((l) => l.startsWith("BOTH LAYERS PASSED")) ?? "Both layers passed.");
  process.exit(0);
}
if (r.status === 1) {
  save({ failed: digest });
  const lines = out.split(/\r?\n/).filter((l) => !/^\s+pass {2}/.test(l) && l.trim() !== "");
  process.stderr.write(
    `VERIFICATION FAILED. Fix the code (never the answer keys or the Kane tests), then finish again.\n${lines.slice(-45).join("\n")}\n`,
  );
  process.exit(2);
}
if (r.status === 3) {
  say("A verification is already running, so this one was skipped. The agent's turn ends without a fresh result.");
  process.exit(0);
}
say(`Verification could not run (exit ${r.status ?? "timeout"}):${out.split(/\r?\n/).filter(Boolean).slice(-2).join(" | ")}`);
process.exit(0);
