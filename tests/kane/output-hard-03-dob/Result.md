---
test: ../hard-03-dob_test.md
status: passed
started: 2026-11-26T22:46:21.398Z
duration_s: 57
session_id: 120b3b90-5feb-404a-9327-edc9112d6279
---

# hard-03: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.8s)
md5: e1a2496fd382a5bc06cb91e16dc92d8d
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text '22 Nov 1991'.
