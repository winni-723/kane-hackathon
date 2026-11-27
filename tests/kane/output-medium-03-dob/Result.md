---
test: ../medium-03-dob_test.md
status: passed
started: 2026-11-26T23:04:39.315Z
duration_s: 63
session_id: 2c2da444-15e2-44ad-926b-474439dd8282
---

# medium-03: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (29.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (29.7s)
md5: 5cc0dacfe10fdf192d2560e8cb2ecd6e
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text '02/11/1967'.
