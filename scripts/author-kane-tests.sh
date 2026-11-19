#!/usr/bin/env bash
# Generate and record new Kane tests from answer keys.
#
#   bash scripts/author-kane-tests.sh <record_id>:<pii_type> [...]
#   bash scripts/author-kane-tests.sh --from <file with one pair per line>
#   bash scripts/author-kane-tests.sh hand:tests/kane/<name>_test.md
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
  # hand:<path> is a test written by hand (a UI test, say). The file exists
  # already; it is recorded only if it has no recording yet.
  if [ "$record" = "hand" ]; then
    file="$kind"
    stem="$(basename "$file" _test.md)"
    if [ ! -f "$file" ]; then
      echo "  could not find $file"
    elif [ -d "$(dirname "$file")/output-$stem/.internal" ]; then
      echo "  skip  $file (already recorded)"
    else
      new+=("$file")
    fi
    continue
  fi
  # upload:<document_id> is a test of the file-upload path (sample menu).
  if [ "$record" = "upload" ]; then
    file="tests/kane/upload-${kind}_test.md"
    gen=(node scripts/gen-kane-upload-test.mjs "$kind")
  else
    file="tests/kane/${record}-${kind}_test.md"
    gen=(node scripts/gen-kane-test.mjs "$record" "$kind")
  fi
  if [ -e "$file" ]; then
    echo "  skip  $file (exists)"
  elif "${gen[@]}" > /dev/null; then
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
. scripts/kane-lock.sh

node scripts/cleanup-kane-chrome.mjs | sed 's/^/  /'
stamp="$(date +%Y%m%d-%H%M%S)"
log="$LOG_DIR/author-$stamp.ndjson"
KANE_CLI_USER_AGENT=claude-code kane-cli testrun run "${new[@]}" \
  --agent --headless --parallel "${KANE_PARALLEL:-3}" \
  > "$log" 2> "$LOG_DIR/author-$stamp.err"
echo "kane exit $?  (log: $log)"
node scripts/kane-authored-check.mjs "$log"
