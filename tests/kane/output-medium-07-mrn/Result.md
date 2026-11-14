---
test: ../medium-07-mrn_test.md
status: passed
started: 2026-11-14T03:51:21.635Z
duration_s: 86
session_id: c6731bb1-af9a-4f3e-9e9d-9d9f5a6036ac
---

# medium-07: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (39.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (41.7s)
md5: b95bad9f8307fc5a2409436c2318f09f
Verify that the Result region contains the text 'laparoscopic cholecystectomy'.
Verify that the Result region contains the text 'symptomatic cholelithiasis'.
Verify that the Result region contains the text 'Amlodipine 5 mg daily'.
Verify that the Result region contains the text 'No known drug allergies'.
Verify that the Result region contains the text 'ASA class II'.
Verify that the Result region contains the text 'nothing by mouth after midnight'.
Verify that the Result region does not contain the text '60917734'.
