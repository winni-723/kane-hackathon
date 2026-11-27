---
test: ../medium-104-dob_test.md
status: passed
started: 2026-11-26T23:21:56.308Z
duration_s: 56
session_id: 84589b35-7d1c-4b92-810a-104e4db44548
---

# medium-104: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.3s)
md5: b8efe36e2f8b909189989ee2c4ae4243
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'March 21, 1957'.
