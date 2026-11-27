---
test: ../hard-103-dob_test.md
status: passed
started: 2026-11-26T22:57:52.776Z
duration_s: 63
session_id: 70148ba7-a6d1-4014-9d91-76dc45e95014
---

# hard-103: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (35s)
md5: c99dfefc4bc0424c104d0073108c13f1
Verify that the Result region contains the text 'MMSE 21/30'.
Verify that the Result region contains the text 'Geriatric Depression Scale 9/15'.
Verify that the Result region contains the text 'Two falls in the past 3 months'.
Verify that the Result region contains the text 'orthostatic drop of 24 mmHg systolic'.
Verify that the Result region contains the text 'stop Diphenhydramine'.
Verify that the Result region contains the text 'Donepezil 10 mg nightly'.
Verify that the Result region contains the text 'Braden score 16'.
Verify that the Result region does not contain the text 'Christmas Eve, 1936'.
