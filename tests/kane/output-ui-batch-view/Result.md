---
test: ../ui-batch-view_test.md
status: passed
started: 2026-11-19T20:57:40.928Z
duration_s: 144
session_id: 7dbd33a5-1ba3-4b14-a8e3-4582339ed2d7
---

# Records opened from the research table are de-identified, and Next moves on — Result

## Build the table from the three-record sample ✓ passed (43.4s)
md5: 144424a92b99ee4bb01d5ca76979b7ba
Select 'Three records (text)' in the sample file menu and click the Load sample file button.
Click the Process button.
Verify that the Research table heading is visible.

## Open the second record ✓ passed (37.9s)
md5: 49fb54ac28e1cec6671bdb2a6b977bf5
Click the 'View record 2' button.
Verify that the Result region contains the text 'admitted with community-acquired pneumonia'.
Verify that the Result region does not contain the text 'Bernard Kowalczyk'.
Verify that the Result region does not contain the text '740 Sycamore Hollow Road'.

## Move on to the third record ✓ passed (33.4s)
md5: eddbb2b317faee083cf2da23893ac92a
Click the Next button.
Verify that the Result region contains the text 'Diagnosis: post-traumatic stress disorder'.
Verify that the Result region does not contain the text 'Malik'.
Verify that the Result region does not contain the text '(518) 555 0118'.

## The details say which record this is ✓ passed (19.7s)
md5: 88cc1d805ecbcda7efbb9e4d6b8078a3
Verify that the Record details region contains the text 'Record 3 of 3'.
