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
