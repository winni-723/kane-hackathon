import { SAMPLE_BATCH, SAMPLE_RECORD } from "./samples";

interface Extracted {
  age: string | null;
  age_source: "stated" | "dob" | null;
  sex: "F" | "M" | null;
  diagnoses: string[];
  medications: string[];
}

interface ProcessResponse {
  text: string;
  extracted: Extracted;
}

function mustGet<T extends HTMLElement>(id: string): T {
  const el = document.getElementById(id);
  if (!el) throw new Error(`missing element #${id}`);
  return el as T;
}

const inputPanel = mustGet<HTMLElement>("input-panel");
const resultPanel = mustGet<HTMLElement>("result-panel");
const detailsPanel = mustGet<HTMLElement>("details-panel");
const batchPanel = mustGet<HTMLElement>("batch-panel");
const input = mustGet<HTMLTextAreaElement>("input");
const processButton = mustGet<HTMLButtonElement>("process");
const editButton = mustGet<HTMLButtonElement>("edit");
const downloadButton = mustGet<HTMLButtonElement>("download");
const output = mustGet<HTMLPreElement>("output");
const sampleButton = mustGet<HTMLButtonElement>("sample");
const sampleBatchButton = mustGet<HTMLButtonElement>("sample-batch");
const fileInput = mustGet<HTMLInputElement>("file");
const summary = mustGet<HTMLParagraphElement>("summary");
const detailsBody = mustGet<HTMLTableSectionElement>("details-body");
const backButton = mustGet<HTMLButtonElement>("back");
const batchBody = mustGet<HTMLTableSectionElement>("batch-body");
const csvButton = mustGet<HTMLButtonElement>("csv");
const restartButton = mustGet<HTMLButtonElement>("restart");

// The error message exists in the page only while there is an error, so an
// empty placeholder can never be mistaken for the result area.
let errorEl: HTMLParagraphElement | null = null;

function clearError(): void {
  errorEl?.remove();
  errorEl = null;
}

function showError(message: string): void {
  clearError();
  errorEl = document.createElement("p");
  errorEl.className = "error";
  errorEl.setAttribute("role", "alert");
  errorEl.textContent = message;
  inputPanel.append(errorEl);
}

// Records waiting to be processed together. While this is set, Process
// builds the research table instead of reading the text box.
let pendingBatch: string[] | null = null;
let batchNote: HTMLParagraphElement | null = null;
// The processed batch, kept so a record can be opened from the table.
let batchResults: ProcessResponse[] = [];

function setPendingBatch(records: string[] | null): void {
  pendingBatch = records;
  batchNote?.remove();
  batchNote = null;
  if (records) {
    batchNote = document.createElement("p");
    batchNote.className = "batch-note";
    batchNote.setAttribute("role", "status");
    batchNote.textContent = `${records.length} records loaded. Press Process to build the research table.`;
    processButton.before(batchNote);
    input.value = "";
  }
}

// --- the placeholders the engine writes ---------------------------------------

const CATEGORIES: Record<string, { css: string; one: string; many: string }> = {
  NAME: { css: "name", one: "name", many: "names" },
  PHONE: { css: "contact", one: "phone number", many: "phone numbers" },
  EMAIL: { css: "contact", one: "email address", many: "email addresses" },
  DATE: { css: "date", one: "date", many: "dates" },
  ADDRESS: { css: "place", one: "street address", many: "street addresses" },
  CITY: { css: "place", one: "town", many: "towns" },
  ZIP: { css: "place", one: "ZIP code", many: "ZIP codes" },
  LOCATION: { css: "place", one: "described location", many: "described locations" },
  FACILITY: { css: "place", one: "facility name", many: "facility names" },
  SSN: { css: "id", one: "Social Security number", many: "Social Security numbers" },
  MRN: { css: "id", one: "record number", many: "record numbers" },
  INSURANCE_ID: { css: "id", one: "insurance ID", many: "insurance IDs" },
  ID_NUMBER: { css: "id", one: "ID number", many: "ID numbers" },
};

