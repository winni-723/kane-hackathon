---
test: ../easy-06-email_test.md
status: passed
started: 2026-11-14T03:45:15.587Z
duration_s: 83
session_id: d5a88301-ea2a-4f73-83f6-9d3f05b8802f
---

# easy-06: email is removed, medical content is kept — Result

## Submit the record ✓ passed (34.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (43.5s)
md5: 8ecff558a83da142e429f399c2614373
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text 'r.achterberg@example.org'.
