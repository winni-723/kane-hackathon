#!/usr/bin/env bash
# Two-layer verification. Run from the repo root:  bash scripts/verify.sh
#
#   Layer 1: fixtures well-formed, cargo build, clippy (server and browser
#            builds), cargo test, the browser engine equals the native one,
#            tsc + vite build
#   Layer 2: every saved Kane test in tests/kane, replayed against the local server
#
# Stops at the first failing layer. Exit 0 only when both layers pass.
# Kane tests are replayed with --no-adaptive-heal so a regression fails instead
# of Kane rewriting the test to fit the page.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
PORT="${PORT:-3000}"
LOG_DIR=".kane-logs"
mkdir -p "$LOG_DIR"

# Only one verification at a time. Two runs would restart the server under each
# other and stop each other's browsers, and every test would fail for no reason.
LOCK="$LOG_DIR/verify.lock"
if [ -d "$LOCK" ] && [ -n "$(find "$LOCK" -maxdepth 0 -mmin -30 2>/dev/null)" ]; then
  echo "ANOTHER VERIFICATION IS RUNNING (started $(cat "$LOCK/started" 2>/dev/null)); not starting a second one"
  exit 3
fi
rm -rf "$LOCK"
mkdir -p "$LOCK"
date > "$LOCK/started"
trap 'rm -rf "$LOCK"' EXIT

step() {
  local name="$1"; shift
  if "$@" > "$LOG_DIR/layer1-$name.log" 2>&1; then
    echo "  pass  $name"
  else
    echo "  FAIL  $name   (see $LOG_DIR/layer1-$name.log)"
    tail -25 "$LOG_DIR/layer1-$name.log"
    echo "LAYER 1 FAILED"
    exit 1
  fi
}

stop_server() { taskkill //F //IM deid-server.exe > /dev/null 2>&1 || pkill -f deid-server > /dev/null 2>&1 || true; }

echo "== Layer 1"
stop_server   # the running binary would block the rebuild on Windows
step fixtures node scripts/validate-fixtures.mjs
step build    cargo build
step clippy   cargo clippy --all-targets -- -D warnings
step test     cargo test
step clippy-wasm cargo clippy --lib --target wasm32-unknown-unknown --no-default-features --features wasm -- -D warnings
step wasm     bash scripts/build-wasm.sh
step native   sh -c 'cargo run -q --example native_outputs > "$0/native-outputs.json"' "$LOG_DIR"
step wasm-matches-native node scripts/wasm-matches-native.mjs "$LOG_DIR/native-outputs.json"
step web      npm --prefix web run build
grep -h "test result" "$LOG_DIR/layer1-test.log" | sed 's/^/        /'

echo "== Layer 2"
# A verification replays recordings; it never makes them. A test with no
# recording would be recorded here, at about 30 credits each, and possibly
# while scripts/author-kane-tests.sh is recording the same test.
unrecorded=()
for test_file in tests/kane/*_test.md; do
  [ -d "tests/kane/output-$(basename "$test_file" _test.md)/.internal" ] || unrecorded+=("$test_file")
done
if [ ${#unrecorded[@]} -gt 0 ]; then
  echo "  ${#unrecorded[@]} Kane test(s) have no recording yet:"
  printf '    %s\n' "${unrecorded[@]}" | head -10
  echo "KANE DID NOT RUN: record them first with scripts/redo-kane-recordings.sh"
  exit 2
fi
# Leftover Kane browsers hold its ports and make new runs fail before they start.
node scripts/cleanup-kane-chrome.mjs | sed 's/^/  /'
nohup ./target/debug/deid-server > "$LOG_DIR/server.log" 2>&1 &
disown
for _ in $(seq 1 20); do
  curl -s -o /dev/null "http://127.0.0.1:$PORT/" && break
  sleep 0.5
done
if ! curl -s -o /dev/null "http://127.0.0.1:$PORT/"; then
  echo "  FAIL  server did not start (see $LOG_DIR/server.log)"
  exit 1
fi

shopt -s nullglob
tests=(tests/kane/*_test.md)
if [ ${#tests[@]} -eq 0 ]; then
  echo "  no Kane tests yet"
  echo "BOTH LAYERS PASSED (layer 2 empty)"
  exit 0
fi

KANE_CLI_USER_AGENT=claude-code kane-cli testrun run "${tests[@]}" \
  --agent --headless --no-adaptive-heal --parallel "${KANE_PARALLEL:-3}" \
  > "$LOG_DIR/layer2-suite.ndjson" 2> "$LOG_DIR/layer2-suite.err"
kane_exit=$?

node scripts/kane-suite-summary.mjs "$LOG_DIR/layer2-suite.ndjson"
# A member that could not get a browser is "broken", not a test failure.
if grep -q "chrome slot failed" "$LOG_DIR/layer2-suite.err"; then
  echo "LAYER 2 DID NOT RUN CLEANLY: some tests could not start a browser (see $LOG_DIR/layer2-suite.err)"
  exit 2
fi
case $kane_exit in
  0) echo "BOTH LAYERS PASSED (${#tests[@]} Kane tests)"; exit 0 ;;
  2) echo "LAYER 2 DID NOT START (sign-in, Chrome or config; see $LOG_DIR/layer2-suite.err)"; exit 2 ;;
  *) echo "LAYER 2 FAILED (kane exit $kane_exit)"; exit 1 ;;
esac
