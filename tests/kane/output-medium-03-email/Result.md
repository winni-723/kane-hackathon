---
test: ../medium-03-email_test.md
status: passed
started: 2026-11-26T23:04:50.163Z
duration_s: 58
session_id: ff2b521e-c3b6-46f4-be77-41065a068ffa
---

# medium-03: email is removed, medical content is kept — Result

## Submit the record ✓ passed (24.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (29.1s)
md5: 326ce6f25999547f0c0ff699a9f47a2f
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text 'naomi.feldstein@example.com'.
