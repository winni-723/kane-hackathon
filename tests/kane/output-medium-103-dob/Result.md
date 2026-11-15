---
test: ../medium-103-dob_test.md
status: passed
started: 2026-11-15T17:58:27.100Z
duration_s: 55
session_id: 002f34a7-9b09-4a5a-a889-5175db85781b
---

# medium-103: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (27.5s)
md5: 377d4588820ba84be2e6f32be3951b2d
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text '9/9/1968'.
