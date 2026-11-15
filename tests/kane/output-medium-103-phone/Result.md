---
test: ../medium-103-phone_test.md
status: passed
started: 2026-11-15T17:59:25.141Z
duration_s: 55
session_id: e8f3b514-ad6d-4420-9825-10f2a89cdbb9
---

# medium-103: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (23.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (26.8s)
md5: aa6bcded8ed5663cde4af50028ea7910
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text '(808) 555-0155'.
