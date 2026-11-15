---
test: ../hard-102-dob_test.md
status: passed
started: 2026-11-15T17:50:09.833Z
duration_s: 57
session_id: c97cc15b-10d8-47a2-941d-c8fe6903814c
---

# hard-102: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (29.6s)
md5: bd2159e98b0731549e805fb86abee9f8
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'tenth of October, 1983'.
