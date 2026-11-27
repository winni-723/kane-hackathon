---
test: ../easy-103-insurance_id_test.md
status: passed
started: 2026-11-26T22:37:03.598Z
duration_s: 55
session_id: 365e5abc-162c-4035-a00a-dcb75e1d1933
---

# easy-103: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (20.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (30.8s)
md5: 2ec49244750f31712e010b3b205d0515
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '7731-0092-6'.
