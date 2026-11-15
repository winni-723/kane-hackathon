---
test: ../medium-103-ssn_test.md
status: passed
started: 2026-11-15T17:59:29.563Z
duration_s: 58
session_id: 9a4466cf-8447-4f23-9977-db7633c7b0e7
---

# medium-103: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (25.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (28.4s)
md5: eb91eb4665337a958a4ab6f571fd83b3
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text '000-77-1234'.
