---
test: ../easy-06-ssn_test.md
status: passed
started: 2026-11-26T22:29:20.152Z
duration_s: 63
session_id: fd7ece16-5d7c-430d-b851-f083dbc6136b
---

# easy-06: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (32.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (26.1s)
md5: 22429c8e1a69bba062b09f1cccee1612
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '000-45-6789'.
