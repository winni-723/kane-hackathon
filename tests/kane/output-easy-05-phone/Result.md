---
test: ../easy-05-phone_test.md
status: passed
started: 2026-11-26T22:26:51.893Z
duration_s: 56
session_id: ea9fc263-3664-4d40-b99a-5689e37b553f
---

# easy-05: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (20.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.4s)
md5: fcac82513e1118bf36ad6dc683530088
Verify that the Result region contains the text 'Wheezing and nighttime cough for 3 days'.
Verify that the Result region contains the text 'Mild persistent asthma'.
Verify that the Result region contains the text 'Albuterol inhaler 2 puffs every 4 hours as needed'.
Verify that the Result region contains the text 'Fluticasone 44 mcg twice daily'.
Verify that the Result region contains the text 'SpO2 95% on room air'.
Verify that the Result region contains the text 'respiratory rate 28'.
Verify that the Result region does not contain the text '(713) 555-0176'.
