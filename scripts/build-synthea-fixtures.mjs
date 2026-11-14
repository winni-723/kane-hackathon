// Turns Synthea output into held-out fixtures with answer keys.
//
//   node scripts/build-synthea-fixtures.mjs <synthea-output-dir> <fixtures-dir>
//
// Synthea (https://github.com/synthetichealth/synthea) generates synthetic
// patients. Its text export carries only the name and birth date, so each
// fixture is:
//   - Synthea's own text header, verbatim (name, underline, demographics)
//   - the remaining identifiers from patients.csv, one per line, labelled with
//     Synthea's own CSV column names (SSN, Drivers, Passport, Address, City,
//     State, Zip)
//   - the first lines of Synthea's ALLERGIES, MEDICATIONS and CONDITIONS
//     sections, verbatim
//
// The answer key comes straight from the CSV values, so it does not depend on
// anyone's reading of the record. Every value is synthetic.
//
// To regenerate the same patients (Synthea v4.0.0, Java 17+):
//   java -jar synthea-with-dependencies.jar -s 20261114 -cs 20261114 -r 20261101 -p 50 \
//     --exporter.text.export=true --exporter.csv.export=true --exporter.fhir.export=false \
//     --exporter.hospital.fhir.export=false --exporter.practitioner.fhir.export=false \
//     --generate.append_numbers_to_person_names=false Massachusetts
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [synthea, outDir] = process.argv.slice(2);
if (!synthea || !outDir) {
  console.error("usage: node scripts/build-synthea-fixtures.mjs <synthea-output-dir> <fixtures-dir>");
  process.exit(2);
}

const SECTION_LINES = { ALLERGIES: 6, MEDICATIONS: 10, CONDITIONS: 10 };
const KEEP_PER_SECTION = 3;

function parseCsv(text) {
  const rows = [];
  let row = [];
  let cell = "";
  let quoted = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) {
      if (c === '"' && text[i + 1] === '"') { cell += '"'; i++; }
      else if (c === '"') quoted = false;
      else cell += c;
    } else if (c === '"') quoted = true;
    else if (c === ",") { row.push(cell); cell = ""; }
    else if (c === "\n") { row.push(cell); rows.push(row); row = []; cell = ""; }
    else if (c !== "\r") cell += c;
  }
  if (cell !== "" || row.length > 0) { row.push(cell); rows.push(row); }
  const [header, ...body] = rows;
  return body.filter((r) => r.length === header.length)
    .map((r) => Object.fromEntries(header.map((h, i) => [h, r[i]])));
}

const patients = new Map(
  parseCsv(readFileSync(join(synthea, "csv", "patients.csv"), "utf8")).map((p) => [p.Id, p]),
);

function sections(text) {
  const lines = text.split(/\r?\n/);
  const rule = (l) => /^-{20,}$/.test(l);
  const firstRule = lines.findIndex(rule);
  const header = lines.slice(0, firstRule);
  const out = { header, parts: {} };
  let name = null;
  for (const line of lines.slice(firstRule)) {
    if (rule(line)) { name = null; continue; }
    const m = /^([A-Z ]+):$/.exec(line);
    if (m) { name = m[1]; out.parts[name] = []; continue; }
    if (name && line.trim() !== "") out.parts[name].push(line);
  }
  return out;
}

// "  2026-10-19[CURRENT] : albuterol 0.83 MG/ML Inhalation Solution for Childhood asthma (disorder)"
// -> "albuterol 0.83 MG/ML Inhalation Solution"
const describe = (line) => line.split(" : ").slice(1).join(" : ").split(" for ")[0].trim();

rmSync(outDir, { recursive: true, force: true });
mkdirSync(outDir, { recursive: true });

const files = readdirSync(join(synthea, "text")).filter((f) => f.endsWith(".txt")).sort();
let written = 0;
for (const file of files) {
  const id = /_([0-9a-f-]{36})\.txt$/.exec(file)?.[1];
  const patient = patients.get(id);
  if (!patient) continue;

  const { header, parts } = sections(readFileSync(join(synthea, "text", file), "utf8"));
  const label = (k, v) => `${`${k}:`.padEnd(21)}${v}`;
  const extra = [
    label("SSN", patient.SSN),
    patient.DRIVERS && label("Drivers", patient.DRIVERS),
    patient.PASSPORT && label("Passport", patient.PASSPORT),
    label("Address", patient.ADDRESS),
    label("City", patient.CITY),
    label("State", patient.STATE),
    label("Zip", patient.ZIP),
  ].filter(Boolean);

  const body = [];
  const keep = [];
  for (const [section, max] of Object.entries(SECTION_LINES)) {
    const lines = (parts[section] ?? []).slice(0, max);
    if (lines.length === 0) continue;
    body.push("-".repeat(80), `${section}:`, ...lines);
    for (const d of [...new Set(lines.map(describe))].slice(0, KEEP_PER_SECTION)) {
      if (d.length >= 6) keep.push(d);
    }
  }
  const text = [...header, ...extra, ...body].join("\n");

  const pii = [];
  const add = (type, value) => {
    if (value && text.includes(value) && !pii.some((p) => p.text === value)) pii.push({ type, text: value });
  };
  // The name line is "First [Middle] Last"; each part is a separate identifier.
  for (const part of [patient.FIRST, patient.MIDDLE, patient.LAST]) add("name", part);
  add("dob", patient.BIRTHDATE);
  add("ssn", patient.SSN);
  add("id_number", patient.DRIVERS);
  add("id_number", patient.PASSPORT);
  const unit = /^(.*?)\s+((?:Apt|Unit|Suite)\s+\S+)$/.exec(patient.ADDRESS);
  if (unit) { add("address", unit[1]); add("address", unit[2]); } else add("address", patient.ADDRESS);
  add("address", patient.CITY);
  add("address", patient.ZIP);

  // A must_keep string may not overlap an identifier (e.g. a town inside a term).
  const mustKeep = [...new Set(keep)].filter(
    (k) => text.includes(k) && !pii.some((p) => k.includes(p.text) || p.text.includes(k)),
  );
  if (mustKeep.length === 0) continue;

  written += 1;
  const record_id = `easy-s${String(written).padStart(2, "0")}`;
  writeFileSync(
    join(outDir, `${record_id}.json`),
    JSON.stringify({ record_id, difficulty: "easy", source: "synthea", text, planted_pii: pii, must_keep: mustKeep }, null, 2) + "\n",
  );
}
console.log(`wrote ${written} fixtures to ${outDir}`);
