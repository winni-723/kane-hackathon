import { assureRecords, processRecord, readDocument } from "./engine";
import { readScan } from "./ocr";
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

// The engine's assurance report (src/assurance.rs). It holds counts,
// categories and line numbers, never record text.
interface Located {
  record: number;
  line: number;
  category: string;
  action: string;
}

interface AssuranceReport {
  report_format: string;
  generated_on: string;
  engine: { version: string; rules_version: string; method: string };
  dataset: { records: number; deidentified_sha256?: string };
  replaced: Record<string, number>;
  residual: { high: number; medium: number; findings: (Located & { severity: string })[]; note: string };
  privacy: {
    quasi_identifiers: (Located & { level: string })[];
    group_sizes: { records_counted: number; smallest_group: number | null; records_alone: number } | null;
    assumptions: string[];
  };
  utility: {
    completeness: { rows: number; age: number; sex: number; diagnoses: number; medications: number };
    records_with_every_line_kept: number;
    lowest_text_kept_percent: number;
  };
  review: { status: string; release: string };
  disclaimer: string;
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
const dropzone = mustGet<HTMLDivElement>("dropzone");
const fileInput = mustGet<HTMLInputElement>("file");
const sampleDocSelect = mustGet<HTMLSelectElement>("sample-doc");
const sampleDocButton = mustGet<HTMLButtonElement>("load-doc");
const summary = mustGet<HTMLParagraphElement>("summary");
const detailsBody = mustGet<HTMLTableSectionElement>("details-body");
const backButton = mustGet<HTMLButtonElement>("back");
const recordNav = mustGet<HTMLParagraphElement>("record-nav");
const prevButton = mustGet<HTMLButtonElement>("prev");
const nextButton = mustGet<HTMLButtonElement>("next");
const position = mustGet<HTMLSpanElement>("position");
const batchBody = mustGet<HTMLTableSectionElement>("batch-body");
const csvButton = mustGet<HTMLButtonElement>("csv");
const allRecordsButton = mustGet<HTMLButtonElement>("all-records");
const restartButton = mustGet<HTMLButtonElement>("restart");
const startOverButton = mustGet<HTMLButtonElement>("start-over");
const moreRecords = mustGet<HTMLDivElement>("more-records");
const firstNum = mustGet<HTMLSpanElement>("first-num");
const addRecordButton = mustGet<HTMLButtonElement>("add-record");
const assurancePanel = mustGet<HTMLElement>("assurance-panel");
const reviewStatus = mustGet<HTMLParagraphElement>("review-status");
const replacedEl = mustGet<HTMLParagraphElement>("replaced");
const residualSummary = mustGet<HTMLParagraphElement>("residual-summary");
const residualTable = mustGet<HTMLTableElement>("residual-table");
const residualBody = mustGet<HTMLTableSectionElement>("residual-body");
const groupsEl = mustGet<HTMLParagraphElement>("groups");
const quasiTable = mustGet<HTMLTableElement>("quasi-table");
const quasiBody = mustGet<HTMLTableSectionElement>("quasi-body");
const assumptionsEl = mustGet<HTMLUListElement>("assumptions");
const utilityEl = mustGet<HTMLUListElement>("utility");
const engineInfo = mustGet<HTMLParagraphElement>("engine-info");
const disclaimer = mustGet<HTMLParagraphElement>("disclaimer");
const reportButton = mustGet<HTMLButtonElement>("report");
const statsEl = mustGet<HTMLDivElement>("stats");
const stepItems = [...document.querySelectorAll<HTMLLIElement>("#steps li")];

// The error message exists in the page only while there is an error, so an
// empty placeholder can never be mistaken for the result area.
let errorEl: HTMLParagraphElement | null = null;

function clearError(): void {
  errorEl?.remove();
  errorEl = null;
  ocrOffer?.remove();
  ocrOffer = null;
  ocrNote?.remove();
  ocrNote = null;
}

function showError(message: string): void {
  clearError();
  errorEl = document.createElement("p");
  errorEl.className = "error";
  errorEl.setAttribute("role", "alert");
  errorEl.textContent = message;
  inputPanel.append(errorEl);
}

// Files loaded together, listed before Process so the visitor can check
// them, take one out, or see why one could not be read. A file's name is
// shown only in this list, on this device: it never reaches the table, the
// CSV or the report, since a file is often named after its patient.
interface LoadedFile {
  name: string;
  kind: "text" | "docx" | "csv" | "pdf" | null;
  records: string[];
  error: string | null;
  // Kept so a scan can still be read by OCR if the visitor asks.
  file: File;
  // The text was read from a scan by OCR.
  ocr: boolean;
}
let loadedFiles: LoadedFile[] = [];
// Records waiting to be processed together. While this is set, Process
// builds the research table instead of reading the text box.
let pendingBatch: string[] | null = null;
let batchNote: HTMLParagraphElement | null = null;
let fileList: HTMLElement | null = null;
// The processed batch, kept so a record can be opened from the table.
let batchResults: ProcessResponse[] = [];

const KIND_LABEL = { text: "Text", docx: "Word", csv: "CSV", pdf: "PDF" };

function setLoadedFiles(files: LoadedFile[]): void {
  loadedFiles = files;
  batchNote?.remove();
  batchNote = null;
  fileList?.remove();
  fileList = null;
  if (files.length === 0) {
    pendingBatch = null;
    return;
  }
  const records = files.flatMap((file) => file.records);
  pendingBatch = records;
  batchNote = document.createElement("p");
  batchNote.className = "batch-note";
  batchNote.setAttribute("role", "status");
  batchNote.textContent =
    records.length === 0
      ? "No records could be read from these files."
      : `${plural(records.length, "record", "records")} loaded. Press Process to build the research table.`;
  fileList = renderFileList(files);
  (processButton.parentElement ?? processButton).before(batchNote, fileList);
  input.value = "";
  clearRecordBoxes();
}

function clearLoadedFiles(): void {
  setLoadedFiles([]);
}

function renderFileList(files: LoadedFile[]): HTMLElement {
  const wrap = document.createElement("div");
  wrap.className = "file-list";
  wrap.setAttribute("role", "region");
  wrap.setAttribute("aria-label", "Loaded files");
  const failed = files.filter((file) => file.error !== null).length;
  const head = document.createElement("p");
  head.className = "file-list-head";
  head.textContent =
    plural(files.length, "file", "files") +
    (failed > 0 ? ` · ${failed} could not be read and will be left out` : "");
  const list = document.createElement("ul");
  for (const file of files) {
    const row = document.createElement("li");
    row.className = file.error === null ? "file-row" : "file-row failed";
    const name = document.createElement("span");
    name.className = "file-name";
    name.textContent = file.name;
    name.title = file.name;
    const kind = document.createElement("span");
    kind.className = "file-kind";
    kind.textContent = file.kind === null ? "?" : KIND_LABEL[file.kind];
    const info = document.createElement("span");
    info.className = "file-info";
    info.textContent =
      file.error ?? plural(file.records.length, "record", "records") + (file.ocr ? " · read by OCR" : "");
    const remove = document.createElement("button");
    remove.type = "button";
    remove.className = "remove";
    remove.textContent = "Remove";
    remove.setAttribute("aria-label", `Remove ${file.name}`);
    remove.addEventListener("click", () => {
      setLoadedFiles(loadedFiles.filter((other) => other !== file));
      input.focus();
    });
    row.append(kind, name, info);
    if (isScan(file)) {
      const read = document.createElement("button");
      read.type = "button";
      read.className = "read-ocr";
      read.textContent = "Read with OCR";
      read.setAttribute("aria-label", `Read ${file.name} with OCR`);
      read.addEventListener("click", async () => {
        read.disabled = true;
        remove.disabled = true;
        row.classList.remove("failed");
        read.textContent = "Reading…";
        info.textContent = "Reading the scan with OCR, in this browser. The first scan takes longer.";
        const done = await ocrFile(file);
        setLoadedFiles(loadedFiles.map((other) => (other === file ? done : other)));
      });
      row.append(read);
    }
    row.append(remove);
    list.append(row);
  }
  wrap.append(head, list);
  return wrap;
}

// --- reading scans by OCR -----------------------------------------------------
// A scanned PDF has no text, only a picture of the page. It is refused like
// any file that cannot be read, and the visitor may then choose to have the
// picture read by OCR, here in the browser. OCR can misread, so the text it
// gives is the visitor's to check.

const SCAN_MESSAGE = "Scanned documents cannot be read here";

function isScan(file: LoadedFile): boolean {
  return file.kind === "pdf" && file.error !== null && file.error.includes(SCAN_MESSAGE);
}

// The file once OCR has read it, or with the reason it could not.
async function ocrFile(file: LoadedFile): Promise<LoadedFile> {
  try {
    const text = await readScan(new Uint8Array(await file.file.arrayBuffer()));
    return { ...file, records: [text], error: null, ocr: true };
  } catch (error) {
    const why = error instanceof Error && error.message ? error.message : "The scan could not be read.";
    return { ...file, error: `OCR could not read this scan. ${why}` };
  }
}

// Offered under the error message when the one file loaded is a scan.
let ocrOffer: HTMLDivElement | null = null;
// Shown over the text box when the text in it was read by OCR.
let ocrNote: HTMLParagraphElement | null = null;

function offerOcr(file: LoadedFile): void {
  const offer = document.createElement("div");
  offer.className = "ocr-offer";
  const text = document.createElement("p");
  text.textContent =
    "You can have the picture read by OCR, in this browser. The scan is not sent anywhere. OCR can misread, so check the text before you share the result.";
  const read = document.createElement("button");
  read.type = "button";
  read.textContent = "Read the scan with OCR";
  read.addEventListener("click", async () => {
    read.disabled = true;
    read.textContent = "Reading…";
    text.textContent = "Reading the scan with OCR, in this browser. The first scan takes longer.";
    const done = await ocrFile(file);
    if (done.error !== null) {
      text.textContent = done.error;
      read.remove();
      return;
    }
    clearError();
    clearLoadedFiles();
    clearRecordBoxes();
    input.value = done.records[0];
    ocrNote = document.createElement("p");
    ocrNote.className = "ocr-note";
    ocrNote.setAttribute("role", "status");
    ocrNote.textContent = "This text was read from a scan by OCR. Check it against the scan, then press Process.";
    input.before(ocrNote);
    input.focus();
  });
  offer.append(text, read);
  ocrOffer = offer;
  (errorEl ?? inputPanel.lastElementChild ?? inputPanel).after(offer);
}

// --- the placeholders the engine writes ---------------------------------------

const CATEGORIES: Record<string, { css: string; one: string; many: string }> = {
  NAME: { css: "name", one: "name", many: "names" },
  PHONE: { css: "contact", one: "phone number", many: "phone numbers" },
  EMAIL: { css: "contact", one: "email address", many: "email addresses" },
  URL: { css: "contact", one: "web address", many: "web addresses" },
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

// The report on what was last processed, shown under the result or the
// research table until the visitor goes back to the input.
let report: AssuranceReport | null = null;

function showOnly(...panels: HTMLElement[]): void {
  for (const panel of [inputPanel, resultPanel, detailsPanel, batchPanel]) {
    panel.hidden = !panels.includes(panel);
  }
  // A new view starts at its top: after a long input, the page would
  // otherwise stay scrolled to the middle of the result.
  window.scrollTo(0, 0);
  assurancePanel.hidden = report === null || panels.includes(inputPanel);
  // The progress line: upload while the input is on screen, then review.
  markStep(panels.includes(inputPanel) ? "1" : "2");
}

// Upload, review, then download once something has been saved.
function markStep(step: "1" | "2" | "3"): void {
  for (const item of stepItems) {
    const current = item.dataset.step === step;
    item.classList.toggle("current", current);
    if (current) item.setAttribute("aria-current", "step");
    else item.removeAttribute("aria-current");
  }
}

// `index` is the record's place in the research table, or null for a record
// that was processed on its own.
let shownIndex: number | null = null;

function showResult(result: ProcessResponse, index: number | null): void {
  shownIndex = index;
  renderResult(result.text);
  summary.textContent = removedSummary(result.text);
  detailsBody.replaceChildren();
  addCells(detailsBody.insertRow(), columns(result.extracted));
  recordNav.hidden = index === null;
  if (index !== null) {
    position.textContent = `Record ${index + 1} of ${batchResults.length}`;
    prevButton.disabled = index === 0;
    nextButton.disabled = index === batchResults.length - 1;
  }
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
    view.addEventListener("click", () => showResult(result, index));
    row.insertCell().append(view);
  });
  showOnly(batchPanel);
}

