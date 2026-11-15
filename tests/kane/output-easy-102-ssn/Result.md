---
test: ../easy-102-ssn_test.md
status: passed
started: 2026-11-15T18:07:22.551Z
duration_s: 82
session_id: d63756b1-05f9-4c64-b9d1-5a8d359fa949
---

# easy-102: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (32.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (44s)
md5: 7d575458cc65c0b60f8e65008277771f
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '900-31-4477'.
