// Summarize a `kane-cli run --agent` NDJSON log: the steps, the verdict, and
// exactly what each assertion looked at (element locator or screenshot).
// Usage: node scripts/kane-summary.mjs <file.ndjson> [...more]
import { readFileSync } from "node:fs";

for (const file of process.argv.slice(2)) {
  const events = readFileSync(file, "utf8")
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

  console.log(`=== ${file} ===`);
  for (const e of events) {
    if (e.step !== undefined && e.status !== "running") {
      console.log(`  [${e.status}] ${String(e.remark).slice(0, 110)}`);
    }
  }

  const end = events.find((e) => e.type === "run_end");
  if (!end) {
    console.log("  NO run_end: the run did not start or was cut off");
    continue;
  }
  console.log(`  status: ${end.status} | ${end.duration}s | ${Number(end.credits_consumed).toFixed(1)} credits`);
  console.log(`  reason: ${end.reason}`);
  for (const [name, m] of Object.entries(end.context?.memory ?? {})) {
    console.log(`  check "${m.condition}"`);
    console.log(`    name=${name} analyzer=${m.analyzer_type} must_be=${m.must_be ?? "-"}`);
    console.log(`    locators=${JSON.stringify(m.locators ?? null)} code=${m.code_js ?? "-"}`);
    console.log(`    ${m.reasoning}`);
  }
  if (end.verdict?.root_cause) {
    console.log(`  verdict: ${end.verdict.family}/${end.verdict.category}: ${end.verdict.root_cause}`);
  }
}