function showInput(): void {
  output.replaceChildren();
  batchResults = [];
  report = null;
  showOnly(inputPanel);
  input.focus();
}

// Edit original keeps the text to change it; Start over begins again empty.
function startOver(): void {
  clearError();
  clearLoadedFiles();
  clearRecordBoxes();
  input.value = "";
  showInput();
}

// De-identified here in the browser, by the engine in a Web Worker: the
// record is not sent anywhere.
async function processText(text: string): Promise<ProcessResponse> {
  return JSON.parse(await processRecord(text)) as ProcessResponse;
}

// The report also carries a fingerprint of the de-identified output (never
// of the original, which could be used to confirm a guess at it), so a
// shared report can be matched to the file it describes.
async function buildReport(originals: string[], results: ProcessResponse[]): Promise<AssuranceReport> {
  const built = JSON.parse(await assureRecords(originals)) as AssuranceReport & { error?: string };
  if (built.error) throw new Error(built.error);
  const bytes = new TextEncoder().encode(results.map((r) => r.text).join("\n\u0000\n"));
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  built.dataset.deidentified_sha256 = [...digest].map((b) => b.toString(16).padStart(2, "0")).join("");
  return built;
}

function plural(n: number, one: string, many: string): string {
  return `${n} ${n === 1 ? one : many}`;
}

