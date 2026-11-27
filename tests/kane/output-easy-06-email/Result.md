---
test: ../easy-06-email_test.md
status: passed
started: 2026-11-26T22:28:04.154Z
duration_s: 58
session_id: 61026420-c073-4bd6-aa4d-fdb9f7ce048e
---

# easy-06: email is removed, medical content is kept — Result

## Submit the record ✓ passed (23.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (31.2s)
md5: 8ecff558a83da142e429f399c2614373
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text 'r.achterberg@example.org'.
