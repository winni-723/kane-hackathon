---
test: ../hard-03-address_test.md
status: passed
started: 2026-11-14T04:45:15.415Z
duration_s: 84
session_id: 9addc111-087b-47a0-abf4-f32439761311
---

# hard-03: address is removed, medical content is kept — Result

## Submit the record ✓ passed (33.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (45.2s)
md5: 06a0c34dcd10054efe824858f1f30370
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text "blue duplex behind St. Brendan's church".
Verify that the Result region does not contain the text 'Calloway Flats'.
