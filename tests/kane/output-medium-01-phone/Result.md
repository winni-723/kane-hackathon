---
test: ../medium-01-phone_test.md
status: passed
started: 2026-11-26T23:01:21.728Z
duration_s: 64
session_id: 0258cac3-3162-4280-858c-88bd3f1fe630
---

# medium-01: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (28.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.6s)
md5: 4c0b1b5cbef7d7a486d6f83ca67588d1
Verify that the Result region contains the text '72-year-old woman'.
Verify that the Result region contains the text 'atrial fibrillation'.
Verify that the Result region contains the text 'INR today is 2.4'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region contains the text 'Metoprolol 25 mg twice daily'.
Verify that the Result region contains the text 'Recheck INR in 4 weeks'.
Verify that the Result region does not contain the text '555-0198'.
Verify that the Result region does not contain the text '(508) 555-0114'.
