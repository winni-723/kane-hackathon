---
test: ../medium-02-dob_test.md
status: passed
started: 2026-11-26T23:02:28.150Z
duration_s: 60
session_id: 6f4ed415-b1ab-4ec6-ba3c-8aa41b1f4812
---

# medium-02: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (24.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (32.2s)
md5: ee5e8e5cb0bbd903347e8e1269b36a7c
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '06/17/1980'.
