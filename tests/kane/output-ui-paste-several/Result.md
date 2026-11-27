---
test: ../ui-paste-several_test.md
status: passed
started: 2026-11-26T23:28:10.340Z
duration_s: 67
session_id: 1d67bc94-aef7-4393-8c61-3a148206ed45
---

# Two pasted records become a research table — Result

## Paste two records ✓ passed (39.7s)
md5: 4c26c8fb417884d3d6e9f18b9b47ca31
Type {{first}} into the medical record text box.
Click the '+ Add another record' button.
Type {{second}} into the Record 2 text box.
Click the Process button.
Verify that the Research table heading is visible.

## The table holds both records and no phone numbers ✓ passed (23.3s)
md5: b85cddb962c006e21b8a2007cb2cbeeb
Verify that the Research table region, taken as a whole, contains the text 'Migraine without aura'.
Verify that the Research table region, taken as a whole, contains the text 'Seasonal allergic rhinitis'.
Verify that the Research table region, taken as a whole, contains the text 'Record 2'.
Verify that the Research table region, taken as a whole, does not contain the text '(617) 555-0142'.
Verify that the Research table region, taken as a whole, does not contain the text '(413) 555-0177'.