function fillRows(body: HTMLTableSectionElement, rows: string[][]): void {
  body.replaceChildren();
  for (const values of rows) addCells(body.insertRow(), values);
}

function listItems(list: HTMLUListElement, texts: string[]): void {
  list.replaceChildren(
    ...texts.map((text) => {
      const item = document.createElement("li");
      item.textContent = text;
      return item;
    }),
  );
}

// The headline numbers at the top of the report; the sentences below them
// say the same in full.
function renderStats(r: AssuranceReport): void {
  const replacedTotal = Object.values(r.replaced).reduce((a, b) => a + b, 0);
  const findings = r.residual.findings.length;
  const c = r.utility.completeness;
  const cells = c.rows * 4;
  const filled = c.age + c.sex + c.diagnoses + c.medications;
  const stats: [string, string, string][] = [
    ["Identifiers replaced", String(replacedTotal), ""],
    ["Possible leftovers", String(findings), findings === 0 ? "good" : "check"],
    ["Records with all lines kept",`${r.utility.records_with_every_line_kept}/${r.dataset.records}`, ""],
    ["Table cells filled", `${filled}/${cells}`, ""],
  ];
  statsEl.replaceChildren(
    ...stats.map(([label, value, tone]) => {
      const card = document.createElement("div");
      card.className = tone ? `stat ${tone}` : "stat";
      const l = document.createElement("p");
      l.className = "stat-label";
      l.textContent = label;
      const v = document.createElement("p");
      v.className = "stat-value";
      v.textContent = value;
      card.append(l, v);
      return card;
    }),
  );
}

