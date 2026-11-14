---
test: ../hard-06-dob_test.md
status: passed
started: 2026-11-14T04:22:34.230Z
duration_s: 87
session_id: ced86935-12a4-49b0-a4b3-41b047481518
---

# hard-06: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (37.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (41.9s)
md5: 8ca6efd05a5924bbdf048398fcaa4dc7
Verify that the Result region contains the text 'end-stage COPD'.
Verify that the Result region contains the text '3 L/min home oxygen'.
Verify that the Result region contains the text 'Tiotropium 18 mcg daily'.
Verify that the Result region contains the text 'Albuterol nebulizers every 4 hours'.
Verify that the Result region contains the text 'Hospice referral discussed'.
Verify that the Result region contains the text 'GOLD stage 4'.
Verify that the Result region contains the text 'mMRC dyspnea grade 4'.
Verify that the Result region does not contain the text '09-06-48'.
