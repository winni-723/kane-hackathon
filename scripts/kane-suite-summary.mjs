// Summarize a `kane-cli testrun run --agent` NDJSON log: one line per test,
// and for each failed test the failing step taken from its Result.md.
// Usage: node scripts/kane-suite-summary.mjs <suite.ndjson>
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";

const events = readFileSync(process.argv[2], "utf8")
  .split(/\r?\n/)
  .filter((l) => l.startsWith("{"))
  .map((l) => {
    try {
      return JSON.parse(l);
    } catch {
      return null;
    }
  })
  .filter(Boolean);

for (const e of events.filter((e) => e.type === "testrun_member_end")) {
  const name = basename(e.path);
  console.log(`  ${e.status === "passed" ? "pass" : "FAIL"}  ${name}  (${e.duration_s}s)`);
  if (e.status === "passed") continue;

  // The member's own event log says which check failed and why.
  if (e.log_path && existsSync(e.log_path)) {
    const member = readFileSync(e.log_path, "utf8")
      .split(/\r?\n/)
      .filter((l) => l.startsWith("{"))
      .map((l) => {
        try {
          return JSON.parse(l);
        } catch {
          return null;
        }
      })
      .filter(Boolean);
    for (const m of member) {
      if (m.type === "run_end" && m.status !== "passed") {
        console.log(`        reason: ${m.reason}`);
        if (m.verdict?.root_cause) console.log(`        cause:  ${m.verdict.root_cause}`);
      }
    }
  }
  const result = join(dirname(e.path), `output-${name.replace(/_test\.md$/, "")}`, "Result.md");
  console.log(`        details: ${result}`);
}

const done = events.find((e) => e.type === "testrun_done");
console.log(done ? `  suite: ${done.overall_status}` : "  suite: NO testrun_done (the suite did not finish)");
