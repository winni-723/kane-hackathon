# Sourced by the scripts that record Kane tests. Takes the same lock as
# scripts/verify.sh, so a verification (the Stop hook starts one whenever a
# Kane test file changes) cannot run while tests are being recorded. Two Kane
# suites at once take each other's browser ports, restart the server under
# each other, and a verification that meets a test with no recording yet
# records it itself and spends the credits a second time.
LOCK=".kane-logs/verify.lock"
mkdir -p .kane-logs
if [ -d "$LOCK" ] && [ -n "$(find "$LOCK" -maxdepth 0 -mmin -30 2>/dev/null)" ]; then
  echo "A VERIFICATION OR RECORDING IS RUNNING (started $(cat "$LOCK/started" 2>/dev/null)); not starting"
  exit 3
fi
rm -rf "$LOCK"
mkdir -p "$LOCK"
date > "$LOCK/started"
# Recording many tests takes longer than the 30 minutes after which a lock is
# treated as abandoned, so keep it fresh while this script is alive.
( while sleep 300; do touch "$LOCK" 2>/dev/null || exit; done ) &
KANE_LOCK_KEEPER=$!
trap 'kill "$KANE_LOCK_KEEPER" 2>/dev/null; rm -rf "$LOCK"' EXIT
