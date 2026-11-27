---
test: ../easy-103-phone_test.md
status: passed
started: 2026-11-26T22:37:59.724Z
duration_s: 60
session_id: 30f301e7-5a6d-420c-a488-9db9658247da
---

# easy-103: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (21.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (34.6s)
md5: 0c2a4659ca7f9c98c19dae2fdb8c3e92
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '614 555 0193'.
