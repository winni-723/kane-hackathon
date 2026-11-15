---
test: ../easy-103-name_test.md
status: passed
started: 2026-11-15T18:09:07.836Z
duration_s: 79
session_id: b0e8d975-c315-4660-869d-a462bfe70266
---

# easy-103: name is removed, medical content is kept — Result

## Submit the record ✓ passed (32s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (41.4s)
md5: b34b35a77e0ac1be388f3eef7e3bbdcf
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text 'Kwame Asante-Boateng'.
