---
test: ../easy-06-address_test.md
status: passed
started: 2026-11-14T04:09:10.343Z
duration_s: 95
session_id: 8562b4b9-4f97-426c-b2a4-ceed0472b134
---

# easy-06: address is removed, medical content is kept — Result

## Submit the record ✓ passed (39.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (49.6s)
md5: 7c7080b2ed6af2a5d1ee5ac6f85061c8
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '615 Quarry Hill Avenue'.
Verify that the Result region does not contain the text 'Unit 12'.
Verify that the Result region does not contain the text 'Stonehaven Ridge'.
Verify that the Result region does not contain the text '80299'.