const PLACEHOLDER = new RegExp(`\\[(${Object.keys(CATEGORIES).join("|")})\\]`, "g");

// Fills the Result with the same characters as `text`; placeholders are
// wrapped in a coloured span, which changes how they look and nothing else.
function renderResult(text: string): void {
  output.replaceChildren();
  let last = 0;
  for (const match of text.matchAll(PLACEHOLDER)) {
    output.append(text.slice(last, match.index));
    const tag = document.createElement("span");
    tag.className = `tag tag-${CATEGORIES[match[1]].css}`;
    tag.textContent = match[0];
    output.append(tag);
    last = match.index + match[0].length;
  }
  output.append(text.slice(last));
}

function removedSummary(text: string): string {
  const counts = new Map<string, number>();
  for (const match of text.matchAll(PLACEHOLDER)) {
    counts.set(match[1], (counts.get(match[1]) ?? 0) + 1);
  }
  const total = [...counts.values()].reduce((a, b) => a + b, 0);
  if (total === 0) return "No identifiers were found in this record.";
  const parts = [...counts].map(([kind, n]) => `${n} ${n === 1 ? CATEGORIES[kind].one : CATEGORIES[kind].many}`);
  return `${total} ${total === 1 ? "identifier" : "identifiers"} removed: ${parts.join(", ")}.`;
}

// --- the research columns ------------------------------------------------------

function ageCell(e: Extracted): string {
  if (!e.age) return "";
  return e.age_source === "dob" ? `${e.age}*` : e.age;
}

function columns(e: Extracted): string[] {
  return [ageCell(e), e.sex ?? "", e.diagnoses.join("; "), e.medications.join("; ")];
}

function addCells(row: HTMLTableRowElement, values: string[]): void {
  for (const value of values) {
    const cell = row.insertCell();
    cell.textContent = value;
  }
}

// --- panels: only one of input, single result and table is on screen ----------

function showOnly(...panels: HTMLElement[]): void {
  for (const panel of [inputPanel, resultPanel, detailsPanel, batchPanel]) {
    panel.hidden = !panels.includes(panel);
  }
}

function showResult(result: ProcessResponse, fromTable: boolean): void {
  renderResult(result.text);
  summary.textContent = removedSummary(result.text);
  detailsBody.replaceChildren();
  addCells(detailsBody.insertRow(), columns(result.extracted));
  backButton.hidden = !fromTable;
  showOnly(resultPanel, detailsPanel);
}

function showTable(): void {
  batchBody.replaceChildren();
  batchResults.forEach((result, index) => {
    const row = batchBody.insertRow();
    addCells(row, [`Record ${index + 1}`, ...columns(result.extracted)]);
    const view = document.createElement("button");
    view.type = "button";
    view.textContent = "View";
    view.setAttribute("aria-label", `View record ${index + 1}`);
    view.addEventListener("click", () => showResult(result, true));
    row.insertCell().append(view);
  });
  showOnly(batchPanel);
}

function showInput(): void {
  output.replaceChildren();
  batchResults = [];
  showOnly(inputPanel);
  input.focus();
}

async function processText(text: string): Promise<ProcessResponse> {
  const res = await fetch("/api/process", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ text }),
  });
  if (!res.ok) throw new Error(`Server returned ${res.status}`);
  return (await res.json()) as ProcessResponse;
}

processButton.addEventListener("click", async () => {
  clearError();
  processButton.disabled = true;
  try {
    if (pendingBatch) {
      batchResults = await Promise.all(pendingBatch.map(processText));
      setPendingBatch(null);
      showTable();
    } else {
      showResult(await processText(input.value), false);
    }
  } catch (e) {
    showError(e instanceof Error ? e.message : "Request failed");
  } finally {
    processButton.disabled = false;
  }
});

// --- starting points ------------------------------------------------------------

const MAX_FILE_BYTES = 1_000_000;
const MAX_FILES = 200;

sampleButton.addEventListener("click", () => {
  clearError();
  setPendingBatch(null);
  input.value = SAMPLE_RECORD;
  input.focus();
});

