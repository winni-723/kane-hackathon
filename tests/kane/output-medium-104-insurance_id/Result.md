---
test: ../medium-104-insurance_id_test.md
status: passed
started: 2026-11-15T18:02:04.582Z
duration_s: 79
session_id: 9f49831f-c5fc-4b24-8c3c-9567928088a6
---

# medium-104: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (32.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (40.6s)
md5: 7747def8d239346e1150c84d81b87db4
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'MED-5540129'.
