---
test: ../easy-05-phone_test.md
status: passed
started: 2026-11-14T03:26:53.826Z
duration_s: 94
session_id: 33fca171-789c-431c-b082-53be2f8a95bb
---

# easy-05: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (44.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (41.6s)
md5: fcac82513e1118bf36ad6dc683530088
Verify that the Result region contains the text 'Wheezing and nighttime cough for 3 days'.
Verify that the Result region contains the text 'Mild persistent asthma'.
Verify that the Result region contains the text 'Albuterol inhaler 2 puffs every 4 hours as needed'.
Verify that the Result region contains the text 'Fluticasone 44 mcg twice daily'.
Verify that the Result region contains the text 'SpO2 95% on room air'.
Verify that the Result region contains the text 'respiratory rate 28'.
Verify that the Result region does not contain the text '(713) 555-0176'.
