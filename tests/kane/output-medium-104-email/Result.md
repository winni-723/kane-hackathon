---
test: ../medium-104-email_test.md
status: passed
started: 2026-11-15T18:00:34.837Z
duration_s: 80
session_id: 065eef2d-9235-421e-8524-801798b56635
---

# medium-104: email is removed, medical content is kept — Result

## Submit the record ✓ passed (32.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (41.3s)
md5: 9369581166737bda5b8b09445c560371
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'declan.os@example.net'.
