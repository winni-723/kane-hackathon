---
test: ../medium-07-dob_test.md
status: passed
started: 2026-11-26T23:12:42.169Z
duration_s: 56
session_id: f60f5c59-290f-43d9-bb63-da023036995a
---

# medium-07: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.2s)
md5: c2208638c7a08f856bfcb0259e25dd5c
Verify that the Result region contains the text 'laparoscopic cholecystectomy'.
Verify that the Result region contains the text 'symptomatic cholelithiasis'.
Verify that the Result region contains the text 'Amlodipine 5 mg daily'.
Verify that the Result region contains the text 'No known drug allergies'.
Verify that the Result region contains the text 'ASA class II'.
Verify that the Result region contains the text 'nothing by mouth after midnight'.
Verify that the Result region does not contain the text '3/8/1967'.
