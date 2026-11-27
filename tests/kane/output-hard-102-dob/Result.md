---
test: ../hard-102-dob_test.md
status: passed
started: 2026-11-26T22:56:48.514Z
duration_s: 56
session_id: 6350b785-2fe1-474b-8e57-f2e0ab02666e
---

# hard-102: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (24.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (27.4s)
md5: bd2159e98b0731549e805fb86abee9f8
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'tenth of October, 1983'.
