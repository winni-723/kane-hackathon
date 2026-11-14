---
test: ../easy-06-ssn_test.md
status: passed
started: 2026-11-14T03:57:31.839Z
duration_s: 91
session_id: 8d10ba72-0cc9-4e57-a4f3-8ea7e5b918d0
---

# easy-06: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (43s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (43.3s)
md5: 22429c8e1a69bba062b09f1cccee1612
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text '000-45-6789'.
