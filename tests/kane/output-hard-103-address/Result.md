---
test: ../hard-103-address_test.md
status: passed
started: 2026-11-15T17:50:10.449Z
duration_s: 64
session_id: b818fe81-f2d1-4fcb-97b9-7d665dff03bc
---

# hard-103: address is removed, medical content is kept — Result

## Submit the record ✓ passed (25.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (33.5s)
md5: 4b230e7ae9dcfd94a6296248aaaa9c4c
Verify that the Result region contains the text 'MMSE 21/30'.
Verify that the Result region contains the text 'Geriatric Depression Scale 9/15'.
Verify that the Result region contains the text 'Two falls in the past 3 months'.
Verify that the Result region contains the text 'orthostatic drop of 24 mmHg systolic'.
Verify that the Result region contains the text 'stop Diphenhydramine'.
Verify that the Result region contains the text 'Donepezil 10 mg nightly'.
Verify that the Result region contains the text 'Braden score 16'.
Verify that the Result region does not contain the text 'stone cottage at the end of Quaker Bridge Lane'.
