---
test: ../easy-102-phone_test.md
status: passed
started: 2026-11-26T22:35:36.777Z
duration_s: 57
session_id: 48205bc1-ce52-4285-805f-4d1699bee5f6
---

# easy-102: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (21.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.4s)
md5: d70f7a9982e3a2bfb7225c0ec55fc9b2
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '(504) 555-0148'.
