---
test: ../hard-02-phone_test.md
status: passed
started: 2026-11-26T22:45:10.812Z
duration_s: 61
session_id: 221d4f03-acd0-4206-a16e-a4d66b6a2c67
---

# hard-02: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (24.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.9s)
md5: 437f79570a52b8a119d00fccc800e2ea
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'area code 617, 555-0156'.
