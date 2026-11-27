// Reads scanned PDFs by OCR, in the visitor's browser, in a worker of its own.
// The OCR build of the engine and its two models (about 20 MB in all) are
// fetched from this site only when a visitor first asks for a scan to be
// read; nothing is sent anywhere.
import init, { ocr_load, ocr_read } from "./ocr-wasm/ocr.js";

export type OcrRequest = { id: number; bytes: Uint8Array; models: string };
export type OcrReply = { id: number; json: string } | { id: number; crash: string };

let ready: Promise<void> | null = null;

async function load(models: string): Promise<void> {
  await init();
  const fetchModel = async (name: string): Promise<Uint8Array> => {
    const response = await fetch(new URL(name, models));
    if (!response.ok) throw new Error(`The text recognition model could not be fetched (${response.status}).`);
    return new Uint8Array(await response.arrayBuffer());
  };
  const [detection, recognition] = await Promise.all([
    fetchModel("text-detection.onnx"),
    fetchModel("text-recognition.onnx"),
  ]);
  const answer: { error?: string } = JSON.parse(ocr_load(detection, recognition));
  if (answer.error !== undefined) throw new Error(answer.error);
}

const reply = (message: OcrReply) => (globalThis as unknown as Worker).postMessage(message);

globalThis.addEventListener("message", async (event: Event) => {
  const request = (event as MessageEvent<OcrRequest>).data;
  try {
    ready ??= load(request.models);
    await ready;
    reply({ id: request.id, json: ocr_read(request.bytes) });
  } catch (error) {
    ready = null;
    reply({ id: request.id, crash: error instanceof Error ? error.message : String(error) });
  }
});
