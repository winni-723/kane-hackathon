---
test: ../easy-02-ssn_test.md
status: passed
started: 2026-11-26T22:21:20.599Z
duration_s: 57
session_id: 5f0ecd5f-f83f-4051-bbd4-f2008b67d095
---

# easy-02: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (24.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (28.8s)
md5: 8022e1937dc3b37588051e60ccd05da9
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text '987-65-4321'.
