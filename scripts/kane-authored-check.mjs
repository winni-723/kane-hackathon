// After authoring new tests with `kane-cli testrun run`, check each one:
// did it pass, and was every answer-key check an exact text comparison?
// Usage: node scripts/kane-authored-check.mjs <suite.ndjson>
// Exit 1 if any test failed or used a screenshot for an answer-key check.
import { existsSync, readFileSync } from "node:fs";
import { basename } from "node:path";

const parse = (file) =>
  readFileSync(file, "utf8")
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

const ends = parse(process.argv[2]).filter(
  (e) => e.type === "testrun_member_end" || e.type === "testrun_authored_member_end",
);
let bad = 0;
let credits = 0;
for (const m of ends) {
  let exact = 0;
  let visual = 0;
  let cost = 0;
  if (m.log_path && existsSync(m.log_path)) {
    for (const e of parse(m.log_path)) {
      if (e.type !== "run_end") continue;
      cost += Number(e.credits_consumed ?? 0);
      for (const c of Object.values(e.context?.memory ?? {})) {
        if (!/contains? the text/.test(c.condition ?? "")) continue;
        if (Array.isArray(c.locators) && c.locators.length > 0 && c.code_js) exact += 1;
        else visual += 1;
      }
    }
  }
  credits += cost;
  const ok = m.status === "passed" && visual === 0 && exact > 0;
  if (!ok) bad += 1;
  console.log(
    `  ${ok ? "ok  " : "REDO"}  ${basename(m.path).padEnd(34)} ${m.status.padEnd(7)} exact ${exact} visual ${visual}  ${cost.toFixed(1)} credits`,
  );
}
console.log(`  ${ends.length} tests, ${bad} to redo, ${credits.toFixed(1)} credits in steps`);
process.exit(bad > 0 ? 1 : 0);
