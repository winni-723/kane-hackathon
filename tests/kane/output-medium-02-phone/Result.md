---
test: ../medium-02-phone_test.md
status: passed
started: 2026-11-14T03:28:37.319Z
duration_s: 89
session_id: f2f03992-f5d5-40bc-a736-6942370fd9c5
---

# medium-02: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (36.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (46.2s)
md5: e93352ab457e0431faebb48b2e1da8d5
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '213-555-0147'.
