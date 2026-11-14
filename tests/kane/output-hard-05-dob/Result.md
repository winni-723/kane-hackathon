---
test: ../hard-05-dob_test.md
status: passed
started: 2026-11-14T04:12:44.326Z
duration_s: 86
session_id: 32181c1d-8389-4578-9498-1225133c1503
---

# hard-05: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (33.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (47.6s)
md5: 0e675a430d957d3990706d46b74cae3b
Verify that the Result region contains the text 'Stage 4 chronic kidney disease'.
Verify that the Result region contains the text 'IgA nephropathy'.
Verify that the Result region contains the text 'eGFR is 22 mL/min'.
Verify that the Result region contains the text 'Blood pressure 148/92'.
Verify that the Result region contains the text 'Losartan 100 mg daily'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region contains the text 'arteriovenous fistula evaluation'.
Verify that the Result region contains the text 'potassium is 4.9'.
Verify that the Result region does not contain the text '1962-05-27'.
