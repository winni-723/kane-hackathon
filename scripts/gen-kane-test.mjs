// Compiles one answer key into one Kane test.
//
//   node scripts/gen-kane-test.mjs <record_id> <pii_type>
//
// Writes tests/kane/<record_id>-<pii_type>_test.md. The test submits the
// record through the page and checks, against the answer key, that every
// planted item of that type is gone from the Result and every must_keep
// string is still there.
//
// An existing test is never overwritten: once a Kane test exists it is part
// of the regression suite and must not change.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [recordId, piiType] = process.argv.slice(2);
if (!recordId || !piiType) {
  console.error("usage: node scripts/gen-kane-test.mjs <record_id> <pii_type>");
  process.exit(2);
}

// The record may come from the development set or from a held-out set.
const fixtureDir = ["tests/fixtures", "tests/heldout", "tests/synthea"].find((d) =>
  existsSync(join(d, `${recordId}.json`)),
);
if (!fixtureDir) {
  console.error(`no fixture named ${recordId}`);
  process.exit(2);
}
const fixture = JSON.parse(readFileSync(join(fixtureDir, `${recordId}.json`), "utf8"));
const planted = fixture.planted_pii.filter((p) => p.type === piiType).map((p) => p.text);
if (planted.length === 0) {
  console.error(`${recordId} has no planted PII of type "${piiType}"`);
  process.exit(2);
}

const outDir = "tests/kane";
const outFile = join(outDir, `${recordId}-${piiType}_test.md`);
if (existsSync(outFile)) {
  console.error(`${outFile} already exists; existing Kane tests are never rewritten`);
  process.exit(2);
}

// Quote a literal for the objective with whichever quote mark it does not contain.
function quote(s) {
  if (!s.includes("'")) return `'${s}'`;
  if (!s.includes('"')) return `"${s}"`;
  throw new Error(`cannot quote a string containing both quote marks: ${s}`);
}

const label = piiType.replace(/_/g, " ");
const recordBlock = fixture.text
  .split("\n")
  .map((line) => (line === "" ? "" : `      ${line}`))
  .join("\n");

// The kept-checks and removed-checks share one step on purpose. When a step
// holds only "does not contain" checks, Kane judges them from a screenshot;
// alongside "contains" checks it does an exact text comparison on the Result
// region for both.
const gone = planted
  .map((s) => `Verify that the Result region does not contain the text ${quote(s)}.`)
  .join("\n");
const kept = fixture.must_keep
  .map((s) => `Verify that the Result region contains the text ${quote(s)}.`)
  .join("\n");

const test = `---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [${piiType}, ${fixture.difficulty}]
variables:
  record:
    value: |-
${recordBlock}
---

# ${recordId}: ${label} is removed, medical content is kept

Generated from ${fixtureDir}/${recordId}.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ${label} is gone
${kept}
${gone}
`;

mkdirSync(outDir, { recursive: true });
writeFileSync(outFile, test);
console.log(`wrote ${outFile} (${planted.length} removed-checks, ${fixture.must_keep.length} kept-checks)`);
