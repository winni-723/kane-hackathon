---
test: ../easy-02-dob_test.md
status: passed
started: 2026-11-26T22:19:14.399Z
duration_s: 56
session_id: 1c234fe1-9c31-4b4d-9a69-2f795588ae1d
---

# easy-02: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (21.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (28.8s)
md5: 061fe50b4163c24e4ebffd6db4b98d86
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text '11/02/1971'.
