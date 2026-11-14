---
test: ../medium-104-name_test.md
status: passed
started: 2026-11-14T23:40:03.531Z
duration_s: 98
session_id: 0f9b4919-742a-48c1-a658-e9bd86d3462d
---

# medium-104: name is removed, medical content is kept — Result

## Submit the record ✓ passed (38s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (44s)
md5: 4612770a0804bba5bee186d3800b8a28
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text "Margaret O'Sullivan".
Verify that the Result region does not contain the text 'Margaret'.
