---
test: ../easy-103-dob_test.md
status: passed
started: 2026-11-15T18:08:54.599Z
duration_s: 84
session_id: accb613b-b9bc-487b-9f8d-9f84dc34f3c4
---

# easy-103: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (36.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (41.2s)
md5: 1dcac5e0bbed4ac32faf6b0fb9edcf18
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '15 August 1952'.