function renderReport(r: AssuranceReport): void {
  renderStats(r);
  reviewStatus.textContent = `Review status: ${r.review.status}. Release: ${r.review.release}.`;

  const replaced = Object.entries(r.replaced).map(([kind, n]) =>
    CATEGORIES[kind] ? plural(n, CATEGORIES[kind].one, CATEGORIES[kind].many) : `${n} ${kind}`,
  );
  replacedEl.textContent = replaced.length === 0 ? "None." : `${replaced.join(", ")}.`;

  const findings = r.residual.findings;
  residualSummary.textContent =
    findings.length === 0
      ? `Nothing of the checked shapes was found. ${r.residual.note}`
      : `${r.residual.high} high, ${r.residual.medium} medium. ${r.residual.note}`;
  residualTable.hidden = findings.length === 0;
  fillRows(
    residualBody,
    findings.map((f) => [`Record ${f.record}`, String(f.line), f.category, f.severity, f.action]),
  );

  const g = r.privacy.group_sizes;
  groupsEl.textContent =
    g === null
      ? "Group sizes are counted for a set of more than one record."
      : g.smallest_group === null
        ? "No record has both an age and a sex in the table, so no groups were counted."
        : `Smallest group sharing a 10-year age band and sex: ${g.smallest_group} ` +
          `(${plural(g.records_alone, "record", "records")} alone in their group, of ${g.records_counted} counted).`;
  const quasi = r.privacy.quasi_identifiers;
  quasiTable.hidden = quasi.length === 0;
  fillRows(
    quasiBody,
    quasi.map((q) => [`Record ${q.record}`, q.line === 0 ? "table" : String(q.line), q.category, q.level, q.action]),
  );
  listItems(assumptionsEl, r.privacy.assumptions);

  const u = r.utility;
  const c = u.completeness;
  listItems(utilityEl, [
    `Records with every line kept: ${u.records_with_every_line_kept} of ${r.dataset.records}`,
    `Lowest share of original text kept in a record: ${u.lowest_text_kept_percent}%`,
    `Research table filled: age ${c.age}/${c.rows}, sex ${c.sex}/${c.rows}, ` +
      `diagnoses ${c.diagnoses}/${c.rows}, medications ${c.medications}/${c.rows}`,
  ]);

  engineInfo.textContent =
    `Engine ${r.engine.version}, rules ${r.engine.rules_version}. ` +
    `${plural(r.dataset.records, "record", "records")}, report of ${r.generated_on}.`;
  disclaimer.textContent = r.disclaimer;
}

