---
test: ../hard-03-mrn_test.md
status: passed
started: 2026-11-26T22:46:26.977Z
duration_s: 58
session_id: f24a019f-129c-4be6-baab-f9515d6dd6b0
---

# hard-03: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (23.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (31s)
md5: 9f0161f53d990e14289d6e13485dbccc
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text '4471-208-93'.
