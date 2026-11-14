---
test: ../hard-03-name_test.md
status: passed
started: 2026-11-14T04:45:26.984Z
duration_s: 91
session_id: f7e8d919-ce5d-4f29-9619-83caeac9c500
---

# hard-03: name is removed, medical content is kept — Result

## Submit the record ✓ passed (36.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (48.8s)
md5: 39af1daef13f49460707c3f8a3ed4487
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text 'J. R. Abernathy-Cole'.
Verify that the Result region does not contain the text 'June'.