processButton.addEventListener("click", async () => {
  clearError();
  if (pendingBatch !== null && pendingBatch.length === 0) {
    showError("None of the loaded files could be read. Remove them, or load other files.");
    return;
  }
  processButton.disabled = true;
  try {
    const pasted = pastedRecords();
    const batch = pendingBatch ?? (pasted.length > 1 ? pasted : null);
    if (batch) {
      // One at a time: the engine works through them in its own thread.
      const results: ProcessResponse[] = [];
      for (const record of batch) results.push(await processText(record));
      report = await buildReport(batch, results);
      renderReport(report);
      batchResults = results;
      clearLoadedFiles();
      showTable();
    } else {
      const text = pasted[0] ?? input.value;
      const result = await processText(text);
      report = await buildReport([text], [result]);
      renderReport(report);
      showResult(result, null);
    }
  } catch (e) {
    showError(e instanceof Error ? e.message : "The record could not be processed.");
  } finally {
    processButton.disabled = false;
  }
});

// --- starting points ------------------------------------------------------------

const MAX_FILE_BYTES = 1_000_000;
const MAX_FILES = 200;

input.addEventListener("input", () => clearLoadedFiles());

// --- pasting several records ---------------------------------------------------
// The first box is the one every Kane test types into; more boxes are added
// below it. Two or more filled boxes are processed as a set, like several
// files, into the research table.

const MAX_PASTED = 20;
let boxCount = 0;

function extraBoxes(): HTMLTextAreaElement[] {
  return [...moreRecords.querySelectorAll<HTMLTextAreaElement>("textarea")];
}

// Numbers the boxes 1, 2, 3... after one is added or removed.
function renumberBoxes(): void {
  const boxes = extraBoxes();
  boxes.forEach((box, index) => {
    const item = box.closest(".record-box");
    const label = item?.querySelector("label");
    const remove = item?.querySelector("button");
    if (label) label.textContent = `Record ${index + 2}`;
    remove?.setAttribute("aria-label", `Remove record ${index + 2}`);
  });
  firstNum.hidden = boxes.length === 0;
  addRecordButton.disabled = boxes.length + 1 >= MAX_PASTED;
}

