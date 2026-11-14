// Kane CLI sometimes leaves headless Chrome processes behind. They hold its
// debugging ports (9222-9230), and once all are taken new runs fail with
// "All CDP ports 9222-9230 are in use" before any test executes.
//
// This stops only the Chrome processes Kane started: the ones whose profile
// directory is one of Kane's temporary `kane-clean-*` folders. Your own Chrome
// windows use a different profile and are never touched.
//
// Usage: node scripts/cleanup-kane-chrome.mjs
import { execFileSync } from "node:child_process";

if (process.platform !== "win32") {
  try {
    execFileSync("pkill", ["-f", "kane-clean-"], { stdio: "ignore" });
  } catch {
    // pkill exits non-zero when nothing matched.
  }
  process.exit(0);
}

const command = [
  "$p = Get-CimInstance Win32_Process |",
  "  Where-Object { $_.Name -eq 'chrome.exe' -and $_.CommandLine -match 'kane-clean-' };",
  "$p | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue };",
  "Write-Output $p.Count",
].join(" ");

try {
  const out = execFileSync("powershell.exe", ["-NoProfile", "-Command", command], {
    encoding: "utf8",
  }).trim();
  console.log(`stopped ${out || 0} leftover Kane Chrome process(es)`);
} catch (e) {
  console.log(`could not check for leftover Kane Chrome processes: ${e.message}`);
}
