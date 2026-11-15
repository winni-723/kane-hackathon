---
test: ../hard-103-phone_test.md
status: passed
started: 2026-11-15T18:35:52.498Z
duration_s: 138
session_id: e1107215-6394-4207-897f-972f5bc91ac5
---

# hard-103: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (35.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (44s)
md5: d5dbbbc86163c6809c176463d8fc9e7c
Verify that the Result region contains the text 'MMSE 21/30'.
Verify that the Result region contains the text 'Geriatric Depression Scale 9/15'.
Verify that the Result region contains the text 'Two falls in the past 3 months'.
Verify that the Result region contains the text 'orthostatic drop of 24 mmHg systolic'.
Verify that the Result region contains the text 'stop Diphenhydramine'.
Verify that the Result region contains the text 'Donepezil 10 mg nightly'.
Verify that the Result region contains the text 'Braden score 16'.
Verify that the Result region does not contain the text '3 0 2 5 5 5 0 1 3 7'.
