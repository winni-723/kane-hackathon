// Summarize a `kane-cli testmd run --agent` NDJSON log: overall result, credits,
// and what every assertion actually inspected.
// Usage: node scripts/kane-testmd-summary.mjs <file.ndjson> [...more]
import { readFileSync } from "node:fs";

let untrusted = 0;
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
  let credits = 0;
  for (const e of events) {
    if (e.type !== "run_end") continue;
    credits += Number(e.credits_consumed ?? 0);
    console.log(`  step: ${e.status} | ${e.reason}`);
    for (const m of Object.values(e.context?.memory ?? {})) {
      const exact = Array.isArray(m.locators) && m.locators.length > 0 && m.code_js;
      if (!exact) untrusted += 1;
      console.log(`    ${exact ? "exact " : "VISUAL"} ${m.condition} => ${m.reasoning}`);
      if (exact) console.log(`           ${m.locators[0]} :: ${m.code_js}`);
    }
  }
  const done = events.find((e) => e.type === "test_md_done");
  console.log(
    done
      ? `  overall: ${done.overall_status} | ${done.duration_s}s | ${credits.toFixed(1)} credits`
      : "  NO test_md_done: the test did not finish",
  );
}
if (untrusted > 0) {
  console.log(`\n${untrusted} check(s) were judged from a screenshot, not by exact text comparison`);
  process.exit(1);
}
