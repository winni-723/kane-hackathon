---
test: ../medium-104-insurance_id_test.md
status: passed
started: 2026-11-26T23:22:49.190Z
duration_s: 61
session_id: 4dff0cb6-b571-4310-9d3a-59098b4007fb
---

# medium-104: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (29.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (28.3s)
md5: 7747def8d239346e1150c84d81b87db4
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'MED-5540129'.
