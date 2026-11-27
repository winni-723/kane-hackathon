---
test: ../medium-03-phone_test.md
status: passed
started: 2026-11-26T23:05:49.652Z
duration_s: 55
session_id: 1c8941d0-cc5c-4696-a3e6-13c6d460d5da
---

# medium-03: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (21.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (29.7s)
md5: 41b13955fa20379c7f8c851b53301597
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text '646-555-0159'.
