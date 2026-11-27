---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui, upload, pdf, scan, ocr]
---

# A scanned PDF read by OCR, at the visitor's choice, then de-identified

Hand-written UI test. Do not edit: this test is part of the regression suite.
The scan is the page's fictional sample, pdf-03 printed and scanned. OCR runs
in the browser and takes up to a minute.

## Load the scanned sample and choose OCR
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the page contains the text 'Scanned documents cannot be read here'.
Click the 'Read the scan with OCR' button.

## Wait for the scan to be read
Wait until the text 'This text was read from a scan by OCR' is visible, for up to 90 seconds.
Verify that the page contains the text 'This text was read from a scan by OCR'.

## De-identify the text read
Click the Process button.
Verify that the Result heading is visible.

## The medical content stays, the identifiers go
Verify that the Result region contains the text 'Exertional chest pain for 3 weeks' and does not contain the text '987-65-4331'.
Verify that the Result region contains the text 'new T wave inversions in V4-V6' and does not contain the text 'Miriam Okafor'.
Verify that the Result region contains the text 'Lisinopril 20 mg daily' and does not contain the text '1093847562'.
