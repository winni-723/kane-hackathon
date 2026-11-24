// The page's way into the engine. Each call goes to a Web Worker running the
// engine in this browser; if the engine ever crashes, that worker is replaced
// and the call fails with a message the page can show.
import type { EngineReply, EngineRequest } from "./engine-worker";

type Pending = { resolve: (json: string) => void; reject: (error: Error) => void };

let worker: Worker | null = null;
let nextId = 1;
const pending = new Map<number, Pending>();

function start(): Worker {
  const w = new Worker(new URL("./engine-worker.ts", import.meta.url), { type: "module" });
  w.addEventListener("message", (event: MessageEvent<EngineReply>) => {
    const reply = event.data;
    const call = pending.get(reply.id);
    if (!call) return;
    pending.delete(reply.id);
    if ("json" in reply) {
      call.resolve(reply.json);
    } else {
      call.reject(new Error("The engine stopped while reading this input. Try again, or try another file."));
      restart("The engine stopped. Try again.");
    }
  });
  w.addEventListener("error", () => restart("The engine could not be started."));
  return w;
}

// Drops the worker (the next call starts a fresh one) and fails every call
// still waiting on it.
function restart(message: string): void {
  worker?.terminate();
  worker = null;
  for (const call of pending.values()) call.reject(new Error(message));
  pending.clear();
}

type Request =
  | { op: "process"; text: string; today: [number, number, number] }
  | { op: "read"; kind: string; bytes: Uint8Array }
  | { op: "assure"; records: string[]; today: [number, number, number] };

function call(request: Request): Promise<string> {
  worker ??= start();
  const id = nextId++;
  const w = worker;
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject });
    w.postMessage({ ...request, id } as EngineRequest);
  });
}

// Ages from a date of birth are counted up to the visitor's today.
function today(): [number, number, number] {
  const now = new Date();
  return [now.getFullYear(), now.getMonth() + 1, now.getDate()];
}

/** De-identifies one record; the answer is JSON in the engine's shape. */
export function processRecord(text: string): Promise<string> {
  return call({ op: "process", text, today: today() });
}

/** Reads a Word, CSV or PDF file into records; JSON `{records}` or `{error}`. */
export function readDocument(kind: string, bytes: Uint8Array): Promise<string> {
  return call({ op: "read", kind, bytes });
}

/** The assurance report on a set of original records; JSON with no record text. */
export function assureRecords(records: string[]): Promise<string> {
  return call({ op: "assure", records, today: today() });
}
