---
test: ../medium-104-phone_test.md
status: passed
started: 2026-11-26T23:23:08.333Z
duration_s: 58
session_id: 4375b850-827d-4e39-ba61-620e713135ce
---

# medium-104: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (24.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (29.7s)
md5: 260e81dd9b72292d338f3225a71c8973
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text '207.555.0171'.
