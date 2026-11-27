---
test: ../hard-06-address_test.md
status: passed
started: 2026-11-26T22:53:11.545Z
duration_s: 55
session_id: c5d06b63-59a5-44c4-b12e-839b4ec81cc7
---

# hard-06: address is removed, medical content is kept — Result

## Submit the record ✓ passed (20s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (30.8s)
md5: 5a819d54bf345abad98f5220c5d1d3cb
Verify that the Result region contains the text 'end-stage COPD'.
Verify that the Result region contains the text '3 L/min home oxygen'.
Verify that the Result region contains the text 'Tiotropium 18 mcg daily'.
Verify that the Result region contains the text 'Albuterol nebulizers every 4 hours'.
Verify that the Result region contains the text 'Hospice referral discussed'.
Verify that the Result region contains the text 'GOLD stage 4'.
Verify that the Result region contains the text 'mMRC dyspnea grade 4'.
Verify that the Result region does not contain the text "room above Holloway's Hardware on the corner of Finch and Lowell".
