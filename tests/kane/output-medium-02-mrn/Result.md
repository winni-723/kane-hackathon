---
test: ../medium-02-mrn_test.md
status: passed
started: 2026-11-14T03:48:26.165Z
duration_s: 86
session_id: bbf2db0d-0263-4386-9576-07bf5328892f
---

# medium-02: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (37.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (43.6s)
md5: 2995d88d2e1f74f95c5368b9ec40d83f
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '55801276'.