sampleBatchButton.addEventListener("click", () => {
  clearError();
  setPendingBatch(SAMPLE_BATCH);
});

input.addEventListener("input", () => setPendingBatch(null));

const MAX_DOCUMENT_BYTES = 10_000_000;

// The kind of document a file is, or null for one that cannot be loaded.
// "text" is read here in the browser; the others are read by the server.
function fileKind(file: File): "text" | "docx" | null {
  if (/\.docx$/i.test(file.name)) return "docx";
  if (file.type === "text/plain" || (file.type === "" && /\.txt$/i.test(file.name))) return "text";
  return null;
}

// The plain text of one file. A Word document is sent to the server to be
// read into text; its name is not sent, since file names often hold a
// patient's name.
async function readFile(file: File): Promise<string> {
  const kind = fileKind(file);
  if (kind === "text") return file.text();
  const response = await fetch(`/api/read-file/${kind}`, {
    method: "POST",
    headers: { "content-type": "application/octet-stream" },
    body: file,
  });
  const body: { text?: string; error?: string } = await response.json();
  if (!response.ok || body.text === undefined) {
    throw new Error(body.error ?? "A file could not be read.");
  }
  return body.text;
}

// A text file only fills the text box or the waiting batch. A Word document
// is read by the server first; either way nothing is de-identified or shown
// until Process is pressed.
fileInput.addEventListener("change", async () => {
  clearError();
  const files = [...(fileInput.files ?? [])];
  fileInput.value = "";
  if (files.length === 0) return;
  if (files.length > MAX_FILES) {
    showError(`Choose at most ${MAX_FILES} files at a time.`);
    return;
  }
  for (const file of files) {
    const kind = fileKind(file);
    if (kind === null) {
      showError("Only plain text (.txt) and Word (.docx) files can be loaded.");
      return;
    }
    if (kind === "text" && file.size > MAX_FILE_BYTES) {
      showError("A text file is larger than 1 MB.");
      return;
    }
    if (file.size > MAX_DOCUMENT_BYTES) {
      showError("A document is larger than 10 MB.");
      return;
    }
  }
  try {
    const texts = await Promise.all(files.map(readFile));
    if (texts.length === 1) {
      setPendingBatch(null);
      input.value = texts[0];
      input.focus();
    } else {
      setPendingBatch(texts);
    }
  } catch (error) {
    showError(error instanceof Error && error.message ? error.message : "A file could not be read.");
  }
});

// --- downloads ------------------------------------------------------------------

function save(content: string, type: string, name: string): void {
  const url = URL.createObjectURL(new Blob([content], { type }));
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  document.body.append(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
}

// Saves exactly what the Result shows, so the file can never contain text
// that was not on screen.
downloadButton.addEventListener("click", () => {
  save(output.textContent ?? "", "text/plain;charset=utf-8", "deidentified-record.txt");
});

function csvField(value: string): string {
  // A leading =, +, - or @ would be run as a formula by a spreadsheet.
  const safe = /^[=+\-@]/.test(value) ? `'${value}` : value;
  return `"${safe.replaceAll('"', '""')}"`;
}

csvButton.addEventListener("click", () => {
  const header = ["record", "age", "age_source", "sex", "diagnoses", "medications", "deidentified_text"];
  const rows = batchResults.map((result, index) => {
    const e = result.extracted;
    return [
      `Record ${index + 1}`,
      e.age ?? "",
      e.age_source ?? "",
      e.sex ?? "",
      e.diagnoses.join("; "),
      e.medications.join("; "),
      result.text,
    ];
  });
  const csv = [header, ...rows].map((row) => row.map(csvField).join(",")).join("\r\n");
  // The byte-order mark makes spreadsheets read the file as UTF-8.
  save(`﻿${csv}\r\n`, "text/csv;charset=utf-8", "research-table.csv");
});

editButton.addEventListener("click", showInput);
restartButton.addEventListener("click", showInput);
backButton.addEventListener("click", showTable);
