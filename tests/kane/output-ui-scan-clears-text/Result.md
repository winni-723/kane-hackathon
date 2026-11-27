---
test: ../ui-scan-clears-text_test.md
status: passed
started: 2026-11-27T07:30:41.424Z
duration_s: 108
session_id: 1eb01f78-8051-4b56-b3bc-fe07b8513b77
---

# A scan loaded after a sample leaves nothing of the sample to process — Result

## Load a sample record, then the scan ✓ passed (35.9s)
md5: 4a92aaa9537177eda87cea905e675b6c
Select 'Progress note (text)' in the sample file menu and click the Load sample file button.
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the page contains the text 'Scanned documents cannot be read here'.

## Process without reading the scan ✓ passed (17.6s)
md5: 23f46ac807d927d7741396490e7a8ea0
Click the Process button.
Verify that the Record details heading is visible.

## The earlier sample was not processed ✓ passed (21.2s)
md5: adddf2bdfb90cdd41b5bdcc70ec909a9
Verify that the Record details region, taken as a whole, contains the text 'No identifiers were found in this record'.
Verify that the Record details region, taken as a whole, does not contain the text 'Warfarin 5 mg daily'.
Verify that the Record details region, taken as a whole, does not contain the text 'atrial fibrillation'.
