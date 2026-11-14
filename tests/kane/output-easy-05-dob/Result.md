---
test: ../easy-05-dob_test.md
status: passed
started: 2026-11-14T04:08:54.073Z
duration_s: 79
session_id: 5f7c62f8-6649-4f2b-8ad3-fc14fc9a53ec
---

# easy-05: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (31.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (42.2s)
md5: 01f80c8c38593df60fc6cae92561e9a7
Verify that the Result region contains the text 'Wheezing and nighttime cough for 3 days'.
Verify that the Result region contains the text 'Mild persistent asthma'.
Verify that the Result region contains the text 'Albuterol inhaler 2 puffs every 4 hours as needed'.
Verify that the Result region contains the text 'Fluticasone 44 mcg twice daily'.
Verify that the Result region contains the text 'SpO2 95% on room air'.
Verify that the Result region contains the text 'respiratory rate 28'.
Verify that the Result region does not contain the text '09/18/2015'.
