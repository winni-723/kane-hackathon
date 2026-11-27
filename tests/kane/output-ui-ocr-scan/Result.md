---
test: ../ui-ocr-scan_test.md
status: passed
started: 2026-11-26T23:27:49.491Z
duration_s: 201
session_id: 58fc7b88-d02f-40fa-ad3b-6d22df8d8d11
---

# A scanned PDF read by OCR, at the visitor's choice, then de-identified — Result

## Load the scanned sample and choose OCR ✓ passed (38.5s)
md5: 7f6860ad64724c6e3d2558626e67eff6
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the page contains the text 'Scanned documents cannot be read here'.
Click the 'Read the scan with OCR' button.

## Wait for the scan to be read ✓ passed (111.6s)
md5: dfaf65a866393e5b7a468a1a09741faf
Wait until the text 'This text was read from a scan by OCR' is visible, for up to 90 seconds.
Verify that the page contains the text 'This text was read from a scan by OCR'.

## De-identify the text read ✓ passed (21.2s)
md5: 167407626c87811fbdb2115478dd6038
Click the Process button.
Verify that the Result heading is visible.

## The medical content stays, the identifiers go ✓ passed (25.2s)
md5: 0841e1bc1877783f143d1b75c34b2e5e
Verify that the Result region contains the text 'Exertional chest pain for 3 weeks' and does not contain the text '987-65-4331'.
Verify that the Result region contains the text 'new T wave inversions in V4-V6' and does not contain the text 'Miriam Okafor'.
Verify that the Result region contains the text 'Lisinopril 20 mg daily' and does not contain the text '1093847562'.
