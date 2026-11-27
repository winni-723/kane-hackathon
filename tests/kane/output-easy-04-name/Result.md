---
test: ../easy-04-name_test.md
status: passed
started: 2026-11-26T22:24:46.341Z
duration_s: 53
session_id: 3b113661-d13c-4980-8a11-ff3fb805654b
---

# easy-04: name is removed, medical content is kept — Result

## Submit the record ✓ passed (19.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (28.4s)
md5: 78048a1dbc258208cf020f675fafc9ef
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text 'Walter Nakamura'.
