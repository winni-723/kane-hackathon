---
test: ../easy-102-name_test.md
status: passed
started: 2026-11-15T18:05:49.759Z
duration_s: 83
session_id: 1c7b088b-868f-4b91-b587-d8effbeb2ebd
---

# easy-102: name is removed, medical content is kept — Result

## Submit the record ✓ passed (35s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (40.6s)
md5: da16535606f98e48515c058a4573d9db
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text 'Delphine Moreau'.
