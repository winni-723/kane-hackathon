---
test: ../easy-103-insurance_id_test.md
status: passed
started: 2026-11-15T18:08:54.823Z
duration_s: 82
session_id: 73a272a6-69da-48bc-94d5-f401a43e1eb8
---

# easy-103: insurance id is removed, medical content is kept — Result

## Submit the record ✓ passed (35.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted insurance id is gone ✓ passed (40.3s)
md5: 2ec49244750f31712e010b3b205d0515
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '7731-0092-6'.
