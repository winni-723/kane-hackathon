---
test: ../medium-01-dob_test.md
status: passed
started: 2026-11-26T23:00:11.498Z
duration_s: 62
session_id: 361154b5-a5a8-4359-bdfd-61e09e52dfc1
---

# medium-01: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (25.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (32.2s)
md5: 77343de248f6895cfef6e25923cae2f6
Verify that the Result region contains the text '72-year-old woman'.
Verify that the Result region contains the text 'atrial fibrillation'.
Verify that the Result region contains the text 'INR today is 2.4'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region contains the text 'Metoprolol 25 mg twice daily'.
Verify that the Result region contains the text 'Recheck INR in 4 weeks'.
Verify that the Result region does not contain the text 'April 9, 1953'.
