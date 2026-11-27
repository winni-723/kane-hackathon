---
test: ../easy-06-dob_test.md
status: passed
started: 2026-11-26T22:27:56.094Z
duration_s: 58
session_id: 30545764-021d-4920-975d-bc941b5f7751
---

# easy-06: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (31.2s)
md5: 09b083ecb8cd51977e529e14208da01c
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '12/30/1963'.
