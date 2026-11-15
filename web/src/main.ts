interface ProcessResponse {
  text: string;
}

function mustGet<T extends HTMLElement>(id: string): T {
  const el = document.getElementById(id);
  if (!el) throw new Error(`missing element #${id}`);
  return el as T;
}

const inputPanel = mustGet<HTMLElement>("input-panel");
const resultPanel = mustGet<HTMLElement>("result-panel");
const input = mustGet<HTMLTextAreaElement>("input");
const processButton = mustGet<HTMLButtonElement>("process");
const editButton = mustGet<HTMLButtonElement>("edit");
const downloadButton = mustGet<HTMLButtonElement>("download");
const output = mustGet<HTMLPreElement>("output");
const sampleButton = mustGet<HTMLButtonElement>("sample");
const fileInput = mustGet<HTMLInputElement>("file");

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

function showResult(text: string): void {
  output.textContent = text;
  inputPanel.hidden = true;
  resultPanel.hidden = false;
}

function showInput(): void {
  output.textContent = "";
  resultPanel.hidden = true;
  inputPanel.hidden = false;
  input.focus();
}

async function processText(text: string): Promise<string> {
  const res = await fetch("/api/process", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ text }),
  });
  if (!res.ok) throw new Error(`Server returned ${res.status}`);
  const data = (await res.json()) as ProcessResponse;
  return data.text;
}

processButton.addEventListener("click", async () => {
  clearError();
  processButton.disabled = true;
  try {
    showResult(await processText(input.value));
  } catch (e) {
    showError(e instanceof Error ? e.message : "Request failed");
  } finally {
    processButton.disabled = false;
  }
});

// An invented record (the same text as tests/fixtures/medium-01.json), so a
// visitor can try the page without having a record of their own.
const SAMPLE_RECORD = `Progress Note

Grace Whitfield is a 72-year-old woman seen today for follow-up of atrial fibrillation. She was born on April 9, 1953 and lives alone at 18 Larkspur Lane in Dunmore Heights. Her daughter Mary called the clinic from 555-0198 this morning to ask whether the Warfarin dose should change.

INR today is 2.4, within the target range. Continue Warfarin 5 mg daily and Metoprolol 25 mg twice daily. No bleeding or bruising reported. Recheck INR in 4 weeks. If Grace cannot be reached, call her at (508) 555-0114.`;

const MAX_FILE_BYTES = 1_000_000;

sampleButton.addEventListener("click", () => {
  clearError();
  input.value = SAMPLE_RECORD;
  input.focus();
});

// The file is read in the browser and only fills the text box; nothing is
// sent until Process is pressed.
fileInput.addEventListener("change", async () => {
  clearError();
  const file = fileInput.files?.[0];
  fileInput.value = "";
  if (!file) return;
  const isText = file.type === "text/plain" || (file.type === "" && /\.txt$/i.test(file.name));
  if (!isText) {
    showError("Only plain text (.txt) files can be loaded.");
    return;
  }
  if (file.size > MAX_FILE_BYTES) {
    showError("That file is larger than 1 MB.");
    return;
  }
  try {
    input.value = await file.text();
    input.focus();
  } catch {
    showError("The file could not be read.");
  }
});

// Saves exactly what the Result shows, so the file can never contain text
// that was not on screen.
function downloadResult(): void {
  const blob = new Blob([output.textContent ?? ""], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = "deidentified-record.txt";
  document.body.append(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
}

downloadButton.addEventListener("click", downloadResult);
editButton.addEventListener("click", showInput);
