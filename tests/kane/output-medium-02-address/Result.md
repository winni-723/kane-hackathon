---
test: ../medium-02-address_test.md
status: passed
started: 2026-11-26T23:02:17.469Z
duration_s: 62
session_id: db335910-f2f4-429f-9e6e-4a0f1a134678
---

# medium-02: address is removed, medical content is kept — Result

## Submit the record ✓ passed (24.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (34.1s)
md5: fa77d93b0b66fc46c29b7b2862000bad
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '902 Alameda Verde Avenue'.
Verify that the Result region does not contain the text 'Santa Rilla'.
