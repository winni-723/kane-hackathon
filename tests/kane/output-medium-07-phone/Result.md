---
test: ../medium-07-phone_test.md
status: passed
started: 2026-11-14T03:31:45.057Z
duration_s: 88
session_id: 2e5acf9c-0997-49aa-87b5-cf7829c7026a
---

# medium-07: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (34.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (47.4s)
md5: 250a887c47863d83fb824fa16c258c70
Verify that the Result region contains the text 'laparoscopic cholecystectomy'.
Verify that the Result region contains the text 'symptomatic cholelithiasis'.
Verify that the Result region contains the text 'Amlodipine 5 mg daily'.
Verify that the Result region contains the text 'No known drug allergies'.
Verify that the Result region contains the text 'ASA class II'.
Verify that the Result region contains the text 'nothing by mouth after midnight'.
Verify that the Result region does not contain the text '(404) 555-0135'.
Verify that the Result region does not contain the text '404-555-0172'.
