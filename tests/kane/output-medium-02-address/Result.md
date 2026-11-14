---
test: ../medium-02-address_test.md
status: passed
started: 2026-11-14T04:15:11.674Z
duration_s: 90
session_id: 9c70d123-1ea7-4cb0-acfd-58121b2ded8b
---

# medium-02: address is removed, medical content is kept — Result

## Submit the record ✓ passed (38.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (46.3s)
md5: fa77d93b0b66fc46c29b7b2862000bad
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '902 Alameda Verde Avenue'.
Verify that the Result region does not contain the text 'Santa Rilla'.
