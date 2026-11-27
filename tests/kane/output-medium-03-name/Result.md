---
test: ../medium-03-name_test.md
status: passed
started: 2026-11-26T23:05:48.513Z
duration_s: 64
session_id: aa7eca77-9119-462b-8672-9cac38072374
---

# medium-03: name is removed, medical content is kept — Result

## Submit the record ✓ passed (24.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (36s)
md5: 5023f0091416297477092fcbd0473182
Verify that the Result region contains the text 'LDL cholesterol is 172 mg/dL'.
Verify that the Result region contains the text 'HDL is 38 mg/dL'.
Verify that the Result region contains the text 'Atorvastatin 20 mg nightly'.
Verify that the Result region contains the text 'Mediterranean diet'.
Verify that the Result region contains the text '150 minutes of exercise per week'.
Verify that the Result region contains the text 'early coronary artery disease'.
Verify that the Result region does not contain the text 'Naomi Feldstein'.
Verify that the Result region does not contain the text 'Naomi'.
