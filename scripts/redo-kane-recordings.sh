#!/usr/bin/env bash
# Re-record Kane tests whose first recording used screenshot checks.
#
#   bash scripts/redo-kane-recordings.sh tests/kane/<name>_test.md [...]
#
# Only the recording is redone; the test file itself is not touched. A test
# that is already committed is refused: committed tests are frozen.
set -u
LOG_DIR=".kane-logs"
mkdir -p "$LOG_DIR"

files=()
for file in "$@"; do
  if git ls-files --error-unmatch "$file" > /dev/null 2>&1; then
    echo "  refuse  $file is committed; committed Kane tests are frozen"
    continue
  fi
  stem="$(basename "$file" _test.md)"
  rm -rf "$(dirname "$file")/output-$stem"
  files+=("$file")
done
[ ${#files[@]} -eq 0 ] && { echo "nothing to redo"; exit 0; }

node scripts/cleanup-kane-chrome.mjs | sed 's/^/  /'
stamp="$(date +%Y%m%d-%H%M%S)"
log="$LOG_DIR/redo-$stamp.ndjson"
KANE_CLI_USER_AGENT=claude-code kane-cli testrun run "${files[@]}" \
  --agent --headless --parallel "${KANE_PARALLEL:-3}" \
  > "$log" 2> "$LOG_DIR/redo-$stamp.err"
echo "kane exit $?  (log: $log)"
node scripts/kane-authored-check.mjs "$log"
