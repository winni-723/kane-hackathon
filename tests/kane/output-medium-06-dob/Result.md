---
test: ../medium-06-dob_test.md
status: passed
started: 2026-11-14T04:19:13.301Z
duration_s: 78
session_id: df22d7ba-e3e2-44e3-affd-0cdc751cec30
---

# medium-06: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (35.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (38.7s)
md5: be3b2eec74eb36932266a737cef5aa07
Verify that the Result region contains the text '4-year-old son'.
Verify that the Result region contains the text 'fever of 102.4 F'.
Verify that the Result region contains the text 'right ear pain for two days'.
Verify that the Result region contains the text 'acute otitis media'.
Verify that the Result region contains the text 'Amoxicillin 400 mg twice daily for 10 days'.
Verify that the Result region contains the text 'Acetaminophen 160 mg every 6 hours as needed'.
Verify that the Result region contains the text 'beyond 48 hours'.
Verify that the Result region does not contain the text '08/21/2021'.
