---
test: ../medium-104-email_test.md
status: passed
started: 2026-11-26T23:22:02.955Z
duration_s: 57
session_id: 2665b198-9213-45c6-9ac4-05c530b632ec
---

# medium-104: email is removed, medical content is kept — Result

## Submit the record ✓ passed (22.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (30.9s)
md5: 9369581166737bda5b8b09445c560371
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text 'declan.os@example.net'.
