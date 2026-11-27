---
test: ../hard-102-phone_test.md
status: passed
started: 2026-11-26T22:57:40.059Z
duration_s: 51
session_id: c5b47619-07ff-4623-ad6b-0e4c7a449d0b
---

# hard-102: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (21.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (25.4s)
md5: 9fe68513445d479a41f8e68ef21baf4c
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'five oh three, 555, zero one seven five'.
