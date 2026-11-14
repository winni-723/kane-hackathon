// Checks that the fixtures themselves are well-formed, so a broken answer key
// cannot silently distort every later test.
// Usage: node scripts/validate-fixtures.mjs [fixtures-dir]
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const dir = process.argv[2] ?? "tests/fixtures";
const DIFFICULTIES = ["easy", "medium", "hard"];
const PII_TYPES = [
  "name", "relative_name", "phone", "dob", "address", "ssn", "mrn", "email", "insurance_id",
];

const errors = [];
const counts = { easy: 0, medium: 0, hard: 0 };
const typeCounts = {};
const seenIds = new Set();
const files = readdirSync(dir).filter((f) => f.endsWith(".json")).sort();

for (const file of files) {
  const fail = (msg) => errors.push(`${file}: ${msg}`);
  let r;
  try {
    r = JSON.parse(readFileSync(join(dir, file), "utf8"));
  } catch (e) {
    fail(`invalid JSON: ${e.message}`);
    continue;
  }

  for (const key of ["record_id", "difficulty", "text", "planted_pii", "must_keep"]) {
    if (!(key in r)) fail(`missing field "${key}"`);
  }
  if (typeof r.text !== "string" || !Array.isArray(r.planted_pii) || !Array.isArray(r.must_keep)) {
    continue;
  }

  if (`${r.record_id}.json` !== file) fail(`record_id "${r.record_id}" does not match file name`);
  if (seenIds.has(r.record_id)) fail(`duplicate record_id "${r.record_id}"`);
  seenIds.add(r.record_id);

  if (!DIFFICULTIES.includes(r.difficulty)) fail(`unknown difficulty "${r.difficulty}"`);
  else counts[r.difficulty] += 1;
  if (!r.record_id.startsWith(`${r.difficulty}-`)) fail("record_id prefix does not match difficulty");

  if (r.planted_pii.length === 0) fail("planted_pii is empty");
  if (r.must_keep.length === 0) fail("must_keep is empty");

  const piiTexts = [];
  for (const item of r.planted_pii) {
    if (!PII_TYPES.includes(item?.type)) fail(`unknown PII type "${item?.type}"`);
    if (typeof item?.text !== "string" || item.text.trim() === "") {
      fail("planted_pii entry has no text");
      continue;
    }
    if (!r.text.includes(item.text)) fail(`planted_pii not found in text: "${item.text}"`);
    if (piiTexts.includes(item.text)) fail(`duplicate planted_pii: "${item.text}"`);
    piiTexts.push(item.text);
    typeCounts[item.type] = (typeCounts[item.type] ?? 0) + 1;

    // Every phone number written in digits must use the fictional 555-01xx range.
    if (item.type === "phone") {
      const digits = item.text.replace(/\D/g, "");
      if (digits.length >= 7 && !/55501\d\d$/.test(digits)) {
        fail(`phone is not in the fictional 555-01xx range: "${item.text}"`);
      }
    }
    if (item.type === "email" && !/@example\.(com|org|net)$/.test(item.text)) {
      fail(`email is not on a reserved example domain: "${item.text}"`);
    }
    // SSNs must come from ranges that are never issued.
    if (item.type === "ssn") {
      const d = item.text.replace(/\D/g, "");
      if (!(d.startsWith("000") || d.startsWith("9"))) {
        fail(`SSN is not from a never-issued range: "${item.text}"`);
      }
    }
  }

  const keepSeen = [];
  for (const keep of r.must_keep) {
    if (typeof keep !== "string" || keep.trim() === "") {
      fail("must_keep entry is empty");
      continue;
    }
    if (!r.text.includes(keep)) fail(`must_keep not found in text: "${keep}"`);
    if (keepSeen.includes(keep)) fail(`duplicate must_keep: "${keep}"`);
    keepSeen.push(keep);
    // A string cannot be both required to stay and required to go.
    for (const pii of piiTexts) {
      if (keep.includes(pii)) fail(`must_keep "${keep}" contains planted PII "${pii}"`);
      if (pii.includes(keep)) fail(`planted PII "${pii}" contains must_keep "${keep}"`);
    }
  }
}

console.log(`fixtures: ${files.length} (easy ${counts.easy}, medium ${counts.medium}, hard ${counts.hard})`);
console.log(`planted PII by type: ${JSON.stringify(typeCounts)}`);
if (errors.length > 0) {
  console.log(`\n${errors.length} problem(s):`);
  for (const e of errors) console.log(`  - ${e}`);
  process.exit(1);
}
console.log("all fixtures are well-formed");
