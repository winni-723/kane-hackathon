---
test: ../medium-02-name_test.md
status: passed
started: 2026-11-26T23:03:28.081Z
duration_s: 61
session_id: 89ddde59-e189-4c71-8c4c-6c9fb0c394d6
---

# medium-02: name is removed, medical content is kept — Result

## Submit the record ✓ passed (25.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (31.6s)
md5: 417d5ec1fef4884e9eaf46445a05d4b3
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text 'Hector Salazar'.
Verify that the Result region does not contain the text 'Hector'.
