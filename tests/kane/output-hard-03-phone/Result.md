---
test: ../hard-03-phone_test.md
status: passed
started: 2026-11-26T22:47:27.065Z
duration_s: 60
session_id: a933cd67-beb1-47a7-ae51-2c3fb37a26c4
---

# hard-03: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (25.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31s)
md5: 32f87fb549888fccc46e237f8571999f
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text '5 5 5 - 0 1 0 9'.
