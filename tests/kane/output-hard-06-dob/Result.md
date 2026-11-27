---
test: ../hard-06-dob_test.md
status: passed
started: 2026-11-26T22:53:13.584Z
duration_s: 58
session_id: 9efa1b05-f782-4518-ac8f-f6cfb2d6ae17
---

# hard-06: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.9s)
md5: 8ca6efd05a5924bbdf048398fcaa4dc7
Verify that the Result region contains the text 'end-stage COPD'.
Verify that the Result region contains the text '3 L/min home oxygen'.
Verify that the Result region contains the text 'Tiotropium 18 mcg daily'.
Verify that the Result region contains the text 'Albuterol nebulizers every 4 hours'.
Verify that the Result region contains the text 'Hospice referral discussed'.
Verify that the Result region contains the text 'GOLD stage 4'.
Verify that the Result region contains the text 'mMRC dyspnea grade 4'.
Verify that the Result region does not contain the text '09-06-48'.
