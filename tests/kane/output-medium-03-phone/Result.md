---
test: ../medium-03-phone_test.md
status: passed
started: 2026-11-14T03:29:58.681Z
duration_s: 90
session_id: 447f80e9-e07c-41bf-b6ff-3d95884cf837
---

# medium-03: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (40.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (44.8s)
md5: 41b13955fa20379c7f8c851b53301597
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text '646-555-0159'.
