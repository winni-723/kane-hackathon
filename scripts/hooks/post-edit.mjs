// Claude Code PostToolUse hook (Write|Edit).
//
// Runs right after the agent edits the engine or the page, and hands the
// result straight back to the agent:
//   - engine change (src/, tests/fixtures_test.rs, Cargo.toml): cargo test,
//     including the answer-key tests
//   - page change (web/src/, web/index.html): tsc
//
// A failure exits 2 with the failing tests on stderr, which Claude Code feeds
// to the agent as an error to act on. A pass reports the current scores on the
// development set and on the held-out sets, so every edit shows its effect.
//
// This is the fast half of the loop (seconds). The Kane half runs from the
// Stop hook (scripts/hooks/stop-verify.mjs).
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { join, relative, resolve } from "node:path";

const root = resolve(process.env.CLAUDE_PROJECT_DIR ?? process.cwd());

let input = {};
try {
  input = JSON.parse(readFileSync(0, "utf8") || "{}");
} catch {
  process.exit(0); // not a payload this hook understands
}
const file = input.tool_input?.file_path ?? input.tool_response?.filePath;
if (!file) process.exit(0);
const rel = relative(root, resolve(root, file)).replace(/\\/g, "/");

const engine = /^(src\/|tests\/fixtures_test\.rs$|Cargo\.toml$)/.test(rel);
const page = /^(web\/src\/|web\/index\.html$)/.test(rel);
if (!engine && !page) process.exit(0);

const env = {
  ...process.env,
  PATH: `${join(homedir(), ".cargo", "bin")}${process.platform === "win32" ? ";" : ":"}${process.env.PATH}`,
  // A separate build directory: the running server keeps target/ locked.
  CARGO_TARGET_DIR: "target-dev",
};
// One command string through the shell: npm is a .cmd shim on Windows.
const run = (cmd, args) =>
  spawnSync([cmd, ...args].join(" "), { cwd: root, env, encoding: "utf8", shell: true });

function fail(title, output) {
  const lines = output.split(/\r?\n/);
  const picked = lines.filter((l) =>
    /FAILED|panicked|error(\[|:)|left:|right:|lost "|still present|leaked|LOST|TS\d{4}/.test(l),
  );
  const body = (picked.length > 0 ? picked : lines.slice(-25)).slice(0, 40).join("\n");
  process.stderr.write(`LAYER 1 FAILED after editing ${rel}: ${title}\n${body}\n`);
  process.exit(2);
}

if (page) {
  const r = run("npm", ["--prefix", "web", "run", "typecheck"]);
  if (r.status !== 0) fail("tsc", `${r.stdout}\n${r.stderr}`);
  console.log(JSON.stringify({
    hookSpecificOutput: { hookEventName: "PostToolUse", additionalContext: `Layer 1 after editing ${rel}: tsc passed.` },
  }));
  process.exit(0);
}

const r = run("cargo", ["test", "--", "--nocapture"]);
const out = `${r.stdout}\n${r.stderr}`;
if (r.status !== 0) fail("cargo test", out);

// "== tests/heldout: 10 records" ... "total removed: 62/88" ... "medical content kept: 61/61"
const scores = [];
let current = null;
for (const line of out.split(/\r?\n/)) {
  const title = /^== (.+?): \d+ records/.exec(line);
  if (title) current = { name: title[1].replace(/ \(.*\)$/, "") };
  const removed = /^total removed: (\d+\/\d+)/.exec(line);
  if (removed && current) current.removed = removed[1];
  const kept = /^medical content kept: (\d+)\/(\d+)/.exec(line);
  if (kept && current) {
    current.kept = `${kept[1]}/${kept[2]}`;
    current.lost = Number(kept[2]) - Number(kept[1]);
    scores.push(current);
    current = null;
  }
}
const summary = scores
  .map((s) => `${s.name}: removed ${s.removed}, medical kept ${s.kept}${s.lost > 0 ? " (MEDICAL CONTENT LOST)" : ""}`)
  .join("; ");
const passed = (out.match(/test result: ok\. (\d+) passed/g) ?? [])
  .map((m) => Number(/(\d+) passed/.exec(m)[1]))
  .reduce((a, b) => a + b, 0);

console.log(JSON.stringify({
  hookSpecificOutput: {
    hookEventName: "PostToolUse",
    additionalContext: `Layer 1 after editing ${rel}: cargo test passed (${passed} tests). ${summary}`,
  },
}));