function addRecordBox(text = ""): HTMLTextAreaElement {
  boxCount += 1;
  const id = `record-box-${boxCount}`;
  const item = document.createElement("div");
  item.className = "record-box";
  const head = document.createElement("div");
  head.className = "record-head";
  const label = document.createElement("label");
  label.htmlFor = id;
  label.className = "record-num";
  const remove = document.createElement("button");
  remove.type = "button";
  remove.className = "remove";
  remove.textContent = "Remove";
  const box = document.createElement("textarea");
  box.id = id;
  box.placeholder = "Paste another record here";
  box.value = text;
  box.addEventListener("input", () => clearLoadedFiles());
  remove.addEventListener("click", () => {
    item.remove();
    renumberBoxes();
    input.focus();
  });
  head.append(label, remove);
  item.append(head, box);
  moreRecords.append(item);
  renumberBoxes();
  return box;
}

function clearRecordBoxes(): void {
  moreRecords.replaceChildren();
  renumberBoxes();
}

// The text of every filled box, in order.
function pastedRecords(): string[] {
  return [input, ...extraBoxes()].map((box) => box.value).filter((text) => text.trim() !== "");
}

addRecordButton.addEventListener("click", () => addRecordBox().focus());

const MAX_DOCUMENT_BYTES = 10_000_000;
// A CSV file is one record per row, so one file can hold many.
const MAX_RECORDS = 1000;

// The kind of document a file is, or null for one that cannot be loaded.
// "text" is read as it is; the others are read by the engine.
function fileKind(file: File): "text" | "docx" | "csv" | "pdf" | null {
  if (/\.docx$/i.test(file.name)) return "docx";
  if (/\.csv$/i.test(file.name)) return "csv";
  if (/\.pdf$/i.test(file.name) || file.type === "application/pdf") return "pdf";
  if (file.type === "text/plain" || (file.type === "" && /\.txt$/i.test(file.name))) return "text";
  return null;
}

// The records in one file: one for a text file, a Word document or a PDF,
// one per row for a CSV file. Every file is read here in the browser, by the
// engine; neither the file nor its name is sent anywhere.
async function readFile(file: File): Promise<string[]> {
  const kind = fileKind(file);
  if (kind === "text") return [await file.text()];
  if (kind === null) throw new Error("This kind of file is not supported.");
  const body: { records?: string[]; error?: string } = JSON.parse(
    await readDocument(kind, new Uint8Array(await file.arrayBuffer())),
  );
  if (body.records === undefined) throw new Error(body.error ?? "A file could not be read.");
  return body.records;
}

// Loading a file only fills the text box or the waiting batch; nothing is
// de-identified or shown until Process is pressed.
fileInput.addEventListener("change", async () => {
  clearError();
  const files = [...(fileInput.files ?? [])];
  fileInput.value = "";
  await loadFiles(files);
});

// Files dropped on the upload area take the same path as chosen ones.
dropzone.addEventListener("dragover", (event) => {
  event.preventDefault();
  dropzone.classList.add("dragging");
});
dropzone.addEventListener("dragleave", () => dropzone.classList.remove("dragging"));
dropzone.addEventListener("drop", async (event) => {
  event.preventDefault();
  dropzone.classList.remove("dragging");
  clearError();
  await loadFiles([...(event.dataTransfer?.files ?? [])]);
});

