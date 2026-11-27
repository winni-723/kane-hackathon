---
test: ../easy-102-dob_test.md
status: passed
started: 2026-11-26T22:34:27.910Z
duration_s: 58
session_id: 91ef13b8-fce9-4952-b7b8-46e099cfd9fb
---

# easy-102: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (31.7s)
md5: 1b8332cd878396fc8bfcbb1b371522f4
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '02-19-1990'.
