---
test: ../ui-paste-several_test.md
status: passed
started: 2026-11-26T01:40:32.786Z
duration_s: 89
session_id: fab835e2-5183-4ac3-ae79-555a201e359a
---

# Two pasted records become a research table — Result

## Paste two records ✓ passed (48.9s)
md5: 4c26c8fb417884d3d6e9f18b9b47ca31
Type {{first}} into the medical record text box.
Click the '+ Add another record' button.
Type {{second}} into the Record 2 text box.
Click the Process button.
Verify that the Research table heading is visible.

## The table holds both records and no phone numbers ✓ passed (35.2s)
md5: b85cddb962c006e21b8a2007cb2cbeeb
Verify that the Research table region, taken as a whole, contains the text 'Migraine without aura'.
Verify that the Research table region, taken as a whole, contains the text 'Seasonal allergic rhinitis'.
Verify that the Research table region, taken as a whole, contains the text 'Record 2'.
Verify that the Research table region, taken as a whole, does not contain the text '(617) 555-0142'.
Verify that the Research table region, taken as a whole, does not contain the text '(413) 555-0177'.
