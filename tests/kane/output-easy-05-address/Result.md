---
test: ../easy-05-address_test.md
status: passed
started: 2026-11-14T04:07:46.211Z
duration_s: 90
session_id: defc740a-af66-4150-9cab-5736fb45adef
---

# easy-05: address is removed, medical content is kept — Result

## Submit the record ✓ passed (35.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (47.6s)
md5: a1ccc2abc15aca892774776cb8504ee0
Verify that the Result region contains the text 'Wheezing and nighttime cough for 3 days'.
Verify that the Result region contains the text 'Mild persistent asthma'.
Verify that the Result region contains the text 'Albuterol inhaler 2 puffs every 4 hours as needed'.
Verify that the Result region contains the text 'Fluticasone 44 mcg twice daily'.
Verify that the Result region contains the text 'SpO2 95% on room air'.
Verify that the Result region contains the text 'respiratory rate 28'.
Verify that the Result region does not contain the text '3305 Pecan Grove Street'.
Verify that the Result region does not contain the text 'Marlow Springs'.
Verify that the Result region does not contain the text '77099'.
