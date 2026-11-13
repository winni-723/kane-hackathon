interface ProcessResponse {
  text: string;
}

function mustGet<T extends HTMLElement>(id: string): T {
  const el = document.getElementById(id);
  if (!el) throw new Error(`missing element #${id}`);
  return el as T;
}

const input = mustGet<HTMLTextAreaElement>("input");
const button = mustGet<HTMLButtonElement>("process");
const output = mustGet<HTMLPreElement>("output");
const error = mustGet<HTMLParagraphElement>("error");

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

button.addEventListener("click", async () => {
  error.textContent = "";
  button.disabled = true;
  try {
    output.textContent = await processText(input.value);
  } catch (e) {
    output.textContent = "";
    error.textContent = e instanceof Error ? e.message : "Request failed";
  } finally {
    button.disabled = false;
  }
});
