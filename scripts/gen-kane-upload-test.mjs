// Compiles one document answer key into one Kane test of the upload path.
//
//   node scripts/gen-kane-upload-test.mjs <document_id>
//
// Writes tests/kane/upload-<document_id>_test.md. The test picks the document
// from the page's "Load sample file" menu (web/public/samples/ holds the very
// test files, checked byte for byte by tests/pdf_test.rs), so the file goes
// through the same upload request a file the visitor chooses does. Then:
//
//   Word / PDF   Process, and check against the answer key that every planted
//                identifier is gone from the Result and every must_keep string
//                is still there.
//   CSV          Process, and check the research table: every diagnosis and
//                medication of the key's table is shown, no planted identifier.
//   scan         the page says a scan cannot be read, and shows no Result.
//
// An existing test is never overwritten: once a Kane test exists it is part
// of the regression suite and must not change.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

// The page's sample menu: document id -> the option the visitor sees.
const SAMPLES = {
  "docx-03": { option: "Clinic note (Word)", key: "tests/documents/keys/docx-03.json" },
  "csv-01": { option: "Patient list (CSV)", key: "tests/documents/keys/csv-01.json" },
  "pdf-01": { option: "Lab report (PDF)", key: "tests/documents/pdf/keys/pdf-01.json" },
  "pdf-06": { option: "Scanned fax (PDF, no text)", key: null },
};

const [docId] = process.argv.slice(2);
const sample = SAMPLES[docId];
if (!sample) {
  console.error(`usage: node scripts/gen-kane-upload-test.mjs <${Object.keys(SAMPLES).join("|")}>`);
  process.exit(2);
}

const outDir = "tests/kane";
const outFile = join(outDir, `upload-${docId}_test.md`);
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

const load = `Select '${sample.option}' in the sample file menu and click the Load sample file button.`;
const header = (title, source, tags) => `---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [${tags.join(", ")}]
---

# upload-${docId}: ${title}

Generated from ${source} by scripts/gen-kane-upload-test.mjs.
Do not edit: this test is part of the regression suite.
`;

let test;
let summary;
if (sample.key === null) {
  test = `${header("a scanned PDF is refused with a clear message", "the page's sample menu", ["upload", "pdf", "scan"])}
## Load the scanned sample
${load}
Verify that the page contains the text 'Scanned documents cannot be read here'.
Verify that the Result heading is not visible.
`;
  summary = "scan refusal";
} else {
  const key = JSON.parse(readFileSync(sample.key, "utf8"));
  const planted = [...new Set(key.planted_pii.map((p) => p.text))];
  const tags = ["upload", key.format, key.difficulty];
  // The kept-checks and removed-checks share one step on purpose: alongside
  // "contains" checks Kane compares the text exactly, not from a screenshot.
  if (key.format === "csv") {
    const kept = [...new Set(key.table.flatMap((r) => [...r.diagnoses, ...r.medications]))];
    const checks = [
      ...kept.map((s) => `Verify that the Research table region contains the text ${quote(s)}.`),
      ...planted.map((s) => `Verify that the Research table region does not contain the text ${quote(s)}.`),
    ];
    test = `${header("every row of a CSV file reaches the research table, de-identified", sample.key, tags)}
## Load the sample file
${load}
Verify that the page contains the text '${key.rows} records loaded'.

## Build the research table
Click the Process button.
Verify that the Research table heading is visible.

## Diagnoses and medications are shown and planted identifiers are not
${checks.join("\n")}
`;
    summary = `${kept.length} kept-checks, ${planted.length} removed-checks`;
  } else {
    const checks = [
      ...key.must_keep.map((s) => `Verify that the Result region contains the text ${quote(s)}.`),
      ...planted.map((s) => `Verify that the Result region does not contain the text ${quote(s)}.`),
    ];
    test = `${header(`a ${key.format === "pdf" ? "PDF" : "Word"} file is de-identified, medical content is kept`, sample.key, tags)}
## Load the sample file
${load}
Verify that the medical record text box contains the text ${quote(key.must_keep[0])}.

## Process it
Click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted identifiers are gone
${checks.join("\n")}
`;
    summary = `${key.must_keep.length} kept-checks, ${planted.length} removed-checks`;
  }
}

mkdirSync(outDir, { recursive: true });
writeFileSync(outFile, test);
console.log(`wrote ${outFile} (${summary})`);
