---
test: ../easy-103-dob_test.md
status: passed
started: 2026-11-26T22:36:41.906Z
duration_s: 60
session_id: 2deb373a-24ba-4632-962a-0ec1d0fc9e42
---

# easy-103: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (25.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.2s)
md5: 1dcac5e0bbed4ac32faf6b0fb9edcf18
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '15 August 1952'.
