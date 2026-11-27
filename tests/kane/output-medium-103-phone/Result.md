---
test: ../medium-103-phone_test.md
status: passed
started: 2026-11-26T23:20:38.402Z
duration_s: 54
session_id: 00c85b3a-442d-4541-ba93-fa69ca10d64a
---

# medium-103: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (23.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (27s)
md5: aa6bcded8ed5663cde4af50028ea7910
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text '(808) 555-0155'.
