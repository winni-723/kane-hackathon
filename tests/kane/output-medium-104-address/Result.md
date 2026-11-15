---
test: ../medium-104-address_test.md
status: passed
started: 2026-11-15T18:00:27.677Z
duration_s: 93
session_id: 42345f4f-f469-4ffe-9ccc-917b860863ea
---

# medium-104: address is removed, medical content is kept — Result

## Submit the record ✓ passed (36.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (50.2s)
md5: c082f9225a9f841ac007dce14ee83c58
Verify that the Result region contains the text 'left total knee replacement'.
Verify that the Result region contains the text 'Knee flexion is 85 degrees'.
Verify that the Result region contains the text 'Quadriceps strength 3+/5'.
Verify that the Result region contains the text 'Timed Up and Go is 19 seconds'.
Verify that the Result region contains the text 'flexion to 120 degrees in 6 weeks'.
Verify that the Result region contains the text 'independent stair negotiation'.
Verify that the Result region does not contain the text '15 Lighthouse Cove Road'.
Verify that the Result region does not contain the text 'Port Elswick'.
Verify that the Result region does not contain the text '04099'.
