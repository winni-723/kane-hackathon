// A fingerprint of the files a verification ran on: every file git tracks,
// except evidence/ (the results themselves), each named with the id git gives
// its contents. Git's ids ignore Windows line endings, so the same files give
// the same fingerprint on any machine, and a clean checkout of the commit that
// carries an evidence file gives the fingerprint written in it.
//
// Usage: node scripts/tree-hash.mjs        (prints the fingerprint)
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync } from "node:fs";
import { pathToFileURL } from "node:url";

export function treeHash(cwd = process.cwd()) {
  const git = (args, input) => execFileSync("git", args, { cwd, encoding: "utf8", input, maxBuffer: 64 << 20 });
  const files = git(["ls-files", "-z"])
    .split("\0")
    .filter((f) => f && !f.startsWith("evidence/") && existsSync(`${cwd}/${f}`))
    .sort();
  const ids = git(["hash-object", "--stdin-paths"], `${files.join("\n")}\n`).trim().split("\n");
  const hash = createHash("sha256");
  files.forEach((f, i) => hash.update(`${f}\0${ids[i]}\n`));
  return { files: files.length, sha256: hash.digest("hex") };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  console.log(treeHash().sha256);
}
