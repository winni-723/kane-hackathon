#!/usr/bin/env bash
# Generate and record new Kane tests from answer keys.
#
#   bash scripts/author-kane-tests.sh <record_id>:<pii_type> [...]
#   bash scripts/author-kane-tests.sh --from <file with one pair per line>
#
# The local server must already be running the engine under test
# (bash scripts/verify.sh leaves it running). Existing tests are skipped, never
# rewritten. Afterwards every new test is checked: it must pass and its
# answer-key checks must be exact text comparisons. Exit 1 lists tests to redo.
set -u
LOG_DIR=".kane-logs"
mkdir -p "$LOG_DIR"

if [ "${1:-}" = "--from" ]; then
  mapfile -t pairs < <(tr -d '\r' < "$2")
else
  pairs=("$@")
fi

new=()
for pair in "${pairs[@]}"; do
  [ -z "$pair" ] && continue
  record="${pair%%:*}"
  kind="${pair##*:}"
  file="tests/kane/${record}-${kind}_test.md"
  if [ -e "$file" ]; then
    echo "  skip  $file (exists)"
  elif node scripts/gen-kane-test.mjs "$record" "$kind" > /dev/null; then
    new+=("$file")
  else
    echo "  could not generate $file"
  fi
done

if [ ${#new[@]} -eq 0 ]; then
  echo "nothing to record"
  exit 0
fi
echo "recording ${#new[@]} new test(s)"

node scripts/cleanup-kane-chrome.mjs | sed 's/^/  /'
stamp="$(date +%Y%m%d-%H%M%S)"
log="$LOG_DIR/author-$stamp.ndjson"
KANE_CLI_USER_AGENT=claude-code kane-cli testrun run "${new[@]}" \
  --agent --headless --parallel "${KANE_PARALLEL:-3}" \
  > "$log" 2> "$LOG_DIR/author-$stamp.err"
echo "kane exit $?  (log: $log)"
node scripts/kane-authored-check.mjs "$log"
