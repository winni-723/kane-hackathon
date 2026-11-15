---
test: ../easy-102-mrn_test.md
status: passed
started: 2026-11-15T18:05:49.464Z
duration_s: 88
session_id: 3b0f4b89-1af2-455b-ba7e-4a2656826a95
---

# easy-102: mrn is removed, medical content is kept — Result

## Submit the record ✓ passed (37.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone ✓ passed (41.4s)
md5: b3f96d0a89dd9c0f7c9a35c59285a2ee
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '40077213'.
