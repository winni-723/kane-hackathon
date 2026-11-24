// Runs the de-identification engine (Rust compiled to WebAssembly) off the
// page's main thread. Records and files are handled here, in the visitor's
// browser; nothing is sent to the server.
import init, { assure, process as deidentify, read_file } from "./engine-wasm/engine.js";

export type EngineRequest =
  | { id: number; op: "process"; text: string; today: [number, number, number] }
  | { id: number; op: "read"; kind: string; bytes: Uint8Array }
  | { id: number; op: "assure"; records: string[]; today: [number, number, number] };

export type EngineReply = { id: number; json: string } | { id: number; crash: string };

const ready = init();
const reply = (message: EngineReply) => (globalThis as unknown as Worker).postMessage(message);

globalThis.addEventListener("message", async (event: Event) => {
  const request = (event as MessageEvent<EngineRequest>).data;
  try {
    await ready;
    const json =
      request.op === "process"
        ? deidentify(request.text, ...request.today)
        : request.op === "assure"
          ? assure(JSON.stringify(request.records), ...request.today)
          : read_file(request.kind, request.bytes);
    reply({ id: request.id, json });
  } catch (error) {
    // A panic inside the engine stops this instance for good; the page
    // starts a new worker for the next request.
    reply({ id: request.id, crash: String(error) });
  }
});
