// Layer 1: the engine built for the browser must answer exactly as the native
// engine does, on every test input (pasted records, Word, CSV, PDF, the scan).
//
//   node scripts/wasm-matches-native.mjs <native.json>
//
// <native.json> comes from `cargo run --example native_outputs`; the browser
// engine is web/src/engine-wasm/ from scripts/build-wasm.sh. Also feeds it
// broken PDFs, which must come back as an error, not a crash.
import { readFileSync, readdirSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";
import { initSync, process as deidentify, read_file } from "../web/src/engine-wasm/engine.js";

initSync({ module: readFileSync(new URL("../web/src/engine-wasm/engine_bg.wasm", import.meta.url)) });
const TODAY = [2026, 11, 14]; // the day examples/native_outputs.rs uses

const native = JSON.parse(readFileSync(process.argv[2], "utf8"));
const processed = (records) => ({ records: records.map((t) => JSON.parse(deidentify(t, ...TODAY))) });

const browser = {};
for (const dir of ["tests/fixtures", "tests/heldout", "tests/synthea"])
  for (const f of readdirSync(dir).filter((f) => f.endsWith(".json")))
    browser[`${dir}/${f}`] = processed([JSON.parse(readFileSync(`${dir}/${f}`, "utf8")).text]);
for (const dir of ["tests/documents", "tests/documents/pdf"])
  for (const f of readdirSync(dir).filter((f) => /\.(docx|csv|pdf)$/.test(f))) {
    const read = JSON.parse(read_file(f.split(".").pop(), readFileSync(`${dir}/${f}`)));
    browser[`${dir}/${f}`] = read.error !== undefined ? { error: read.error } : processed(read.records);
  }

let bad = 0;
for (const k of new Set([...Object.keys(native), ...Object.keys(browser)])) {
  if (!isDeepStrictEqual(native[k], browser[k])) {
    bad += 1;
    console.log(`  DIFFERENT  ${k}`);
  }
}

// A broken PDF must be an error the page can show, not a crash of the engine.
const pdf = readFileSync("tests/documents/pdf/pdf-01.pdf");
for (const [name, bytes] of [
  ["truncated PDF", pdf.subarray(0, 4000)],
  ["PDF header then junk", Buffer.concat([Buffer.from("%PDF-1.7\n"), Buffer.alloc(5000, 7)])],
]) {
  let answer;
  try {
    answer = JSON.parse(read_file("pdf", bytes));
  } catch (e) {
    answer = { crash: String(e) };
  }
  if (typeof answer.error !== "string") {
    bad += 1;
    console.log(`  ${name}: expected an error, got ${JSON.stringify(answer).slice(0, 120)}`);
  }
}

const inputs = Object.keys(native).length;
console.log(`browser engine vs native: ${inputs} inputs, ${bad} problem(s)`);
process.exit(bad === 0 && inputs > 0 ? 0 : 1);
