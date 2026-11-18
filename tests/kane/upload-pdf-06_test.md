---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [upload, pdf, scan]
---

# upload-pdf-06: a scanned PDF is refused with a clear message

Generated from the page's sample menu by scripts/gen-kane-upload-test.mjs.
Do not edit: this test is part of the regression suite.

## Load the scanned sample
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the page contains the text 'Scanned documents cannot be read here'.
Verify that the Result heading is not visible.
