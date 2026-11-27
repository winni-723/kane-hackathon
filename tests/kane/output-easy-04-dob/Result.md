---
test: ../easy-04-dob_test.md
status: passed
started: 2026-11-26T22:23:42.668Z
duration_s: 55
session_id: 4782d287-1dcd-463d-a78d-8caffd4c16c9
---

# easy-04: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (28s)
md5: 0581c7a789ab67310f7d40e60ec02042
Verify that the Result region contains the text 'Resting tremor of the right hand'.
Verify that the Result region contains the text "Parkinson's disease".
Verify that the Result region contains the text 'Hoehn and Yahr stage 2'.
Verify that the Result region contains the text 'Carbidopa-Levodopa 25-100 mg three times daily'.
Verify that the Result region contains the text 'fall precautions'.
Verify that the Result region does not contain the text '01/05/1946'.
