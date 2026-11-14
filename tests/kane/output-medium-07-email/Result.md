---
test: ../medium-07-email_test.md
status: passed
started: 2026-11-14T03:53:39.122Z
duration_s: 95
session_id: 3bb506ae-d239-45d5-bdc1-8e8fc2a937de
---

# medium-07: email is removed, medical content is kept — Result

## Submit the record ✓ passed (38.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (51.5s)
md5: 557b867476eef3d38bed88f2a4a62dc2
Verify that the Result region contains the text 'laparoscopic cholecystectomy'.
Verify that the Result region contains the text 'symptomatic cholelithiasis'.
Verify that the Result region contains the text 'Amlodipine 5 mg daily'.
Verify that the Result region contains the text 'No known drug allergies'.
Verify that the Result region contains the text 'ASA class II'.
Verify that the Result region contains the text 'nothing by mouth after midnight'.
Verify that the Result region does not contain the text 's.oyelaran@example.net'.
