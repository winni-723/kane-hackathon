// The page's way into OCR: a worker started only when a visitor first asks
// for a scanned PDF to be read. If it fails, it is dropped and the next
// request starts a fresh one.
import type { OcrReply, OcrRequest } from "./ocr-worker";

let worker: Worker | null = null;
let nextId = 1;
const pending = new Map<number, { resolve: (text: string) => void; reject: (error: Error) => void }>();

function stop(message: string): void {
  worker?.terminate();
  worker = null;
  for (const call of pending.values()) call.reject(new Error(message));
  pending.clear();
}

function start(): Worker {
  const w = new Worker(new URL("./ocr-worker.ts", import.meta.url), { type: "module" });
  w.addEventListener("message", (event: MessageEvent<OcrReply>) => {
    const reply = event.data;
    const call = pending.get(reply.id);
    if (!call) return;
    pending.delete(reply.id);
    if ("crash" in reply) {
      call.reject(new Error(reply.crash || "The scan could not be read."));
      stop("The scan could not be read.");
      return;
    }
    const body: { records?: string[]; error?: string } = JSON.parse(reply.json);
    if (body.records?.[0] !== undefined) call.resolve(body.records[0]);
    else call.reject(new Error(body.error ?? "The scan could not be read."));
  });
  w.addEventListener("error", () => stop("Text recognition could not be started."));
  return w;
}

/** The text of a scanned PDF, read by OCR in this browser. */
export function readScan(bytes: Uint8Array): Promise<string> {
  worker ??= start();
  const w = worker;
  const id = nextId++;
  // The models are served with the page, next to index.html.
  const models = new URL("models/", document.baseURI).href;
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject });
    w.postMessage({ id, bytes, models } as OcrRequest);
  });
}
