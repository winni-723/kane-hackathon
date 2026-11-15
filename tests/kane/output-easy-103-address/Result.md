---
test: ../easy-103-address_test.md
status: passed
started: 2026-11-15T18:07:26.482Z
duration_s: 91
session_id: 9d7bec02-f9f2-44bf-8813-bd166e6a4232
---

# easy-103: address is removed, medical content is kept — Result

## Submit the record ✓ passed (40.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (45.7s)
md5: 582331d149ffc7f85463fea678dda781
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '2040 Old Mill Pike'.
Verify that the Result region does not contain the text 'Wexley Township'.
Verify that the Result region does not contain the text '43099'.
