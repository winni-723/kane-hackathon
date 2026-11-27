---
test: ../easy-04-address_test.md
status: passed
started: 2026-11-26T22:23:32.723Z
duration_s: 66
session_id: 49b01cdb-d304-45c0-b2bb-d40c804dd51c
---

# easy-04: address is removed, medical content is kept — Result

## Submit the record ✓ passed (29.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (33s)
md5: c29533f7eca0b76f2e01225b53649e10
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text '9 Thistle Ridge Drive'.
Verify that the Result region does not contain the text 'Corwin Bay'.
Verify that the Result region does not contain the text '98199'.