// The samples are fictional. The sample documents are test files served with
// the page, and a chosen one goes through exactly the path a file chosen from
// disk does; the text samples go where pasted text or a batch of files would.
sampleDocButton.addEventListener("click", async () => {
  clearError();
  const name = sampleDocSelect.value;
  if (name === "inline:record") {
    clearLoadedFiles();
    clearRecordBoxes();
    input.value = SAMPLE_RECORD;
    input.focus();
    return;
  }
  // The three records go into three paste boxes: a set of pasted records.
  if (name === "inline:batch") {
    clearLoadedFiles();
    clearRecordBoxes();
    const [first, ...rest] = SAMPLE_BATCH;
    input.value = first;
    for (const record of rest) addRecordBox(record);
    return;
  }
  try {
    const response = await fetch(`samples/${name}`);
    if (!response.ok) throw new Error(`Server returned ${response.status}`);
    await loadFiles([new File([await response.blob()], name)]);
  } catch (error) {
    showError(error instanceof Error && error.message ? error.message : "The sample could not be loaded.");
  }
});

// One file, read on its own: a file that cannot be read says why, and the
// others are still loaded.
async function readOne(file: File): Promise<LoadedFile> {
  const kind = fileKind(file);
  const loaded = (records: string[], error: string | null): LoadedFile => ({
    name: file.name,
    kind,
    records,
    error,
    file,
    ocr: false,
  });
  if (kind === null) {
    return loaded([], "Only plain text (.txt), Word (.docx), CSV (.csv) and PDF (.pdf) files can be loaded.");
  }
  if (kind === "text" && file.size > MAX_FILE_BYTES) return loaded([], "A text file is larger than 1 MB.");
  if (file.size > MAX_DOCUMENT_BYTES) return loaded([], "A document is larger than 10 MB.");
  try {
    return loaded(await readFile(file), null);
  } catch (error) {
    return loaded([], error instanceof Error && error.message ? error.message : "A file could not be read.");
  }
}

// A single file with one record opens in the text box, as a pasted record
// would; a single file that cannot be read says why and empties the text
// box, so nothing loaded before it is left to process. Anything more joins
// the list of loaded files, which Process turns into the research table.
async function loadFiles(files: File[]): Promise<void> {
  if (files.length === 0) return;
  if (loadedFiles.length + files.length > MAX_FILES) {
    showError(`Choose at most ${MAX_FILES} files at a time.`);
    return;
  }
  const read = await Promise.all(files.map(readOne));
  if (loadedFiles.length === 0 && read.length === 1) {
    const [only] = read;
    if (only.error !== null) {
      // The text of an earlier record must not stay behind, to be processed
      // as if it came from this file.
      clearRecordBoxes();
      input.value = "";
      showError(only.error);
      if (isScan(only)) offerOcr(only);
      return;
    }
    if (only.records.length === 1) {
      clearLoadedFiles();
      clearRecordBoxes();
      input.value = only.records[0];
      input.focus();
      return;
    }
  }
  const all = [...loadedFiles, ...read];
  const total = all.reduce((n, file) => n + file.records.length, 0);
  if (total > MAX_RECORDS) {
    showError(`These files hold ${total} records; at most ${MAX_RECORDS} can be processed at a time.`);
    return;
  }
  setLoadedFiles(all);
}

// --- downloads ------------------------------------------------------------------

function save(content: string, type: string, name: string): void {
  markStep("3");
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

// Every de-identified record in one file. Like Download result, it holds
// only text the engine returned, never the original.
allRecordsButton.addEventListener("click", () => {
  const parts = batchResults.map(
    (result, index) => `===== Record ${index + 1} =====\r\n\r\n${result.text.replace(/\r?\n/g, "\r\n")}\r\n`,
  );
  save(parts.join("\r\n"), "text/plain;charset=utf-8", "deidentified-records.txt");
});

// The report as the engine built it, plus the output's fingerprint: counts,
// categories and line numbers, no record text.
reportButton.addEventListener("click", () => {
  if (report) save(`${JSON.stringify(report, null, 2)}\n`, "application/json", "assurance-report.json");
});

editButton.addEventListener("click", showInput);
restartButton.addEventListener("click", startOver);
startOverButton.addEventListener("click", startOver);
backButton.addEventListener("click", showTable);

function step(by: number): void {
  if (shownIndex === null) return;
  const index = shownIndex + by;
  if (index >= 0 && index < batchResults.length) showResult(batchResults[index], index);
}
prevButton.addEventListener("click", () => step(-1));
nextButton.addEventListener("click", () => step(1));
