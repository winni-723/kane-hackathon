---
test: ../easy-06-phone_test.md
status: passed
started: 2026-11-14T03:28:24.051Z
duration_s: 85
session_id: 8da7574a-8119-42dc-bc79-7be3686174bb
---

# easy-06: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (36.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (43.6s)
md5: fb0cdb31fb94444e6a303eb49c532a71
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '+1 303 555 0154'.
